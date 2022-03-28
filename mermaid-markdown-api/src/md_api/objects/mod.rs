use crate::md_api::syntax::CoreSyntaxFunctions;

pub mod connection;
pub mod node;

pub trait DiagramObject<T: CoreSyntaxFunctions> {
    fn add_object_to_schema(
        &self,
        schema: &mut T,
        id: Option<&str>,
        extra_length_num: Option<u8>,
    );
}

// TODO: Maybe make a switch on a passed argument enum repesenting the syntax to generate the config file
// TODO: String to ENUM and what that allows for configuration objects
