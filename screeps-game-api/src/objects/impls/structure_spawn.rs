use stdweb::unstable::TryInto;

use {Part, ReturnCode};
use objects::StructureSpawn;

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

            return @{&self.0}.spawnCreep(body, @{name});
        }).try_into()
            .expect("expected StructureSpawn::spawnCreep to return an integer return code"))
    }
}
