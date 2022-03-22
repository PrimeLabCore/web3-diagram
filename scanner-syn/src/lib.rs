#![recursion_limit = "128"]
extern crate proc_macro;

mod core_impl;

use std::io::Read;

use proc_macro2::TokenStream;
use walkdir::WalkDir;

use self::core_impl::*;
use proc_macro2::Span;
use quote::quote;
use syn::visit::Visit;
use syn::{File, ItemEnum, ItemImpl, ItemStruct, ItemTrait};
use syn::__private::ToTokens;
mod contract_descriptor;

use contract_descriptor::{FunctionInfo};

/// `metadata` generates the metadata method and should be placed at the very end of the `lib.rs` file.
// TODO: Once Rust allows inner attributes and custom procedural macros for modules we should switch this
// to be `#![metadata]` attribute at the top of the contract file instead. https://github.com/rust-lang/rust/issues/54727
pub fn metadata(item: proc_macro2::TokenStream) -> TokenStream {
    if let Ok(input) = syn::parse2::<File>(item) {
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

pub fn display_metadata() {
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().map(|s| s == "rs").unwrap_or(false) {
            let mut file = std::fs::File::open(&entry.path()).expect("Unable to open file");
            let mut src = String::new();
            file.read_to_string(&mut src).expect("Unable to read file");
            let syntax = syn::parse_file(&src).expect("Unable to parse file");
            let tokens = metadata(syntax.to_token_stream());
            println!("\n{}:\n{}", entry.path().display(), tokens);
        }
    }
}