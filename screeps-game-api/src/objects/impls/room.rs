use stdweb::unstable::TryInto;

use objects::{Room, StructureController, StructureStorage, StructureTerminal};
use constants::{FindConstant, LookConstant};
use HasPosition;

simple_accessors! {
    Room;
    (controller -> controller -> Option<StructureController>),
    (energy_available -> energyAvailable -> i32),
    (energy_capacity_available -> energyCapacityAvailable -> i32),
    (name -> name -> String),
    (storage -> storage -> Option<StructureStorage>),
    (terminal -> terminal -> Option<StructureTerminal>),
}

impl Room {
    pub fn find<T>(&self, ty: T) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_array!(@{&self.0}.find(@{ty.find_code()}))
    }

    pub fn look_for_at<T, U>(&self, ty: T, target: U) -> Vec<T::Item>
    where
        T: LookConstant,
        U: HasPosition,
    {
        let pos = target.pos();
        js_unwrap_array!(@{&self.0}.lookForAt(
            __look_num_to_str(@{ty.look_code() as i32}),
            @{pos.as_ref()}
        ))
    }
}
