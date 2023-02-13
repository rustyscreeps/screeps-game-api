//! Game method implementations on `Position`
use crate::{
    constants::{
        look::{LookConstant, LookResult},
        Color, ErrorCode, FindConstant, ReturnCode, StructureType,
    },
    local::{RoomCoordinate, RoomName},
    objects::{CostMatrix, FindPathOptions, Path, RoomPosition},
    pathfinder::RoomCostResult,
    prelude::*,
};
use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

use super::Position;

impl Position {
    /// Creates a [`ConstructionSite`] at this position. If it's a
    /// [`StructureSpawn`], a name can optionally be assigned for the structure.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.createConstructionSite)
    ///
    /// [`ConstructionSite`]: crate::objects::ConstructionSite
    /// [`StructureSpawn`]: crate::objects::StructureSpawn
    #[inline]
    pub fn create_construction_site(
        self,
        ty: StructureType,
        name: Option<&JsString>,
    ) -> ReturnCode {
        RoomPosition::from(self).create_construction_site(ty, name)
    }

    /// Creates a [`Flag`] at this position. If successful, returns the name of
    /// the created flag.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.createFlag)
    ///
    /// [`Flag`]: crate::objects::Flag
    #[inline]
    pub fn create_flag(
        self,
        name: Option<&JsString>,
        color: Option<Color>,
        secondary_color: Option<Color>,
    ) -> Result<JsString, ErrorCode> {
        RoomPosition::from(self).create_flag(name, color, secondary_color)
    }

    // todo typed options and version that allows passing target roomobjects
    /// Find the closest object by path among a list of objects, or use
    /// a [`find` constant] to search for all objects of that type in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findClosestByPath)
    ///
    /// [`find` constant]: crate::constants::find
    #[inline]
    pub fn find_closest_by_path<T>(self, ty: T, options: Option<&Object>) -> Option<T::Item>
    where
        T: FindConstant,
        <T as FindConstant>::Item: From<JsValue>,
    {
        RoomPosition::from(self).find_closest_by_path(ty, options)
    }

    // todo version for passing target roomobjects
    /// Find the closest object by range among a list of objects, or use
    /// a [`find` constant] to search for all objects of that type in the room.
    /// Will not work for objects in other rooms.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findClosestByRange)
    ///
    /// [`find` constant]: crate::constants::find
    #[inline]
    pub fn find_closest_by_range<T>(self, ty: T) -> Option<T::Item>
    where
        T: FindConstant,
    {
        RoomPosition::from(self).find_closest_by_range(ty)
    }

    // todo version for passing target roomobjects
    /// Find all relevant objects within a certain range among a list of
    /// objects, or use a [`find` constant] to search all objects of that type
    /// in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findInRange)
    ///
    /// [`find` constant]: crate::constants::find
    #[inline]
    pub fn find_in_range<T>(self, ty: T, range: u8) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        RoomPosition::from(self).find_in_range(ty, range)
    }

    /// Find a path from this position to a position or room object, with an
    /// optional options object
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findPathTo)
    #[inline]
    pub fn find_path_to<T, F, R>(&self, target: &T, options: Option<FindPathOptions<F, R>>) -> Path
    where
        T: HasPosition,
        F: FnMut(RoomName, CostMatrix) -> R,
        R: RoomCostResult,
    {
        RoomPosition::from(self).find_path_to(target, options)
    }

    /// Find a path from this position to the given coordinates in the same
    /// room, with an optional options object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findPathTo)
    #[inline]
    pub fn find_path_to_xy<F, R>(
        self,
        x: RoomCoordinate,
        y: RoomCoordinate,
        options: Option<FindPathOptions<F, R>>,
    ) -> Path
    where
        F: FnMut(RoomName, CostMatrix) -> R,
        R: RoomCostResult,
    {
        RoomPosition::from(self).find_path_to_xy(x, y, options)
    }

    /// Get all objects at this position.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.look)
    #[inline]
    pub fn look(self) -> Vec<LookResult> {
        RoomPosition::from(self).look()
    }

    /// Get all objects of a given type at this position, if any.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.lookFor)
    #[inline]
    pub fn look_for<T>(self, ty: T) -> Vec<T::Item>
    where
        T: LookConstant,
    {
        RoomPosition::from(self).look_for(ty)
    }
}
