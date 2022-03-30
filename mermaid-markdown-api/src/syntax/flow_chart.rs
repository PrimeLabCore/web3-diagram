use crate::objects::connection::{Connection, ConnectionType};
use crate::objects::node::{ActionType, Node};
use crate::syntax::{CoreSyntaxFunctions, FlowDirection, SyntaxConfigFile};
use enum_as_inner::EnumAsInner;
use strum::EnumProperty;
use strum_macros::EnumProperty;

/// The various different shapes enabled by this syntax.
#[derive(EnumProperty, Debug)]
pub enum Shape {
    #[strum(props(Left = "((", Right = "))"))]
    Circle,
    #[strum(props(Left = "{{", Right = "}}"))]
    Hexagon,
    #[strum(props(Left = "[", Right = "]"))]
    Rectangle,
    #[strum(props(Left = ">", Right = "]"))]
    Flag,
}

/// The various different line types enabled by this syntax.
#[derive(EnumProperty, Debug)]
pub enum LineType {
    #[strum(props(Complete = "--", Addition = "-"))]
    Solid,
    #[strum(props(Left = "-.", Right = ".-", Addition = "."))]
    Dashed,
}

/// The various different arrow types enabled by this syntax.
#[derive(AsRefStr, EnumProperty, Debug, Clone, Copy)]
pub enum ArrowType {
    #[strum(props(Left = "<", Right = ">"))]
    Standard,
    #[strum(props(Left = "x", Right = "x"))]
    X,
    #[strum(props(Left = "o", Right = "o"))]
    O,
}

/// The various different arrow directions enabled by this syntax.
#[derive(AsRefStr, Debug)]
pub enum ArrowDirection {
    BiDirectional,
    Left,
    Right,
    None,
}

/// An enum representation of either a [NodeConfig] or a [ConnectionConfig].
#[derive(EnumProperty, EnumAsInner, Debug)]
pub enum ObjectConfig<'a> {
    NodeConfig(NodeConfig<'a>),
    ConnectionConfig(ConnectionConfig),
}

/// A struct representing the possible attributes for a [Node].
#[derive(Debug)]
pub struct NodeConfig<'a> {
    /// The ID that will be assigned to this node
    pub id: &'a str,
    /// An optional class name to assign to the node
    pub class: Option<&'a str>,
    /// The shape of the node
    pub shape: Shape,
    /// The text to be displayed within the node
    pub inner_text: &'a str,
}

/// A struct representing the possible attributes for a [Connection].
#[derive(Debug)]
pub struct ConnectionConfig {
    /// The enum representation of the type of line you want
    pub line_type: LineType,
    /// The enum representation of the type of arrow you want
    pub arrow_type: ArrowType,
    /// The enum representation of the direction you want the arrows to point
    pub arrow_direction: ArrowDirection,
    /// An optional amount of additional flags to increase line length
    pub extra_length_num: Option<u8>,
}

/// This is the root struct for an individual flow chart.
pub struct FlowChart {
    /// This is the data location of the string data for the markdown
    data: String,
}

impl FlowChart {
    /// Creates a [Mermaid.js Dotted/Dashed Line](https://mermaid-js.github.io/mermaid/#/flowchart?id=dotted-link) with the supplied attributes & appends it to the current data of the flow chart struct (i.e. `self.data`).
    ///
    /// # Arguments
    ///
    /// * `extra_length_num` - An optional amount of additional flags to increase line length
    fn add_dashed_line(
        &mut self,
        extra_length_num: Option<u8>,
    ) {
        // Push the left half of the dashed line flag
        self.data
            .push_str(LineType::Dashed.get_str("Left").unwrap());

        // Check to see if an additional length was requested
        if let Some(extra_length_num) = extra_length_num {
            // Range over `extra_length_num` to add the appropriate number of length additions
            for _ in 0..extra_length_num {
                // Add in a `.`
                self.data
                    .push_str(LineType::Dashed.get_str("Addition").unwrap());
            }
        }

        // Push the right half of the dashed line flag
        self.data
            .push_str(LineType::Dashed.get_str("Right").unwrap());
    }

    /// Creates a [Mermaid.js Solid Line](https://mermaid-js.github.io/mermaid/#/flowchart?id=a-link-with-arrow-head) with the supplied attributes & appends it to the current data of the flow chart struct (i.e. `self.data`).
    ///
    /// # Arguments
    ///
    /// * `extra_length_num` - An optional amount of additional flags to increase line length
    fn add_solid_line(
        &mut self,
        extra_length_num: Option<u8>,
    ) {
        // Push the main portion of the solid line flag
        self.data
            .push_str(LineType::Solid.get_str("Complete").unwrap());

        // Check to see if an additional length was requested
        if let Some(extra_length_num) = extra_length_num {
            // Range over `extra_length_num` to add the appropriate number of length additions
            for _ in 0..extra_length_num {
                // Add in a `-`
                self.data
                    .push_str(LineType::Solid.get_str("Addition").unwrap());
            }
        }
    }

