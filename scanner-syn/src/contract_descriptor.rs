use proc_macro2::Span;
use std::io::Read;
use std::{fs::File, path::Path};

use crate::core_impl::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::__private::ToTokens;
use syn::visit::Visit;
use walkdir::WalkDir;

#[derive(Default, Debug)]
pub struct FunctionInfo {
    pub name: String,
    /// Whether method is exported
    pub is_public: bool,
    /// Whether this is a trait implementation.
    pub is_trait_impl: bool,
    /// Whether method does not modify the state.
    pub is_init: bool,
    /// Whether method accepting $NEAR.
    pub is_payable: bool,
    /// Whether method does not modify the state.
    pub is_view: bool,
    /// Whether method can modify the state.
    pub is_mutable: bool,
    /// Whether method doesn't return a value.
    pub is_process: bool,
    /// Whether method can accept calls from self (current account)
    pub is_private_cccalls: bool,
    /// Whether `impl` section decorated with `#[near_bindgen]`
    pub is_out_of_contract_scope: bool,
    /// Whether method is part of `NearEvent` trait
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
        let is_payable: bool = self.is_payable;
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
                is_payable: #is_payable,
                is_view: #is_view,
                is_mutable: #is_mutable,
                is_process: #is_process,
                is_private_cccalls: #is_private_cccalls,
                is_out_of_contract_scope: #is_out_of_contract_scope,
                is_event: #is_event
            }
        });
    }

    ///Converts to TokenStream
    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }

    ///gets the token stream
    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_token_stream()
    }
}
///Trait Near smart contracts descriptor
pub trait ContractDescriptor {
    fn get_contract_info_for_crate(&self) -> ContractInfo;
    fn get_tokens_from_file_path(&self, file_path: &Path) -> MetadataInfo;
    fn get_tokens_from_source(&self, src: String) -> MetadataInfo;
}

pub struct MetadataInfo {
    functions_info: Vec<FunctionInfo>,
    connections_info: Vec<TokenStream>,
}

pub struct DefaultContractDescriptor;

impl DefaultContractDescriptor {
    pub fn new() -> Self {
        Self {}
    }
    fn metadata(&self, item: proc_macro2::TokenStream) -> syn::Result<MetadataInfo> {
        if let Ok(input) = syn::parse2::<syn::File>(item) {
            let mut visitor = MetadataVisitor::new();
            visitor.visit_file(&input);
            let connections_info = visitor.get_connections();
            visitor
                .generate_metadata_method()
                .map(|functions_info| MetadataInfo {
                    functions_info,
                    connections_info,
                })
        //     let generated = match visitor.generate_metadata_method() {
        //         Ok(x) => x,
        //         Err(err) => return err.to_compile_error(),
        //     };
        //     quote! {
        //         //#input
        //         #generated
        //     }
        } else {
            syn::__private::Err(syn::Error::new(
                Span::call_site(),
                "Failed to parse code decorated with `metadata!{}` macro. Only valid Rust is supported.",
            ))
        }
    }
}

impl ContractDescriptor for DefaultContractDescriptor {
    fn get_contract_info_for_crate(&self) -> ContractInfo {
        let mut contract_functions = vec![];
        // Walk into every dir to find every `rs` file
        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().map(|s| s == "rs").unwrap_or(false) {
                println!("\n{}", entry.path().display());
                let MetadataInfo {
                    functions_info,
                    connections_info,
                } = self.get_tokens_from_file_path(entry.path());
                println!("\n{:?}", functions_info);
                contract_functions.extend(functions_info);
                println!(
                    "{}",
                    quote! {
                        #(#connections_info);*
                    }
                )
            }
        }
        ContractInfo {
            functions: contract_functions,
        }
    }

    fn get_tokens_from_file_path(&self, file_path: &Path) -> MetadataInfo {
        let mut file = File::open(file_path).expect("Unable to open file");
        let mut src = String::new();
        file.read_to_string(&mut src).expect("Unable to read file");
        self.get_tokens_from_source(src)
    }

    fn get_tokens_from_source(&self, src: String) -> MetadataInfo {
        let syntax = syn::parse_file(&src).expect("Unable to parse file");
        self.metadata(syntax.to_token_stream()).unwrap()
    }
}
