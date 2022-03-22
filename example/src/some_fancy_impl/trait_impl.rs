use crate::*;

pub trait SomeTrait {
    fn view_trait_fn(&self) -> u64;
    fn call_trait_fn(&mut self);
}

#[near_bindgen]
impl SomeTrait for Contract {
    fn view_trait_fn(&self) {
        self.counter * 10
    }

    fn call_trait_fn(&mut self) {
        self.counter += 1;
    }
}
