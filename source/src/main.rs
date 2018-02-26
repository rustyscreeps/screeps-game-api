#![recursion_limit = "256"]
#[macro_use]
extern crate enum_primitive;
extern crate fern;
#[macro_use]
extern crate log;
extern crate num_traits;
#[macro_use]
extern crate stdweb;

mod logging;
pub mod api;

use api::{Part, ReturnCode};
use api::{find, RoomObjectProperties};

fn main() {
    stdweb::initialize();
    logging::setup_logging(1);

    js! {
        module.exports.loop = @{game_loop};
    }
}

fn game_loop() {
    info!("hello, world!");
    info!("starting CPU: {:?}", api::game::cpu::get_used());

    for spawn in api::game::spawns::values() {
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

    for creep in api::game::creeps::values() {
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
    info!("ending cpu: {}", api::game::cpu::get_used())
}
