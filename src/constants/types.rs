//! `*Type` constants.
use enum_iterator::Sequence;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use wasm_bindgen::prelude::*;

use crate::{JsCollectionFromValue, JsCollectionIntoValue};

macro_rules! named_enum_serialize_deserialize {
    ($ty:ty) => {
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let str = <&'de str>::deserialize(deserializer)?;
                Ok(<$ty>::from_str(str).unwrap_or(<$ty>::__Nonexhaustive))
            }
        }
        impl serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.to_str())
            }
        }
    };
}

/// Translates `STRUCTURE_*` constants.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Sequence)]
pub enum StructureType {
    Spawn = "spawn",
    Extension = "extension",
    Road = "road",
    Wall = "constructedWall",
    Rampart = "rampart",
    KeeperLair = "keeperLair",
    Portal = "portal",
    Controller = "controller",
    Link = "link",
    Storage = "storage",
    Tower = "tower",
    Observer = "observer",
    PowerBank = "powerBank",
    PowerSpawn = "powerSpawn",
    Extractor = "extractor",
    Lab = "lab",
    Terminal = "terminal",
    Container = "container",
    Nuker = "nuker",
    Factory = "factory",
    InvaderCore = "invaderCore",
}

named_enum_serialize_deserialize!(StructureType);

impl StructureType {
    /// Translates the `CONSTRUCTION_COST` constant.
    #[inline]
    pub fn construction_cost(self) -> Option<u32> {
        use self::StructureType::*;

        let cost = match self {
            Spawn => 15_000,
            Extension => 3_000,
            Road => 300,
            Wall => 1,
            Rampart => 1,
            Link => 5_000,
            Storage => 30_000,
            Tower => 5_000,
            Observer => 8_000,
            PowerSpawn => 100_000,
            Extractor => 5_000,
            Lab => 50_000,
            Terminal => 100_000,
            Container => 5_000,
            Nuker => 100_000,
            Factory => 100_000,
            _ => return None,
        };
        Some(cost)
    }

    /// Translates the `CONTROLLER_STRUCTURES` constant
    #[inline]
    pub fn controller_structures(self, current_rcl: u32) -> u32 {
        use self::StructureType::*;

        match self {
            Spawn => match current_rcl {
                0 => 0,
                1..=6 => 1,
                7 => 2,
                _ => 3,
            },
            Extension => match current_rcl {
                0 | 1 => 0,
                2 => 5,
                3 => 10,
                4 => 20,
                5 => 30,
                6 => 40,
                7 => 50,
                _ => 60,
            },
            Road => 2500,
            Wall => match current_rcl {
                0 | 1 => 0,
                _ => 2500,
            },
            Rampart => match current_rcl {
                0 | 1 => 0,
                _ => 2500,
            },
            Link => match current_rcl {
                0..=4 => 0,
                5 => 2,
                6 => 3,
                7 => 4,
                _ => 6,
            },
            Storage => match current_rcl {
                0..=3 => 0,
                _ => 1,
            },
            Tower => match current_rcl {
                0..=2 => 0,
                3 | 4 => 1,
                5 | 6 => 2,
                7 => 3,
                _ => 6,
            },
            Observer => match current_rcl {
                0..=7 => 0,
                _ => 1,
            },
            PowerSpawn => match current_rcl {
                0..=7 => 0,
                _ => 1,
            },
            Extractor => match current_rcl {
                0..=5 => 0,
                _ => 1,
            },
            Lab => match current_rcl {
                0..=5 => 0,
                6 => 3,
                7 => 6,
                _ => 10,
            },
            Terminal => match current_rcl {
                0..=5 => 0,
                _ => 1,
            },
            Container => 5,
            Nuker => match current_rcl {
                0..=7 => 0,
                _ => 1,
            },
            Factory => match current_rcl {
                0..=6 => 0,
                _ => 1,
            },
            _ => 0,
        }
    }

