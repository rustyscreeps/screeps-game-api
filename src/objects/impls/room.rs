use std::convert::TryInto;

use crate::{
    constants::{look::*, Color, ExitDirection, Find, Look, ReturnCode, StructureType},
    containers::JsContainerFromValue,
    objects::*,
    prelude::*,
    FindConstant, RoomCostResult, RoomName,
};

#[cfg(not(feature = "disable-terminal"))]
use crate::objects::StructureTerminal;

use js_sys::{Array, JsString, Object};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    /// A reference to a [`Room`] object, a 50x50 chunk of the Screeps game
    /// world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room)
    #[derive(Clone)]
    pub type Room;

    /// The [`StructureController`] for the room, or `None` in rooms that cannot
    /// be claimed.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.controller)
    #[wasm_bindgen(method, getter)]
    pub fn controller(this: &Room) -> Option<StructureController>;

    /// Energy available for spawning at the start of the current tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.energyAvailable)
    #[wasm_bindgen(method, getter = energyAvailable)]
    pub fn energy_available(this: &Room) -> u32;

    /// Total energy capacity of all spawns and extensions in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.energyCapacityAvailable)
    #[wasm_bindgen(method, getter = energyCapacityAvailable)]
    pub fn energy_capacity_available(this: &Room) -> u32;

    /// A shortcut to `Memory.rooms[room.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.memory)
    #[wasm_bindgen(method, getter)]
    pub fn memory(this: &Room) -> JsValue;

    /// Sets a new value to `Memory.rooms[room.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.memory)
    #[wasm_bindgen(method, setter)]
    pub fn set_memory(this: &Room, val: &JsValue);

    /// The room's name as an owned reference to a [`JsString`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.name)
    #[wasm_bindgen(method, getter = name)]
    fn name_internal(this: &Room) -> JsString;

    /// The [`StructureStorage`] built in the room, or `None` in rooms where
    /// there isn't one.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.storage)
    #[wasm_bindgen(method, getter)]
    pub fn storage(this: &Room) -> Option<StructureStorage>;

    /// The [`StructureTerminal`] built in the room, or `None` in rooms where
    /// there isn't one.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.terminal)
    #[cfg(not(feature = "disable-terminal"))]
    #[wasm_bindgen(method, getter)]
    pub fn terminal(this: &Room) -> Option<StructureTerminal>;

    /// Serialize a path array from [`Room::find_path`] into a string
    /// representation safe to store in memory.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.serializePath)
    #[wasm_bindgen(static_method_of = Room, js_name = serializePath)]
    pub fn serialize_path(path: &Array) -> JsString;

    /// Deserialize a string representation from [`Room::serialize_path`] back
    /// to a path array.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.deserializePath)
    #[wasm_bindgen(static_method_of = Room, js_name = deserializePath)]
    pub fn deserialize_path(path: &JsString) -> Array;

    /// Creates a construction site at given coordinates within this room. If
    /// it's a [`StructureSpawn`], a name can optionally be assigned for the
    /// structure.
    ///
    /// See [`RoomPosition::create_construction_site`] to create at a specified
    /// position.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.createConstructionSite)
    ///
    /// [`StructureSpawn`]: crate::objects::StructureSpawn
    /// [`RoomPosition::create_construction_site`]:
    /// crate::objects::RoomPosition::create_construction_site
    #[wasm_bindgen(final, method, js_name = createConstructionSite)]
    pub fn create_construction_site(
        this: &Room,
        x: u8,
        y: u8,
        ty: StructureType,
        name: Option<&JsString>,
    ) -> ReturnCode;

    /// Creates a [`Flag`] at given coordinates within this room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.createFlag)
    #[wasm_bindgen(final, method, js_name = createFlag)]
    pub fn create_flag(
        this: &Room,
        x: u8,
        y: u8,
        name: Option<&JsString>,
        color: Option<Color>,
        secondary_color: Option<Color>,
    ) -> ReturnCode;

    /// Find all objects of the specified type in the room, without passing
    /// additional options.
    ///
    /// Returns an [`Array`] containing the found objects, which should be
    /// converted into the type of object you searched for.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.find)
    #[wasm_bindgen(method, js_name = find)]
    //TODO: wiarchbe: Find options!
    fn find_internal(this: &Room, ty: Find, options: Option<&Object>) -> Array;

    /// Find an exit from the current room which leads to a target room, either
    /// a [`Room`] object or [`JsString`] representation of the room name.
    ///
    /// Returns an [`Array`] containing the found objects, which should be
    /// converted into the type of object you searched for.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.findExitTo)
    #[wasm_bindgen(final, method, js_name = findExitTo)]
    pub fn find_exit_to(this: &Room, room: &JsValue) -> ExitDirection;

    // todo FindPathOptions
    /// Find a path within the room from one position to another.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.findPath)
    #[wasm_bindgen(final, method, js_name = findPath)]
    pub fn find_path(
        this: &Room,
        origin: &RoomPosition,
        goal: &RoomPosition,
        options: Option<&Object>,
    ) -> Array;

    // todo event log

    /// Gets the [`RoomPosition`] for the given coordinates.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.getPositionAt)
    #[wasm_bindgen(final, method, js_name = getPositionAt)]
    pub fn get_position_at(this: &Room, x: u8, y: u8) -> RoomPosition;

    /// Gets the [`RoomTerrain`] object for this room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.getTerrain)
    #[wasm_bindgen(final, method, js_name = getTerrain)]
    pub fn get_terrain(this: &Room) -> RoomTerrain;

    /// Get an array of all objects at a position.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.lookAt)
    #[wasm_bindgen(final, method, js_name = lookAt)]
    pub fn look_at(this: &Room, target: &RoomPosition) -> Array;

    /// Get an array of all objects at the given coordinates.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.lookAt)
    #[wasm_bindgen(final, method, js_name = lookAt)]
    pub fn look_at_xy(this: &Room, x: u8, y: u8) -> Array;

    /// Get an array of all objects in a certain area, in either object or array
    /// format depending on the `as_array` option.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.lookAtArea)
    #[wasm_bindgen(final, method, js_name = lookAtArea)]
    pub fn look_at_area(
        this: &Room,
        top_y: u8,
        left_x: u8,
        bottom_y: u8,
        right_x: u8,
        as_array: bool,
    ) -> JsValue;

    /// Get an array of all objects of a given type at this position, if any.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.lookForAt)
    #[wasm_bindgen(method, js_name = lookForAt)]
    fn look_for_at_internal(this: &Room, ty: Look, target: &RoomPosition) -> Option<Array>;

    /// Get an array of all objects of a given type at the given coordinates, if
    /// any.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.lookForAt)
    #[wasm_bindgen(method, js_name = lookForAt)]
    fn look_for_at_xy_internal(this: &Room, ty: Look, x: u8, y: u8) -> Option<Array>;

    /// Get an array of all objects in a certain area, in either object or array
    /// format depending on the `as_array` option.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.lookAtArea)
    #[wasm_bindgen(method, js_name = lookForAtArea)]
    fn look_for_at_area_internal(
        this: &Room,
        ty: Look,
        top_y: u8,
        left_x: u8,
        bottom_y: u8,
        right_x: u8,
        as_array: bool,
    ) -> JsValue;
}