    /// Creates a [Mermaid.js Connection Line with no arrow](https://mermaid-js.github.io/mermaid/#/flowchart?id=links-between-nodes) with the supplied attributes & appends it to the current data of the flow chart struct (i.e. `self.data`).
    ///
    /// # Arguments
    ///
    /// * `line_type` - The enum representation of the line type you want
    /// * `extra_length_num` - An optional amount of additional flags to increase line length
    fn add_line(
        &mut self,
        line_type: LineType,
        extra_length_num: Option<u8>,
    ) {
        match line_type {
            LineType::Solid => self.add_solid_line(extra_length_num),
            LineType::Dashed => self.add_dashed_line(extra_length_num),
        }
    }

    /// Creates a [Mermaid.js Arrow](https://mermaid-js.github.io/mermaid/#/flowchart?id=new-arrow-types) with the supplied attributes & appends it to the current data of the flow chart struct (i.e. `self.data`).
    ///
    /// # Arguments
    ///
    /// * `arrow_type` - The enum representation of the arrow type you want
    /// * `arrow_direction` - The enum representation of the direction you want the arrow to
    fn add_arrow(
        &mut self,
        arrow_type: ArrowType,
        arrow_direction: ArrowDirection,
    ) {
        // Get the `arrow_direction` as a str to use as the key for the `ArrowType` enum property to then add the correct arrow flag
        self.data
            .push_str(arrow_type.get_str(arrow_direction.as_ref()).unwrap())
    }

    /// Determines which [Shape] to put in a [NodeConfig].
    ///
    /// # Arguments
    ///
    /// * `node` - The [Node] that is being represented
    fn get_shape_from_node(
        &self,
        node: &Node,
    ) -> Shape {
        match node.action {
            ActionType::Mutation => Shape::Hexagon,
            ActionType::View => Shape::Circle,
            ActionType::Process => Shape::Rectangle,
            ActionType::Event => Shape::Flag,
        }
    }

    /// Determines which [LineType] & [ArrowType] to put in a [ConnectionConfig].
    ///
    /// # Arguments
    ///
    /// * `connection` - The [Connection] that is being represented
    fn get_line_and_arrow_type_from_connection(
        &self,
        connection: &Connection,
    ) -> (LineType, ArrowType, ArrowDirection) {
        match connection.connection_type {
            ConnectionType::DirectConnection => {
                (LineType::Solid, ArrowType::Standard, ArrowDirection::Right)
            }
            ConnectionType::CrossContractConnection => {
                (LineType::Solid, ArrowType::Standard, ArrowDirection::Right)
            }
            ConnectionType::Emission => {
                (LineType::Solid, ArrowType::Standard, ArrowDirection::Right)
            }
        }
    }
}

impl CoreSyntaxFunctions for FlowChart {
    fn new(direction: FlowDirection) -> Self {
        // Instantiate the starting point for the diagram schema
        let mut schema_root = "flowchart ".to_string();

        // Add in `direction`
        schema_root.push_str(direction.as_ref());

        // Instantiate `FlowChart`
        let mut result = FlowChart { data: schema_root };

        // Add a new line
        result.add_linebreak(None);

        result
    }

    fn add_node(
        &mut self,
        node_config: SyntaxConfigFile,
    ) {
        let node_config: NodeConfig = node_config
            .into_flow_chart()
            .unwrap()
            .into_node_config()
            .unwrap();

        // Push the ID
        self.data.push_str(node_config.id);

        // Push the left shape flag
        self.data
            .push_str(node_config.shape.get_str("Left").unwrap());

        // Push the inner text
        self.data.push_str(node_config.inner_text);

        // Push the left shape flag
        self.data
            .push_str(node_config.shape.get_str("Right").unwrap());

        // If a class name was passed push it to `self.data`
        if let Some(class) = node_config.class {
            self.data.push_str(":::");
            self.data.push_str(class);
        }
    }

