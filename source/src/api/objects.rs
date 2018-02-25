use stdweb::{Value, Reference};
use stdweb::unstable::{TryInto, TryFrom};

macro_rules! reference_wrappers {
    ($name:ident) => {
        pub struct $name(Reference);

        impl AsRef<Reference> for $name {
            fn as_ref(&self) -> &Reference {
                &self.0
            }
        }
        impl TryFrom<Value> for $name {
            type Error = <Value as TryInto<Reference>>::Error;

            fn try_from(v: Value) -> Result<$name, Self::Error> {
                Ok($name(v.try_into()?))
            }
        }
    };
    ($($name:ident),* $(,)*) => {
        $(
            reference_wrappers!($name);
        )*
    };
}

reference_wrappers!(
    ConstructionSite, Creep, Flag, Mineral, Nuke, Resource, Room, RoomPosition, Source,
    StructureContainer, StructureController, StructureExtension, StructureExtractor,
    StructureKeeperLair, StructureLab, StructureLink, StructureNuker, StructureObserver,
    StructurePowerBank, StructurePowerSpawn, StructurePortal, StructureRampart, StructureRoad,
    StructureSpawn, StructureStorage, StructureTerminal, StructureTower, StructureWall, Structure,
);

macro_rules! js_unwrap {
    ($($code:tt)*) => ((js! { $($code)* }).try_into().unwrap())
}

impl Creep {
    pub fn pos(&self) -> RoomPosition {
        RoomPosition(js_unwrap!(@{&self.0}.pos))
    }

    pub fn room(&self) -> Room {
        Room(js_unwrap!(@{&self.0}.room))
    }
}

pub fn names() -> Vec<String> {
    (js! {
        return Object.keys(Game.creeps);
    }).try_into()
        .unwrap()
}

pub fn pos(name: &str) -> (String, i32, i32) {
    let list: Vec<Value> = (js! {
        let pos = Game.creeps[@{name}].pos;
        return [pos.x, pos.y, pos.roomName];
    }).try_into()
        .unwrap();

    let mut it = list.into_iter();
    let i0 = it.next().unwrap();
    let i1 = it.next().unwrap();
    let i2 = it.next().unwrap();

    (
        i0.try_into().unwrap(),
        TryInto::<f64>::try_into(i1).unwrap() as i32,
        TryInto::<f64>::try_into(i2).unwrap() as i32,
    )
}
