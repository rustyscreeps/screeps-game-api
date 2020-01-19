use std::collections::HashMap;

use crate::constants::ResourceType;

#[derive(Clone, Debug)]
pub struct FactoryRecipe {
    /// Amount of the component that this recipe creates
    pub amount: u32,
    /// Cooldown of the factory after creating this recipe
    pub cooldown: u32,
    /// Components - resource type and amount
    pub components: HashMap<ResourceType, u32>,
    /// Required factory level to be able to create this commodity, if restricted
    pub level: Option<u32>,
}

impl ResourceType {
    /// Translates the `REACTIONS` constant.
    #[inline]
    pub fn reaction_components(self) -> Option<Vec<ResourceType>> {
        use ResourceType::*;
        let components = match self {
            // OH: O + H,
            Hydroxide => vec![Oxygen, Hydrogen],
            // ZK: Z + K,
            ZynthiumKeanite => vec![Zynthium, Keanium],
            // UL: U + L,
            UtriumLemergite => vec![Utrium, Lemergium],
            // G: UL + ZK,
            Ghodium => vec![UtriumLemergite, ZynthiumKeanite],
            // UH: U + H,
            UtriumHydride => vec![Utrium, Hydrogen],
            // UH2O: UH + OH,
            UtriumAcid => vec![UtriumHydride, Hydroxide],
            // XUH2O: UH20 + X,
            CatalyzedUtriumAcid => vec![UtriumAcid, Catalyst],
            // UO: U + O,
            UtriumOxide => vec![Utrium, Oxygen],
            // UHO2: UO + OH,
            UtriumAlkalide => vec![UtriumOxide, Hydroxide],
            // XUHO2: UHO2 + X,
            CatalyzedUtriumAlkalide => vec![UtriumAlkalide, Catalyst],
            // KH: K + H,
            KeaniumHydride => vec![Keanium, Hydrogen],
            // KH2O: KH + OH,
            KeaniumAcid => vec![KeaniumHydride, Hydroxide],
            // XKH2O: KH2O + X,
            CatalyzedKeaniumAcid => vec![KeaniumAcid, Catalyst],
            // KO: K + O,
            KeaniumOxide => vec![Keanium, Oxygen],
            // KHO2: KO + OH,
            KeaniumAlkalide => vec![KeaniumOxide, Hydroxide],
            // XKHO2: KHO2 + X,
            CatalyzedKeaniumAlkalide => vec![KeaniumAlkalide, Catalyst],
            // LH: L + H,
            LemergiumHydride => vec![Lemergium, Hydrogen],
            // LH2O: LH + OH,
            LemergiumAcid => vec![LemergiumHydride, Hydroxide],
            // XLH2O: LH2O + X,
            CatalyzedLemergiumAcid => vec![LemergiumAcid, Catalyst],
            // LO: L + O,
            LemergiumOxide => vec![Lemergium, Oxygen],
            // LHO2: LO + OH,
            LemergiumAlkalide => vec![LemergiumOxide, Hydroxide],
            // XLHO2: LHO2 + X,
            CatalyzedLemergiumAlkalide => vec![LemergiumAlkalide, Catalyst],
            // ZH: Z + H,
            ZynthiumHydride => vec![Zynthium, Hydrogen],
            // ZH2O: ZH + OH,
            ZynthiumAcid => vec![ZynthiumHydride, Hydroxide],
            // XZH2O: ZH2O + Z,
            CatalyzedZynthiumAcid => vec![ZynthiumAcid, Catalyst],
            // ZO: Z + O,
            ZynthiumOxide => vec![Zynthium, Oxygen],
            // ZHO2: ZO + OH,
            ZynthiumAlkalide => vec![ZynthiumOxide, Hydroxide],
            // XZHO2: ZHO2 + X,
            CatalyzedZynthiumAlkalide => vec![ZynthiumAlkalide, Catalyst],
            // GH: G + H,
            GhodiumHydride => vec![Ghodium, Hydrogen],
            // GH2O: GH + OH,
            GhodiumAcid => vec![GhodiumHydride, Hydroxide],
            // XGH2O: GH2O + X,
            CatalyzedGhodiumAcid => vec![GhodiumAcid, Catalyst],
            // GO: G + O,
            GhodiumOxide => vec![Ghodium, Oxygen],
            // GHO2: GO + OH,
            GhodiumAlkalide => vec![GhodiumOxide, Hydroxide],
            // XGHO2: GHO2 + X,
            CatalyzedGhodiumAlkalide => vec![GhodiumAlkalide, Catalyst],
            // non-molecule resources
            _ => return None,
        };
        Some(components)
    }

