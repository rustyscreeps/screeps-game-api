use crate::{constants::ReturnCode, objects::StructureRampart};

simple_accessors! {
    impl StructureRampart {
        pub fn is_public() -> bool = isPublic;
    }
}

impl StructureRampart {
    pub fn set_public(&self, is_public: bool) -> ReturnCode {
        js_unwrap! { @{self.as_ref()}.setPublic( @{is_public} ) }
    }
}
