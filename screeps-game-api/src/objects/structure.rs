use stdweb::{InstanceOf, Reference, ReferenceType, Value};

use {
    constants::StructureType,
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

unsafe impl Attackable for Structure {
    fn hits(&self) -> u32 {
        match self {
            Structure::Controller(_) => 0,
            _ => js_unwrap! { @{self.as_ref()}.hits },
        }
    }

    fn hits_max(&self) -> u32 {
        match self {
            Structure::Controller(_) => 0,
            _ => js_unwrap! { @{self.as_ref()}.hitsMax },
        }
    }
}