impl Room {
    pub fn name(&self) -> RoomName {
        self.name_internal()
            .try_into()
            .expect("expected parseable room name")
    }

    pub fn visual(&self) -> RoomVisual {
        RoomVisual::new(Some(self.name()))
    }

    //TODO: wiarchbe: Find options!
    pub fn find<T>(&self, ty: T) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        self.find_internal(ty.find_code(), None)
            .iter()
            .map(T::convert_and_check_item)
            .collect()
    }

    pub fn look_for_at<T, U>(&self, _ty: T, target: &U) -> Vec<T::Item>
    where
        T: LookConstant,
        U: HasPosition,
    {
        let pos = target.pos().into();

        self.look_for_at_internal(T::look_code(), &pos)
            .map(|arr| arr.iter().map(T::convert_and_check_item).collect())
            .unwrap_or_else(Vec::new)
    }

    pub fn look_for_at_xy<T>(&self, _ty: T, x: u8, y: u8) -> Vec<T::Item>
    where
        T: LookConstant,
    {
        self.look_for_at_xy_internal(T::look_code(), x, y)
            .map(|arr| arr.iter().map(T::convert_and_check_item).collect())
            .unwrap_or_else(Vec::new)
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Room) -> bool {
        self.name() == other.name()
    }
}

impl Eq for Room {}

