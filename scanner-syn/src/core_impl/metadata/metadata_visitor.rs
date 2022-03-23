//! We use `#![metadata]` attribute to generate metadata from the methods in the module
//! it decorates. Note, that this in an inner attribute. For it to work we should be
//! able to visit every method in the module intended to be a contract method.
//! For this we implement the visitor.
use crate::contract_descriptor::FunctionInfo;
use crate::{ItemFnInfo, ItemImplInfo};


use quote::ToTokens;
use syn::visit::Visit;
use syn::{Error, ItemFn, ItemImpl};

use super::metadata_generator::metadata_fn_struct;

/// Information relevant to metadata extracted from the `impl` section decorated with `#[near_bindgen]`.
#[derive(Default)]
pub struct MetadataVisitor {
    impl_item_infos: Vec<ItemImplInfo>,
    fn_items_infos: Vec<ItemFnInfo>,
    /// Errors that occurred while extracting the data.
    errors: Vec<Error>,
}

impl<'ast> Visit<'ast> for MetadataVisitor {
    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        let has_near_sdk_attr = i
            .attrs
            .iter()
            .any(|attr| attr.path.to_token_stream().to_string().as_str() == "near_bindgen");
        // if has_near_sdk_attr {
        //     match ItemImplInfo::new(&mut i.clone()) {
        //         Ok(info) => self.impl_item_infos.push(info),
        //         Err(err) => self.errors.push(err),
        //     }
        // }
        match ItemImplInfo::new(&mut i.clone(), has_near_sdk_attr) {
            Ok(info) => self.impl_item_infos.push(info),
            Err(err) => self.errors.push(err),
        }
        syn::visit::visit_item_impl(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        match ItemFnInfo::new(&mut i.clone()) {
            Ok(info) => self.fn_items_infos.push(info),
            Err(err) => self.errors.push(err),
        }
        syn::visit::visit_item_fn(self, i);
    }
}

impl MetadataVisitor {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn generate_metadata_method(&self) -> syn::Result<Vec<FunctionInfo>> {
        if !self.errors.is_empty() {
            return Err(self.errors[0].clone());
        }
        let mut methods: Vec<FunctionInfo> = self
            .impl_item_infos
            .iter()
            .flat_map(|i| {
                (i.methods)
                    .iter()
                    .map(move |m| m.metadata_struct(i.is_trait_impl, i.has_near_sdk_attr))
            })
            .collect();
        let functions: Vec<FunctionInfo> = self
            .fn_items_infos
            .iter()
            .map(|s| metadata_fn_struct(&s.attr_signature_info))
            .collect();
        methods.extend(functions);
        Ok(methods)
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use quote::quote;
    use super::*;

    #[test]
    fn several_methods() {
        let code = quote! {
            #[near_bindgen]
            impl Hello {
                pub fn f1(&self) { }
                pub fn f2(&mut self, arg0: FancyStruct, arg1: u64) { }
            }

            #[near_bindgen]
            impl SomeTrait for Hello {
                fn f3(&mut self, arg0: FancyStruct, arg1: u64) -> Result<IsOk, Error> { }
            }
        };

        let file: syn::File = syn::parse2(code).unwrap();

        let mut visitor = MetadataVisitor::new();
        visitor.visit_file(&file);

        let actual = visitor.generate_metadata_method().unwrap();
        let expected = quote!(
            #[cfg(target_arch = "wasm32")]
            #[no_mangle]
            pub extern "C" fn metadata() {
                near_sdk::env::setup_panic_hook();
                use borsh::*;
                let metadata = near_sdk::Metadata::new(vec![
                    near_sdk::MethodMetadata {
                        name: "f1".to_string(),
                        is_view: true,
                        is_init: false,
                        args: None,
                        callbacks: vec![],
                        callbacks_vec: None,
                        result: None
                    },
                    near_sdk::MethodMetadata {
                        name: "f2".to_string(),
                        is_view: false,
                        is_init: false,
                        args: {
                            #[derive(borsh::BorshSchema)]
                            #[allow(dead_code)]
                            #[derive(near_sdk :: serde :: Deserialize)]
                            #[serde(crate = "near_sdk::serde")]
                            struct Input {
                                arg0: FancyStruct,
                                arg1: u64,
                            }
                            Some(Input::schema_container())
                        },
                        callbacks: vec![],
                        callbacks_vec: None,
                        result: None
                    },
                    near_sdk::MethodMetadata {
                        name: "f3".to_string(),
                        is_view: false,
                        is_init: false,
                        args: {
                            #[derive(borsh::BorshSchema)]
                            #[allow(dead_code)]
                            #[derive(near_sdk :: serde :: Deserialize)]
                            #[serde(crate = "near_sdk::serde")]
                            struct Input {
                                arg0: FancyStruct,
                                arg1: u64,
                            }
                            Some(Input::schema_container())
                        },
                        callbacks: vec![],
                        callbacks_vec: None,
                        result: Some(Result < IsOk, Error > ::schema_container())
                    }
                ]);
                let data = near_sdk::borsh::BorshSerialize::try_to_vec(&metadata)
                    .expect("Failed to serialize the metadata using Borsh");
                near_sdk::env::value_return(&data);
            }
        );
        assert_eq!(expected.to_string(), 
        quote! {
            #(#actual),*
        }.to_string());
    }
}
