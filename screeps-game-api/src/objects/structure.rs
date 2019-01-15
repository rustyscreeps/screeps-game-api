use stdweb::{InstanceOf, Reference, ReferenceType, Value};

use {
    constants::StructureType,
    objects::{Attackable, CanDecay, CanStoreEnergy, HasCooldown, HasEnergyForSpawn, HasStore},
    traits::{FromExpectedType, IntoExpectedType, TryFrom, TryInto},
    ConversionError,
};

use super::*;

pub enum Structure {
    Container(StructureContainer),
    Controller(StructureController),
    Extension(StructureExtension),
    Extractor(StructureExtractor),
    KeeperLair(StructureKeeperLair),
    Lab(StructureLab),
    Link(StructureLink),
    Nuker(StructureNuker),
    Observer(StructureObserver),
    PowerBank(StructurePowerBank),
    PowerSpawn(StructurePowerSpawn),
    Portal(StructurePortal),
    Rampart(StructureRampart),
    Road(StructureRoad),
    Spawn(StructureSpawn),
    Storage(StructureStorage),
    Terminal(StructureTerminal),
    Tower(StructureTower),
    Wall(StructureWall),
}

impl Structure {
    pub fn as_transferable(&self) -> Option<&dyn Transferable> {
        match_some_structure_variants!(
            self,
            {
                Container, Extension, Lab, Link, Nuker, PowerSpawn, Spawn, Storage, Terminal, Tower
            },
            v => v
        )
    }

    pub fn as_withdrawable(&self) -> Option<&dyn Withdrawable> {
        match_some_structure_variants!(
            self,
            {
                Container, Extension, Lab, Link, PowerSpawn, Spawn, Storage, Terminal, Tower
            },
            v => v
        )
    }

    pub fn as_attackable(&self) -> Option<&dyn Attackable> {
        // not using match_some_structure_variants! so we won't have a `_ => None` branch and we'll
        // be forced to add new structures to the match explicitly. Others would be using `_ =>
        // None` anyways since they have fewer None branches.
        match self {
            Structure::Controller(_) => None,
            Structure::Container(v) => Some(v),
            Structure::Extension(v) => Some(v),
            Structure::Extractor(v) => Some(v),
            Structure::KeeperLair(v) => Some(v),
            Structure::Lab(v) => Some(v),
            Structure::Link(v) => Some(v),
            Structure::Nuker(v) => Some(v),
            Structure::Observer(v) => Some(v),
            Structure::PowerBank(v) => Some(v),
            Structure::PowerSpawn(v) => Some(v),
            Structure::Portal(v) => Some(v),
            Structure::Rampart(v) => Some(v),
            Structure::Road(v) => Some(v),
            Structure::Spawn(v) => Some(v),
            Structure::Storage(v) => Some(v),
            Structure::Terminal(v) => Some(v),
            Structure::Tower(v) => Some(v),
            Structure::Wall(v) => Some(v),
        }
    }

    pub fn as_owned(&self) -> Option<&dyn OwnedStructureProperties> {
        // not using match_some_structure_variants! so we won't have a `_ => None` branch and we'll
        // be forced to add new structures to the match explicitly. Others would be using `_ =>
        // None` anyways since they have fewer None branches.
        match self {
            Structure::Container(_) => None,
            Structure::Controller(v) => Some(v),
            Structure::Extension(v) => Some(v),
            Structure::Extractor(v) => Some(v),
            Structure::KeeperLair(v) => Some(v),
            Structure::Lab(v) => Some(v),
            Structure::Link(v) => Some(v),
            Structure::Nuker(v) => Some(v),
            Structure::Observer(v) => Some(v),
            Structure::PowerBank(v) => Some(v),
            Structure::PowerSpawn(v) => Some(v),
            Structure::Portal(v) => Some(v),
            Structure::Rampart(v) => Some(v),
            Structure::Road(_) => None,
            Structure::Spawn(v) => Some(v),
            Structure::Storage(v) => Some(v),
            Structure::Terminal(v) => Some(v),
            Structure::Tower(v) => Some(v),
            Structure::Wall(_) => None,
        }
    }

    pub fn as_can_decay(&self) -> Option<&dyn CanDecay> {
        match_some_structure_variants!(
            self,
            {
                Container, Portal, PowerBank, Rampart, Road
            },
            v => v
        )
    }

    pub fn as_can_store_energy(&self) -> Option<&dyn CanStoreEnergy> {
        match_some_structure_variants!(
            self,
            {
                Extension, Lab, Link, Nuker, PowerSpawn, Spawn, Tower
            },
            v => v
        )
    }

    pub fn as_has_cooldown(&self) -> Option<&dyn HasCooldown> {
        match_some_structure_variants!(
            self,
            {
                Extractor, Lab, Link, Nuker, Terminal
            },
            v => v
        )
    }

    pub fn as_has_energy_for_spawn(&self) -> Option<&dyn HasEnergyForSpawn> {
        match_some_structure_variants!(
            self,
            {
                Extension, Spawn
            },
            v => v
        )
    }

    pub fn as_has_store(&self) -> Option<&dyn HasStore> {
        match_some_structure_variants!(
            self,
            {
                Container, Storage, Terminal
            },
            v => v
        )
    }
}

impl AsRef<Reference> for Structure {
    fn as_ref(&self) -> &Reference {
        match_structure_variants!(self, v => v.as_ref())
    }
}

impl From<Structure> for Reference {
    fn from(wrapper: Structure) -> Reference {
        match_structure_variants!(wrapper, v => v.0)
    }
}

impl FromExpectedType<Reference> for Structure {
    fn from_expected_type(reference: Reference) -> Result<Self, ConversionError> {
        let structure_type = js!(return @{&reference}.structureType;).try_into()?;

        let structure = construct_structure_variants!(
            structure_type => reference.into_expected_type()?
        );

        Ok(structure)
    }
}

impl TryFrom<Reference> for Structure {
    type Error = ConversionError;

    fn try_from(reference: Reference) -> Result<Self, ConversionError> {
        let structure_type = js!(return @{&reference}.structureType;).try_into()?;

        let structure = construct_structure_variants!(
            structure_type => reference.try_into()?
        );

        Ok(structure)
    }
}

impl InstanceOf for Structure {
    fn instance_of(reference: &Reference) -> bool {
        js_unwrap!(@{reference} instanceof Structure)
    }
}

impl TryFrom<Value> for Structure {
    type Error = ConversionError;

    fn try_from(v: Value) -> Result<Structure, Self::Error> {
        Reference::try_from(v).and_then(Self::try_from)
    }
}

impl ReferenceType for Structure {
    unsafe fn from_reference_unchecked(reference: Reference) -> Self {
        let structure_type = js_unwrap!(@{&reference}.structureType);

        construct_structure_variants!(
            structure_type => ReferenceType::from_reference_unchecked(reference)
        )
    }
}