    /// Translates the `*_HITS` constants, initial hits for structures
    #[inline]
    pub fn initial_hits(self) -> Option<u32> {
        use self::StructureType::*;
        use super::numbers::*;

        let hits = match self {
            Spawn => SPAWN_HITS,
            Extension => EXTENSION_HITS,
            Road => ROAD_HITS,
            Wall => WALL_HITS,
            Rampart => RAMPART_HITS,
            Link => LINK_HITS,
            Storage => STORAGE_HITS,
            Tower => TOWER_HITS,
            Observer => OBSERVER_HITS,
            PowerBank => POWER_BANK_HITS,
            PowerSpawn => POWER_SPAWN_HITS,
            Extractor => EXTENSION_HITS,
            Lab => LAB_HITS,
            Terminal => TERMINAL_HITS,
            Container => CONTAINER_HITS,
            Nuker => NUKER_HITS,
            Factory => FACTORY_HITS,
            InvaderCore => INVADER_CORE_HITS,
            _ => return None,
        };
        Some(hits)
    }
}

/// Translates `SUBSCRIPTION_TOKEN` and `INTERSHARD_RESOURCES` constants.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Sequence)]
pub enum IntershardResourceType {
    // no longer used, not implemented
    // SubscriptionToken = "token",
    CpuUnlock = "cpuUnlock",
    Pixel = "pixel",
    AccessKey = "accessKey",
}

