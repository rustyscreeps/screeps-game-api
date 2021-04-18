use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;
use crate::{RoomCostResult, RoomName, objects::*};
use crate::enums::StructureObject;
use wasm_bindgen::JsCast;

/// Translates `FIND_*` constants.
#[wasm_bindgen]
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash,
)]
#[repr(u16)]
pub enum Find {
    /// Find all exit positions at the top of the room
    ExitTop = 1,
    ExitRight = 3,
    ExitBottom = 5,
    ExitLeft = 7,
    Exit = 10,
    Creeps = 101,
    MyCreeps = 102,
    HostileCreeps = 103,
    SourcesActive = 104,
    Sources = 105,
    DroppedResources = 106,
    Structures = 107,
    MyStructures = 108,
    HostileStructures = 109,
    Flags = 110,
    ConstructionSites = 111,
    MySpawns = 112,
    HostileSpawns = 113,
    MyConstructionSites = 114,
    HostileConstructionSites = 115,
    Minerals = 116,
    Nukes = 117,
    Tombstones = 118,
    PowerCreeps = 119,
    MyPowerCreeps = 120,
    HostilePowerCreeps = 121,
    Deposits = 122,
    Ruins = 123,
    // todo these seem to not work when conditionally compiled out - they're not hurting to leave
    // in but need to figure that out
    //#[cfg(feature = "enable-score")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreContainers = 10011,
    //#[cfg(feature = "enable-score")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreCollectors = 10012,
    //#[cfg(feature = "enable-symbols")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
    SymbolContainers = 10021,
    //#[cfg(feature = "enable-symbols")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
    SymbolDecoders = 10022,
}

/// Trait representing things which can be used in the 'find' function.
///
/// Typically used with zero-sized structs in the
/// [`find`][crate::constants::find] module.
pub trait FindConstant {
    type Item: From<JsValue>;

    fn find_code(&self) -> Find;
}

/// Useful for finding any [`RoomObject`] with
/// a dynamically-chosen find constant.
///
/// If you know ahead of time what constant you'll use, then the
/// all-upper-case constants in [this module][crate::constants::find] will
/// be more helpful, and won't require casting the result types.
///
/// *Note*: To avoid ambiguity with [`RoomObject`], you should refer to this
/// enum as `find::RoomObject` rather than importing it directly.
///
/// [`RoomObject`]: crate::objects::RoomObject
#[derive(
    Debug, PartialEq, Eq, Clone, Copy, Hash,
)]
#[repr(i16)]
pub enum RoomObject {
    Creeps = 101,
    MyCreeps = 102,
    HostileCreeps = 103,
    SourcesActive = 104,
    Sources = 105,
    DroppedResources = 106,
    Structures = 107,
    MyStructures = 108,
    HostileStructures = 109,
    Flags = 110,
    ConstructionSites = 111,
    MySpawns = 112,
    HostileSpawns = 113,
    MyConstructionSites = 114,
    HostileConstructionSites = 115,
    Minerals = 116,
    Nukes = 117,
    Tombstones = 118,
    PowerCreeps = 119,
    MyPowerCreeps = 120,
    HostilePowerCreeps = 121,
    Deposits = 122,
    Ruins = 123,
}

impl Into<Find> for RoomObject {
    fn into(self) -> Find {
        match self {
            RoomObject::Creeps => Find::Creeps,
            RoomObject::MyCreeps => Find::MyCreeps,
            RoomObject::HostileCreeps => Find::HostileCreeps,
            RoomObject::SourcesActive => Find::SourcesActive,
            RoomObject::Sources => Find::Sources,
            RoomObject::DroppedResources => Find::DroppedResources,
            RoomObject::Structures => Find::Structures,
            RoomObject::MyStructures => Find::MyStructures,
            RoomObject::HostileStructures => Find::HostileStructures,
            RoomObject::Flags => Find::Flags,
            RoomObject::ConstructionSites => Find::ConstructionSites,
            RoomObject::MySpawns => Find::MySpawns,
            RoomObject::HostileSpawns => Find::HostileSpawns,
            RoomObject::MyConstructionSites => Find::MyConstructionSites,
            RoomObject::HostileConstructionSites => Find::HostileConstructionSites,
            RoomObject::Minerals => Find::Minerals,
            RoomObject::Nukes => Find::Nukes,
            RoomObject::Tombstones => Find::Tombstones,
            RoomObject::PowerCreeps => Find::PowerCreeps,
            RoomObject::MyPowerCreeps => Find::MyPowerCreeps,
            RoomObject::HostilePowerCreeps => Find::HostilePowerCreeps,
            RoomObject::Deposits => Find::Deposits,
            RoomObject::Ruins => Find::Ruins,
        }
    }
}

impl FindConstant for RoomObject {
    type Item = crate::objects::RoomObject;

    #[inline]
    fn find_code(&self) -> Find {
        (*self).into()
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash,
)]
#[repr(i16)]
pub enum Exit {
    Top = 1,
    Right = 3,
    Bottom = 5,
    Left = 7,
    All = 10,
}

