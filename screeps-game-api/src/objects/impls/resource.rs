use objects::Resource;
use constants::ResourceType;

impl Resource {
    pub fn resource_type(&self) -> ResourceType {
        js_unwrap!(__resource_type_str_to_num(@{&self.0}.resourceType))
    }
}

simple_accessors! {
    Resource;
    (amount -> amount -> i32),
}
