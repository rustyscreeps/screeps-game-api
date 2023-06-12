use js_sys::{Array, JsString, Object};
use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    constants::{Direction, Part, ReturnCode},
    objects::{Creep, OwnedStructure, RoomObject, Store, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureSpawn`], which creates your creeps.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
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

    /// The [`Store`] of the spawn, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureSpawn) -> Store;

    /// Create a new creep with the specified body part [`Array`], name
    /// [`JsString`], and optional spawning options. Note that successfully
    /// spawning will store data in `Memory.creeps[creep_name]` _regardless
    /// of whether any memory data was passed in the options object_ and enable
    /// the default serialization behavior of the `Memory` object, which may
    /// hamper attempts to directly use `RawMemory`. todo, add note+docs
    /// about how to replace Memory and/or delete RawMemory._parsed
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.spawnCreep)
    #[wasm_bindgen(method, js_name = spawnCreep)]
    fn spawn_creep_internal(
        this: &StructureSpawn,
        body: &Array,
        name: &str,
        options: Option<&Object>,
    ) -> ReturnCode;

    /// Kill a [`Creep`] in melee range, returning 100% of its TTL-adjusted
    /// resources (5x more than if the creep is killed another way). Can be used
    /// while spawning.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.recycleCreep)
    #[wasm_bindgen(method, js_name = recycleCreep)]
    pub fn recycle_creep(this: &StructureSpawn, creep: &Creep) -> ReturnCode;

    /// Renew a [`Creep`] in melee range, removing all boosts adding to its TTL.
    /// Cannot be used while spawning.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.renewCreep)
    #[wasm_bindgen(method, js_name = renewCreep)]
    pub fn renew_creep(this: &StructureSpawn, creep: &Creep) -> ReturnCode;
}

impl StructureSpawn {
    pub fn spawn_creep(&self, body: &[Part], name: &str) -> ReturnCode {
        let body = body.iter().cloned().map(JsValue::from).collect();

        Self::spawn_creep_internal(self, &body, name, None)
    }

    pub fn spawn_creep_with_options(
        &self,
        body: &[Part],
        name: &str,
        opts: &SpawnOptions,
    ) -> ReturnCode {
        let body = body.iter().cloned().map(JsValue::from).collect();

        let js_opts = ObjectExt::unchecked_from_js(JsValue::from(Object::new()));

        if let Some(mem) = &opts.memory {
            ObjectExt::set(&js_opts, "memory", mem);
        }

        if let Some(array) = &opts.energy_structures {
            ObjectExt::set(&js_opts, "energyStructures", array);
        }

        if opts.dry_run {
            ObjectExt::set(&js_opts, "dryRun", &true.into());
        }

        if let Some(array) = &opts.directions {
            ObjectExt::set(&js_opts, "directions", array);
        }

        Self::spawn_creep_internal(self, &body, name, Some(&js_opts))
    }
}

impl JsCollectionFromValue for StructureSpawn {
    fn from_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}

impl HasStore for StructureSpawn {
    fn store(&self) -> Store {
        Self::store(self)
    }
}

#[derive(Default)]
pub struct SpawnOptions {
    memory: Option<JsValue>,
    energy_structures: Option<Array>,
    dry_run: bool,
    directions: Option<Array>,
}

impl SpawnOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn memory(mut self, mem: JsValue) -> Self {
        self.memory = Some(mem);
        self
    }

    /// Structures other than [`StructureSpawn`] and [`StructureExtension`] will
    /// be ignored.
    ///
    /// [`StructureExtension`]: crate::objects::StructureExtension
    pub fn energy_structures<T: IntoIterator<Item = V>, V: AsRef<Structure>>(
        mut self,
        structures: T,
    ) -> Self {
        self.energy_structures = Some(
            structures
                .into_iter()
                .map(|structure| JsValue::from(structure.as_ref()))
                .collect(),
        );
        self
    }

    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    pub fn directions(mut self, directions: &[Direction]) -> Self {
        self.directions = Some(
            directions
                .iter()
                .map(|&d| JsValue::from(d as u32))
                .collect(),
        );
        self
    }
}

#[wasm_bindgen]
extern "C" {
    /// Object with info on what a [`StructureSpawn`] or
    /// [`StructureInvaderCore`] is currently spawning.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn-Spawning)
    ///
    /// [`StructureInvaderCore`]: crate::objects::StructureInvaderCore
    #[wasm_bindgen(js_namespace = StructureSpawn)]
    pub type Spawning;

    /// Allowed directions for the creep to exit the spawn; can be changed with
    /// [`Spawning::set_directions`].
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

    /// Get a reference to the [`Structure`] spawning the creep, either a
    /// [`StructureSpawn`] or a [`StructureInvaderCore`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.spawn)
    #[wasm_bindgen(method, getter)]
    pub fn spawn(this: &Spawning) -> Structure;

    /// Cancel spawning this creep, without refunding any energy.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.cancel)
    #[wasm_bindgen(method)]
    pub fn cancel(this: &Spawning) -> ReturnCode;

    /// Change allowed directions for the creep to leave the spawn once it's
    /// ready.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureSpawn.Spawning.setDirections)
    #[wasm_bindgen(method, js_name = setDirections)]
    pub fn set_directions(this: &Spawning, directions: &Array) -> ReturnCode;
}
