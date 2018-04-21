use std::ops::Range;

use constants::{Color, FindConstant, LookConstant, ReturnCode, StructureType};
use objects::{Room, StructureController, StructureStorage, StructureTerminal};
use positions::LocalRoomName;
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
        T: HasPosition,
    {
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

    /// Looks for a given thing over a given area of bounds.
    ///
    /// To keep with `Range` convention, the start is inclusive, and the end
    /// is _exclusive_.
    ///
    /// Note: to ease the implementation and efficiency of the rust interface, this is limited to
    /// returning an array of values without their positions. If position data is needed, all room
    /// objects *should* contain positions alongside them. (for terrain data, I would recommend
    /// using a different method?)
    ///
    /// If you really do need more information here, I would recommend making a PR to add it!
    ///
    /// # Panics
    ///
    /// Panics if start>end for either range, or if end>50 for either range.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use screeps::constants::look;
    /// room.look_for_at_area(look::ENERGY, 20..26, 20..26)
    /// ```
    pub fn look_for_at_area<T>(&self, ty: T, horiz: Range<u8>, vert: Range<u8>) -> Vec<T::Item>
    where
        T: LookConstant,
    {
        assert!(horiz.start <= horiz.end);
        assert!(vert.start <= vert.end);
        assert!(horiz.end <= 50);
        assert!(vert.end <= 50);

        js_unwrap_array!(@{self.as_ref()}.lookForAtArea(
            __look_num_to_str(@{ty.look_code() as i32}),
            @{vert.start},
            @{horiz.start},
            @{vert.end},
            @{horiz.end},
            true
        ).map((obj) => obj[__look_num_to_str(@{ty.look_code() as i32})]))
    }

    pub fn name_local(&self) -> LocalRoomName {
        js_unwrap!(@{self.as_ref()}.name)
    }
}