    /// Translates the `REACTION_TIME` constant.
    #[inline]
    pub fn reaction_time(self) -> Option<u32> {
        use ResourceType::*;
        let time = match self {
            // these comments copied directly from JavaScript 'constants.js' file.
            // OH: 20,
            Hydroxide => 20,
            // ZK: 5,
            ZynthiumKeanite => 5,
            // UL: 5,
            UtriumLemergite => 5,
            // G: 5,
            Ghodium => 5,
            // UH: 10,
            UtriumHydride => 10,
            // UH2O: 5,
            UtriumAcid => 5,
            // XUH2O: 60,
            CatalyzedUtriumAcid => 60,
            // UO: 10,
            UtriumOxide => 10,
            // UHO2: 5,
            UtriumAlkalide => 5,
            // XUHO2: 60,
            CatalyzedUtriumAlkalide => 60,
            // KH: 10,
            KeaniumHydride => 10,
            // KH2O: 5,
            KeaniumAcid => 5,
            // XKH2O: 60,
            CatalyzedKeaniumAcid => 60,
            // KO: 10,
            KeaniumOxide => 10,
            // KHO2: 5,
            KeaniumAlkalide => 5,
            // XKHO2: 60,
            CatalyzedKeaniumAlkalide => 60,
            // LH: 15,
            LemergiumHydride => 15,
            // LH2O: 10,
            LemergiumAcid => 10,
            // XLH2O: 65,
            CatalyzedLemergiumAcid => 65,
            // LO: 10,
            LemergiumOxide => 10,
            // LHO2: 5,
            LemergiumAlkalide => 5,
            // XLHO2: 60,
            CatalyzedLemergiumAlkalide => 60,
            // ZH: 20,
            ZynthiumHydride => 20,
            // ZH2O: 40,
            ZynthiumAcid => 40,
            // XZH2O: 160,
            CatalyzedZynthiumAcid => 160,
            // ZO: 10,
            ZynthiumOxide => 10,
            // ZHO2: 5,
            ZynthiumAlkalide => 5,
            // XZHO2: 60,
            CatalyzedZynthiumAlkalide => 60,
            // GH: 10,
            GhodiumHydride => 10,
            // GH2O: 15,
            GhodiumAcid => 15,
            // XGH2O: 80,
            CatalyzedGhodiumAcid => 80,
            // GO: 10,
            GhodiumOxide => 10,
            // GHO2: 30,
            GhodiumAlkalide => 30,
            // XGHO2: 150,
            CatalyzedGhodiumAlkalide => 150,
            // non-molecule resources
            _ => return None,
        };
        Some(time)
    }

