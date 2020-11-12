use crate::objects::{Creep, OwnedStructure, RoomObject, Structure, Store};
use js_sys::{Array, Object, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureSpawn`], which creates your creeps.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureSpawn;

    /// A shortcut to `Memory.spawns[spawn.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.memory)
    #[wasm_bindgen(method, getter)]
    pub fn memory(this: &StructureSpawn) -> JsValue;

    /// Sets a new value to `Memory.spawns[spawn.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.memory)
    #[wasm_bindgen(method, setter)]
    pub fn set_memory(this: &StructureSpawn, val: &JsValue);

    /// The spawn's name as an owned reference to a [`JsString`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.name)
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &StructureSpawn) -> JsString;

    /// Information about the spawning creep, if one is currently being spawned.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.spawning)
    #[wasm_bindgen(method, getter)]
    pub fn spawning(this: &StructureSpawn) -> Option<Spawning>;

    /// The [`Store`] of the spawn, which contains information about what resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureSpawn) -> Store;

    /// Create a new creep with the specified body part [`Array`], name [`JsString`], and optional spawning options.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.spawnCreep)
    #[wasm_bindgen(method, js_name = spawnCreep)]
    pub fn spawn_creep(this: &StructureSpawn, body: &Array, name: &JsString, options: Option<Object>) -> i8;

    /// Kill a [`Creep`] in melee range, returning 100% of its TTL-adjusted resources (5x more than if the creep is killed another way). Can be used while spawning.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.recycleCreep)
    #[wasm_bindgen(method, js_name = recycleCreep)]
    pub fn recycle_creep(this: &StructureSpawn, creep: &Creep) -> i8;

    /// Renew a [`Creep`] in melee range, removing all boosts adding to its TTL. Cannot be used while spawning.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.renewCreep)
    #[wasm_bindgen(method, js_name = renewCreep)]
    pub fn renew_creep(this: &StructureSpawn, creep: &Creep) -> i8;
}


#[wasm_bindgen]
extern "C" {
    /// Object with info on what a [`StructureSpawn`] is currently spawning.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn-Spawning)
    #[wasm_bindgen(js_namespace = StructureSpawn)]
    pub type Spawning;

    /// Allowed directions for the creep to exit the spawn; can be changed with [`Spawning::set_directions`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.directions)
    #[wasm_bindgen(method, getter)]
    pub fn directions(this: &Spawning) -> Array;

    /// The name of the spawning creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.name)
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Spawning) -> JsString;

    /// Total time needed to spawn this creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.needTime)
    #[wasm_bindgen(method, getter = needTime)]
    pub fn need_time(this: &Spawning) -> u32;

    /// Total time remaining to spawn this creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.remainingTime)
    #[wasm_bindgen(method, getter = remainingTime)]
    pub fn remaining_time(this: &Spawning) -> u32;

    /// Get a reference to the spawn.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.spawn)
    #[wasm_bindgen(method, getter)]
    pub fn spawn(this: &Spawning) -> StructureSpawn;

    /// Cancel spawning this creep, without refunding any energy.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.cancel)
    #[wasm_bindgen(method)]
    pub fn cancel(this: &Spawning) -> i8;

    /// Change allowed directions for the creep to leave the spawn once it's ready.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.setDirections)
    #[wasm_bindgen(method, js_name = setDirections)]
    pub fn set_directions(this: &Spawning) -> i8;
}


// use stdweb::Reference;

// use crate::{
//     constants::{Direction, Part, ReturnCode},
//     memory::MemoryReference,
//     objects::{Creep, HasEnergyForSpawn, SizedRoomObject, Spawning, StructureSpawn},
//     traits::TryInto,
// };

// simple_accessors! {
//     impl StructureSpawn {
//         pub fn name() -> String = name;
//         pub fn spawning() -> Option<Spawning> = spawning;
//     }
// }

// impl StructureSpawn {
//     pub fn memory(&self) -> MemoryReference {
//         js_unwrap!(@{self.as_ref()}.memory)
//     }

