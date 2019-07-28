use crate::{
    constants::ReturnCode,
    macros::*,
    objects::{HasPosition, StructureNuker},
};

simple_accessors! {
    StructureNuker;
    (ghodium -> ghodium -> u32),
    (ghodium_capacity -> ghodiumCapacity -> u32),
}

impl StructureNuker {
    pub fn launch_nuke<T: HasPosition + ?Sized>(&self, target: &T) -> ReturnCode {
        let pos = target.pos();
        js_unwrap! {@{self.as_ref()}.launchNuke(pos_from_packed(@{pos.packed_repr()}))}
    }
}