impl JsContainerFromValue for Room {
    fn from_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type JsFindOptions;

    #[wasm_bindgen(method, setter = ignoreCreeps)]
    pub fn ignore_creeps(this: &JsFindOptions, ignore: bool);

    #[wasm_bindgen(method, setter = ignoreDestructibleStructures)]
    pub fn ignore_destructible_structures(this: &JsFindOptions, ignore: bool);

    #[wasm_bindgen(method, setter = costCallback)]
    pub fn cost_callback(
        this: &JsFindOptions,
        callback: &Closure<dyn FnMut(JsString, CostMatrix) -> JsValue>,
    );

    #[wasm_bindgen(method, setter = maxOps)]
    pub fn max_ops(this: &JsFindOptions, ops: u32);

    #[wasm_bindgen(method, setter = heuristicWeight)]
    pub fn heuristic_weight(this: &JsFindOptions, weight: f64);

    #[wasm_bindgen(method, setter = serialize)]
    pub fn serialize(this: &JsFindOptions, serialize: bool);

    #[wasm_bindgen(method, setter = maxRooms)]
    pub fn max_rooms(this: &JsFindOptions, max: u8);

    #[wasm_bindgen(method, setter = range)]
    pub fn range(this: &JsFindOptions, range: u32);

    #[wasm_bindgen(method, setter = plainCost)]
    pub fn plain_cost(this: &JsFindOptions, cost: u8);

    #[wasm_bindgen(method, setter = swampCost)]
    pub fn swamp_cost(this: &JsFindOptions, cost: u8);
}

impl JsFindOptions {
    pub fn new() -> JsFindOptions {
        Object::new().unchecked_into()
    }
}

pub struct FindOptions<F, R>
where
    F: FnMut(RoomName, CostMatrix) -> R,
    R: RoomCostResult,
{
    pub(crate) ignore_creeps: Option<bool>,
    pub(crate) ignore_destructible_structures: Option<bool>,
    pub(crate) cost_callback: F,
    pub(crate) max_ops: Option<u32>,
    pub(crate) heuristic_weight: Option<f64>,
    pub(crate) serialize: Option<bool>,
    pub(crate) max_rooms: Option<u8>,
    pub(crate) range: Option<u32>,
    pub(crate) plain_cost: Option<u8>,
    pub(crate) swamp_cost: Option<u8>,
}

impl<R> Default for FindOptions<fn(RoomName, CostMatrix) -> R, R>
where
    R: RoomCostResult + Default,
{
    fn default() -> Self {
        FindOptions {
            ignore_creeps: None,
            ignore_destructible_structures: None,
            cost_callback: |_, _| R::default(),
            max_ops: None,
            heuristic_weight: None,
            serialize: None,
            max_rooms: None,
            range: None,
            plain_cost: None,
            swamp_cost: None,
        }
    }
}

impl<R> FindOptions<fn(RoomName, CostMatrix) -> R, R>
where
    R: RoomCostResult + Default,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<F, R> FindOptions<F, R>
