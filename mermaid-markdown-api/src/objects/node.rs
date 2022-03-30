use crate::objects::connection::Connection;
use crate::objects::DiagramObject;
use crate::syntax::CoreSyntaxFunctions;
use std::collections::VecDeque;
use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug)]
pub enum ScopeType {
    Private,
    Public,
    Trait,
    Payable,
}

#[derive(AsRefStr, Debug)]
pub enum ActionType {
    Mutation,
    View,
    Process,
    Event,
}

pub struct Node {
    pub name: String,
    pub scope: ScopeType,
    pub action: ActionType,
    pub connections: Vec<Connection>,
}

impl<T: CoreSyntaxFunctions> DiagramObject<T> for Node {
    fn add_object_to_schema(
        &self,
        schema: &mut T,
        id: Option<&str>,
        _extra_length_num: Option<u8>,
    ) {
        let config = schema.build_node_config(self, id);
        schema.add_node(config);
    }
}

impl Node {
    fn parse_node(
        &self,
        schema: &mut impl CoreSyntaxFunctions,
    ) {
        for connection in &self.connections {
            self.add_object_to_schema(schema, None, None);
            connection.add_object_to_schema(schema, None, None);
            connection.node.add_object_to_schema(schema, None, None);
            schema.add_linebreak(None);
        }
    }
    pub fn traverse(
        &self,
        schema: &mut impl CoreSyntaxFunctions,
    ) {
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            node.parse_node(schema);
            for connection in &node.connections {
                queue.push_back(&connection.node);
            }
        }
    }
}
