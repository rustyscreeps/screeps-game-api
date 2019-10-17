use crate::{
    constants::ReturnCode,
    objects::{HasPosition, StructureNuker},
};

simple_accessors! {
    impl StructureNuker {
        pub fn ghodium() -> u32 = ghodium;
        pub fn ghodium_capacity() -> u32 = ghodiumCapacity;
    }
}

impl StructureNuker {
    pub fn launch_nuke<T: HasPosition + ?Sized>(&self, target: &T) -> ReturnCode {
        let pos = target.pos();
        js_unwrap! {@{self.as_ref()}.launchNuke(pos_from_packed(@{pos.packed_repr()}))}
    }
}
