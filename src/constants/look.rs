//! Constants for use with [`Room::look_for_at`] and related functions.
//!
//! *Note:* Types in this module have purposefully ambiguous names, and are
//! intended to be used as, for example, `look::CREEPS`, not `CREEPS`.
//!
//! You can do this my importing the module itself, rather than any individual
//! constant, and then just referring to the constants relative to the module.
//!
//! [`Room::look_for_at`]: crate::objects::Room::look_for_at
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{constants::Terrain, enums::StructureObject, objects::*};

/// Translates `LOOK_*` constants for interal API calls
///
/// Unless you're storing the type of look constant to be used for a call, you
/// likely want the constants which implement the `LookConstant` trait to make
/// calls to look methods.
///
/// This is hidden from the documentation to avoid confusion due to its
/// narrow use case, but wasm_bindgen requires it remain public.
#[doc(hidden)]
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Sequence)]
pub enum Look {
    Creeps = "creep",
    Energy = "energy",
    Resources = "resource",
    Sources = "source",
    Minerals = "mineral",
    Structures = "structure",
    Flags = "flag",
    ConstructionSites = "constructionSite",
    Nukes = "nuke",
    Terrain = "terrain",
    Tombstones = "tombstone",
    PowerCreeps = "powerCreep",
    Deposits = "deposit",
    Ruins = "ruin",
    // todo these seem to not work when conditionally compiled out - they're not hurting to leave
    // in but need to figure that out
    //#[cfg(feature = "seasonal-season-1")]
    ScoreContainers = "scoreContainer",
    //#[cfg(feature = "seasonal-season-1")]
    ScoreCollectors = "scoreCollector",
    //#[cfg(feature = "seasonal-season-2")]
    SymbolContainers = "symbolContainer",
    //#[cfg(feature = "seasonal-season-2")]
    SymbolDecoders = "symbolDecoder",
    //#[cfg(feature = "seasonal-season-5")]
    Reactors = "reactor",
}

//TODO: wiarchbe: Add back in calculated doc.
macro_rules! typesafe_look_constants {
    (
        $(
            $vis:vis struct $constant_name:ident = ($value:expr, $result:path, $conversion_method:expr);
        )*
    ) => (
        $(
            #[allow(bad_style)]
            $vis struct $constant_name;
            impl LookConstant for $constant_name {
                type Item = $result;

                fn convert_and_check_item(reference: JsValue) -> Self::Item {
                    $conversion_method(reference)
                }

                #[inline]
                fn look_code() -> Look {
                    $value
                }
            }
        )*
    );
}

pub trait LookConstant {
    type Item;

    fn convert_and_check_item(reference: JsValue) -> Self::Item;

    fn look_code() -> Look;
}

typesafe_look_constants! {
    pub struct CREEPS = (Look::Creeps, Creep, Into::into);
    pub struct ENERGY = (Look::Energy, Resource, Into::into);
    pub struct RESOURCES = (Look::Resources, Resource, Into::into);
    pub struct SOURCES = (Look::Sources, Source, Into::into);
    pub struct MINERALS = (Look::Minerals, Mineral, Into::into);
    pub struct DEPOSITS = (Look::Deposits, Deposit, Into::into);
    pub struct STRUCTURES = (Look::Structures, StructureObject, Into::into);
    pub struct FLAGS = (Look::Flags, Flag, Into::into);
    pub struct CONSTRUCTION_SITES = (Look::ConstructionSites, ConstructionSite,
        Into::into);
    pub struct NUKES = (Look::Nukes, Nuke, Into::into);
    pub struct TERRAIN = (Look::Terrain, Terrain, Terrain::from_look_constant_jsvalue);
    pub struct TOMBSTONES = (Look::Tombstones, Tombstone, Into::into);
    pub struct POWER_CREEPS = (Look::PowerCreeps, PowerCreep, Into::into);
    pub struct RUINS = (Look::Ruins, Ruin, Into::into);
}

#[cfg(feature = "seasonal-season-1")]
typesafe_look_constants! {
    pub struct SCORE_CONTAINERS = (Look::ScoreContainers, ScoreContainer, Into::into);
    pub struct SCORE_COLLECTORS = (Look::ScoreCollectors, ScoreCollector, Into::into);
}

#[cfg(feature = "seasonal-season-2")]
typesafe_look_constants! {
    pub struct SYMBOL_CONTAINERS = (Look::SymbolContainers, SymbolContainer, Into::into);
    pub struct SYMBOL_DECODERS = (Look::SymbolDecoders, SymbolDecoder, Into::into);
}

#[cfg(feature = "seasonal-season-5")]
typesafe_look_constants! {
    pub struct REACTORS = (Look::Reactors, Reactor, Into::into);
}

#[derive(Debug)]
pub enum LookResult {
    Creep(Creep),
    Energy(Resource),
    Resource(Resource),
    Source(Source),
    Mineral(Mineral),
    Deposit(Deposit),
    Structure(Structure),
    Flag(Flag),
    ConstructionSite(ConstructionSite),
    Nuke(Nuke),
    Terrain(Terrain),
    Tombstone(Tombstone),
    PowerCreep(PowerCreep),
    Ruin(Ruin),
    #[cfg(feature = "seasonal-season-1")]
    ScoreContainer(ScoreContainer),
    #[cfg(feature = "seasonal-season-1")]
    ScoreCollector(ScoreCollector),
    #[cfg(feature = "seasonal-season-2")]
    SymbolContainer(SymbolContainer),
    #[cfg(feature = "seasonal-season-2")]
    SymbolDecoder(SymbolDecoder),
    #[cfg(feature = "seasonal-season-5")]
    Reactor(Reactor),
}

