use crate::{
    constants::{ResourceType, ReturnCode},
    objects::StructureFactory,
};

simple_accessors! {
    impl StructureFactory {
        pub fn level() -> Option<u32> = level;
    }
}

impl StructureFactory {
    pub fn produce(&self, ty: ResourceType) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.produce(__resource_type_num_to_str(@{ty as u32}))}
    }
}
