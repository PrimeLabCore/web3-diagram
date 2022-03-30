use crate::objects::node::Node;
use crate::objects::DiagramObject;
use crate::syntax::CoreSyntaxFunctions;

pub enum ConnectionType {
    DirectConnection,
    CrossContractConnection,
    Emission,
}

pub struct Connection {
    pub(crate) connection_type: ConnectionType,
    pub(crate) node: Node,
}

impl<T: CoreSyntaxFunctions> DiagramObject<T> for Connection {
    fn add_object_to_schema(
        &self,
        schema: &mut T,
        _id: Option<&str>,
        extra_length_num: Option<u8>,
    ) {
        let config = schema.build_connection_config(self, extra_length_num);
        schema.add_connection(config);
    }
}

// TODO: Extra length
// TODO: Separate via generic vs param
