use crate::{constants::ReturnCode, macros::*, objects::StructureRampart};
simple_accessors! {
    StructureRampart;
    (is_public -> isPublic -> bool),
}

impl StructureRampart {
    pub fn set_public(&self, is_public: bool) -> ReturnCode {
        js_unwrap! { @{self.as_ref()}.setPublic( @{is_public} ) }
    }
}
