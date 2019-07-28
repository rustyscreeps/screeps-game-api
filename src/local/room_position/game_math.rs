//! Utilities for doing math on [`LocalRoomPosition`]s which are present in the
//! JavaScript API.
use crate::{constants::Direction, objects::HasPosition};

use super::LocalRoomPosition;

impl LocalRoomPosition {
    pub fn get_direction_to<T>(self, target: &T) -> Option<Direction>
    where
        T: ?Sized + HasPosition,
    {
        // Logic copied from https://github.com/screeps/engine/blob/
        // 020ba168a1fde9a8072f9f1c329d5c0be8b440d7/src/utils.js#L73-L107
        let (dx, dy) = self - target.pos();
        if dx.abs() > dy.abs() * 2 {
            if dx > 0 {
                Some(Direction::Right)
            } else {
                Some(Direction::Left)
            }
        } else if dy.abs() > dx.abs() * 2 {
            if dy > 0 {
                Some(Direction::Bottom)
            } else {
                Some(Direction::Top)
            }
        } else {
            if dx > 0 && dy > 0 {
                Some(Direction::BottomRight)
            } else if dx > 0 && dy < 0 {
                Some(Direction::TopRight)
            } else if dx < 0 && dy > 0 {
                Some(Direction::BottomLeft)
            } else if dx < 0 && dy < 0 {
                Some(Direction::TopLeft)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn get_range_to<T>(self, target: &T) -> u32
    where
        T: ?Sized + HasPosition,
    {
        let (dx, dy) = self - target.pos();
        dx.abs().max(dy.abs()) as u32
    }

    #[inline]
    pub fn in_range_to<T>(self, target: &T, range: u32) -> bool
    where
        T: ?Sized + HasPosition,
    {
        self.get_range_to(target) < range
    }

    #[inline]
    pub fn is_equal_to<T>(self, target: &T) -> bool
    where
        T: ?Sized + HasPosition,
    {
        self == target.pos()
    }

    /// True if this position is in the same room as the target, and the range
    /// is at most 1.
    #[inline]
    pub fn is_near_to<T>(self, target: &T) -> bool
    where
        T: ?Sized + HasPosition,
    {
        let pos = target.pos();
        self.room_name() == pos.room_name()
            && (self.x() as i32 - pos.x() as i32).abs() <= 1
            && (self.y() as i32 - pos.y() as i32).abs() <= 1
    }
}
