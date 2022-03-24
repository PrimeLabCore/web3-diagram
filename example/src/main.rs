mod events;
mod lib;
mod some_fancy_impl;
use scanner_syn::contract_descriptor::{ContractDescriptor, DefaultContractDescriptor};
fn main() {
    let desc = DefaultContractDescriptor::new();
    desc.get_contract_info_for_crate();
}