    fn add_connection(
        &mut self,
        connection_config: SyntaxConfigFile,
    ) {
        // Unwrap the `SyntaxConfigFile` into the needed `ConnectionConfig`
        let connection_config: ConnectionConfig = connection_config
            .into_flow_chart()
            .unwrap()
            .into_connection_config()
            .unwrap();

        // Push a preceding space
        self.data.push(' ');

        // Depending on the arrow direction wanted make calls to `self.add_arrow` & `self.add_line`
        match connection_config.arrow_direction {
            ArrowDirection::BiDirectional => {
                self.add_arrow(connection_config.arrow_type, ArrowDirection::Left);
                self.add_line(
                    connection_config.line_type,
                    connection_config.extra_length_num,
                );
                self.add_arrow(connection_config.arrow_type, ArrowDirection::Right)
            }
            ArrowDirection::Left => {
                self.add_arrow(
                    connection_config.arrow_type,
                    connection_config.arrow_direction,
                );
                self.add_line(
                    connection_config.line_type,
                    connection_config.extra_length_num,
                )
            }
            ArrowDirection::Right => {
                self.add_line(
                    connection_config.line_type,
                    connection_config.extra_length_num,
                );
                self.add_arrow(
                    connection_config.arrow_type,
                    connection_config.arrow_direction,
                )
            }
            ArrowDirection::None => {
                self.add_line(
                    connection_config.line_type,
                    connection_config.extra_length_num,
                );
            }
        }

        // Push a trailing space
        self.data.push(' ');
    }

    fn add_linebreak(
        &mut self,
        num_of_indents: Option<u8>,
    ) {
        // Get the number of indents to use
        let number_of_indents = num_of_indents.unwrap_or(1);

        // Add the new line
        self.data += "\n";

        // Range over `number_of_indents` to add the appropriate number of tabs
        for _ in 0..number_of_indents {
            // Add in a tab
            self.data += "\t";
        }
    }