where
    F: FnMut(RoomName, CostMatrix) -> R,
    R: RoomCostResult,
{
    /// Sets whether the algorithm considers creeps as walkable. Default: False.
    pub fn ignore_creeps(mut self, ignore: bool) -> Self {
        self.ignore_creeps = Some(ignore);
        self
    }

    /// Sets whether the algorithm considers destructible structure as
    /// walkable. Default: False.
    pub fn ignore_destructible_structures(mut self, ignore: bool) -> Self {
        self.ignore_destructible_structures = Some(ignore);
        self
    }

    /// Sets cost callback - default `|_, _| {}`.
    pub fn cost_callback<F2, R2>(self, cost_callback: F2) -> FindOptions<F2, R2>
    where
        F2: FnMut(RoomName, CostMatrix) -> R2,
        R2: RoomCostResult,
    {
        let FindOptions {
            ignore_creeps,
            ignore_destructible_structures,
            max_ops,
            heuristic_weight,
            serialize,
            max_rooms,
            range,
            plain_cost,
            swamp_cost,
            ..
        } = self;

        FindOptions {
            ignore_creeps,
            ignore_destructible_structures,
            cost_callback,
            max_ops,
            heuristic_weight,
            serialize,
            max_rooms,
            range,
            plain_cost,
            swamp_cost,
        }
    }

    /// Sets maximum ops - default `2000`.
    pub fn max_ops(mut self, ops: u32) -> Self {
        self.max_ops = Some(ops);
        self
    }

    /// Sets heuristic weight - default `1.2`.
    pub fn heuristic_weight(mut self, weight: f64) -> Self {
        self.heuristic_weight = Some(weight);
        self
    }

    /// Sets whether the returned path should be passed to `Room.serializePath`.
    pub fn serialize(mut self, s: bool) -> Self {
        self.serialize = Some(s);
        self
    }

    /// Sets maximum rooms - default `16`, max `16`.
    pub fn max_rooms(mut self, rooms: u8) -> Self {
        self.max_rooms = Some(rooms);
        self
    }

    pub fn range(mut self, k: u32) -> Self {
        self.range = Some(k);
        self
    }

    /// Sets plain cost - default `1`.
    pub fn plain_cost(mut self, cost: u8) -> Self {
        self.plain_cost = Some(cost);
        self
    }

    /// Sets swamp cost - default `5`.
    pub fn swamp_cost(mut self, cost: u8) -> Self {
        self.swamp_cost = Some(cost);
        self
    }

    pub(crate) fn as_js_options<CR>(self, callback: impl Fn(&JsFindOptions) -> CR) -> CR {
        let mut raw_callback = self.cost_callback;

        let mut owned_callback = move |room: RoomName, cost_matrix: CostMatrix| -> JsValue {
            raw_callback(room, cost_matrix).into()
        };

        //
        // Type erased and boxed callback: no longer a type specific to the closure
        // passed in, now unified as &Fn
        //

        let callback_type_erased: &mut (dyn FnMut(RoomName, CostMatrix) -> JsValue) =
            &mut owned_callback;

        // Overwrite lifetime of reference so it can be passed to javascript.
        // It's now pretending to be static data. This should be entirely safe
        // because we control the only use of it and it remains valid during the
        // pathfinder callback. This transmute is necessary because "some lifetime
        // above the current scope but otherwise unknown" is not a valid lifetime.
        //

        let callback_lifetime_erased: &'static mut (dyn FnMut(RoomName, CostMatrix) -> JsValue) =
            unsafe { std::mem::transmute(callback_type_erased) };

        let boxed_callback = Box::new(move |room: JsString, cost_matrix: CostMatrix| -> JsValue {
            let room = room
                .try_into()
                .expect("expected room name in cost callback");

            callback_lifetime_erased(room, cost_matrix)
        }) as Box<dyn FnMut(JsString, CostMatrix) -> JsValue>;

        let closure = Closure::wrap(boxed_callback);

        //
        // Create JS object and set properties.
        //

        let js_options = JsFindOptions::new();

        js_options.cost_callback(&closure);

        if let Some(ignore_creeps) = self.ignore_creeps {
            js_options.ignore_creeps(ignore_creeps);
        }

        if let Some(ignore_destructible_structures) = self.ignore_destructible_structures {
            js_options.ignore_destructible_structures(ignore_destructible_structures);
        }

        if let Some(max_ops) = self.max_ops {
            js_options.max_ops(max_ops);
        }

        if let Some(heuristic_weight) = self.heuristic_weight {
            js_options.heuristic_weight(heuristic_weight);
        }

        if let Some(serialize) = self.serialize {
            js_options.serialize(serialize);
        }

        if let Some(max_rooms) = self.max_rooms {
            js_options.max_rooms(max_rooms);
        }

        if let Some(range) = self.range {
            js_options.range(range);
        }

        if let Some(plain_cost) = self.plain_cost {
            js_options.plain_cost(plain_cost);
        }

        if let Some(swamp_cost) = self.swamp_cost {
            js_options.swamp_cost(swamp_cost);
        }

        callback(&js_options)
    }
}

