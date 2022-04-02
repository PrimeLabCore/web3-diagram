mod events;
mod lib;
mod some_fancy_impl;
use mermaid_markdown_api::scanner_pipeline::{ScannerPipeline};
use mermaid_markdown_api::syntax::FlowDirection;
use scanner_syn::contract_descriptor::{ContractDescriptor, DefaultContractDescriptor};
fn main() {
    let desc = DefaultContractDescriptor::new();
    let contract_info=desc.get_contract_info_for_crate();
    let markdown=ScannerPipeline::from(contract_info,FlowDirection::TD);
    println!("{:?}",markdown.content);
}
