use proc_macro2::{Ident, Span};
use std::iter::IntoIterator;
use std::io::Read;
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
#[derive(Clone,Default, Debug)]
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
    pub functions: Vec<FunctionInfo>,
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
    fn get_tokens_from_file_path(&self, file_path: &Path) -> Vec<FunctionInfo>;
    fn get_tokens_from_source(&self, src: String) -> Vec<FunctionInfo>;
}

///Default Near contract descriptor
pub struct DefaultContractDescriptor;

///Implementation of Near contract descriptor
impl DefaultContractDescriptor {
    pub fn new() -> Self {
        Self {}
    }
    fn get_inner_calls(&self,fn_name:String,connections: Vec<FunctionInfo>,  fn_iter:&mut impl Iterator<Item=FunctionInfo>)
    -> Vec<FunctionInfo> {
        let fn_info=connections
            .into_iter()
            .find(|el| fn_name == el.name)
            .unwrap();
        let inner_calls = 
            
            fn_info
            .inner_calls
            .unwrap()
            .into_iter()
            .filter_map(|ic| -> Option<FunctionInfo> {
                println!("{:?}",ic.name);
               let ofn= fn_iter.find(|f| f.name == ic.name);
               if ofn.is_some(){
                                  println!("Found");

                   return Some(ofn.unwrap().clone())
               }
               println!("Not found");
               None
            })
            .collect::<Vec<_>>();

        inner_calls
    }
    fn metadata(&self, item: proc_macro2::TokenStream) -> syn::Result<Vec<FunctionInfo>> {
        if let Ok(input) = syn::parse2::<syn::File>(item) {
            let mut visitor = MetadataVisitor::new();
            visitor.visit_file(&input);
            let connections = visitor.get_connections();

            // println!(
            //     "\n{}",
            //     quote! {
            //         #(#connections_info);*
            //     }
            // );
            let fns = visitor.generate_metadata_method().unwrap();
            let mut iiter=fns.iter().cloned();
            println!("{:?}", iiter);
            let result =fns.iter()
                .map(|f_info| {
                    let mut minfo = FunctionInfo {
                        inner_calls:None,
                        ..f_info.clone()
                    };

                    minfo.inner_calls= Some(self.get_inner_calls(minfo.name.clone(), connections.clone(), &mut iiter));
                   // println!("{:?}", minfo.name);

                   // println!("{:?}", minfo.inner_calls);
                    minfo
                     
                })
                .collect::<Vec<FunctionInfo>>();

            syn::Result::Ok(result.to_vec())
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
                "Failed to parse code decorated with `metadataa!{}` macro. Only valid Rust is supported.",
            ))
        }
    }
}

///Implement contract descriptor trait for DefaultContractDescriptor
impl ContractDescriptor for DefaultContractDescriptor {
    fn get_contract_info_for_crate(&self) -> ContractInfo {
        let mut contract_functions = vec![];
        // Walk into every dir to find every `rs` file
        for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().map(|s| s == "rs").unwrap_or(false) {
                println!("\n{}", entry.path().display());
                let functions = self.get_tokens_from_file_path(entry.path());
                //println!("\n{:?}", functions);
                contract_functions.extend(functions);
            }
        }

        ContractInfo {
            functions: contract_functions,
        }
    }

    fn get_tokens_from_file_path(&self, file_path: &Path) -> Vec<FunctionInfo> {
        let mut file = File::open(file_path).expect("Unable to open file");
        let mut src = String::new();
        file.read_to_string(&mut src).expect("Unable to read file");
        self.get_tokens_from_source(src)
    }

    fn get_tokens_from_source(&self, src: String) -> Vec<FunctionInfo> {
        let syntax = syn::parse_file(&src).expect("Unable to parse file");
        self.metadata(syntax.to_token_stream()).unwrap()
    }
}
