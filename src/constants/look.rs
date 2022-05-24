use crate::{enums::StructureObject, objects::*};
use enum_iterator::IntoEnumIterator;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Translates `LOOK_*` constants.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, IntoEnumIterator)]
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
    //#[cfg(feature = "enable-score")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreContainers = "scoreContainer",
    //#[cfg(feature = "enable-score")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
    ScoreCollectors = "scoreCollector",
    //#[cfg(feature = "enable-symbols")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
    SymbolContainers = "symbolContainer",
    //#[cfg(feature = "enable-symbols")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
    SymbolDecoders = "symbolDecoder",
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
    //TODO: wiarchbe: Add back in terrain type.
    //pub struct TERRAIN = (Look::Terrain, Terrain, TryInto::try_into);
    pub struct TOMBSTONES = (Look::Tombstones, Tombstone, Into::into);
    pub struct POWER_CREEPS = (Look::PowerCreeps, PowerCreep, Into::into);
    pub struct RUINS = (Look::Ruins, Ruin, Into::into);
}

#[cfg(feature = "enable-score")]
typesafe_look_constants! {
    pub struct SCORE_CONTAINERS = (Look::ScoreContainers, ScoreContainer, Into::into);
    pub struct SCORE_COLLECTORS = (Look::ScoreCollectors, ScoreCollector, Into::into);
}

#[cfg(feature = "enable-symbols")]
typesafe_look_constants! {
    pub struct SYMBOL_CONTAINERS = (Look::SymbolContainers, SymbolContainer, Into::into);
    pub struct SYMBOL_DECODERS = (Look::SymbolDecoders, SymbolDecoder, Into::into);
}
