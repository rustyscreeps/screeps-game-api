use objects::{Room, StructureController, StructureStorage, StructureTerminal};
use constants::{FindConstant, LookConstant, StructureType, Color, ReturnCode};
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
    pub fn create_construction_site<T>(&self, at: T, ty: StructureType) -> ReturnCode
    where
        T: HasPosition, {
        let pos = at.pos();
        js_unwrap!(@{self.as_ref()}.createConstructionSite(
            @{pos.as_ref()},
            __structure_type_num_to_str(@{ty as i32})
        ))
    }

    pub fn create_named_construction_site<T>(
        &self,
        at: T,
        ty: StructureType,
        name: &str,
    ) -> ReturnCode
    where
        T: HasPosition,
    {
        let pos = at.pos();
        js_unwrap!(@{self.as_ref()}.createConstructionSite(
            @{pos.as_ref()},
            __structure_type_num_to_str(@{ty as i32}),
            @{name}
        ))
    }

    pub fn create_flag<T>(
        &self,
        at: T,
        name: &str,
        main_color: Color,
        secondary_color: Color,
    ) -> ReturnCode
    where
        T: HasPosition,
    {
        let pos = at.pos();
        js_unwrap!(@{self.as_ref()}.createFlag(
            @{pos.as_ref()},
            @{name},
            @{main_color as i32},
            @{secondary_color as i32}
        ))
    }

    pub fn find<T>(&self, ty: T) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_array!(@{self.as_ref()}.find(@{ty.find_code()}))
    }

    pub fn look_for_at<T, U>(&self, ty: T, target: U) -> Vec<T::Item>
    where
        T: LookConstant,
        U: HasPosition,
    {
        let pos = target.pos();
        js_unwrap_array!(@{self.as_ref()}.lookForAt(
            __look_num_to_str(@{ty.look_code() as i32}),
            @{pos.as_ref()}
        ))
    }
}