//     pub fn spawn_creep(&self, body: &[Part], name: &str) -> ReturnCode {
//         let ints = body.iter().map(|p| *p as u32).collect::<Vec<u32>>();
//         (js! {
//             var body = (@{ints}).map(__part_num_to_str);

//             return @{self.as_ref()}.spawnCreep(body, @{name});
//         })
//         .try_into()
//         .expect("expected StructureSpawn::spawnCreep to return an integer return code")
//     }

//     pub fn spawn_creep_with_options(
//         &self,
//         body: &[Part],
//         name: &str,
//         opts: &SpawnOptions,
//     ) -> ReturnCode {
//         let body_ints = body.iter().map(|p| *p as u32).collect::<Vec<u32>>();

//         let js_opts = js!(return {dryRun: @{opts.dry_run}};);

//         if let Some(ref mem) = opts.memory {
//             js! { @(no_return)
//                 @{&js_opts}.memory = @{mem.as_ref()};
//             }
//         }
//         if !opts.energy_structures.is_empty() {
//             js! { @(no_return)
//                 @{&js_opts}.energyStructures = @{&opts.energy_structures};
//             }
//         }
//         if !opts.directions.is_empty() {
//             js! { @(no_return)
//                 @{&js_opts}.directions = @{&opts.directions};
//             }
//         }
//         (js! {
//             var body = (@{body_ints}).map(__part_num_to_str);

//             return @{self.as_ref()}.spawnCreep(body, @{name}, @{js_opts});
//         })
//         .try_into()
//         .expect("expected StructureSpawn::spawnCreep to return an integer return code")
//     }

//     // TODO: support actually using Spawning properties.
//     pub fn is_spawning(&self) -> bool {
//         js_unwrap!(Boolean(@{self.as_ref()}.spawning))
//     }

//     pub fn recycle_creep(&self, target: &Creep) -> ReturnCode {
//         js_unwrap! {@{self.as_ref()}.recycleCreep(@{target.as_ref()})}
//     }

//     pub fn renew_creep(&self, target: &Creep) -> ReturnCode {
//         js_unwrap! {@{self.as_ref()}.renewCreep(@{target.as_ref()})}
//     }
// }

// #[derive(Default)]
// pub struct SpawnOptions {
//     memory: Option<MemoryReference>,
//     energy_structures: Vec<Reference>,
//     dry_run: bool,
//     directions: Vec<u32>,
// }

// impl SpawnOptions {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     pub fn memory<T: Into<Option<MemoryReference>>>(mut self, mem: T) -> Self {
//         self.memory = mem.into();
//         self
//     }

//     /// This is most useful with the `.as_structure()` method on structures.
//     pub fn energy_structures<T>(mut self, structures: T) -> Self
//     where
//         T: IntoIterator,
//         <T as IntoIterator>::Item: HasEnergyForSpawn + SizedRoomObject,
//     {
//         self.energy_structures = structures.into_iter().map(|s| s.into()).collect();
//         self
//     }

//     pub fn dry_run(mut self, dry_run: bool) -> Self {
//         self.dry_run = dry_run;
//         self
//     }

//     pub fn directions(mut self, directions: &[Direction]) -> Self {
//         self.directions = directions.iter().map(|d| *d as u32).collect();
//         self
//     }
// }

// simple_accessors! {
//     impl Spawning {
//         pub fn directions() -> Vec<Direction> = directions;
//         pub fn name() -> String = name;
//         pub fn need_time() -> u32 = needTime;
//         pub fn remaining_time() -> u32 = remainingTime;
//         pub fn spawn() -> StructureSpawn = spawn;
//     }
// }

// impl Spawning {
//     pub fn cancel(&self) -> ReturnCode {
//         js_unwrap!(@{self.as_ref()}.cancel())
//     }

//     pub fn set_directions(&self, directions: &[Direction]) -> ReturnCode {
//         let int_dirs: Vec<u32> = directions.iter().map(|d| *d as u32).collect();
//         js_unwrap!(@{self.as_ref()}.setDirections(@{int_dirs}))
//     }
// }
