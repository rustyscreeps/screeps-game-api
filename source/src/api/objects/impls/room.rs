use stdweb;
use stdweb::unstable::TryInto;

use api::objects::{Room, StructureController, StructureStorage, StructureTerminal};
use api::constants::{FindConstant, LookConstant};
use api::HasPosition;

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
        let value = js_unwrap!(@{&self.0}.find(@{ty.find_code()}));

        // since find returns not "Array" but array from outside the container,
        // we need to do an unsafe cast to get stdweb to treat it like an array.
        let as_arr: stdweb::Array = unsafe {
            use stdweb::ReferenceType;
            stdweb::Array::from_reference_unchecked(value)
        };

        as_arr
            .try_into()
            .expect("expected Room.find array contain correct types")
    }

    pub fn look_for_at<T, U>(&self, ty: T, target: U) -> Vec<T::Item>
    where
        T: LookConstant,
        U: HasPosition,
    {
        let pos = target.pos();
        let value = js_unwrap!(@{&self.0}.lookForAt(
            __look_num_to_str(@{ty.look_code() as i32}),
            @{pos.as_ref()}
        ));

        let as_arr: stdweb::Array = unsafe {
            use stdweb::ReferenceType;
            stdweb::Array::from_reference_unchecked(value)
        };

        as_arr
            .try_into()
            .expect("expected Room.lookForAt array to contain correct types")
    }
}