impl LookResult {
    pub(crate) fn from_result_with_type(result: JsLookResult, t: Look) -> Self {
        match t {
            Look::Creeps => Self::Creep(result.creep()),
            Look::Energy => Self::Energy(result.energy()),
            Look::Resources => Self::Resource(result.resource()),
            Look::Sources => Self::Source(result.source()),
            Look::Minerals => Self::Mineral(result.mineral()),
            Look::Deposits => Self::Deposit(result.deposit()),
            Look::Structures => Self::Structure(result.structure()),
            Look::Flags => Self::Flag(result.flag()),
            Look::ConstructionSites => Self::ConstructionSite(result.construction_site()),
            Look::Nukes => Self::Nuke(result.nuke()),
            Look::Terrain => Self::Terrain(Terrain::from_look_constant_str(&result.terrain())),
            Look::Tombstones => Self::Tombstone(result.tombstone()),
            Look::PowerCreeps => Self::PowerCreep(result.power_creep()),
            Look::Ruins => Self::Ruin(result.ruin()),
            #[cfg(feature = "seasonal-season-1")]
            Look::ScoreContainers => Self::ScoreContainer(result.score_container()),
            #[cfg(feature = "seasonal-season-1")]
            Look::ScoreCollectors => Self::ScoreCollector(result.score_collector()),
            #[cfg(feature = "seasonal-season-2")]
            Look::SymbolContainers => Self::SymbolContainer(result.symbol_container()),
            #[cfg(feature = "seasonal-season-2")]
            Look::SymbolDecoders => Self::SymbolDecoder(result.symbol_decoder()),
            #[cfg(feature = "seasonal-season-5")]
            Look::Reactors => Self::Reactor(result.reactor()),
            _ => panic!("look result type not matched, object type feature may be disabled?"),
        }
    }

    pub(crate) fn from_jsvalue_unknown_type(v: JsValue) -> Self {
        let result: JsLookResult = v.unchecked_into();
        let rt = result.result_type();
        Self::from_result_with_type(result, rt)
    }
}

#[derive(Debug)]
pub struct PositionedLookResult {
    pub x: u8,
    pub y: u8,
    pub look_result: LookResult,
}

impl PositionedLookResult {
    pub(crate) fn from_jsvalue_with_type(v: JsValue, t: Look) -> Self {
        let result: JsLookResult = v.unchecked_into();
        let x = result.x();
        let y = result.y();
        let look_result = LookResult::from_result_with_type(result, t);
        Self { x, y, look_result }
    }

    pub(crate) fn from_jsvalue_unknown_type(v: JsValue) -> Self {
        let result: JsLookResult = v.unchecked_into();
        let rt = result.result_type();
        let x = result.x();
        let y = result.y();
        let look_result = LookResult::from_result_with_type(result, rt);
        Self { x, y, look_result }
    }
}

// internal accessors for results for look functions, any of which may be
// undefined in different kinds of look return calls
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub(crate) type JsLookResult;
    #[wasm_bindgen(method, getter = type)]
    fn result_type(this: &JsLookResult) -> Look;
    #[wasm_bindgen(method, getter)]
    fn x(this: &JsLookResult) -> u8;
    #[wasm_bindgen(method, getter)]
    fn y(this: &JsLookResult) -> u8;
    #[wasm_bindgen(method, getter)]
    fn creep(this: &JsLookResult) -> Creep;
    #[wasm_bindgen(method, getter)]
    fn energy(this: &JsLookResult) -> Resource;
    #[wasm_bindgen(method, getter)]
    fn resource(this: &JsLookResult) -> Resource;
    #[wasm_bindgen(method, getter)]
    fn source(this: &JsLookResult) -> Source;
    #[wasm_bindgen(method, getter)]
    fn mineral(this: &JsLookResult) -> Mineral;
    #[wasm_bindgen(method, getter)]
    fn deposit(this: &JsLookResult) -> Deposit;
    #[wasm_bindgen(method, getter)]
    fn structure(this: &JsLookResult) -> Structure;
    #[wasm_bindgen(method, getter)]
    fn flag(this: &JsLookResult) -> Flag;
    #[wasm_bindgen(method, getter = constructionSite)]
    fn construction_site(this: &JsLookResult) -> ConstructionSite;
    #[wasm_bindgen(method, getter)]
    fn nuke(this: &JsLookResult) -> Nuke;
    // note that this one is a string representing a terrain constant, and must be
    // converted
    #[wasm_bindgen(method, getter)]
    fn terrain(this: &JsLookResult) -> String;
    #[wasm_bindgen(method, getter)]
    fn tombstone(this: &JsLookResult) -> Tombstone;
    #[wasm_bindgen(method, getter = powerCreep)]
    fn power_creep(this: &JsLookResult) -> PowerCreep;
    #[wasm_bindgen(method, getter)]
    fn ruin(this: &JsLookResult) -> Ruin;
    #[cfg(feature = "seasonal-season-1")]
    #[wasm_bindgen(method, getter = scoreContainer)]
    fn score_container(this: &JsLookResult) -> ScoreContainer;
    #[cfg(feature = "seasonal-season-1")]
    #[wasm_bindgen(method, getter = scoreCollector)]
    fn score_collector(this: &JsLookResult) -> ScoreCollector;
    #[cfg(feature = "seasonal-season-2")]
    #[wasm_bindgen(method, getter = symbolContainer)]
    fn symbol_container(this: &JsLookResult) -> SymbolContainer;
    #[cfg(feature = "seasonal-season-2")]
    #[wasm_bindgen(method, getter = symbolDecoder)]
    fn symbol_decoder(this: &JsLookResult) -> SymbolDecoder;
    #[cfg(feature = "seasonal-season-5")]
    #[wasm_bindgen(method, getter = reactor)]
    fn reactor(this: &JsLookResult) -> Reactor;
}