impl Exit {
    #[inline]
    pub fn top() -> Self {
        Exit::Top
    }

    #[inline]
    pub fn right() -> Self {
        Exit::Right
    }

    #[inline]
    pub fn bottom() -> Self {
        Exit::Bottom
    }

    #[inline]
    pub fn left() -> Self {
        Exit::Left
    }

    #[inline]
    pub fn all() -> Self {
        Exit::All
    }
}

impl Into<Find> for Exit {
    fn into(self) -> Find {
        match self {
            Exit::Top => Find::ExitTop,
            Exit::Right => Find::ExitRight,
            Exit::Bottom => Find::ExitBottom,
            Exit::Left => Find::ExitLeft,
            Exit::All => Find::Exit
        }
    }
}

impl FindConstant for Exit {
    //TODO: wiarchbe: Check this is correct?
    type Item = RoomPosition;

    #[inline]
    fn find_code(&self) -> Find {
        (*self).into()
    }
}

//TODO: wiarchbe: Add back in calculated doc.
macro_rules! typesafe_find_constants {
    (
        $(
            $vis:vis struct $constant_name:ident = ($value:expr, $result:path);
        )*
    ) => (
        $(
            #[allow(bad_style)]
            $vis struct $constant_name;            
            impl FindConstant for $constant_name {
                type Item = $result;               

                #[inline]
                fn find_code(&self) -> Find {
                    $value
                }
            }
        )*
    );
}

typesafe_find_constants! {
    pub struct CREEPS = (Find::Creeps, Creep);
    pub struct MY_CREEPS = (Find::MyCreeps, Creep);
    pub struct HOSTILE_CREEPS = (Find::HostileCreeps, Creep);
    pub struct SOURCES_ACTIVE = (Find::SourcesActive, Source);
    pub struct SOURCES = (Find::Sources, Source);
    pub struct DROPPED_RESOURCES = (Find::DroppedResources, Resource);
    pub struct STRUCTURES = (Find::Structures, StructureObject);
    pub struct MY_STRUCTURES = (Find::MyStructures, StructureObject);
    pub struct HOSTILE_STRUCTURES = (Find::HostileStructures, StructureObject);
    pub struct FLAGS = (Find::Flags, Flag);
    pub struct CONSTRUCTION_SITES = (Find::ConstructionSites, ConstructionSite);
    pub struct MY_SPAWNS = (Find::MySpawns, StructureSpawn);
    pub struct HOSTILE_SPAWNS = (Find::HostileSpawns, StructureSpawn);
    pub struct MY_CONSTRUCTION_SITES = (Find::MyConstructionSites, ConstructionSite);
    pub struct HOSTILE_CONSTRUCTION_SITES = (Find::HostileConstructionSites, ConstructionSite);
    pub struct MINERALS = (Find::Minerals, Mineral);
    pub struct NUKES = (Find::Nukes, Nuke);
    pub struct TOMBSTONES = (Find::Tombstones, Tombstone);
    pub struct POWER_CREEPS = (Find::PowerCreeps, PowerCreep);
    pub struct MY_POWER_CREEPS = (Find::MyPowerCreeps, PowerCreep);
    pub struct HOSTILE_POWER_CREEPS = (Find::HostilePowerCreeps, PowerCreep);
    pub struct DEPOSITS = (Find::Deposits, Deposit);
    pub struct RUINS = (Find::Ruins, Ruin);
    pub struct EXIT_TOP = (Find::ExitTop, RoomPosition);
    pub struct EXIT_RIGHT = (Find::ExitRight, RoomPosition);
    pub struct EXIT_BOTTOM = (Find::ExitBottom, RoomPosition);
    pub struct EXIT_LEFT = (Find::ExitLeft, RoomPosition);
    pub struct EXIT = (Find::Exit, RoomPosition);
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
    pub fn cost_callback(this: &JsFindOptions, callback: &Closure<dyn FnMut(JsString, CostMatrix) -> JsValue>);
    
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

impl<R> FindOptions<fn(RoomName, CostMatrix) -> R, R> where R: RoomCostResult + Default,
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

        let callback_type_erased: &mut (dyn FnMut(RoomName, CostMatrix) -> JsValue) = &mut owned_callback;
    
        // Overwrite lifetime of reference so it can be passed to javascript.
        // It's now pretending to be static data. This should be entirely safe
        // because we control the only use of it and it remains valid during the
        // pathfinder callback. This transmute is necessary because "some lifetime
        // above the current scope but otherwise unknown" is not a valid lifetime.
        //

        let callback_lifetime_erased: &'static mut (dyn FnMut(RoomName, CostMatrix) -> JsValue) = unsafe { std::mem::transmute(callback_type_erased) };    
    
        let boxed_callback = Box::new(move |room: JsString, cost_matrix: CostMatrix| -> JsValue {
            callback_lifetime_erased(room.into(), cost_matrix)
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