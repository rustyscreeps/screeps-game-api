mod construction_site;
mod cost_matrix;
mod creep;
mod deposit;
mod flag;
mod mineral;
mod nuke;
mod owned_structure;
mod power_creep;
mod resource;
mod room;
mod room_object;
mod room_position;
mod room_terrain;
mod room_visual;
mod ruin;
mod source;
mod store;
mod structure;
mod structure_container;
mod structure_controller;
mod structure_extension;
mod structure_extractor;
mod structure_factory;
mod structure_invader_core;
mod structure_keeper_lair;
mod structure_lab;
mod structure_link;
mod structure_nuker;
mod structure_observer;
mod structure_portal;
mod structure_power_bank;
mod structure_power_spawn;
mod structure_rampart;
mod structure_road;
mod structure_spawn;
mod structure_storage;
mod structure_terminal;
mod structure_tower;
mod structure_wall;
mod tombstone;

#[cfg(feature = "enable-score")]
#[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
mod score_collector;
#[cfg(feature = "enable-score")]
#[cfg_attr(docsrs, doc(cfg(feature = "enable-score")))]
mod score_container;

#[cfg(feature = "enable-symbols")]
#[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
mod symbol_container;
#[cfg(feature = "enable-symbols")]
#[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
mod symbol_decoder;

pub use self::{
    construction_site::ConstructionSite,
    cost_matrix::CostMatrix,
    cost_matrix::CostMatrixSet,
    creep::Creep,
    deposit::Deposit,
    flag::Flag,
    mineral::Mineral,
    nuke::Nuke,
    owned_structure::{OwnedStructure, Owner},
    power_creep::PowerCreep,
    resource::Resource,
    room::Room,
    room_object::RoomObject,
    room_position::RoomPosition,
    room_terrain::RoomTerrain,
    ruin::Ruin,
    source::Source,
    store::Store,
    structure::Structure,
    structure_container::StructureContainer,
    structure_controller::{Reservation, Sign, StructureController},
    structure_extension::StructureExtension,
    structure_extractor::StructureExtractor,
    structure_factory::StructureFactory,
    structure_invader_core::StructureInvaderCore,
    structure_keeper_lair::StructureKeeperLair,
    structure_lab::StructureLab,
    structure_link::StructureLink,
    structure_nuker::StructureNuker,
    structure_observer::StructureObserver,
    structure_portal::StructurePortal,
    structure_power_bank::StructurePowerBank,
    structure_power_spawn::StructurePowerSpawn,
    structure_rampart::StructureRampart,
    structure_road::StructureRoad,
    structure_spawn::{Spawning, StructureSpawn},
    structure_storage::StructureStorage,
    structure_terminal::StructureTerminal,
    structure_tower::StructureTower,
    structure_wall::StructureWall,
    tombstone::Tombstone,
};

pub use self::room_visual::{CircleStyle, FontStyle, LineDrawStyle, LineStyle, PolyStyle, RectStyle, RoomVisual, TextAlign, TextStyle, Visual};

#[cfg(feature = "enable-score")]
pub use self::{score_collector::ScoreCollector, score_container::ScoreContainer};

#[cfg(feature = "enable-symbols")]
pub use self::{symbol_container::SymbolContainer, symbol_decoder::SymbolDecoder};
