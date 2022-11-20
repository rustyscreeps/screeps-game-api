//! Game method implementations on `Position`
use crate::{
    constants::{Color, FindConstant, LookConstant, LookResult, ReturnCode, StructureType},
    objects::RoomPosition,
};
use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;

use super::Position;

impl Position {
    /// Creates a [`ConstructionSite`] at this position. If it's a
    /// [`StructureSpawn`], a name can optionally be assigned for the structure.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.createConstructionSite)
    ///
    /// [`ConstructionSite`]: crate::objects::ConstructionSite
    #[inline]
    pub fn create_construction_site(
        self,
        ty: StructureType,
        name: Option<&JsString>,
    ) -> ReturnCode {
        RoomPosition::from(self).create_construction_site(ty, name)
    }

    /// Creates a [`Flag`] at this position.
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
    ) -> ReturnCode {
        RoomPosition::from(self).create_flag(name, color, secondary_color)
    }

    /// Find the closest object by path among an [`Array`] of objects, or among
    /// a [`FindType`] to search for all objects of that type in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findClosestByPath)
    #[inline]
    pub fn find_closest_by_path<T>(self, ty: T, options: Option<&Object>) -> Option<T::Item>
    where
        T: FindConstant,
        <T as FindConstant>::Item: From<JsValue>,
    {
        RoomPosition::from(self).find_closest_by_path(ty, options)
    }

    /// Find the closest object by range among an [`Array`] of objects, or among
    /// a [`FindType`] to search for all objects of that type in the room. Will
    /// not work for objects in other rooms.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findClosestByRange)
    #[inline]
    pub fn find_closest_by_range<T>(self, ty: T) -> Option<T::Item>
    where
        T: FindConstant,
    {
        RoomPosition::from(self).find_closest_by_range(ty)
    }

    /// Find all relevant objects within a certain range among an [`Array`] of
    /// objects, or among a [`FindType`] to search all objects of that type in
    /// the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findInRange)
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
    pub fn find_path_to(self, goal: &JsValue, options: Option<&Object>) -> Array {
        RoomPosition::from(self).find_path_to(goal, options)
    }

    /// Find a path from this position to the given coordinates in the same
    /// room, with an optional options object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomPosition.findPathTo)
    #[inline]
    pub fn find_path_to_xy(self, x: u8, y: u8, options: Option<&Object>) -> Array {
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
