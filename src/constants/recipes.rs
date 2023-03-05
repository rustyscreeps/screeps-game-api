use std::collections::HashMap;

use crate::constants::ResourceType;

/// Returned values from [`ResourceType::commodity_recipe`] representing a
/// commodity that can be produced in factories.
#[derive(Clone, Debug)]
pub struct FactoryRecipe {
    /// Amount of the component that this recipe creates
    pub amount: u32,
    /// Cooldown of the factory after creating this recipe
    pub cooldown: u32,
    /// Components - resource type and amount
    pub components: HashMap<ResourceType, u32>,
    /// Required factory level to be able to create this commodity, if
    /// restricted
    pub level: Option<u32>,
}

impl ResourceType {
    /// Translates the `REACTIONS` constant.
    #[inline]
    pub fn reaction_components(self) -> Option<[ResourceType; 2]> {
        use ResourceType::*;
        let components = match self {
            // OH: O + H,
            Hydroxide => [Oxygen, Hydrogen],
            // ZK: Z + K,
            ZynthiumKeanite => [Zynthium, Keanium],
            // UL: U + L,
            UtriumLemergite => [Utrium, Lemergium],
            // G: UL + ZK,
            Ghodium => [UtriumLemergite, ZynthiumKeanite],
            // UH: U + H,
            UtriumHydride => [Utrium, Hydrogen],
            // UH2O: UH + OH,
            UtriumAcid => [UtriumHydride, Hydroxide],
            // XUH2O: UH20 + X,
            CatalyzedUtriumAcid => [UtriumAcid, Catalyst],
            // UO: U + O,
            UtriumOxide => [Utrium, Oxygen],
            // UHO2: UO + OH,
            UtriumAlkalide => [UtriumOxide, Hydroxide],
            // XUHO2: UHO2 + X,
            CatalyzedUtriumAlkalide => [UtriumAlkalide, Catalyst],
            // KH: K + H,
            KeaniumHydride => [Keanium, Hydrogen],
            // KH2O: KH + OH,
            KeaniumAcid => [KeaniumHydride, Hydroxide],
            // XKH2O: KH2O + X,
            CatalyzedKeaniumAcid => [KeaniumAcid, Catalyst],
            // KO: K + O,
            KeaniumOxide => [Keanium, Oxygen],
            // KHO2: KO + OH,
            KeaniumAlkalide => [KeaniumOxide, Hydroxide],
            // XKHO2: KHO2 + X,
            CatalyzedKeaniumAlkalide => [KeaniumAlkalide, Catalyst],
            // LH: L + H,
            LemergiumHydride => [Lemergium, Hydrogen],
            // LH2O: LH + OH,
            LemergiumAcid => [LemergiumHydride, Hydroxide],
            // XLH2O: LH2O + X,
            CatalyzedLemergiumAcid => [LemergiumAcid, Catalyst],
            // LO: L + O,
            LemergiumOxide => [Lemergium, Oxygen],
            // LHO2: LO + OH,
            LemergiumAlkalide => [LemergiumOxide, Hydroxide],
            // XLHO2: LHO2 + X,
            CatalyzedLemergiumAlkalide => [LemergiumAlkalide, Catalyst],
            // ZH: Z + H,
            ZynthiumHydride => [Zynthium, Hydrogen],
            // ZH2O: ZH + OH,
            ZynthiumAcid => [ZynthiumHydride, Hydroxide],
            // XZH2O: ZH2O + Z,
            CatalyzedZynthiumAcid => [ZynthiumAcid, Catalyst],
            // ZO: Z + O,
            ZynthiumOxide => [Zynthium, Oxygen],
            // ZHO2: ZO + OH,
            ZynthiumAlkalide => [ZynthiumOxide, Hydroxide],
            // XZHO2: ZHO2 + X,
            CatalyzedZynthiumAlkalide => [ZynthiumAlkalide, Catalyst],
            // GH: G + H,
            GhodiumHydride => [Ghodium, Hydrogen],
            // GH2O: GH + OH,
            GhodiumAcid => [GhodiumHydride, Hydroxide],
            // XGH2O: GH2O + X,
            CatalyzedGhodiumAcid => [GhodiumAcid, Catalyst],
            // GO: G + O,
            GhodiumOxide => [Ghodium, Oxygen],
            // GHO2: GO + OH,
            GhodiumAlkalide => [GhodiumOxide, Hydroxide],
            // XGHO2: GHO2 + X,
            CatalyzedGhodiumAlkalide => [GhodiumAlkalide, Catalyst],
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
            Organism => FactoryRecipe {
                amount: 1,
                cooldown: 600,
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