    /// Translates the `COMMODITIES` constant to recipes that can be used by a
    /// factory to make each commodity
    pub fn commodity_recipe(self) -> Option<FactoryRecipe> {
        use ResourceType::*;
        let recipe = match self {
            // these comments copied directly from JavaScript 'constants.js' file.
            // [exports.RESOURCE_UTRIUM_BAR]: {
            //     amount: 100,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_UTRIUM]: 500,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            UtriumBar => FactoryRecipe {
                amount: 100,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Utrium, 500);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_UTRIUM]: {
            //     amount: 500,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_UTRIUM_BAR]: 100,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            Utrium => FactoryRecipe {
                amount: 500,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(UtriumBar, 100);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_LEMERGIUM_BAR]: {
            //     amount: 100,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_LEMERGIUM]: 500,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            LemergiumBar => FactoryRecipe {
                amount: 100,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Lemergium, 500);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_LEMERGIUM]: {
            //     amount: 500,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_LEMERGIUM_BAR]: 100,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            Lemergium => FactoryRecipe {
                amount: 500,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(LemergiumBar, 100);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_ZYNTHIUM_BAR]: {
            //     amount: 100,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_ZYNTHIUM]: 500,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            ZynthiumBar => FactoryRecipe {
                amount: 100,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Zynthium, 500);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_ZYNTHIUM]: {
            //     amount: 500,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_ZYNTHIUM_BAR]: 100,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            Zynthium => FactoryRecipe {
                amount: 500,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(ZynthiumBar, 100);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_KEANIUM_BAR]: {
            //     amount: 100,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_KEANIUM]: 500,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            KeaniumBar => FactoryRecipe {
                amount: 100,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Keanium, 500);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_KEANIUM]: {
            //     amount: 500,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_KEANIUM_BAR]: 100,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            Keanium => FactoryRecipe {
                amount: 500,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(KeaniumBar, 100);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_GHODIUM_MELT]: {
            //     amount: 100,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_GHODIUM]: 500,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            GhodiumMelt => FactoryRecipe {
                amount: 100,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Ghodium, 500);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_GHODIUM]: {
            //     amount: 500,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_GHODIUM_MELT]: 100,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            Ghodium => FactoryRecipe {
                amount: 500,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(GhodiumMelt, 100);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_OXIDANT]: {
            //     amount: 100,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_OXYGEN]: 500,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            Oxidant => FactoryRecipe {
                amount: 100,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Oxygen, 500);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_OXYGEN]: {
            //     amount: 500,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_OXIDANT]: 100,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            Oxygen => FactoryRecipe {
                amount: 500,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Oxidant, 100);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_REDUCTANT]: {
            //     amount: 100,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_HYDROGEN]: 500,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            Reductant => FactoryRecipe {
                amount: 100,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Hydrogen, 500);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_HYDROGEN]: {
            //     amount: 500,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_REDUCTANT]: 100,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            Hydrogen => FactoryRecipe {
                amount: 500,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Reductant, 100);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_PURIFIER]: {
            //     amount: 100,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_CATALYST]: 500,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            Purifier => FactoryRecipe {
                amount: 100,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Catalyst, 500);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_CATALYST]: {
            //     amount: 500,
            //     cooldown: 20,
            //     components: {
            //         [exports.RESOURCE_PURIFIER]: 100,
            //         [exports.RESOURCE_ENERGY]: 200
            //     }
            // },
            Catalyst => FactoryRecipe {
                amount: 500,
                cooldown: 20,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Purifier, 100);
                    components.insert(Energy, 200);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_BATTERY]: {
            //     amount: 50,
            //     cooldown: 10,
            //     components: {
            //         [exports.RESOURCE_ENERGY]: 600
            //     }
            // },
            Battery => FactoryRecipe {
                amount: 50,
                cooldown: 10,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Energy, 600);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_ENERGY]: {
            //     amount: 500,
            //     cooldown: 10,
            //     components: {
            //         [exports.RESOURCE_BATTERY]: 50
            //     }
            // },
            Energy => FactoryRecipe {
                amount: 500,
                cooldown: 10,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Battery, 50);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_COMPOSITE]: {
            //     level: 1,
            //     amount: 20,
            //     cooldown: 50,
            //     components: {
            //         [exports.RESOURCE_UTRIUM_BAR]: 20,
            //         [exports.RESOURCE_ZYNTHIUM_BAR]: 20,
            //         [exports.RESOURCE_ENERGY]: 20
            //     }
            // },
            Composite => FactoryRecipe {
                amount: 20,
                cooldown: 50,
                components: {
                    let mut components = HashMap::new();
                    components.insert(UtriumBar, 20);
                    components.insert(ZynthiumBar, 20);
                    components.insert(Energy, 20);
                    components
                },
                level: Some(1),
            },
            // [exports.RESOURCE_CRYSTAL]: {
            //     level: 2,
            //     amount: 6,
            //     cooldown: 21,
            //     components: {
            //         [exports.RESOURCE_LEMERGIUM_BAR]: 6,
            //         [exports.RESOURCE_KEANIUM_BAR]: 6,
            //         [exports.RESOURCE_PURIFIER]: 6,
            //         [exports.RESOURCE_ENERGY]: 45
            //     }
            // },
            Crystal => FactoryRecipe {
                amount: 6,
                cooldown: 21,
                components: {
                    let mut components = HashMap::new();
                    components.insert(LemergiumBar, 6);
                    components.insert(KeaniumBar, 6);
                    components.insert(Purifier, 6);
                    components.insert(Energy, 45);
                    components
                },
                level: Some(2),
            },
            // [exports.RESOURCE_LIQUID]: {
            //     level: 3,
            //     amount: 12,
            //     cooldown: 60,
            //     components: {
            //         [exports.RESOURCE_OXIDANT]: 12,
            //         [exports.RESOURCE_REDUCTANT]: 12,
            //         [exports.RESOURCE_GHODIUM_MELT]: 12,
            //         [exports.RESOURCE_ENERGY]: 90
            //     }
            // },
            Liquid => FactoryRecipe {
                amount: 12,
                cooldown: 60,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Oxidant, 12);
                    components.insert(Reductant, 12);
                    components.insert(GhodiumMelt, 12);
                    components.insert(Energy, 90);
                    components
                },
                level: Some(3),
            },
            // [exports.RESOURCE_WIRE]: {
            //     amount: 20,
            //     cooldown: 8,
            //     components: {
            //         [exports.RESOURCE_UTRIUM_BAR]: 20,
            //         [exports.RESOURCE_SILICON]: 100,
            //         [exports.RESOURCE_ENERGY]: 40
            //     }
            // },
            Wire => FactoryRecipe {
                amount: 20,
                cooldown: 8,
                components: {
                    let mut components = HashMap::new();
                    components.insert(UtriumBar, 20);
                    components.insert(Silicon, 100);
                    components.insert(Energy, 40);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_SWITCH]: {
            //     level: 1,
            //     amount: 5,
            //     cooldown: 70,
            //     components: {
            //         [exports.RESOURCE_WIRE]: 40,
            //         [exports.RESOURCE_OXIDANT]: 95,
            //         [exports.RESOURCE_UTRIUM_BAR]: 35,
            //         [exports.RESOURCE_ENERGY]: 20
            //     }
            // },
            Switch => FactoryRecipe {
                amount: 5,
                cooldown: 70,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Wire, 40);
                    components.insert(Oxidant, 95);
                    components.insert(UtriumBar, 35);
                    components.insert(Energy, 20);
                    components
                },
                level: Some(1),
            },
            // [exports.RESOURCE_TRANSISTOR]: {
            //     level: 2,
            //     amount: 1,
            //     cooldown: 59,
            //     components: {
            //         [exports.RESOURCE_SWITCH]: 4,
            //         [exports.RESOURCE_WIRE]: 15,
            //         [exports.RESOURCE_REDUCTANT]: 85,
            //         [exports.RESOURCE_ENERGY]: 8
            //     }
            // },
            Transistor => FactoryRecipe {
                amount: 1,
                cooldown: 59,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Switch, 4);
                    components.insert(Wire, 15);
                    components.insert(Reductant, 85);
                    components.insert(Energy, 8);
                    components
                },
                level: Some(2),
            },
            // [exports.RESOURCE_MICROCHIP]: {
            //     level: 3,
            //     amount: 1,
            //     cooldown: 250,
            //     components: {
            //         [exports.RESOURCE_TRANSISTOR]: 2,
            //         [exports.RESOURCE_COMPOSITE]: 50,
            //         [exports.RESOURCE_WIRE]: 117,
            //         [exports.RESOURCE_PURIFIER]: 25,
            //         [exports.RESOURCE_ENERGY]: 16
            //     }
            // },
            Microchip => FactoryRecipe {
                amount: 1,
                cooldown: 250,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Transistor, 2);
                    components.insert(Composite, 50);
                    components.insert(Wire, 117);
                    components.insert(Purifier, 25);
                    components.insert(Energy, 16);
                    components
                },
                level: Some(3),
            },
            // [exports.RESOURCE_CIRCUIT]: {
            //     level: 4,
            //     amount: 1,
            //     cooldown: 800,
            //     components: {
            //         [exports.RESOURCE_MICROCHIP]: 1,
            //         [exports.RESOURCE_TRANSISTOR]: 5,
            //         [exports.RESOURCE_SWITCH]: 4,
            //         [exports.RESOURCE_OXIDANT]: 115,
            //         [exports.RESOURCE_ENERGY]: 32
            //     }
            // },
            Circuit => FactoryRecipe {
                amount: 1,
                cooldown: 800,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Microchip, 1);
                    components.insert(Transistor, 5);
                    components.insert(Switch, 4);
                    components.insert(Oxidant, 115);
                    components.insert(Energy, 32);
                    components
                },
                level: Some(4),
            },
            // [exports.RESOURCE_DEVICE]: {
            //     level: 5,
            //     amount: 1,
            //     cooldown: 600,
            //     components: {
            //         [exports.RESOURCE_CIRCUIT]: 1,
            //         [exports.RESOURCE_MICROCHIP]: 3,
            //         [exports.RESOURCE_CRYSTAL]: 110,
            //         [exports.RESOURCE_GHODIUM_MELT]: 150,
            //         [exports.RESOURCE_ENERGY]: 64
            //     }
            // },
            Device => FactoryRecipe {
                amount: 1,
                cooldown: 600,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Circuit, 1);
                    components.insert(Microchip, 3);
                    components.insert(Crystal, 110);
                    components.insert(GhodiumMelt, 150);
                    components.insert(Energy, 64);
                    components
                },
                level: Some(5),
            },
            // [exports.RESOURCE_CELL]: {
            //     amount: 20,
            //     cooldown: 8,
            //     components: {
            //         [exports.RESOURCE_LEMERGIUM_BAR]: 20,
            //         [exports.RESOURCE_BIOMASS]: 100,
            //         [exports.RESOURCE_ENERGY]: 40
            //     }
            // },
            Cell => FactoryRecipe {
                amount: 20,
                cooldown: 8,
                components: {
                    let mut components = HashMap::new();
                    components.insert(LemergiumBar, 20);
                    components.insert(Biomass, 100);
                    components.insert(Energy, 40);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_PHLEGM]: {
            //     level: 1,
            //     amount: 2,
            //     cooldown: 35,
            //     components: {
            //         [exports.RESOURCE_CELL]: 20,
            //         [exports.RESOURCE_OXIDANT]: 36,
            //         [exports.RESOURCE_LEMERGIUM_BAR]: 16,
            //         [exports.RESOURCE_ENERGY]: 8
            //     }
            // },
            Phlegm => FactoryRecipe {
                amount: 2,
                cooldown: 35,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Cell, 20);
                    components.insert(Oxidant, 36);
                    components.insert(LemergiumBar, 16);
                    components.insert(Energy, 8);
                    components
                },
                level: Some(1),
            },
            // [exports.RESOURCE_TISSUE]: {
            //     level: 2,
            //     amount: 2,
            //     cooldown: 164,
            //     components: {
            //         [exports.RESOURCE_PHLEGM]: 10,
            //         [exports.RESOURCE_CELL]: 10,
            //         [exports.RESOURCE_REDUCTANT]: 110,
            //         [exports.RESOURCE_ENERGY]: 16
            //     }
            // },
            Tissue => FactoryRecipe {
                amount: 2,
                cooldown: 164,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Phlegm, 10);
                    components.insert(Cell, 10);
                    components.insert(Reductant, 110);
                    components.insert(Energy, 16);
                    components
                },
                level: Some(2),
            },
            // [exports.RESOURCE_MUSCLE]: {
            //     level: 3,
            //     amount: 1,
            //     cooldown: 250,
            //     components: {
            //         [exports.RESOURCE_TISSUE]: 3,
            //         [exports.RESOURCE_PHLEGM]: 3,
            //         [exports.RESOURCE_ZYNTHIUM_BAR]: 50,
            //         [exports.RESOURCE_REDUCTANT]: 50,
            //         [exports.RESOURCE_ENERGY]: 16
            //     }
            // },
            Muscle => FactoryRecipe {
                amount: 1,
                cooldown: 250,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Tissue, 3);
                    components.insert(Phlegm, 3);
                    components.insert(ZynthiumBar, 50);
                    components.insert(Reductant, 50);
                    components.insert(Energy, 16);
                    components
                },
                level: Some(3),
            },
            // [exports.RESOURCE_ORGANOID]: {
            //     level: 4,
            //     amount: 1,
            //     cooldown: 800,
            //     components: {
            //         [exports.RESOURCE_MUSCLE]: 1,
            //         [exports.RESOURCE_TISSUE]: 5,
            //         [exports.RESOURCE_PURIFIER]: 208,
            //         [exports.RESOURCE_OXIDANT]: 256,
            //         [exports.RESOURCE_ENERGY]: 32
            //     }
            // },
            Organoid => FactoryRecipe {
                amount: 1,
                cooldown: 800,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Muscle, 1);
                    components.insert(Tissue, 5);
                    components.insert(Purifier, 208);
                    components.insert(Oxidant, 256);
                    components.insert(Energy, 32);
                    components
                },
                level: Some(4),
            },
            // [exports.RESOURCE_ORGANISM]: {
            //     level: 5,
            //     amount: 1,
            //     cooldown: 600,
            //     components: {
            //         [exports.RESOURCE_ORGANOID]: 1,
            //         [exports.RESOURCE_LIQUID]: 150,
            //         [exports.RESOURCE_TISSUE]: 6,
            //         [exports.RESOURCE_CELL]: 310,
            //         [exports.RESOURCE_ENERGY]: 64
            //     }
            // },
            Organism => FactoryRecipe {
                amount: 1,
                cooldown: 800,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Organoid, 1);
                    components.insert(Liquid, 150);
                    components.insert(Tissue, 6);
                    components.insert(Cell, 310);
                    components.insert(Energy, 64);
                    components
                },
                level: Some(5),
            },
            // [exports.RESOURCE_ALLOY]: {
            //     amount: 20,
            //     cooldown: 8,
            //     components: {
            //         [exports.RESOURCE_ZYNTHIUM_BAR]: 20,
            //         [exports.RESOURCE_METAL]: 100,
            //         [exports.RESOURCE_ENERGY]: 40
            //     }
            // },
            Alloy => FactoryRecipe {
                amount: 20,
                cooldown: 8,
                components: {
                    let mut components = HashMap::new();
                    components.insert(ZynthiumBar, 20);
                    components.insert(Metal, 100);
                    components.insert(Energy, 40);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_TUBE]: {
            //     level: 1,
            //     amount: 2,
            //     cooldown: 45,
            //     components: {
            //         [exports.RESOURCE_ALLOY]: 40,
            //         [exports.RESOURCE_ZYNTHIUM_BAR]: 16,
            //         [exports.RESOURCE_ENERGY]: 8
            //     }
            // },
            Tube => FactoryRecipe {
                amount: 2,
                cooldown: 45,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Alloy, 40);
                    components.insert(ZynthiumBar, 16);
                    components.insert(Energy, 8);
                    components
                },
                level: Some(1),
            },
            // [exports.RESOURCE_FIXTURES]: {
            //     level: 2,
            //     amount: 1,
            //     cooldown: 115,
            //     components: {
            //         [exports.RESOURCE_COMPOSITE]: 20,
            //         [exports.RESOURCE_ALLOY]: 41,
            //         [exports.RESOURCE_OXIDANT]: 161,
            //         [exports.RESOURCE_ENERGY]: 8
            //     }
            // },
            Fixtures => FactoryRecipe {
                amount: 1,
                cooldown: 115,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Composite, 20);
                    components.insert(Alloy, 41);
                    components.insert(Oxidant, 161);
                    components.insert(Energy, 8);
                    components
                },
                level: Some(2),
            },
            // [exports.RESOURCE_FRAME]: {
            //     level: 3,
            //     amount: 1,
            //     cooldown: 125,
            //     components: {
            //         [exports.RESOURCE_FIXTURES]: 2,
            //         [exports.RESOURCE_TUBE]: 4,
            //         [exports.RESOURCE_REDUCTANT]: 330,
            //         [exports.RESOURCE_ZYNTHIUM_BAR]: 31,
            //         [exports.RESOURCE_ENERGY]: 16
            //     }
            // },
            Frame => FactoryRecipe {
                amount: 1,
                cooldown: 125,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Fixtures, 2);
                    components.insert(Tube, 4);
                    components.insert(Reductant, 330);
                    components.insert(ZynthiumBar, 31);
                    components.insert(Energy, 16);
                    components
                },
                level: Some(3),
            },
            // [exports.RESOURCE_HYDRAULICS]: {
            //     level: 4,
            //     amount: 1,
            //     cooldown: 800,
            //     components: {
            //         [exports.RESOURCE_LIQUID]: 150,
            //         [exports.RESOURCE_FIXTURES]: 3,
            //         [exports.RESOURCE_TUBE]: 15,
            //         [exports.RESOURCE_PURIFIER]: 208,
            //         [exports.RESOURCE_ENERGY]: 32
            //     }
            // },
            Hydraulics => FactoryRecipe {
                amount: 1,
                cooldown: 800,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Liquid, 150);
                    components.insert(Fixtures, 3);
                    components.insert(Tube, 15);
                    components.insert(Purifier, 208);
                    components.insert(Energy, 32);
                    components
                },
                level: Some(4),
            },
            // [exports.RESOURCE_MACHINE]: {
            //     level: 5,
            //     amount: 1,
            //     cooldown: 600,
            //     components: {
            //         [exports.RESOURCE_HYDRAULICS]: 1,
            //         [exports.RESOURCE_FRAME]: 2,
            //         [exports.RESOURCE_FIXTURES]: 3,
            //         [exports.RESOURCE_TUBE]: 12,
            //         [exports.RESOURCE_ENERGY]: 64
            //     }
            // },
            Machine => FactoryRecipe {
                amount: 1,
                cooldown: 600,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Hydraulics, 1);
                    components.insert(Frame, 2);
                    components.insert(Fixtures, 3);
                    components.insert(Tube, 12);
                    components.insert(Energy, 64);
                    components
                },
                level: Some(5),
            },
            // [exports.RESOURCE_CONDENSATE]: {
            //     amount: 20,
            //     cooldown: 8,
            //     components: {
            //         [exports.RESOURCE_KEANIUM_BAR]: 20,
            //         [exports.RESOURCE_MIST]: 100,
            //         [exports.RESOURCE_ENERGY]: 40
            //     }
            // },
            Condensate => FactoryRecipe {
                amount: 20,
                cooldown: 8,
                components: {
                    let mut components = HashMap::new();
                    components.insert(KeaniumBar, 20);
                    components.insert(Mist, 100);
                    components.insert(Energy, 40);
                    components
                },
                level: None,
            },
            // [exports.RESOURCE_CONCENTRATE]: {
            //     level: 1,
            //     amount: 3,
            //     cooldown: 41,
            //     components: {
            //         [exports.RESOURCE_CONDENSATE]: 30,
            //         [exports.RESOURCE_KEANIUM_BAR]: 15,
            //         [exports.RESOURCE_REDUCTANT]: 54,
            //         [exports.RESOURCE_ENERGY]: 12
            //     }
            // },
            Concentrate => FactoryRecipe {
                amount: 3,
                cooldown: 41,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Condensate, 30);
                    components.insert(KeaniumBar, 15);
                    components.insert(Reductant, 54);
                    components.insert(Energy, 12);
                    components
                },
                level: Some(1),
            },
            // [exports.RESOURCE_EXTRACT]: {
            //     level: 2,
            //     amount: 2,
            //     cooldown: 128,
            //     components: {
            //         [exports.RESOURCE_CONCENTRATE]: 10,
            //         [exports.RESOURCE_CONDENSATE]: 30,
            //         [exports.RESOURCE_OXIDANT]: 60,
            //         [exports.RESOURCE_ENERGY]: 16
            //     }
            // },
            Extract => FactoryRecipe {
                amount: 2,
                cooldown: 128,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Concentrate, 10);
                    components.insert(Condensate, 30);
                    components.insert(Oxidant, 60);
                    components.insert(Energy, 16);
                    components
                },
                level: Some(2),
            },
            // [exports.RESOURCE_SPIRIT]: {
            //     level: 3,
            //     amount: 1,
            //     cooldown: 200,
            //     components: {
            //         [exports.RESOURCE_EXTRACT]: 2,
            //         [exports.RESOURCE_CONCENTRATE]: 6,
            //         [exports.RESOURCE_REDUCTANT]: 90,
            //         [exports.RESOURCE_PURIFIER]: 20,
            //         [exports.RESOURCE_ENERGY]: 16
            //     }
            // },
            Spirit => FactoryRecipe {
                amount: 1,
                cooldown: 200,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Extract, 2);
                    components.insert(Concentrate, 6);
                    components.insert(Reductant, 90);
                    components.insert(Purifier, 20);
                    components.insert(Energy, 16);
                    components
                },
                level: Some(3),
            },
            // [exports.RESOURCE_EMANATION]: {
            //     level: 4,
            //     amount: 1,
            //     cooldown: 800,
            //     components: {
            //         [exports.RESOURCE_SPIRIT]: 2,
            //         [exports.RESOURCE_EXTRACT]: 2,
            //         [exports.RESOURCE_CONCENTRATE]: 3,
            //         [exports.RESOURCE_KEANIUM_BAR]: 112,
            //         [exports.RESOURCE_ENERGY]: 32
            //     }
            // },
            Emanation => FactoryRecipe {
                amount: 1,
                cooldown: 800,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Spirit, 2);
                    components.insert(Extract, 2);
                    components.insert(Concentrate, 3);
                    components.insert(KeaniumBar, 112);
                    components.insert(Energy, 32);
                    components
                },
                level: Some(4),
            },
            // [exports.RESOURCE_ESSENCE]: {
            //     level: 5,
            //     amount: 1,
            //     cooldown: 600,
            //     components: {
            //         [exports.RESOURCE_EMANATION]: 1,
            //         [exports.RESOURCE_SPIRIT]: 3,
            //         [exports.RESOURCE_CRYSTAL]: 110,
            //         [exports.RESOURCE_GHODIUM_MELT]: 150,
            //         [exports.RESOURCE_ENERGY]: 64
            //     }
            // },
            Essence => FactoryRecipe {
                amount: 1,
                cooldown: 600,
                components: {
                    let mut components = HashMap::new();
                    components.insert(Emanation, 1);
                    components.insert(Spirit, 3);
                    components.insert(Crystal, 110);
                    components.insert(GhodiumMelt, 150);
                    components.insert(Energy, 64);
                    components
                },
                level: Some(5),
            },
            // non-factory resources
            _ => return None,
        };
        Some(recipe)
    }
}
