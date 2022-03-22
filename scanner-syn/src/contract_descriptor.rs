use proc_macro2::Span;
use std::{fs::File, path::Path};

use crate::core_impl::*;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::__private::ToTokens;
use syn::visit::Visit;

#[derive(Default)]
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
    pub functions: Vec<FunctionInfo>,
}

impl ToTokens for FunctionInfo {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name: &str = &self.name;
        let is_public: bool = self.is_public;
        let is_trait_impl: bool = self.is_trait_impl;
        let is_init: bool = self.is_init;
        let is_view: bool = self.is_view;
        let is_mutable: bool = self.is_mutable;
        let is_process: bool = self.is_process;
        let is_private_cccalls: bool = self.is_private_cccalls;
        let is_out_of_contract_scope: bool = self.is_out_of_contract_scope;
        let is_event: bool = self.is_event;
        tokens.extend(quote! {
            FunctionInfo {
                name: #name,
                is_public: #is_public,
                is_trait_impl: #is_trait_impl,
                is_init: #is_init,
                is_view: #is_view,
                is_mutable: #is_mutable,
                is_process: #is_process,
                is_private_cccalls: #is_private_cccalls,
                is_out_of_contract_scope: #is_out_of_contract_scope,
                is_event: #is_event
            }
        });
    }

    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }

    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_token_stream()
    }
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
