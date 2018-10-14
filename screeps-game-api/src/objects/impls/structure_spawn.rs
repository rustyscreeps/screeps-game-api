use stdweb::Reference;

use {
    constants::{Direction, Part, ReturnCode},
    memory::MemoryReference,
    objects::{Creep, HasEnergyForSpawn, Spawning, StructureSpawn},
};

simple_accessors! {
    StructureSpawn;
    (name -> name -> String),
    (spawning -> spawning -> Spawning),
}

impl StructureSpawn {
    pub fn memory(&self) -> MemoryReference {
        js_unwrap!(@{self.as_ref()}.memory)
    }

    pub fn spawn_creep(&self, body: &[Part], name: &str) -> ReturnCode {
        let ints = body.iter().map(|p| *p as u32).collect::<Vec<u32>>();
        js_unwrap! {
            var body = (@{ints}).map(__part_num_to_str);

            return @{self.as_ref()}.spawnCreep(body, @{name});
        }
    }

    pub fn spawn_creep_with_options(
        &self,
        body: &[Part],
        name: &str,
        opts: &SpawnOptions,
    ) -> ReturnCode {
        let body = body.iter().map(|p| *p as u32).collect::<Vec<u32>>();

        let js_opts = js!(return {dryRun: @{opts.dry_run}};);

        if let Some(ref mem) = opts.memory {
            js!(@{&js_opts}.memory = @{mem.as_ref()};);
        }
        if !opts.energy_structures.is_empty() {
            js!(@{&js_opts}.energyStructures = @{&opts.energy_structures};);
        }
        if !opts.directions.is_empty() {
            js!(@{&js_opts}.directions = @{&opts.directions};);
        }
        js_unwrap!(@{self.as_ref()}.spawnCreep(@{body}, @{name}, @{js_opts}))
    }

    // TODO: support actually using Spawning properties.
    pub fn is_spawning(&self) -> bool {
        js_unwrap!(Boolean(@{self.as_ref()}.spawning))
    }

    pub fn recycle_creep(&self, target: &Creep) -> ReturnCode {
        js_unwrap!{@{self.as_ref()}.recycleCreep(@{target.as_ref()})}
    }

    pub fn renew_creep(&self, target: &Creep) -> ReturnCode {
        js_unwrap!{@{self.as_ref()}.renewCreep(@{target.as_ref()})}
    }
}

pub struct SpawnOptions {
    memory: Option<MemoryReference>,
    energy_structures: Vec<Reference>,
    dry_run: bool,
    directions: Vec<u32>,
}

impl SpawnOptions {
    pub fn new() -> SpawnOptions {
        SpawnOptions {
            memory: None,
            energy_structures: Vec::new(),
            dry_run: false,
            directions: Vec::new(),
        }
    }

    pub fn memory<T: Into<Option<MemoryReference>>>(&mut self, mem: T) {
        self.memory = mem.into();
    }

    /// This is most useful with the `.as_structure()` method on structures.
    pub fn energy_structures<T>(&mut self, structures: T)
    where
        T: IntoIterator,
        <T as IntoIterator>::Item: HasEnergyForSpawn,
    {
        self.energy_structures = structures.into_iter().map(|s| s.into()).collect();
    }

    pub fn dry_run(&mut self, dry_run: bool) {
        self.dry_run = dry_run;
    }

    pub fn directions(&mut self, directions: &[Direction]) {
        self.directions = directions.iter().map(|d| *d as u32).collect();
    }
}

simple_accessors! {
    Spawning;
    (directions -> directions -> Vec<Direction>),
    (name -> name -> String),
    (need_time -> needTime -> u32),
    (remaining_time -> remainingTime -> u32),
    (spawn -> spawn -> StructureSpawn),
}

impl Spawning {
    pub fn cancel(&self) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.cancel())
    }

    pub fn set_directions(&self, directions: &[Direction]) -> ReturnCode {
        let int_dirs: Vec<u32> = directions.iter().map(|d| *d as u32).collect();
        js_unwrap!(@{self.as_ref()}.setDirections(@{int_dirs}))
    }
}
