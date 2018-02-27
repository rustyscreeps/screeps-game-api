use std::collections::HashSet;

use screeps;

use memory::MemoryRoot;

pub fn run(mem: &mut MemoryRoot) {
    let alive_creeps: HashSet<i32> = screeps::game::creeps::keys()
        .into_iter()
        .flat_map(|s| s.parse().ok())
        .collect();

    mem.creeps.retain(|key, _value| alive_creeps.contains(key));

    let game_mem = match screeps::memory::root().dict("creeps") {
        Some(v) => v,
        None => {
            warn!("not cleaning game creep memory: no Memory.creeps dict");
            return;
        }
    };

    for mem_name in game_mem.keys() {
        if let Ok(as_int) = mem_name.parse() {
            if alive_creeps.contains(&as_int) {
                continue;
            } else {
                debug!("cleaning up creep memory of creep {}", mem_name);
            }
        } else {
            warn!(
                "deleting creep memory of creep with nonsense name {}",
                mem_name
            );
        }
        game_mem.del(&mem_name)
    }
}
