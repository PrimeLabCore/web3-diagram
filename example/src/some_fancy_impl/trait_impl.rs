use crate::lib::*;

pub trait SomeTrait {
    fn view_trait_fn(&self) -> u64;
    fn call_trait_fn(&mut self);
}

#[near_bindgen]
impl SomeTrait for Contract {
    fn view_trait_fn(&self) -> u64 {
        self.count * 10
    }

    fn call_trait_fn(&mut self) {
        self.count += 1;
    }
}
