#![recursion_limit = "256"]
extern crate fern;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate log;
extern crate screeps;

mod logging;

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
    info!("starting CPU: {:?}", screeps::game::cpu::get_used());

    debug!("running spawns");
    for spawn in screeps::game::spawns::values() {
        debug!("running spawn {}", spawn.name());
        if spawn.energy() == spawn.energy_capacity() {
            let mut name_end = 0;
            let res = loop {
                info!("spawning worker{}", name_end);
                let res = spawn.spawn_creep(
                    &[Part::Move, Part::Move, Part::Carry, Part::Work],
                    &format!("worker{}", name_end),
                );

                if res == ReturnCode::NameExists {
                    name_end += 1; // todo: actually random stuff here!
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
        debug!("running creep {}", creep.name());
        if creep.spawning() {
            continue;
        }
        if creep.carry_total() == 0 {
            let source = &creep.room().find(find::SOURCES)[0];
            if creep.pos().is_near_to(&source) {
                let r = creep.harvest(&source);
                if r != ReturnCode::Ok {
                    warn!("couldn't harvest: {:?}", r);
                }
            } else {
                creep.move_to(&source);
            }
        } else {
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
        }
    }
    info!("ending cpu: {}", screeps::game::cpu::get_used())
}
