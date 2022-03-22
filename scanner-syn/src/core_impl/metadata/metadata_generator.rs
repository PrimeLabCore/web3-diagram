use crate::{
    contract_descriptor::FunctionInfo,
    core_impl::{metadata::type_is_event, AttrSigInfo},
    BindgenArgType, ImplItemMethodInfo, InputStructType, MethodType, SerializerType,
};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::ReturnType;

impl ImplItemMethodInfo {
    /// Generates metadata struct for this method.
    ///
    /// # Example:
    /// The following method:
    /// ```ignore
    /// fn f3(&mut self, arg0: FancyStruct, arg1: u64) -> Result<IsOk, Error> { }
    /// ```
    /// will produce this struct:
    /// ```ignore
    /// near_sdk::MethodMetadata {
    ///     name: "f3".to_string(),
    ///     is_view: false,
    ///     is_init: false,
    ///     args: {
    ///         #[derive(borsh::BorshSchema)]
    ///         #[derive(serde :: Deserialize, serde :: Serialize)]
    ///         struct Input {
    ///             arg0: FancyStruct,
    ///             arg1: u64,
    ///         }
    ///         Some(Input::schema_container())
    ///     },
    ///     callbacks: vec![],
    ///     callbacks_vec: None,
    ///     result: Some(Result < IsOk, Error > ::schema_container())
    /// }
    /// ```
    /// If args are serialized with Borsh it will not include `#[derive(borsh::BorshSchema)]`.
    pub fn metadata_struct(&self, is_trait_impl: bool, has_near_sdk_attr: bool) -> TokenStream2 {
        let method_name_str = self.attr_signature_info.ident.to_string();

        let is_event = type_is_event(&self.struct_type);
        if !is_event && !has_near_sdk_attr {
            let function_info = FunctionInfo {
                name: method_name_str,
                is_out_of_contract_scope: true,
                ..Default::default()
            };
            return quote! {
                #function_info
            };
        }
        let is_view = matches!(&self.attr_signature_info.method_type, &MethodType::View);
        let is_public = self.is_public || (is_trait_impl && has_near_sdk_attr);
        let is_payable = self.attr_signature_info.is_payable;
        let is_private_cccalls = self.attr_signature_info.is_private;
        let is_init = matches!(
            &self.attr_signature_info.method_type,
            &MethodType::Init | &MethodType::InitIgnoreState
        );
        let mut is_mutable = false;
        let receiver = &self.attr_signature_info.receiver;

        if let Some(receiver) = receiver {
            is_mutable = !(receiver.mutability.is_none() || receiver.reference.is_none());
        }
        let _args = if self.attr_signature_info.input_args().next().is_some() {
            let input_struct = self
                .attr_signature_info
                .input_struct(InputStructType::Deserialization);
            // If input args are JSON then we need to additionally specify schema for them.
            let additional_schema = match &self.attr_signature_info.input_serializer {
                SerializerType::Borsh => TokenStream2::new(),
                SerializerType::JSON => quote! {
                    #[derive(borsh::BorshSchema)]
                },
            };

            quote! {
                {
                    #additional_schema
                    #[allow(dead_code)]
                    #input_struct
                    Some(Input::schema_container())
                }
            }
        } else {
            quote! {
                 None
            }
        };
        let callbacks: Vec<_> = self
            .attr_signature_info
            .args
            .iter()
            .filter(|arg| matches!(arg.bindgen_ty, BindgenArgType::CallbackArg))
            .map(|arg| {
                let ty = &arg.ty;
                quote! {
                    #ty::schema_container()
                }
            })
            .collect();
        let callbacks_vec = match self
            .attr_signature_info
            .args
            .iter()
            .filter(|arg| matches!(arg.bindgen_ty, BindgenArgType::CallbackArgVec))
            .last()
        {
            None => {
                quote! {
                    None
                }
            }
            Some(arg) => {
                let ty = &arg.ty;
                quote! {
                    Some(#ty::schema_container())
                }
            }
        };
        let result = match &self.attr_signature_info.returns {
            ReturnType::Default => {
                quote! {
                    None
                }
            }
            ReturnType::Type(_, ty) => {
                quote! {
                    Some(#ty::schema_container())
                }
            }
        };
        
        let function_info = FunctionInfo{
            name: method_name_str,
            is_public,
            is_trait_impl,
            is_init,
            is_payable,
            is_view,
            is_mutable,
            is_process: false,
            is_private_cccalls,
            is_out_of_contract_scope: false,
            is_event,
        };
        quote! {
            #function_info
        }
    }
}

pub fn metadata_fn_struct(sig_info: &AttrSigInfo) -> TokenStream2 {
    let method_name_str = sig_info.ident.to_string();
    let function_info = FunctionInfo {
        name: method_name_str,
        is_process: true,
        is_out_of_contract_scope: true,
        ..Default::default()
    };
    quote! {
        #function_info
    }
}
