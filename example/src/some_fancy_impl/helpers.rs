use crate::lib::*;

#[near_bindgen]
impl Contract {
    pub fn multiply(&mut self, num: u64) {
        self.count *= num;
    }
}

struct ExampleStruct;
impl ExampleStruct {
    pub fn foo(bar: Vec<u8>) -> usize {
        bar.len()
    }
}

pub fn bar() {
    todo!();
}
