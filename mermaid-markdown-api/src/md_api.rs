use crate::objects::node::Node;
use crate::syntax::{CoreSyntaxFunctions, FlowDirection};

pub struct MdAPI<T: CoreSyntaxFunctions> {
    schema: T,
    hierarchy_root: Node,
}

impl<T: CoreSyntaxFunctions> MdAPI<T> {
    pub fn new(
        flow_direction: FlowDirection,
        hierarchy_root: Node,
    ) -> Self {
        MdAPI {
            schema: T::new(flow_direction),
            hierarchy_root,
        }
    }

    pub fn parse_hierarchy(&mut self) -> String {
        self.hierarchy_root.traverse(&mut self.schema);

        self.schema.return_schema()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::connection::{Connection, ConnectionType};
    use crate::objects::node::{ActionType, ScopeType};
    use crate::syntax::flow_chart::FlowChart;

    #[test]
    fn it_works() {
        let hierarchy_tree_root = Node {
            name: "function_a".to_string(),
            scope: ScopeType::Public,
            action: ActionType::Mutation,
            connections: vec![
                Connection {
                    connection_type: ConnectionType::Emission,
                    node: Node {
                        name: "function_a_event".to_string(),
                        scope: ScopeType::Public,
                        action: ActionType::Event,
                        connections: vec![],
                    },
                },
                Connection {
                    connection_type: ConnectionType::DirectConnection,
                    node: Node {
                        name: "function_b_private".to_string(),
                        scope: ScopeType::Private,
                        action: ActionType::Mutation,
                        connections: vec![],
                    },
                },
            ],
        };

        let mut api = MdAPI::<FlowChart>::new(FlowDirection::TD, hierarchy_tree_root);

        let result = api.parse_hierarchy();

        let expected_string = r#"flowchart TD
	function_a{{function_a}}:::Public --> function_a_event>function_a_event]:::Public
	function_a{{function_a}}:::Public --> function_b_private{{function_b_private}}:::Private"#;

        assert_eq!(result, expected_string);
    }
}
