use crate::{
    constants::ReturnCode,
    objects::{RoomPosition, StructureNuker},
};

simple_accessors! {
    StructureNuker;
    (ghodium -> ghodium -> u32),
    (ghodium_capacity -> ghodiumCapacity -> u32),
}

impl StructureNuker {
    pub fn launch_nuke(&self, pos: &RoomPosition) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.launchNuke(@{pos.as_ref()})}
    }
}
