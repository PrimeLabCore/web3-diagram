#![recursion_limit = "128"]
extern crate proc_macro;

mod core_impl;
use self::core_impl::*;
pub mod contract_descriptor;
