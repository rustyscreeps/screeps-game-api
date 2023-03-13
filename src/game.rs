//! The main interface to objects in the Screeps game world.
//!
//! This contains all functionality from the `Game` object in Screeps. That
//! generally means all state which is true this tick throughout the world.
//!
//! # Safety
//!
//! All returned game objects must be used only during the tick they are
//! retreived or resolved. They are considered "stale" on subsequent ticks, and
//! the behavior of stale game objects is undefined.
//!
//! [Screeps documentation](http://docs.screeps.com/api/#Game)

use js_sys::{JsString, Object};

use wasm_bindgen::prelude::*;

use crate::{
    constants::IntershardResourceType,
    js_collections::{JsHashMap, JsObjectId},
    local::{ObjectId, RawObjectId},
    AccountPowerCreep, ConstructionSite, Creep, Flag, RoomName, StructureObject, StructureSpawn,
};

pub mod cpu;
pub mod gcl;
pub mod gpl;
pub mod map;
pub mod market;
pub mod shard;

use crate::{objects::RoomObject, Room};

#[wasm_bindgen]
extern "C" {
    type Game;

    #[wasm_bindgen(static_method_of = Game, getter = constructionSites)]
    fn construction_sites() -> Object;

    #[wasm_bindgen(static_method_of = Game, getter = creeps)]
    fn creeps() -> Object;

    #[wasm_bindgen(static_method_of = Game, getter = flags)]
    fn flags() -> Object;

    #[wasm_bindgen(static_method_of = Game, getter = powerCreeps)]
    fn power_creeps() -> Object;

    #[wasm_bindgen(static_method_of = Game, getter = resources)]
    fn resources() -> Object;

    #[wasm_bindgen(static_method_of = Game, getter = rooms)]
    fn rooms() -> Object;

    #[wasm_bindgen(static_method_of = Game, getter = spawns)]
    fn spawns() -> Object;

    #[wasm_bindgen(static_method_of = Game, getter = structures)]
    fn structures() -> Object;

    #[wasm_bindgen(static_method_of = Game, getter = time)]
    fn time() -> u32;

    #[cfg(feature = "symbols")]
    #[wasm_bindgen(static_method_of = Game, getter = score)]
    fn score() -> u32;

    #[cfg(feature = "symbols")]
    #[wasm_bindgen(static_method_of = Game, getter = symbols)]
    fn symbols() -> Object;

    #[wasm_bindgen(static_method_of = Game, js_name = getObjectById)]
    fn get_object_by_id(id: &JsString) -> Option<RoomObject>;

    #[wasm_bindgen(static_method_of = Game, js_name = notify)]
    fn notify(message: &JsString, group_interval: Option<u32>);
}

/// Get a [`JsHashMap<RawObjectId, ConstructionSite>`] with all of your
/// construction sites.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.constructionSites)
pub fn construction_sites() -> JsHashMap<RawObjectId, ConstructionSite> {
    Game::construction_sites().into()
}

/// Get a [`JsHashMap<String, Creep>`] with all of your creeps, which has creep
/// names as keys.
///
/// Note that newly spawned creeps are immediately added when spawned, but will
/// not have an id until the following tick.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.creeps)
pub fn creeps() -> JsHashMap<String, Creep> {
    Game::creeps().into()
}

/// Get a [`JsHashMap<String, Flag>`] with all of your flags, which has flag
/// names as keys.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.flags)
pub fn flags() -> JsHashMap<String, Flag> {
    Game::flags().into()
}

/// Get a [`JsHashMap<String, AccountPowerCreep>`] with all of your power
/// creeps, which has power creep names as keys.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.powerCreeps)
pub fn power_creeps() -> JsHashMap<String, AccountPowerCreep> {
    Game::power_creeps().into()
}

/// Get a [`JsHashMap<IntershardResourceType, u32>`] with all of your account
/// resources.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.resources)
pub fn resources() -> JsHashMap<IntershardResourceType, u32> {
    Game::resources().into()
}

/// Get an [`JsHashMap<RoomName, Room>`] with the rooms visible for the current
/// tick.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.rooms)
pub fn rooms() -> JsHashMap<RoomName, Room> {
    Game::rooms().into()
}

/// Get an [`JsHashMap<String, StructureSpawn>`] with all of your spawns, which
/// has spawn names as keys.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.spawns)
pub fn spawns() -> JsHashMap<String, StructureSpawn> {
    Game::spawns().into()
}

/// Get an [`JsHashMap<RawObjectId, StructureObject>`] with all of your owned
/// structures.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.spawns)
pub fn structures() -> JsHashMap<RawObjectId, StructureObject> {
    Game::structures().into()
}

/// Get the current time, the number of ticks the game has been running.
///
/// [Screeps documentation](http://docs.screeps.com/api/#Game.time)
pub fn time() -> u32 {
    Game::time()
}

/// Your current score, as determined by the symbols you have decoded.
///
/// [Screeps documentation](https://docs-season.screeps.com/api/#Game.score)
#[cfg(feature = "symbols")]
pub fn score() -> u32 {
    Game::score()
}

/// The symbols you've decoded after multiplier adjustments, used to
/// determine your score.
///
/// [Screeps documentation](https://docs-season.screeps.com/api/#Game.symbols)
#[cfg(feature = "symbols")]
pub fn symbols() -> JsHashMap<crate::ResourceType, u32> {
    Game::symbols().into()
}

/// Get the typed object represented by a given [`JsObjectId`], if it's
/// still alive and visible.
///
/// [Screeps documentation](http://docs.screeps.com/api/#Game.getObjectById)
pub fn get_object_by_js_id_typed<T>(id: &JsObjectId<T>) -> Option<T>
where
    T: From<JsValue>,
{
    Game::get_object_by_id(&id.raw)
        .map(JsValue::from)
        .map(Into::into)
}

/// Get the typed object represented by a given [`ObjectId`], if it's still
/// alive and visible.
///
/// [Screeps documentation](http://docs.screeps.com/api/#Game.getObjectById)
pub fn get_object_by_id_typed<T>(id: &ObjectId<T>) -> Option<T>
where
    T: From<JsValue>,
{
    // construct a reference to a javascript string using the id data
    let js_str = JsString::from(id.to_string());

    Game::get_object_by_id(&js_str)
        .map(JsValue::from)
        .map(Into::into)
}

/// Get the [`RoomObject`] represented by a given [`RawObjectId`], if it's
/// still alive and visible.
///
/// [Screeps documentation](http://docs.screeps.com/api/#Game.getObjectById)
pub fn get_object_by_id_erased(id: &RawObjectId) -> Option<RoomObject> {
    // construct a reference to a javascript string using the id data
    let js_str = JsString::from(id.to_string());

    Game::get_object_by_id(&js_str)
}

/// Send an email message to yourself with a given message. Set a group
/// interval to only send messages every `group_interval` minutes.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.notify)
pub fn notify(message: &str, group_interval: Option<u32>) {
    let message: JsString = message.into();

    Game::notify(&message, group_interval)
}
