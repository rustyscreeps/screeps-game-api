use stdweb::Reference;
use stdweb::unstable::TryInto;

use objects::StructureSpawn;
use memory::MemoryReference;

use {Direction, Part, ReturnCode, StructureProperties};

simple_accessors! {
    StructureSpawn;
    (energy -> energy -> i32),
    (energy_capacity -> energyCapacity -> i32),
    (name -> name -> String),
}

impl StructureSpawn {
    pub fn spawn_creep(&self, body: &[Part], name: &str) -> ReturnCode {
        let ints = body.iter().map(|p| *p as i32).collect::<Vec<i32>>();
        ((js! {
            var body = (@{ints}).map(__part_num_to_str);

            return @{self.as_ref()}.spawnCreep(body, @{name});
        }).try_into()
            .expect("expected StructureSpawn::spawnCreep to return an integer return code"))
    }

    pub fn spawn<'a>(&'a self, body: &'a [Part], name: &'a str) -> SpawnOptions<'a> {
        SpawnOptions {
            spawn: self,
            body: body,
            name: name,
            memory: None,
            energy_structures: Vec::new(),
            dry_run: false,
            directions: Vec::new(),
        }
    }
}

pub struct SpawnOptions<'a> {
    spawn: &'a StructureSpawn,
    body: &'a [Part],
    name: &'a str,
    memory: Option<MemoryReference>,
    energy_structures: Vec<Reference>,
    dry_run: bool,
    directions: Vec<i32>,
}

impl<'a> SpawnOptions<'a> {
    pub fn name(&mut self, name: &'a str) {
        self.name = name;
    }

    pub fn memory<T: Into<Option<MemoryReference>>>(&mut self, mem: T) {
        self.memory = mem.into();
    }

    /// This is most useful with the `.as_structure()` method on structures.
    pub fn energy_structures<T>(&mut self, structures: T)
    where
        T: IntoIterator,
        <T as IntoIterator>::Item: StructureProperties
    {
        self.energy_structures = structures.into_iter().map(|s| s.as_structure().0).collect();
    }

    pub fn dry_run(&mut self, dry_run: bool) {
        self.dry_run = dry_run;
    }

    pub fn directions(&mut self, directions: &[Direction]) {
        self.directions = directions.iter().map(|d| *d as i32).collect();
    }

    pub fn execute(&self) -> ReturnCode {
        if self.memory.is_none() && self.energy_structures.is_empty() && !self.dry_run && self.directions.is_empty() {
            self.spawn.spawn_creep(self.body, self.name)
        } else {
            let body = self.body.iter().map(|p| *p as i32).collect::<Vec<i32>>();

            let opts = js!({});
            if let Some(mem) = self.memory.as_ref() {
                js!(@{&opts}.memory = @{mem.as_ref()});
            }
            if !self.energy_structures.is_empty() {
                js!(@{&opts}.energyStructures = @{&self.energy_structures});
            }
            if self.dry_run {
                js!(@{&opts}.dryRun = @{self.dry_run});
            }
            if !self.directions.is_empty() {
                js!(@{&opts}.directions = @{&self.directions});
            }
            js_unwrap!(@{self.spawn.as_ref()}.spawnCreep(@{body}, @{self.name}, @{opts}))
        }
    }
}
