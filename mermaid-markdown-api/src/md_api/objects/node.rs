use crate::md_api::objects::connection::Connection;
use crate::md_api::objects::DiagramObject;
use crate::md_api::syntax::flow_chart::{NodeConfig, ObjectConfig, Shape};
use crate::md_api::syntax::{CoreSyntaxFunctions, SyntaxConfigFile};
use std::convert::AsRef;
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
    pub(crate) name: String,
    pub(crate) scope: ScopeType,
    pub(crate) action: ActionType,
    connections: Vec<Connection>,
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

struct NodeIter<'a> {
    children: &'a [(&'a str, &'a Node)],
    parent: Option<Box<NodeIter<'a>>>,
}

impl Node {
    fn iter(&self) -> NodeIter {
        NodeIter {
            children: vec![("A", self)].as_slice(),
            parent: None,
        }
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = (String, &'a Node);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
// impl<'a> IntoIterator for &'a Node {
//     type Item = (String, Node);
//     type IntoIter = NodeIterator<'a>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }
//
// pub struct NodeIterator<'a> {
//     current_node_id_tuple: (String, &'a Node),
//     node_pool: Vec<(String, &'a Node)>,
// }
//
// impl<'a> Iterator for NodeIterator<'a> {
//     type Item = i8;
//     fn next(&mut self) -> Option<i8> {
//         let result = match self.index {
//             0 => self.pixel.r,
//             _ => return None,
//         };
//         self.index += 1;
//         Some(result)
//     }
// }