/// Translates `RESOURCES_ALL` constant, representing all possible in-game
/// (non-intershard) resources.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Sequence)]
pub enum ResourceType {
    Energy = "energy",
    Power = "power",
    Hydrogen = "H",
    Oxygen = "O",
    Utrium = "U",
    Lemergium = "L",
    Keanium = "K",
    Zynthium = "Z",
    Catalyst = "X",
    Ghodium = "G",
    Silicon = "silicon",
    Metal = "metal",
    Biomass = "biomass",
    Mist = "mist",
    Hydroxide = "OH",
    ZynthiumKeanite = "ZK",
    UtriumLemergite = "UL",
    UtriumHydride = "UH",
    UtriumOxide = "UO",
    KeaniumHydride = "KH",
    KeaniumOxide = "KO",
    LemergiumHydride = "LH",
    LemergiumOxide = "LO",
    ZynthiumHydride = "ZH",
    ZynthiumOxide = "ZO",
    GhodiumHydride = "GH",
    GhodiumOxide = "GO",
    UtriumAcid = "UH2O",
    UtriumAlkalide = "UHO2",
    KeaniumAcid = "KH2O",
    KeaniumAlkalide = "KHO2",
    LemergiumAcid = "LH2O",
    LemergiumAlkalide = "LHO2",
    ZynthiumAcid = "ZH2O",
    ZynthiumAlkalide = "ZHO2",
    GhodiumAcid = "GH2O",
    GhodiumAlkalide = "GHO2",
    CatalyzedUtriumAcid = "XUH2O",
    CatalyzedUtriumAlkalide = "XUHO2",
    CatalyzedKeaniumAcid = "XKH2O",
    CatalyzedKeaniumAlkalide = "XKHO2",
    CatalyzedLemergiumAcid = "XLH2O",
    CatalyzedLemergiumAlkalide = "XLHO2",
    CatalyzedZynthiumAcid = "XZH2O",
    CatalyzedZynthiumAlkalide = "XZHO2",
    CatalyzedGhodiumAcid = "XGH2O",
    CatalyzedGhodiumAlkalide = "XGHO2",
    Ops = "ops",
    UtriumBar = "utrium_bar",
    LemergiumBar = "lemergium_bar",
    ZynthiumBar = "zynthium_bar",
    KeaniumBar = "keanium_bar",
    GhodiumMelt = "ghodium_melt",
    Oxidant = "oxidant",
    Reductant = "reductant",
    Purifier = "purifier",
    Battery = "battery",
    Composite = "composite",
    Crystal = "crystal",
    Liquid = "liquid",
    Wire = "wire",
    Switch = "switch",
    Transistor = "transistor",
    Microchip = "microchip",
    Circuit = "circuit",
    Device = "device",
    Cell = "cell",
    Phlegm = "phlegm",
    Tissue = "tissue",
    Muscle = "muscle",
    Organoid = "organoid",
    Organism = "organism",
    Alloy = "alloy",
    Tube = "tube",
    Fixtures = "fixtures",
    Frame = "frame",
    Hydraulics = "hydraulics",
    Machine = "machine",
    Condensate = "condensate",
    Concentrate = "concentrate",
    Extract = "extract",
    Spirit = "spirit",
    Emanation = "emanation",
    Essence = "essence",
    #[cfg(feature = "score")]
    Score = "score",
    #[cfg(feature = "symbols")]
    SymbolAleph = "symbol_aleph",
    #[cfg(feature = "symbols")]
    SymbolBeth = "symbol_beth",
    #[cfg(feature = "symbols")]
    SymbolGimmel = "symbol_gimmel",
    #[cfg(feature = "symbols")]
    SymbolDaleth = "symbol_daleth",
    #[cfg(feature = "symbols")]
    SymbolHe = "symbol_he",
    #[cfg(feature = "symbols")]
    SymbolWaw = "symbol_waw",
    #[cfg(feature = "symbols")]
    SymbolZayin = "symbol_zayin",
    #[cfg(feature = "symbols")]
    SymbolHeth = "symbol_heth",
    #[cfg(feature = "symbols")]
    SymbolTeth = "symbol_teth",
    #[cfg(feature = "symbols")]
    SymbolYodh = "symbol_yodh",
    #[cfg(feature = "symbols")]
    SymbolKaph = "symbol_kaph",
    #[cfg(feature = "symbols")]
    SymbolLamedh = "symbol_lamedh",
    #[cfg(feature = "symbols")]
    SymbolMem = "symbol_mem",
    #[cfg(feature = "symbols")]
    SymbolNun = "symbol_nun",
    #[cfg(feature = "symbols")]
    SymbolSamekh = "symbol_samekh",
    #[cfg(feature = "symbols")]
    SymbolAyin = "symbol_ayin",
    #[cfg(feature = "symbols")]
    SymbolPe = "symbol_pe",
    #[cfg(feature = "symbols")]
    SymbolTsade = "symbol_tsade",
    #[cfg(feature = "symbols")]
    SymbolQoph = "symbol_qoph",
    #[cfg(feature = "symbols")]
    SymbolRes = "symbol_res",
    // sin/sim mismatch is intended here - see official mod:
    // https://github.com/screeps/mod-season2/blob/3dfaa8f6214b2610dbe2a700c6287a10e7960ae8/src/resources.js#L23
    #[cfg(feature = "symbols")]
    SymbolSin = "symbol_sim",
    #[cfg(feature = "symbols")]
    SymbolTaw = "symbol_taw",
    #[cfg(feature = "thorium")]
    Thorium = "T",
}

named_enum_serialize_deserialize!(ResourceType);

