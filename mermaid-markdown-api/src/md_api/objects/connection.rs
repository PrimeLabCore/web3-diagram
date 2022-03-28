use crate::md_api::objects::node::Node;
use crate::md_api::objects::DiagramObject;
use crate::md_api::syntax::flow_chart::{
    ArrowDirection, ArrowType, ConnectionConfig, LineType, ObjectConfig,
};
use crate::md_api::syntax::{CoreSyntaxFunctions, SyntaxConfigFile};

pub enum ConnectionType {
    DirectConnection,
    CrossContractConnection,
    Emission,
}

pub struct Connection {
    pub(crate) connection_type: ConnectionType,
    node: Node,
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
