use crate::md_api::objects::node::Node;
use crate::md_api::syntax::flow_chart::FlowChart;
use crate::md_api::syntax::{CoreSyntaxFunctions, FlowDirection};

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
}