// impl Room {
//     pub fn get_event_log(&self) -> Vec<Event> {
//         serde_json::from_str(&self.get_event_log_raw()).expect("Malformed
// Event Log")     }

//     pub fn get_event_log_raw(&self) -> String {
//         js_unwrap! {@{self.as_ref()}.getEventLog(true)}
//     }

// }

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Step {
//     pub x: u32,
//     pub y: u32,
//     pub dx: i32,
//     pub dy: i32,
//     pub direction: Direction,
// }

// js_deserializable! {Step}
// js_serializable! {Step}

// #[derive(Debug, Deserialize)]
// #[serde(untagged)]
// pub enum Path {
//     Vectorized(Vec<Step>),
//     Serialized(String),
// }

// js_deserializable! {Path}

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Event {
//     pub event: EventType,
//     pub object_id: String,
// }

// impl<'de> Deserialize<'de> for Event {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         #[derive(Deserialize)]
//         #[serde(field_identifier, rename_all = "camelCase")]
//         enum Field {
//             Event,
//             ObjectId,
//             Data,
//         };

//         struct EventVisitor;

//         impl<'de> Visitor<'de> for EventVisitor {
//             type Value = Event;

//             fn expecting(&self, formatter: &mut fmt::Formatter<'_>) ->
// fmt::Result {                 formatter.write_str("struct Event")
//             }

//             fn visit_map<V>(self, mut map: V) -> Result<Event, V::Error>
//             where
//                 V: MapAccess<'de>,
//             {
//                 let mut event_type = None;
//                 let mut obj_id = None;
//                 let mut data = None;
//                 let mut data_buffer: Option<serde_json::Value> = None;

//                 while let Some(key) = map.next_key()? {
//                     match key {
//                         Field::Event => {
//                             if event_type.is_some() {
//                                 return
// Err(de::Error::duplicate_field("event"));                             }
//                             event_type = Some(map.next_value()?);
//                         }
//                         Field::ObjectId => {
//                             if obj_id.is_some() {
//                                 return
// Err(de::Error::duplicate_field("objectId"));                             }
//                             obj_id = Some(map.next_value()?);
//                         }
//                         Field::Data => {
//                             if data.is_some() {
//                                 return
// Err(de::Error::duplicate_field("data"));                             }

//                             match event_type {
//                                 None => data_buffer = map.next_value()?,
//                                 Some(event_id) => {
//                                     data = match event_id {
//                                         1 =>
// Some(EventType::Attack(map.next_value()?)),
// 2 => Some(EventType::ObjectDestroyed(map.next_value()?)),
// 3 => Some(EventType::AttackController),
// 4 => Some(EventType::Build(map.next_value()?)),
// 5 => Some(EventType::Harvest(map.next_value()?)),
// 6 => Some(EventType::Heal(map.next_value()?)),
// 7 => Some(EventType::Repair(map.next_value()?)),
// 8 => Some(EventType::ReserveController(map.next_value()?)),
// 9 => Some(EventType::UpgradeController(map.next_value()?)),
// 10 => Some(EventType::Exit(map.next_value()?)),
// 11 => Some(EventType::Power(map.next_value()?)),
// 12 => Some(EventType::Transfer(map.next_value()?)),
// _ => {                                             return
// Err(de::Error::custom(format!(
// "Event Type Unrecognized: {}",
// event_id                                             )));
//                                         }
//                                     };
//                                 }
//                             };
//                         }
//                     }
//                 }

//                 if data.is_none() {
//                     let err = |e| {
//                         de::Error::custom(format_args!(
//                             "can't parse event data due to inner error {}",
//                             e
//                         ))
//                     };

//                     if let (Some(val), Some(event_id)) = (data_buffer,
// event_type) {                         data = match event_id {
//                             1 =>
// Some(EventType::Attack(serde_json::from_value(val).map_err(err)?)),
//                             2 => Some(EventType::ObjectDestroyed(
//                                 serde_json::from_value(val).map_err(err)?,
//                             )),
//                             3 => Some(EventType::AttackController),
//                             4 =>
// Some(EventType::Build(serde_json::from_value(val).map_err(err)?)),
//                             5 => Some(EventType::Harvest(
//                                 serde_json::from_value(val).map_err(err)?,
//                             )),
//                             6 =>
// Some(EventType::Heal(serde_json::from_value(val).map_err(err)?)),
// 7 => Some(EventType::Repair(serde_json::from_value(val).map_err(err)?)),
//                             8 => Some(EventType::ReserveController(
//                                 serde_json::from_value(val).map_err(err)?,
//                             )),
//                             9 => Some(EventType::UpgradeController(
//                                 serde_json::from_value(val).map_err(err)?,
//                             )),
//                             10 =>
// Some(EventType::Exit(serde_json::from_value(val).map_err(err)?)),
// 11 => Some(EventType::Power(serde_json::from_value(val).map_err(err)?)),
//                             12 => Some(EventType::Transfer(
//                                 serde_json::from_value(val).map_err(err)?,
//                             )),
//                             _ => {
//                                 return Err(de::Error::custom(format!(
//                                     "Event Type Unrecognized: {}",
//                                     event_id
//                                 )));
//                             }
//                         };
//                     }
//                 }

//                 let data = data.ok_or_else(||
// de::Error::missing_field("data"))?;                 let obj_id =
// obj_id.ok_or_else(|| de::Error::missing_field("objectId"))?;

//                 Ok(Event {
//                     event: data,
//                     object_id: obj_id,
//                 })
//             }
//         }

//         const FIELDS: &[&str] = &["event", "objectId", "data"];
//         deserializer.deserialize_struct("Event", FIELDS, EventVisitor)
//     }
// }

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum EventType {
//     Attack(AttackEvent),
//     ObjectDestroyed(ObjectDestroyedEvent),
//     AttackController,
//     Build(BuildEvent),
//     Harvest(HarvestEvent),
//     Heal(HealEvent),
//     Repair(RepairEvent),
//     ReserveController(ReserveControllerEvent),
//     UpgradeController(UpgradeControllerEvent),
//     Exit(ExitEvent),
//     Power(PowerEvent),
//     Transfer(TransferEvent),
// }

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct AttackEvent {
//     pub target_id: String,
//     pub damage: u32,
//     pub attack_type: AttackType,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
// #[repr(u8)]
// pub enum AttackType {
//     Melee = 1,
//     Ranged = 2,
//     RangedMass = 3,
//     Dismantle = 4,
//     HitBack = 5,
//     Nuke = 6,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
// pub struct ObjectDestroyedEvent {
//     #[serde(rename = "type")]
//     pub object_type: String,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct BuildEvent {
//     pub target_id: String,
//     pub amount: u32,
//     pub energy_spent: u32,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct HarvestEvent {
//     pub target_id: String,
//     pub amount: u32,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct HealEvent {
//     pub target_id: String,
//     pub amount: u32,
//     pub heal_type: HealType,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
// #[repr(u8)]
// pub enum HealType {
//     Melee = 1,
//     Ranged = 2,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct RepairEvent {
//     pub target_id: String,
//     pub amount: u32,
//     pub energy_spent: u32,
// }

// #[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ReserveControllerEvent {
//     pub amount: u32,
// }

// #[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct UpgradeControllerEvent {
//     pub amount: u32,
//     pub energy_spent: u32,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ExitEvent {
//     pub room: String,
//     pub x: u32,
//     pub y: u32,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TransferEvent {
//     pub target_id: String,
//     #[serde(deserialize_with = "crate::ResourceType::deserialize_from_str")]
//     pub resource_type: ResourceType,
//     pub amount: u32,
// }

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PowerEvent {
//     pub target_id: String,
//     pub power: PowerType,
// }

// pub enum LookResult {
//     Creep(Creep),
//     Energy(Resource),
//     Resource(Resource),
//     Source(Source),
//     Mineral(Mineral),
//     Deposit(Deposit),
//     Structure(Structure),
//     Flag(Flag),
//     ConstructionSite(ConstructionSite),
//     Nuke(Nuke),
//     Terrain(Terrain),
//     Tombstone(Tombstone),
//     PowerCreep(PowerCreep),
//     Ruin(Ruin),
// }

// impl TryFrom<Value> for LookResult {
//     type Error = ConversionError;

//     fn try_from(v: Value) -> Result<LookResult, Self::Error> {
//         let look_type = js! (
//             return __look_str_to_num(@{&v}.type);
//         )
//         .try_into()?;

//         let lr = match look_type {
//             Look::Creeps => LookResult::Creep(js_unwrap_ref!(@{v}.creep)),
//             Look::Energy => LookResult::Energy(js_unwrap_ref!(@{v}.energy)),
//             Look::Resources =>
// LookResult::Resource(js_unwrap_ref!(@{v}.resource)),
// Look::Sources => LookResult::Source(js_unwrap_ref!(@{v}.source)),
// Look::Minerals => LookResult::Mineral(js_unwrap_ref!(@{v}.mineral)),
//             Look::Deposits =>
// LookResult::Deposit(js_unwrap_ref!(@{v}.deposit)),
// Look::Structures => LookResult::Structure(js_unwrap_ref!(@{v}.structure)),
//             Look::Flags => LookResult::Flag(js_unwrap_ref!(@{v}.flag)),
//             Look::ConstructionSites => {
//
// LookResult::ConstructionSite(js_unwrap_ref!(@{v}.constructionSite))
//             }
//             Look::Nukes => LookResult::Nuke(js_unwrap_ref!(@{v}.nuke)),
//             Look::Terrain =>
// LookResult::Terrain(js_unwrap!(__terrain_str_to_num(@{v}.terrain))),
//             Look::Tombstones =>
// LookResult::Tombstone(js_unwrap_ref!(@{v}.tombstone)),
// Look::PowerCreeps => LookResult::PowerCreep(js_unwrap_ref!(@{v}.powerCreep)),
//             Look::Ruins => LookResult::Ruin(js_unwrap_ref!(@{v}.ruin)),
//         };
//         Ok(lr)
//     }
// }

// pub struct PositionedLookResult {
//     pub x: u32,
//     pub y: u32,
//     pub look_result: LookResult,
// }

// impl TryFrom<Value> for PositionedLookResult {
//     type Error = ConversionError;

//     fn try_from(v: Value) -> Result<PositionedLookResult, Self::Error> {
//         let x: u32 = js!(return @{&v}.x;).try_into()?;
//         let y: u32 = js!(return @{&v}.y;).try_into()?;
//         let look_result: LookResult = v.try_into()?;

//         Ok(PositionedLookResult { x, y, look_result })
//     }
// }
