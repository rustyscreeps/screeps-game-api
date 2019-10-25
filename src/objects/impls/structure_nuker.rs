use crate::{
    constants::ReturnCode,
    objects::{HasPosition, StructureNuker},
};

impl StructureNuker {
    pub fn launch_nuke<T: HasPosition + ?Sized>(&self, target: &T) -> ReturnCode {
        let pos = target.pos();
        js_unwrap! {@{self.as_ref()}.launchNuke(pos_from_packed(@{pos.packed_repr()}))}
    }
}