impl ResourceType {
    /// Translates the `BOOSTS` constant.
    #[inline]
    pub fn boost(self) -> Option<Boost> {
        use ResourceType::*;
        let boost = match self {
            // these comments copied directly from JavaScript 'constants.js' file.
            // UH: {
            //     attack: 2
            // },
            UtriumHydride => Boost::Attack(2),
            // UH2O: {
            //     attack: 3
            // },
            UtriumAcid => Boost::Attack(3),
            // XUH2O: {
            //     attack: 4
            // }
            CatalyzedUtriumAcid => Boost::Attack(4),
            // UO: {
            //     harvest: 3
            // },
            UtriumOxide => Boost::Harvest(3),
            // UHO2: {
            //     harvest: 5
            // },
            UtriumAlkalide => Boost::Harvest(5),
            // XUHO2: {
            //     harvest: 7
            // },
            CatalyzedUtriumAlkalide => Boost::Harvest(7),
            // KH: {
            //     capacity: 2
            // },
            KeaniumHydride => Boost::Carry(2),
            // KH2O: {
            //     capacity: 3
            // },
            KeaniumAcid => Boost::Carry(3),
            // XKH2O: {
            //     capacity: 4
            // }
            CatalyzedKeaniumAcid => Boost::Carry(4),
            // KO: {
            //     rangedAttack: 2,
            //     rangedMassAttack: 2
            // },
            KeaniumOxide => Boost::RangedAttack(2),
            // KHO2: {
            //     rangedAttack: 3,
            //     rangedMassAttack: 3
            // },
            KeaniumAlkalide => Boost::RangedAttack(4),
            // XKHO2: {
            //     rangedAttack: 4,
            //     rangedMassAttack: 4
            // }
            CatalyzedKeaniumAlkalide => Boost::RangedAttack(4),
            // LH: {
            //     build: 1.5,
            //     repair: 1.5
            // },
            LemergiumHydride => Boost::BuildAndRepair(1.5),
            // LH2O: {
            //     build: 1.8,
            //     repair: 1.8
            // },
            LemergiumAcid => Boost::BuildAndRepair(1.8),
            // XLH2O: {
            //     build: 2,
            //     repair: 2
            // },
            CatalyzedLemergiumAcid => Boost::BuildAndRepair(2.0),
            // LO: {
            //     heal: 2,
            //     rangedHeal: 2
            // },
            LemergiumOxide => Boost::Heal(2),
            // LHO2: {
            //     heal: 3,
            //     rangedHeal: 3
            // },
            LemergiumAlkalide => Boost::Heal(3),
            // XLHO2: {
            //     heal: 4,
            //     rangedHeal: 4
            // }
            CatalyzedLemergiumAlkalide => Boost::Heal(4),
            // ZH: {
            //     dismantle: 2
            // },
            ZynthiumHydride => Boost::Dismantle(2),
            // ZH2O: {
            //     dismantle: 3
            // },
            ZynthiumAcid => Boost::Dismantle(3),
            // XZH2O: {
            //     dismantle: 4
            // },
            CatalyzedZynthiumAcid => Boost::Dismantle(4),
            // ZO: {
            //     fatigue: 2
            // },
            ZynthiumOxide => Boost::Move(2),
            // ZHO2: {
            //     fatigue: 3
            // },
            ZynthiumAlkalide => Boost::Move(3),
            // XZHO2: {
            //     fatigue: 4
            // }
            CatalyzedZynthiumAlkalide => Boost::Move(4),
            // GH: {
            //     upgradeController: 1.5
            // },
            GhodiumHydride => Boost::UpgradeController(1.5),
            // GH2O: {
            //     upgradeController: 1.8
            // },
            GhodiumAcid => Boost::UpgradeController(1.8),
            // XGH2O: {
            //     upgradeController: 2
            // }
            CatalyzedGhodiumAcid => Boost::UpgradeController(2.0),
            // GO: {
            //     damage: .7
            // },
            GhodiumOxide => Boost::Tough(0.7),
            // GHO2: {
            //     damage: .5
            // },
            GhodiumAlkalide => Boost::Tough(0.5),
            // XGHO2: {
            //     damage: .3
            // }
            CatalyzedGhodiumAlkalide => Boost::Tough(0.3),
            // non-boost resources
            _ => return None,
        };
        Some(boost)
    }
}

/// Returned values from [`ResourceType::boost`] representing the effect of
/// boosting a creep with the given resource.
#[derive(Copy, Clone, Debug)]
pub enum Boost {
    Harvest(u32),
    BuildAndRepair(f32),
    Dismantle(u32),
    UpgradeController(f32),
    Attack(u32),
    RangedAttack(u32),
    Heal(u32),
    Carry(u32),
    Move(u32),
    Tough(f32),
}

/// Translates all resource types that can be used on the market.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Sequence)]
#[serde(untagged)]
pub enum MarketResourceType {
    Resource(ResourceType),
    IntershardResource(IntershardResourceType),
}

