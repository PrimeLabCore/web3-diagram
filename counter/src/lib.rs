use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};
use near_sdk::borsh;


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    count: u64,
}

#[near_bindgen]
impl Contract {

    // TODO: init method is broken
    #[init]
    pub fn new(count: u64) -> Self {
        Self { count }
    }

    pub fn add(&mut self, amount: u64) {
        if amount == 2 {
            self.count = SomeStruct::add_two(self.count);
        }
        self.add_amount(amount);
    }

    pub fn show_amount(self) -> u64 {
        self.count
    }

    #[payable]
    fn add_amount(&mut self, amount: u64) {
        self.count += amount;
    }
}

#[near_bindgen]
pub struct SomeStruct{
}
#[near_bindgen]
impl SomeStruct{
pub fn add_two(count: u64) -> u64 {
    count + 2
}
}