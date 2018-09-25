use constants::ResourceType;
use objects::Resource;

impl Resource {
    pub fn resource_type(&self) -> ResourceType {
        js_unwrap!(__resource_type_str_to_num(@{self.as_ref()}.resourceType))
    }
}

simple_accessors! {
    Resource;
    (amount -> amount -> i32),
    (id -> id -> String)
}

impl PartialEq for Resource {
    fn eq(&self, other: &Resource) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Resource {}
