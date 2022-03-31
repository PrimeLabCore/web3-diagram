use proc_macro2::{Ident, Span};
use std::io::Read;
use std::iter::IntoIterator;
use std::ops::Deref;
use std::{fs::File, path::Path};
use syn::{Item, ItemStruct};

use crate::core_impl::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::__private::ToTokens;
use syn::visit::Visit;
use walkdir::WalkDir;

///Function information from the code scanned by ContractDescriptor
#[derive(Clone, Default, Debug)]
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
    ///functions are being called by this function
    pub inner_calls: Option<Vec<FunctionInfo>>,
}
///Contract information from the code scanned by ContractDescriptor
pub struct ContractInfo {
    pub contract_metadata: Vec<ContractDescriptorMeta>,
}
#[derive(Debug)]
pub struct ContractDescriptorMeta {
    pub fns: Vec<FunctionInfo>,
    pub connections: Option<Vec<FunctionInfo>>,
    pub tokens: Option<TokenStream>,
}

///Trait for converting tokenstream to extended one
impl ToTokens for FunctionInfo {
    ///Function extends TokenStream with FunctionInfo
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
///Trait near smart contracts descriptor
pub trait ContractDescriptor {
    ///Gets the contract information inside the current crate
    fn get_contract_info_for_crate(&self) -> ContractInfo;
    fn get_tokens_from_file_path(&self, file_path: &Path) -> ContractDescriptorMeta;
    fn get_tokens_from_source(&self, src: String) -> ContractDescriptorMeta;
}

///Default Near contract descriptor
pub struct DefaultContractDescriptor;

///Implementation of Near contract descriptor
impl DefaultContractDescriptor {
    pub fn new() -> Self {
        Self {}
    }
    fn get_inner_calls(
        &self,
        fn_name: String,
        connections: Vec<FunctionInfo>,
        fns: Vec<FunctionInfo>,
    ) -> Option<Vec<FunctionInfo>> {
        let con_info = connections
            .into_iter()
            .find(|el| fn_name == el.name)
            .unwrap();
        let mut fn_iter = fns.into_iter();

        let inner_calls = con_info
            .inner_calls
            .unwrap()
            .into_iter()
            .filter_map(|ic| -> Option<FunctionInfo> {
                fn_iter.find(|f| f.name == ic.name && !f.is_payable && !f.is_init)
            })
            .collect::<Vec<_>>();

        if inner_calls.len() > 0 {
            Some(inner_calls)
        } else {
            None
        }
    }
    fn resolve_call_hierarchy(
        &self,
        metadata: ContractDescriptorMeta,
        fns: Vec<FunctionInfo>,
    ) -> ContractDescriptorMeta {
        let iiter = fns;
        let connections = metadata.connections.unwrap();
        //print!("{:?}",connections);
        let result = metadata
            .fns
            .iter()
            .map(|f_info| {
                FunctionInfo {
                    inner_calls: self.get_inner_calls(
                        f_info.name.clone(),
                        connections.clone(),
                        iiter.clone(),
                    ),
                    ..f_info.clone()
                }
            })
            .collect::<Vec<FunctionInfo>>();

        ContractDescriptorMeta {
            fns: result,
            connections: None,
            tokens: None,
        }
    }

    fn metadata(&self, item: proc_macro2::TokenStream) -> syn::Result<ContractDescriptorMeta> {
        if let Ok(input) = syn::parse2::<syn::File>(item) {
            let mut visitor = MetadataVisitor::new();
            visitor.visit_file(&input);
            let connections = visitor.get_connections();
            let fns = visitor.generate_metadata_method().unwrap();
            syn::Result::Ok(ContractDescriptorMeta {
                fns,
                connections: Some(connections),
                tokens: None,
            })
        } else {
            syn::__private::Err(syn::Error::new(
                Span::call_site(),
                "Failed to parse code decorated with `metadataa!{}` macro. Only valid Rust is supported.",
            ))
        }
    }
}

///Implement contract descriptor trait for DefaultContractDescriptor
impl ContractDescriptor for DefaultContractDescriptor {
    fn get_contract_info_for_crate(&self) -> ContractInfo {
        let mut contract_metadata: Vec<ContractDescriptorMeta> = vec![];
        let mut fns: Vec<FunctionInfo> = vec![];
        // Walk into every dir to find every `rs` file
        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().map(|s| s == "rs").unwrap_or(false) {
                println!("\n{}", entry.path().display());
                let metadata = self.get_tokens_from_file_path(entry.path());
                //println!("\n{:?}", metadata.connections);
                let scoped_fns = metadata.fns.clone();
                fns.extend(scoped_fns);
                contract_metadata.push(metadata);
            }
        }

        let resolved = contract_metadata
            .into_iter()
            .map(|m| self.resolve_call_hierarchy(m, fns.clone()))
            .collect();

        println!("\n{:?}", resolved);
        ContractInfo {
            contract_metadata: resolved,
        }
    }

    fn get_tokens_from_file_path(&self, file_path: &Path) -> ContractDescriptorMeta {
        let mut file = File::open(file_path).expect("Unable to open file");
        let mut src = String::new();
        file.read_to_string(&mut src).expect("Unable to read file");
        self.get_tokens_from_source(src)
    }

    fn get_tokens_from_source(&self, src: String) -> ContractDescriptorMeta {
        let syntax = syn::parse_file(&src).expect("Unable to parse file");
        self.metadata(syntax.to_token_stream()).unwrap()
    }
}
