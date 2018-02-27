#![recursion_limit = "256"]
extern crate base64;
extern crate bincode;
extern crate fern;
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate screeps;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;

mod logging;
mod memory;

use screeps::{Part, ReturnCode};
use screeps::{find, RoomObjectProperties};

fn main() {
    stdweb::initialize();
    logging::setup_logging(logging::Info);

    js! {
        module.exports.loop = @{game_loop};
    }
}

fn game_loop() {
    info!("hello, world!");

    let mut mem = match memory::setup() {
        Some(v) => v,
        None => {
            warn!("waiting a tick for memory");
            return;
        }
    };

    info!("starting CPU: {:?}", screeps::game::cpu::get_used());

    debug!("running spawns");
    for spawn in screeps::game::spawns::values() {
        debug!("running spawn {}", spawn.name());
        let body = [Part::Move, Part::Move, Part::Carry, Part::Work];
        if spawn.energy() >= body.iter().map(Part::cost).sum() {
            let mut name = 0;
            let res = loop {
                info!("spawning {}", name);
                let res = spawn.spawn_creep(&body, &name.to_string());

                if res == ReturnCode::NameExists {
                    name += 1; // todo: actually random stuff here!
                } else {
                    break res;
                }
            };
            if res != ReturnCode::Ok {
                warn!("couldn't spawn: {:?}", res);
            }
        }
    }

    debug!("running creeps");
    for creep in screeps::game::creeps::values() {
        let name = creep.name();
        debug!("running creep {}", name);
        if creep.spawning() {
            continue;
        }
        let id: i32 = match name.parse() {
            Ok(v) => v,
            Err(_) => {
                warn!("creep {} has invalid name: killing.", name);
                creep.suicide();
                continue;
            }
        };

        if mem.creeps.get(&id).cloned().unwrap_or(false) {
            if creep.carry_total() == 0 {
                mem.creeps.insert(id, false);
            }
        } else {
            if creep.carry_total() == creep.carry_capacity() {
                mem.creeps.insert(id, true);
            }
        }

        if mem.creeps.get(&id).cloned().unwrap_or(false) {
            if let Some(c) = creep.room().controller() {
                let r = creep.upgrade_controller(&c);
                if r == ReturnCode::NotInRange {
                    creep.move_to(&c);
                } else if r != ReturnCode::Ok {
                    warn!("couldn't upgrade: {:?}", r);
                }
            } else {
                warn!("creep room has no controller!");
            }
        } else {
            let source = &creep.room().find(find::SOURCES)[0];
            if creep.pos().is_near_to(&source) {
                let r = creep.harvest(&source);
                if r != ReturnCode::Ok {
                    warn!("couldn't harvest: {:?}", r);
                }
            } else {
                creep.move_to(&source);
            }
        }
    }

    let time = screeps::game::time();

    if time % 42 == 0 {
        info!("running memory cleanup");
        memory::cleanup::run(&mut mem);
    }

    memory::save(&mem);

    info!("ending cpu: {}", screeps::game::cpu::get_used())
}
