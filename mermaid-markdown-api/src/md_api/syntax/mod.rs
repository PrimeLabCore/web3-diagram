use crate::md_api::objects::connection::Connection;
use crate::md_api::objects::node::Node;
use crate::md_api::syntax::flow_chart::ObjectConfig;
use enum_as_inner::EnumAsInner;

pub mod flow_chart;

/// An Enum representing the possible options for the direction of flow for the diagram.
#[derive(AsRefStr)]
pub enum FlowDirection {
    /// top to bottom
    TB,
    /// top-down (same as top to bottom)
    TD,
    /// bottom to top
    BT,
    /// right to left
    BL,
    /// left to right
    LR,
}

#[derive(EnumProperty, EnumAsInner, Debug)]
pub enum SyntaxConfigFile<'a> {
    FlowChart(ObjectConfig<'a>),
}

pub trait CoreSyntaxFunctions {
    fn new(direction: FlowDirection) -> Self;

    fn add_node(
        &mut self,
        node_config: SyntaxConfigFile,
    );

    fn add_connection(
        &mut self,
        connection_config: SyntaxConfigFile,
    );

    fn add_linebreak(
        &mut self,
        num_of_indents: Option<u8>,
    );

    fn build_node_config<'a>(
        &self,
        node: &'a Node,
        id: Option<&'a str>,
    ) -> SyntaxConfigFile<'a>;

    fn build_connection_config<'a>(
        &self,
        connection: &'a Connection,
        extra_length_num: Option<u8>,
    ) -> SyntaxConfigFile<'a>;

    fn return_schema(self) -> String;
}
