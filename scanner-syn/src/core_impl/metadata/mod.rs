pub mod metadata_generator;
pub mod metadata_visitor;

use syn::{Path, Type};


pub(crate) fn path_is_event(path: &Path) -> bool {    
    path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.iter().next().unwrap().ident == "NearEvent"
}

/// Equivalent to `path_is_result` except that it works on `Type` values.
pub(crate) fn type_is_event(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) if type_path.qself.is_none() => path_is_event(&type_path.path),
        _ => false,
    }
}