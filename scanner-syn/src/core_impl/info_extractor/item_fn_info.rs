use crate::core_impl::info_extractor::AttrSigInfo;
use syn::ItemFn;

/// Information extracted from `ItemFn`.
pub struct ItemFnInfo {
    /// Information on the attributes and the signature of the function.
    pub attr_signature_info: AttrSigInfo,
}

impl ItemFnInfo {
    /// Process the function and extract important information.
    pub fn new(original: &mut ItemFn) -> syn::Result<Self> {
        let x = AttrSigInfo::new(&mut original.attrs, &mut original.sig)?;
        Ok(Self {
            attr_signature_info: x,
        })
    }
}
