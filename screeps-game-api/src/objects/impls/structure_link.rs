use {
    constants::ReturnCode,
    StructureLink,
};

impl StructureLink {
    pub fn transfer_energy(&self, target: &StructureLink, amount: Option<u32>) -> ReturnCode {
        match amount {
            None => js_unwrap!{@{self.as_ref()}.transferEnergy(@{target.as_ref()})},
            Some(amount) => js_unwrap!{@{self.as_ref()}.transferEnergy(@{target.as_ref()}, @{amount})}
        }
    }
}
