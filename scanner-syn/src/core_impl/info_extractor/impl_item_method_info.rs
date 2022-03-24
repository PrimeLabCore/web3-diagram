use crate::core_impl::info_extractor::AttrSigInfo;
use syn::{ImplItemMethod, Type, Visibility};

/// Information extracted from `ImplItemMethod`.
pub struct ImplItemMethodInfo {
    /// Information on the attributes and the signature of the method.
    pub attr_signature_info: AttrSigInfo,
    /// Whether method has `pub` modifier.
    pub is_public: bool,
    /// Whether method is part of trait implementation.
    pub is_trait_impl: bool,
    /// Whether method is part of `impl` section decorated with `#[near_bindgen]`
    pub has_near_sdk_attr: bool,
    /// The type of the contract struct.
    pub struct_type: Type,
}

impl ImplItemMethodInfo {
    /// Process the method and extract information important for near-sdk.
    pub fn new(
        original: &mut ImplItemMethod,
        is_trait_impl: bool,
        has_near_sdk_attr: bool,
        struct_type: Type,
    ) -> syn::Result<Self> {
        let ImplItemMethod { attrs, sig, .. } = original;
        // TODO:
        // let mut functions_called = vec![];
        // statements_parser::parse_statements(&block.stmts, &mut functions_called);
        let attr_signature_info = AttrSigInfo::new(attrs, sig)?;
        let is_public = matches!(original.vis, Visibility::Public(_));
        Ok(Self {
            attr_signature_info,
            is_public,
            is_trait_impl,
            has_near_sdk_attr,
            struct_type,
        })
    }
}
