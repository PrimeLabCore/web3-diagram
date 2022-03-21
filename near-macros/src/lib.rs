#![recursion_limit = "128"]
extern crate proc_macro;

mod core_impl;

use proc_macro2::TokenStream;

use self::core_impl::*;
use proc_macro2::Span;
use quote::quote;
use syn::visit::Visit;
use syn::{File, ItemEnum, ItemImpl, ItemStruct, ItemTrait};

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