use crate::objects::connection::Connection;
use crate::objects::node::Node;
use crate::syntax::flow_chart::ObjectConfig;
use enum_as_inner::EnumAsInner;
// TODO:
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

/// An enum representation of the available syntax's.
#[derive(EnumProperty, EnumAsInner, Debug)]
pub enum SyntaxConfigFile<'a> {
    FlowChart(ObjectConfig<'a>),
}

pub trait CoreSyntaxFunctions {
    /// Returns a `FlowChart` struct to allow you to build the necessary markdown text.
    ///
    /// # Arguments
    ///
    /// * `direction` - The enum representation of the flow direction of the diagram
    ///
    /// # Examples
    ///
    /// ```
    /// use mermaid_markdown_api::syntax::flow_chart::FlowChart;
    /// use mermaid_markdown_api::syntax::{CoreSyntaxFunctions, FlowDirection};
    ///
    /// let mut flow_chart = FlowChart::new(FlowDirection::TD);
    /// ```
    fn new(direction: FlowDirection) -> Self;

    /// Creates a [Mermaid.js Node](https://mermaid-js.github.io/mermaid/#/flowchart?id=a-node-default) with the supplied [configuration](NodeConfig) & appends it to the current data of the flow chart struct (i.e. `self.data`).
    ///
    /// # Arguments
    /// * `node_config` - [NodeConfig]
    ///
    /// # Examples
    ///
    /// ```
    /// use mermaid_markdown_api::syntax::flow_chart::{FlowChart, NodeConfig, Shape};
    /// use mermaid_markdown_api::syntax::{CoreSyntaxFunctions, FlowDirection, SyntaxConfigFile};
    ///
    /// let mut flow_chart = FlowChart::new(FlowDirection::TD);
    ///
    /// let node_config =  SyntaxConfigFile::FlowChart(ConfigFile::NodeConfig(NodeConfig {
    ///   id: "A",
    ///   class: None,
    ///   shape: Shape::Circle,
    ///   inner_text: "inner text",
    /// }));
    ///
    /// flow_chart.add_node(node_config);
    /// ```
    fn add_node(
        &mut self,
        node_config: SyntaxConfigFile,
    );

    /// Creates a [Mermaid.js Connection](https://mermaid-js.github.io/mermaid/#/flowchart?id=links-between-nodes) with the supplied [configuration] & appends it to the current data of the flow chart struct (i.e. `self.data`).
    ///
    /// # Arguments
    /// * `connection_config` - [ConnectionConfig]
    ///
    /// # Examples
    ///
    /// ```
    /// use mermaid_markdown_api::syntax::flow_chart::{ArrowDirection, ArrowType, ConnectionConfig, FlowChart, LineType, NodeConfig, Shape};
    /// use mermaid_markdown_api::syntax::{CoreSyntaxFunctions, FlowDirection, SyntaxConfigFile};
    /// let mut flow_chart = FlowChart::new(FlowDirection::TD);
    ///
    /// let node_config =  SyntaxConfigFile::FlowChart(ConfigFile::NodeConfig(NodeConfig {
    ///   id: "A",
    ///   class: None,
    ///   shape: Shape::Circle,
    ///   inner_text: "inner text",
    /// }));
    ///
    /// let connection_config = SyntaxConfigFile::FlowChart(ConfigFile::ConnectionConfig(ConnectionConfig {
    ///   line_type: LineType::Dashed,
    ///   arrow_type: ArrowType::Standard,
    ///   arrow_direction: ArrowDirection::Right,
    ///   extra_length_num: None,
    /// }));
    ///
    /// flow_chart.add_node(node_config);
    /// flow_chart.add_connection(connection_config);
    /// ```
    fn add_connection(
        &mut self,
        connection_config: SyntaxConfigFile,
    );

    /// Appends a linebreak & the preceding whitespace to the current data of the flow chart struct (i.e. `self.data`).
    ///
    /// # Arguments
    ///
    /// * `num_of_indents` - Optional number of indents to insert once the new line is added (default it 1)
    ///
    /// # Examples
    ///
    /// ```
    /// use mermaid_markdown_api::syntax::flow_chart::FlowChart;
    /// use mermaid_markdown_api::syntax::{CoreSyntaxFunctions, FlowDirection};
    /// let mut flow_chart = FlowChart::new(FlowDirection::TD);
    ///
    /// flow_chart.add_linebreak(None);
    /// ```
    fn add_linebreak(
        &mut self,
        num_of_indents: Option<u8>,
    );

    /// This method creates a [NodeConfig] referencing data from a supplied [Node].
    ///
    /// # Arguments
    ///
    /// * `node` - The [Node] that is going to determine the configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use mermaid_markdown_api::objects::node::{ActionType, Node, ScopeType};
    /// use mermaid_markdown_api::syntax::{CoreSyntaxFunctions, FlowDirection};
    /// use mermaid_markdown_api::syntax::flow_chart::FlowChart;
    /// let mut flow_chart = FlowChart::new(FlowDirection::TD);
    ///
    /// let node = Node {
    ///     name: "function_a".to_string(),
    ///     scope: ScopeType::Public,
    ///     action: ActionType::Mutation,
    ///     connections: vec![],
    /// };
    ///
    /// let node_config = flow_chart.build_node_config(&node);
    /// ```
    fn build_node_config<'a>(
        &self,
        node: &'a Node,
        id: Option<&'a str>,
    ) -> SyntaxConfigFile<'a>;

    /// This method creates a [ConnectionConfig] referencing data from a supplied [Connection].
    ///
    /// # Arguments
    ///
    /// * `connection` - The [Connection] that is going to determine the configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use mermaid_markdown_api::objects::connection::{Connection, ConnectionType};
    /// use mermaid_markdown_api::objects::node::{ActionType, Node, ScopeType};
    /// use mermaid_markdown_api::syntax::{CoreSyntaxFunctions, FlowDirection};
    /// use mermaid_markdown_api::syntax::flow_chart::FlowChart;
    /// let mut flow_chart = FlowChart::new(FlowDirection::TD);
    ///
    /// let connection = Connection {
    ///     connection_type: ConnectionType::DirectConnection,
    ///     node: Node {
    ///         name: "function_a".to_string(),
    ///         scope: ScopeType::Public,
    ///         action: ActionType::Mutation,
    ///         connections: vec![],
    ///     }
    /// };
    ///
    /// let connection_config = flow_chart.build_connection_config(&connection);
    /// ```
    fn build_connection_config<'a>(
        &self,
        connection: &'a Connection,
        extra_length_num: Option<u8>,
    ) -> SyntaxConfigFile<'a>;

    /// This method returns a clone of `self.data`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mermaid_markdown_api::objects::node::{ActionType, Node, ScopeType};
    /// use mermaid_markdown_api::syntax::{CoreSyntaxFunctions, FlowDirection};
    /// use mermaid_markdown_api::syntax::flow_chart::FlowChart;
    /// let mut flow_chart = FlowChart::new(FlowDirection::TD);
    ///
    /// let node = Node {
    ///     name: "function_a".to_string(),
    ///     scope: ScopeType::Public,
    ///     action: ActionType::Mutation,
    ///     connections: vec![],
    /// };
    ///
    /// let node_config = flow_chart.build_node_config(&node);
    ///
    /// flow_chart.add_node(node_config);
    ///
    /// let markdown_string = flow_chart.return_schema();
    /// ```
    fn return_schema(&self) -> String;
}
