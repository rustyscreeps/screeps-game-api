use {
    constants::{ResourceType, ReturnCode},
    objects::StructureTerminal,
};

impl StructureTerminal {
    pub fn send(
        &self,
        resource_type: ResourceType,
        amount: u32,
        destination: &str,
        description: Option<&str>,
    ) -> ReturnCode {
        js_unwrap! {
            @{self.as_ref()}.send(__resource_type_num_to_str(@{resource_type as u32}),
                                  @{amount},
                                  @{destination},
                                  @{description} || undefined)
        }
    }
}
