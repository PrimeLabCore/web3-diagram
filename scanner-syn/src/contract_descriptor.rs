use proc_macro2::Span;
use std::{fs::File, path::Path};

use crate::core_impl::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::__private::ToTokens;
use syn::visit::Visit;

pub struct FunctionInfo {
    pub name: String,
    pub is_public: bool,
    pub is_trait_impl: bool,
    pub is_init: bool,
    pub is_view: bool,
    pub is_mutable: bool,
    pub is_process: bool,
    pub is_private_cccalls: bool,
    pub is_out_of_contract_scope: bool,
    pub is_event: bool,
}
pub struct ContractInfo {
    pub crate_name: String,
    pub functions: Vec<FunctionInfo>,
}

pub trait ContractDescriptor {
    fn get_contract_info_for_crate(&mut self) -> ContractInfo;
    fn get_contract_info_from_file(&mut self, file_path: File) -> ContractInfo;
    fn get_contract_info_from_file_path(&mut self, file_path: &Path) -> ContractInfo;
    fn get_contract_info_from_source(&mut self, src: String) -> ContractInfo;
}

pub struct DefaultContractDescriptor;

impl DefaultContractDescriptor {
    fn metadata(&mut self, item: proc_macro2::TokenStream) -> TokenStream {
        if let Ok(input) = syn::parse2::<syn::File>(item) {
            let mut visitor = MetadataVisitor::new();
            visitor.visit_file(&input);
            let generated = match visitor.generate_metadata_method() {
                Ok(x) => x,
                Err(err) => return err.to_compile_error(),
            };
            quote! {
                //#input
                #generated
            }
        } else {
            syn::Error::new(
                Span::call_site(),
                "Failed to parse code decorated with `metadata!{}` macro. Only valid Rust is supported.",
            )
            .to_compile_error()
        }
    }
}

impl ContractDescriptor for DefaultContractDescriptor {
    fn get_contract_info_for_crate(&mut self) -> ContractInfo {
        todo!();
    }

    fn get_contract_info_from_file(&mut self, file: File) -> ContractInfo {
        todo!()
    }

    fn get_contract_info_from_file_path(&mut self, file_path: &Path) -> ContractInfo {
        todo!()
    }

    fn get_contract_info_from_source(&mut self, src: String) -> ContractInfo {
        let syntax = syn::parse_file(&src).expect("Unable to parse file");
        let tokens = self.metadata(syntax.to_token_stream());
        todo!()
    }
}
