use crate::{
    md_api::MdAPI,
    objects::{
        connection::{self, Connection, ConnectionType},
        node::{ActionType, Node, ScopeType},
    },
    syntax::{flow_chart::FlowChart, FlowDirection},
};
use scanner_syn::contract_descriptor::{
    ContractDescriptor, ContractInfo, DefaultContractDescriptor, FunctionInfo,
};
use std::{
    ops::{Deref, DerefMut},
    vec::Vec,
};
struct Connections(Vec<Connection>);
impl Deref for Connections {
    type Target = Vec<Connection>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Connections {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<ScopeType> for FunctionInfo {
    fn into(self) -> ScopeType {
        if self.is_public && !self.is_init && !self.is_payable {
            return ScopeType::Public
        }
        if !self.is_public {
            return ScopeType::Private
        }
        if self.is_trait_impl {
            return ScopeType::Trait
        }
        if self.is_init {
            return ScopeType::Initializer
        }
        if self.is_payable {
            return ScopeType::Payable
        } else {
            ScopeType::Public
        }
    }
}
impl Into<ActionType> for FunctionInfo {
    fn into(self) -> ActionType {
        if self.is_event {
            ActionType::Event
        } else if self.is_mutable {
            ActionType::Mutation
        } else if self.is_process {
            ActionType::Process
        } else if self.is_view {
            ActionType::View
        } else {
            ActionType::None
        }
    }
}
impl Into<ConnectionType> for FunctionInfo {
    fn into(self) -> ConnectionType {
        if self.is_event {
            ConnectionType::Emission
        } else if self.is_trait_impl {
            ConnectionType::CrossContractConnection
        } else {
            ConnectionType::DirectConnection
        }
    }
}

impl From<Option<Vec<FunctionInfo>>> for Connections {
    fn from(val: Option<Vec<FunctionInfo>>) -> Self {
        if val.is_some() {
            let finfo = val.unwrap();
            if !finfo.is_empty() {
                let inner = finfo
                    .into_iter()
                    .map(|ifn| -> Connection {
                        Connection {
                            connection_type: ifn.clone().into(),
                            node: Node {
                                name: ifn.name.clone(),
                                scope: ifn.clone().into(),
                                action: ifn.clone().into(),
                                connections: Connections::from(ifn.clone().inner_calls).0,
                            },
                        }
                    })
                    .collect();
                return Connections(inner);
            }
        }
        Connections(Vec::new())
    }
}

pub struct ScannerPipeline {
    pub content: String,
}
impl ScannerPipeline {
    pub fn from(contract: ContractInfo, flow_direction: FlowDirection) -> ScannerPipeline {
        let mut hierarchy_tree_root = Node {
            name: "Contract".to_string(),
            scope: ScopeType::Contract,
            action: ActionType::None,
            connections: Vec::new(),
        };
        contract
            .contract_metadata
            .into_iter()
            .enumerate()
            .for_each(|(_, value)| {
                hierarchy_tree_root
                    .connections
                    .extend(Connections::from(Some(value.fns)).0);
            });

        let mut api = MdAPI::<FlowChart>::new(flow_direction, hierarchy_tree_root);
        let mut result = api.parse_hierarchy();

        result.push_str("\n\rclassDef Public-Mutation fill:#12A5F1,stroke:#333,stroke-width:2px;");
        result.push_str("\n\rclassDef Public-View fill:#12A5F1,stroke:#333,stroke-width:2px;");
        result.push_str("\n\rclassDef Private-View fill:#858585,stroke:#333,stroke-width:2px;");

        result.push_str("\n\rclassDef Private-Mutation fill:#858585,stroke:#333,stroke-width:1px;");
        result.push_str("\n\rclassDef Public-Event fill:#FFDF80,stroke:#333,stroke-width:2px,stroke-dasharray: 4 4");
        result.push_str("\n\rclassDef Private-Event fill:#FFDF80,stroke:#333,stroke-width:1px,stroke-dasharray: 4 4");
        result.push_str("\n\rclassDef Private-None fill:#858585,stroke:#333,stroke-width:1px");
        result.push_str("\n\rclassDef Private-Process fill:#858585,stroke:#333,stroke-width:1px");
        result.push_str("\n\rclassDef Public-Process fill:#858585,stroke:#333,stroke-width:2px");
        result.push_str("\n\rclassDef Public-None fill:#858585,stroke:#333,stroke-width:2px");
        result.push_str("\n\rclassDef Initializer-None fill:#FFA080,stroke:#333,stroke-width:2px");
        result.push_str("\n\rclassDef Payable-None fill:#6AA84F,stroke:#333,stroke-width:2px");
        result.push_str("\n\rclassDef Payable-Mutation fill:#6AA84F,stroke:#333,stroke-width:2px");
        result.push_str("\n\rclassDef Contract-None fill:#C2D5E3,stroke:#333,stroke-width:2px");

        ScannerPipeline { content: result }
    }
}
