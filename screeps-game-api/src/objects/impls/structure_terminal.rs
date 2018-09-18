use {
    constants::{ResourceType, ReturnCode},
    StructureTerminal,
};

impl StructureTerminal {
    pub fn send(&self, 
                resource_type: ResourceType, 
                amount: u32,
                destination: &str,
                description: Option<&str>) -> ReturnCode {
        js_unwrap! {
            @{self.as_ref()}.send(@{resource_type.to_string()},
                                  @{amount},
                                  @{destination},
                                  @{description} || undefined)
        }
    }
}