impl wasm_bindgen::convert::FromWasmAbi for MarketResourceType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> Self {
        let s = <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(js);
        // first try deserialize as ResourceType
        match ResourceType::from_js_value(&s) {
            Some(r) => Self::Resource(r),
            None => {
                // try with IntershardResourceType
                match IntershardResourceType::from_js_value(&s) {
                    Some(r) => Self::IntershardResource(r),
                    None => Self::Resource(ResourceType::__Nonexhaustive),
                }
            }
        }
    }
}

impl wasm_bindgen::convert::IntoWasmAbi for MarketResourceType {
    type Abi = <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        match self {
            MarketResourceType::Resource(r) => {
                <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(r.into())
            }
            MarketResourceType::IntershardResource(r) => {
                <wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(r.into())
            }
        }
    }
}

impl wasm_bindgen::describe::WasmDescribe for MarketResourceType {
    fn describe() {
        <wasm_bindgen::JsValue as wasm_bindgen::describe::WasmDescribe>::describe()
    }
}

/// Translates the `POWER_CLASS` constants, which are classes of power creeps
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Sequence)]
pub enum PowerCreepClass {
    Operator = "operator",
}

/// Translates the `PWR_*` constants, which are types of powers used by power
/// creeps
#[wasm_bindgen]
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    FromPrimitive,
    Deserialize_repr,
    Serialize_repr,
    Sequence,
)]
#[repr(u32)]
pub enum PowerType {
    GenerateOps = 1,
    OperateSpawn = 2,
    OperateTower = 3,
    OperateStorage = 4,
    OperateLab = 5,
    OperateExtension = 6,
    OperateObserver = 7,
    OperateTerminal = 8,
    DisruptSpawn = 9,
    DisruptTower = 10,
    Shield = 12,
    RegenSource = 13,
    RegenMineral = 14,
    DisruptTerminal = 15,
    OperatePower = 16,
    Fortify = 17,
    OperateController = 18,
    OperateFactory = 19,
}

impl JsCollectionFromValue for PowerType {
    fn from_value(val: JsValue) -> Self {
        let power_type_id = if let Some(val) = val.as_string() {
            val.parse::<u32>().expect("expected parseable u32 string")
        } else {
            val.as_f64().expect("expected number value") as u32
        };

        Self::from_u32(power_type_id).expect("unknown power type")
    }
}

impl JsCollectionIntoValue for PowerType {
    fn into_value(self) -> JsValue {
        JsValue::from_f64(self as u32 as f64)
    }
}

/// Translates the `EFFECT_*` constants, which are natural effect types
#[wasm_bindgen]
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    FromPrimitive,
    Serialize_repr,
    Deserialize_repr,
    Sequence,
)]
#[repr(u32)]
pub enum NaturalEffectType {
    Invulnerability = 1001,
    CollapseTimer = 1002,
}

/// Translates effect types on room objects, which can include both `PWR_*` and
/// `EFFECT_*` constants.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Sequence)]
pub enum EffectType {
    PowerEffect(PowerType),
    NaturalEffect(NaturalEffectType),
}

impl wasm_bindgen::convert::IntoWasmAbi for EffectType {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        match self {
            EffectType::PowerEffect(e) => (e as u32).into_abi(),
            EffectType::NaturalEffect(e) => (e as u32).into_abi(),
        }
    }
}

impl wasm_bindgen::convert::FromWasmAbi for EffectType {
    type Abi = u32;

    #[inline]
    unsafe fn from_abi(js: u32) -> Self {
        match PowerType::from_u32(js) {
            Some(pt) => Self::PowerEffect(pt),
            None => {
                Self::NaturalEffect(NaturalEffectType::from_u32(js).expect("unknown effect id!"))
            }
        }
    }
}

impl wasm_bindgen::describe::WasmDescribe for EffectType {
    fn describe() {
        wasm_bindgen::describe::inform(wasm_bindgen::describe::U32)
    }
}
