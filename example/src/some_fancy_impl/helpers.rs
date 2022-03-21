use crate::*;

#[near_bindgen]
impl Contract {
    pub fn multiply(&mut self, num: u64) {
        self.count *= num;
    }
}