use strum::EnumProperty;
use strum_macros::EnumProperty;

/// The various different shapes enabled by this API.
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

/// The various different line types enabled by this API.
#[derive(EnumProperty, Debug)]
pub enum LineType {
	#[strum(props(Complete = "--", Addition = "-"))]
	Solid,
	#[strum(props(Left = "-.", Right = ".-", Addition = "."))]
	Dashed,
}

/// The various different arrow types enabled by this API.
#[derive(AsRefStr, EnumProperty, Debug, Clone, Copy)]
pub enum ArrowType {
	#[strum(props(Left = "<", Right = ">"))]
	Standard,
	#[strum(props(Left = "x", Right = "x"))]
	X,
	#[strum(props(Left = "o", Right = "o"))]
	O,
}

#[derive(AsRefStr, Debug)]
pub enum ArrowDirection {
	BiDirectional,
	Left,
	Right,
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
	/// use mermaid_markdown_api::md_api::flow_chart::*;
	///
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
	/// use mermaid_markdown_api::md_api::flow_chart::*;
	///
	/// let mut flow_chart = FlowChart::new(FlowDirection::TD);
	///
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
	/// use mermaid_markdown_api::md_api::flow_chart::*;
	///
	/// let mut flow_chart = FlowChart::new(FlowDirection::TD);
	///
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
	}

	/// Creates a [Mermaid.js Connection](https://mermaid-js.github.io/mermaid/#/flowchart?id=links-between-nodes) with the supplied attributes & appends it to the current data of the flow chart struct (i.e. `self.data`).
	///
	/// # Arguments
	///
	/// * `line_type` - The enum representation of the type of line you want
	/// * `arrow_type` - The enum representation of the type of arrow you want
	/// * `arrow_direction` -  The enum representation of the direction you want the arrows to point
	/// * `extra_length_num` - An optional amount of additional flags to increase line length
	///
	/// # Examples
	///
	/// ```
	/// use mermaid_markdown_api::md_api::flow_chart::*;
	///
	/// let mut flow_chart = FlowChart::new(FlowDirection::TD);
	///
	/// flow_chart.add_node("A", None, Shape::Circle, "inner text");
	/// flow_chart.add_connection(LineType::Dashed, ArrowType::Standard, ArrowDirection::Right, None);
	/// ```
	pub fn add_connection(
		&mut self,
		line_type: LineType,
		arrow_type: ArrowType,
		arrow_direction: ArrowDirection,
		extra_length_num: Option<u8>,
	) {
		// Push a preceding space
		self.data.push_str(" ");

		// Depending on the arrow direction wanted make calls to `self.add_arrow` & `self.add_line`
		match arrow_direction {
			ArrowDirection::BiDirectional => {
				self.add_arrow(arrow_type, ArrowDirection::Left);
				self.add_line(line_type, extra_length_num);
				self.add_arrow(arrow_type, ArrowDirection::Right)
			}
			ArrowDirection::Left => {
				self.add_arrow(arrow_type, arrow_direction);
				self.add_line(line_type, extra_length_num)
			}
			ArrowDirection::Right => {
				self.add_line(line_type, extra_length_num);
				self.add_arrow(arrow_type, arrow_direction)
			}
		}

		// Push a trailing space
		self.data.push_str(" ");
	}

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
		self.data.push_str(LineType::Solid.get_str("Main").unwrap());

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
	A((inner text))";

		println!("{}", flow_chart.data);

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
	A[inner text]";

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
	A{{inner text}}";

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
	A>inner text]";

		assert_eq!(flow_chart.data, expected);
	}

	#[test]
	fn it_adds_a_dashed_line_with_right_arrow() {
		// Instantiate the flow chart
		let mut flow_chart = FlowChart::new(FlowDirection::TD);

		// Add the beginning node
		flow_chart.add_node("A", None, Shape::Flag, "inner text");

		// Add the line to check afterwards
		flow_chart.add_connection(
			LineType::Dashed,
			ArrowType::Standard,
			ArrowDirection::Right,
			None,
		);

		// Add the trailing node
		flow_chart.add_node("B", None, Shape::Flag, "inner text");

		// The string we are expecting
		let expected = r"flowchart TD
	A>inner text] -..-> B>inner text]";

		assert_eq!(flow_chart.data, expected);
	}

	#[test]
	fn it_adds_a_dashed_line_with_left_arrow() {
		// Instantiate the flow chart
		let mut flow_chart = FlowChart::new(FlowDirection::TD);

		// Add the beginning node
		flow_chart.add_node("A", None, Shape::Flag, "inner text");

		// Add the line to check afterwards
		flow_chart.add_connection(
			LineType::Dashed,
			ArrowType::Standard,
			ArrowDirection::Left,
			None,
		);

		// Add the trailing node
		flow_chart.add_node("B", None, Shape::Flag, "inner text");

		// The string we are expecting
		let expected = r"flowchart TD
	A>inner text] <-..- B>inner text]";

		assert_eq!(flow_chart.data, expected);
	}

	#[test]
	fn it_adds_a_dashed_line_with_bidirectional_arrow() {
		// Instantiate the flow chart
		let mut flow_chart = FlowChart::new(FlowDirection::TD);

		// Add the beginning node
		flow_chart.add_node("A", None, Shape::Flag, "inner text");

		// Add the line to check afterwards
		flow_chart.add_connection(
			LineType::Dashed,
			ArrowType::Standard,
			ArrowDirection::BiDirectional,
			None,
		);

		// Add the trailing node
		flow_chart.add_node("B", None, Shape::Flag, "inner text");

		// The string we are expecting
		let expected = r"flowchart TD
	A>inner text] <-..-> B>inner text]";

		assert_eq!(flow_chart.data, expected);
	}

	#[test]
	fn it_adds_a_dashed_line_with_right_o_arrow() {
		// Instantiate the flow chart
		let mut flow_chart = FlowChart::new(FlowDirection::TD);

		// Add the beginning node
		flow_chart.add_node("A", None, Shape::Flag, "inner text");

		// Add the line to check afterwards
		flow_chart.add_connection(LineType::Dashed, ArrowType::O, ArrowDirection::Right, None);

		// Add the trailing node
		flow_chart.add_node("B", None, Shape::Flag, "inner text");

		// The string we are expecting
		let expected = r"flowchart TD
	A>inner text] -..-o B>inner text]";

		assert_eq!(flow_chart.data, expected);
	}

	#[test]
	fn it_adds_a_dashed_line_with_left_o_arrow() {
		// Instantiate the flow chart
		let mut flow_chart = FlowChart::new(FlowDirection::TD);

		// Add the beginning node
		flow_chart.add_node("A", None, Shape::Flag, "inner text");

		// Add the line to check afterwards
		flow_chart.add_connection(LineType::Dashed, ArrowType::O, ArrowDirection::Left, None);

		// Add the trailing node
		flow_chart.add_node("B", None, Shape::Flag, "inner text");

		// The string we are expecting
		let expected = r"flowchart TD
	A>inner text] o-..- B>inner text]";

		assert_eq!(flow_chart.data, expected);
	}

	#[test]
	fn it_adds_a_dashed_line_with_bidirectional_o_arrow() {
		// Instantiate the flow chart
		let mut flow_chart = FlowChart::new(FlowDirection::TD);

		// Add the beginning node
		flow_chart.add_node("A", None, Shape::Flag, "inner text");

		// Add the line to check afterwards
		flow_chart.add_connection(
			LineType::Dashed,
			ArrowType::O,
			ArrowDirection::BiDirectional,
			None,
		);

		// Add the trailing node
		flow_chart.add_node("B", None, Shape::Flag, "inner text");

		// The string we are expecting
		let expected = r"flowchart TD
	A>inner text] o-..-o B>inner text]";

		assert_eq!(flow_chart.data, expected);
	}

	#[test]
	fn it_adds_a_dashed_line_with_right_x_arrow() {
		// Instantiate the flow chart
		let mut flow_chart = FlowChart::new(FlowDirection::TD);

		// Add the beginning node
		flow_chart.add_node("A", None, Shape::Flag, "inner text");

		// Add the line to check afterwards
		flow_chart.add_connection(LineType::Dashed, ArrowType::X, ArrowDirection::Right, None);

		// Add the trailing node
		flow_chart.add_node("B", None, Shape::Flag, "inner text");

		// The string we are expecting
		let expected = r"flowchart TD
	A>inner text] -..-x B>inner text]";

		assert_eq!(flow_chart.data, expected);
	}

	#[test]
	fn it_adds_a_dashed_line_with_left_x_arrow() {
		// Instantiate the flow chart
		let mut flow_chart = FlowChart::new(FlowDirection::TD);

		// Add the beginning node
		flow_chart.add_node("A", None, Shape::Flag, "inner text");

		// Add the line to check afterwards
		flow_chart.add_connection(LineType::Dashed, ArrowType::X, ArrowDirection::Left, None);

		// Add the trailing node
		flow_chart.add_node("B", None, Shape::Flag, "inner text");

		// The string we are expecting
		let expected = r"flowchart TD
	A>inner text] x-..- B>inner text]";

		assert_eq!(flow_chart.data, expected);
	}

	#[test]
	fn it_adds_a_dashed_line_with_bidirectional_x_arrow() {
		// Instantiate the flow chart
		let mut flow_chart = FlowChart::new(FlowDirection::TD);

		// Add the beginning node
		flow_chart.add_node("A", None, Shape::Flag, "inner text");

		// Add the line to check afterwards
		flow_chart.add_connection(
			LineType::Dashed,
			ArrowType::X,
			ArrowDirection::BiDirectional,
			None,
		);

		// Add the trailing node
		flow_chart.add_node("B", None, Shape::Flag, "inner text");

		// The string we are expecting
		let expected = r"flowchart TD
	A>inner text] x-..-x B>inner text]";

		assert_eq!(flow_chart.data, expected);
	}

	#[test]
	fn it_adds_a_dashed_line_with_bidirectional_x_arrow_and_extra_length() {
		// Instantiate the flow chart
		let mut flow_chart = FlowChart::new(FlowDirection::TD);

		// Add the beginning node
		flow_chart.add_node("A", None, Shape::Flag, "inner text");

		// Add the line to check afterwards
		flow_chart.add_connection(
			LineType::Dashed,
			ArrowType::X,
			ArrowDirection::BiDirectional,
			Some(1),
		);

		// Add the trailing node
		flow_chart.add_node("B", None, Shape::Flag, "inner text");

		// The string we are expecting
		let expected = r"flowchart TD
	A>inner text] x-...-x B>inner text]";

		assert_eq!(flow_chart.data, expected);
	}
}
