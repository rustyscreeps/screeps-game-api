use crate::{
    constants::{ResourceType, ReturnCode},
    objects::StructureFactory,
};

impl StructureFactory {
    pub fn produce(&self, ty: ResourceType) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.produce(__resource_type_num_to_str(@{ty as u32}))}
    }
}
