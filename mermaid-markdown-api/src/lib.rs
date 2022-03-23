extern crate strum;
#[macro_use]
extern crate strum_macros;

use strum::EnumProperty;

/// This is the root struct for an individual flow chart.
pub struct FlowChart {
    /// This is the data location of the string data for the markdown
    data: String,
}

impl FlowChart {
    /// Returns a `FlowChart` struct to allow you to build the necessary markdown text.
    ///
    /// # Arguments
    ///
    /// * `direction` - The enum representation of the flow direction of the diagram
    ///
    /// # Examples
    ///
    /// ```
    /// use mermaid_markdown_api::*;
    /// let mut flow_chart = FlowChart::new(FlowDirection::TD);
    /// ```
    pub fn new(direction: FlowDirection) -> Self {
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

    /// Appends a linebreak & the preceding whitespace to the current data of the flow chart struct (i.e. `self.data`).
    ///
    /// # Arguments
    ///
    /// * `num_of_indents` - Optional number of indents to insert once the new line is added (default it 1)
    ///
    /// # Examples
    ///
    /// ```
    /// use mermaid_markdown_api::*;
    /// let mut flow_chart = FlowChart::new(FlowDirection::TD);
    /// flow_chart.add_linebreak(None);
    /// ```
    pub fn add_linebreak(
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

    /// Creates a [Mermaid.js Node](https://mermaid-js.github.io/mermaid/#/flowchart?id=a-node-default) with the supplied attributes & appends it to the current data of the flow chart struct (i.e. `self.data`).
    ///
    /// # Arguments
    ///
    /// * `id` - The ID that will be assigned to this node
    /// * `class` - An optional class name to assign to the node
    /// * `shape` -  The shape of the node
    /// * `inner_text` - The text to be displayed within the node
    ///
    /// # Examples
    ///
    /// ```
    /// use mermaid_markdown_api::*;
    /// let mut flow_chart = FlowChart::new(FlowDirection::TD);
    /// flow_chart.add_node("A", None, Shape::Circle, "inner text");
    /// ```
    pub fn add_node(
        &mut self,
        id: &str,
        class: Option<&str>,
        shape: Shape,
        inner_text: &str,
    ) {
        // Push the ID
        self.data.push_str(id);

        // Push the left shape flag
        self.data.push_str(shape.get_str("Left").unwrap());

        // Push the inner text
        self.data.push_str(inner_text);

        // Push the left shape flag
        self.data.push_str(shape.get_str("Right").unwrap());

        // If a class name was passed push it to `self.data`
        if let Some(class) = class {
            self.data.push_str(":::");
            self.data.push_str(class);
        }

        // Push a leading space
        self.data.push_str(" ");
    }

    /// Creates a [Mermaid.js Connection](https://mermaid-js.github.io/mermaid/#/flowchart?id=links-between-nodes) with the supplied attributes & appends it to the current data of the flow chart struct (i.e. `self.data`).
    ///
    /// # Arguments
    ///
    /// * `line_type` -  The type of Line
    /// * `arrow_type` - The type of Arrow
    ///
    /// # Examples
    ///
    /// ```
    /// use mermaid_markdown_api::*;
    /// let mut flow_chart = FlowChart::new(FlowDirection::TD);
    /// flow_chart.add_connection(LineType::SolidLine, ArrowType::Arrow);
    /// ```

    pub fn add_connection(
        &mut self,
        line_type: LineType,
        arrow_type: ArrowType,
    ) {
        // Push the LineType
        self.data.push_str(line_type.get_str("LineType").unwrap());
        // Push the ArrowType
        self.data.push_str(arrow_type.get_str("ArrowType").unwrap());

        // Push a leading space
        self.data.push_str(" ");    
              
    }
}

/// The various different shapes enabled by this API.
#[derive(strum_macros::EnumProperty, Debug)]
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

/// The various different Line types enabled by this API.
#[derive(strum_macros::EnumProperty, Debug)]
pub enum LineType {
    #[strum(props(LineType = "--"))]
    SolidLine,
    #[strum(props(LineType = "-.-"))]
    DashedLine,
}

/// The various different Arrow types enabled by this API.
#[derive(strum_macros::EnumProperty, Debug)]
pub enum ArrowType {
    #[strum(props(ArrowType = ">"))]
    Arrow,
    #[strum(props(ArrowType = "o"))]
    Circle,
    #[strum(props(ArrowType = "x"))]
    X,
   
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_circle() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        // Add the node to check afterwards
        flow_chart.add_node("A", None, Shape::Circle, "inner text");

       // The string we are expecting
        let expected = r"flowchart TD
        A((inner text)) -..->B((inner text))";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_creates_a_rectangle() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        // Add the node to check afterwards
        flow_chart.add_node("A", None, Shape::Rectangle, "inner text");

        // The string we are expecting
        let expected = r"flowchart TD
	A[inner text] ";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_creates_a_hexagon() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        // Add the node to check afterwards
        flow_chart.add_node("A", None, Shape::Hexagon, "inner text");

        // The string we are expecting
        let expected = r"flowchart TD
	A{{inner text}} ";

        assert_eq!(flow_chart.data, expected);
    }

    #[test]
    fn it_creates_a_flag() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        // Add the node to check afterwards
        flow_chart.add_node("A", None, Shape::Flag, "inner text");

        // The string we are expecting
        let expected = r"flowchart TD
	A>inner text] ";

        assert_eq!(flow_chart.data, expected);
    }
    #[test]
    fn it_creates_a_circle_with_soldline_arrow() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        // Add the node to check afterwards
        flow_chart.add_node("A", None, Shape::Circle, "inner text");

        // Add the connection to check afterwards
        flow_chart.add_connection(LineType::SolidLine, ArrowType::Arrow);

        // Add the node to check afterwards
        flow_chart.add_node("B", None, Shape::Circle, "inner text");

        println!("{}", flow_chart.data);
     
    }
    #[test]
    fn it_creates_a_rectangle_with_dashedline_circle() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        // Add the node to check afterwards
        flow_chart.add_node("A", None, Shape::Rectangle, "inner text");

        // Add the connection to check afterwards
        flow_chart.add_connection(LineType::DashedLine, ArrowType::Circle);

        // Add the node to check afterwards
        flow_chart.add_node("B", None, Shape::Rectangle, "inner text");

        println!("{}", flow_chart.data);
     
    }
    #[test]
    fn it_creates_a_hexagon_with_dashedline_x() {
        // Instantiate the flow chart
        let mut flow_chart = FlowChart::new(FlowDirection::TD);

        // Add the node to check afterwards
        flow_chart.add_node("A", None, Shape::Hexagon, "inner text");

        // Add the connection to check afterwards
        flow_chart.add_connection(LineType::DashedLine, ArrowType::X);

        // Add the node to check afterwards
        flow_chart.add_node("B", None, Shape::Hexagon, "inner text");

        println!("{}", flow_chart.data);
     
    }
}
