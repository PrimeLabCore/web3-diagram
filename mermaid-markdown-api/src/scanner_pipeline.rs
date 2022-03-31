use scanner_syn::contract_descriptor::{
    ContractDescriptor, ContractInfo,FunctionInfo, DefaultContractDescriptor,
};

use crate::{
    md_api::MdAPI,
    objects::{
        connection::{ConnectionType, Connection},
        node::{ActionType, Node, ScopeType},
    },
    syntax::{flow_chart::FlowChart, FlowDirection},
};
impl Into<ScopeType> for FunctionInfo {
    fn into(self) -> ScopeType {
        if self.is_public{
            ScopeType::Public
        }
        else if !self.is_public{
            ScopeType::Private
        }
        else if self.is_trait_impl{
            ScopeType::Trait
        }
        else if self.is_payable{
            ScopeType::Payable
        }
        else{
            ScopeType::Public
        }
    }
}
impl Into<ActionType> for FunctionInfo {
    fn into(self) -> ActionType {
        if self.is_event{
            ActionType::Event
        }
        else if self.is_mutable{
            ActionType::Mutation
        }
        else if self.is_process{
            ActionType::Process
        }
        else if self.is_view{
            ActionType::View
        }
        else{
            ActionType::Process
        }
    }
}

pub struct ScannerPipeline {
    content: String,
}
impl ScannerPipeline {
    fn from(contract: ContractInfo,flow_direction:FlowDirection) -> ScannerPipeline {
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
        let mut api = MdAPI::<FlowChart>::new(flow_direction, hierarchy_tree_root);
        let result = api.parse_hierarchy();

        ScannerPipeline { content: result }
    }
}