    fn build_node_config<'a>(
        &self,
        node: &'a Node,
        id: Option<&'a str>,
    ) -> SyntaxConfigFile<'a> {
        if let Some(id) = id {
            // If an ID was passed use it
            SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
                id,
                class: Some(node.scope.as_ref()),
                shape: self.get_shape_from_node(node),
                inner_text: &node.name,
            }))
        } else {
            // Else use the Node's name
            SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
                id: &node.name,
                class: Some(node.scope.as_ref()),
                shape: self.get_shape_from_node(node),
                inner_text: &node.name,
            }))
        }
    }

    fn build_connection_config<'a>(
        &self,
        connection: &'a Connection,
        extra_length_num: Option<u8>,
    ) -> SyntaxConfigFile<'a> {
        let (line_type, arrow_type, arrow_direction) =
            self.get_line_and_arrow_type_from_connection(connection);

        SyntaxConfigFile::FlowChart(ObjectConfig::ConnectionConfig(ConnectionConfig {
            line_type,
            arrow_type,
            arrow_direction,
            extra_length_num,
        }))
    }

    fn return_schema(&self) -> String {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_circle() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Circle,
            inner_text: "inner text",
        }));

        // Add the node to check afterwards
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A((inner text))";

        println!("{}", flow_chart.data);

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_creates_a_rectangle() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Rectangle,
            inner_text: "inner text",
        }));

        // Add the node to check afterwards
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A[inner text]";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_creates_a_hexagon() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Hexagon,
            inner_text: "inner text",
        }));

        // Add the node to check afterwards
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A{{inner text}}";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_creates_a_flag() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the node to check afterwards
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text]";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_adds_a_dashed_line_with_right_arrow() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the beginning node
        flow_chart.add_node(node_config);

        let connection_config =
            SyntaxConfigFile::FlowChart(ObjectConfig::ConnectionConfig(ConnectionConfig {
                line_type: LineType::Dashed,
                arrow_type: ArrowType::Standard,
                arrow_direction: ArrowDirection::Right,
                extra_length_num: None,
            }));

        // Add the line to check afterwards
        flow_chart.add_connection(connection_config);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "B",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the trailing node
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text] -..-> B>inner text]";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_adds_a_dashed_line_with_no_arrow() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the beginning node
        flow_chart.add_node(node_config);

        let connection_config =
            SyntaxConfigFile::FlowChart(ObjectConfig::ConnectionConfig(ConnectionConfig {
                line_type: LineType::Dashed,
                arrow_type: ArrowType::Standard,
                arrow_direction: ArrowDirection::None,
                extra_length_num: None,
            }));

        // Add the line to check afterwards
        flow_chart.add_connection(connection_config);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "B",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the trailing node
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text] -..- B>inner text]";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_adds_a_dashed_line_with_left_arrow() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the beginning node
        flow_chart.add_node(node_config);

        let connection_config =
            SyntaxConfigFile::FlowChart(ObjectConfig::ConnectionConfig(ConnectionConfig {
                line_type: LineType::Dashed,
                arrow_type: ArrowType::Standard,
                arrow_direction: ArrowDirection::Left,
                extra_length_num: None,
            }));

        // Add the line to check afterwards
        flow_chart.add_connection(connection_config);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "B",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the trailing node
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text] <-..- B>inner text]";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_adds_a_dashed_line_with_bidirectional_arrow() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the beginning node
        flow_chart.add_node(node_config);

        let connection_config =
            SyntaxConfigFile::FlowChart(ObjectConfig::ConnectionConfig(ConnectionConfig {
                line_type: LineType::Dashed,
                arrow_type: ArrowType::Standard,
                arrow_direction: ArrowDirection::BiDirectional,
                extra_length_num: None,
            }));

        // Add the line to check afterwards
        flow_chart.add_connection(connection_config);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "B",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the trailing node
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text] <-..-> B>inner text]";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_adds_a_dashed_line_with_right_o_arrow() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the beginning node
        flow_chart.add_node(node_config);

        let connection_config =
            SyntaxConfigFile::FlowChart(ObjectConfig::ConnectionConfig(ConnectionConfig {
                line_type: LineType::Dashed,
                arrow_type: ArrowType::O,
                arrow_direction: ArrowDirection::Right,
                extra_length_num: None,
            }));

        // Add the line to check afterwards
        flow_chart.add_connection(connection_config);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "B",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the trailing node
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text] -..-o B>inner text]";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_adds_a_dashed_line_with_left_o_arrow() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the beginning node
        flow_chart.add_node(node_config);

        let connection_config =
            SyntaxConfigFile::FlowChart(ObjectConfig::ConnectionConfig(ConnectionConfig {
                line_type: LineType::Dashed,
                arrow_type: ArrowType::O,
                arrow_direction: ArrowDirection::Left,
                extra_length_num: None,
            }));

        // Add the line to check afterwards
        flow_chart.add_connection(connection_config);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "B",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the trailing node
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text] o-..- B>inner text]";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_adds_a_dashed_line_with_bidirectional_o_arrow() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the beginning node
        flow_chart.add_node(node_config);

        let connection_config =
            SyntaxConfigFile::FlowChart(ObjectConfig::ConnectionConfig(ConnectionConfig {
                line_type: LineType::Dashed,
                arrow_type: ArrowType::O,
                arrow_direction: ArrowDirection::BiDirectional,
                extra_length_num: None,
            }));

        // Add the line to check afterwards
        flow_chart.add_connection(connection_config);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "B",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the trailing node
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text] o-..-o B>inner text]";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_adds_a_dashed_line_with_right_x_arrow() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the beginning node
        flow_chart.add_node(node_config);

        let connection_config =
            SyntaxConfigFile::FlowChart(ObjectConfig::ConnectionConfig(ConnectionConfig {
                line_type: LineType::Dashed,
                arrow_type: ArrowType::X,
                arrow_direction: ArrowDirection::Right,
                extra_length_num: None,
            }));

        // Add the line to check afterwards
        flow_chart.add_connection(connection_config);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "B",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the trailing node
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text] -..-x B>inner text]";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_adds_a_dashed_line_with_left_x_arrow() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the beginning node
        flow_chart.add_node(node_config);

        let connection_config =
            SyntaxConfigFile::FlowChart(ObjectConfig::ConnectionConfig(ConnectionConfig {
                line_type: LineType::Dashed,
                arrow_type: ArrowType::X,
                arrow_direction: ArrowDirection::Left,
                extra_length_num: None,
            }));

        // Add the line to check afterwards
        flow_chart.add_connection(connection_config);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "B",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the trailing node
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text] x-..- B>inner text]";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_adds_a_dashed_line_with_bidirectional_x_arrow() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the beginning node
        flow_chart.add_node(node_config);

        let connection_config =
            SyntaxConfigFile::FlowChart(ObjectConfig::ConnectionConfig(ConnectionConfig {
                line_type: LineType::Dashed,
                arrow_type: ArrowType::X,
                arrow_direction: ArrowDirection::BiDirectional,
                extra_length_num: None,
            }));

        // Add the line to check afterwards
        flow_chart.add_connection(connection_config);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "B",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the trailing node
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text] x-..-x B>inner text]";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_adds_a_dashed_line_with_bidirectional_x_arrow_and_extra_length() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "A",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the beginning node
        flow_chart.add_node(node_config);

        let connection_config =
            SyntaxConfigFile::FlowChart(ObjectConfig::ConnectionConfig(ConnectionConfig {
                line_type: LineType::Dashed,
                arrow_type: ArrowType::X,
                arrow_direction: ArrowDirection::BiDirectional,
                extra_length_num: Some(1),
            }));

        // Add the line to check afterwards
        flow_chart.add_connection(connection_config);

        let node_config = SyntaxConfigFile::FlowChart(ObjectConfig::NodeConfig(NodeConfig {
            id: "B",
            class: None,
            shape: Shape::Flag,
            inner_text: "inner text",
        }));

        // Add the trailing node
        flow_chart.add_node(node_config);

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text] x-...-x B>inner text]";

        assert_eq!(flow_chart.data, expected);
    }
}
