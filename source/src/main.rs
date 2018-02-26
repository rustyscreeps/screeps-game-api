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
            let res = spawn.spawn_creep(
                &[Part::Move, Part::Move, Part::Carry, Part::Work],
                "worker1",
            );
            if res != ReturnCode::Ok {
                warn!("couldn't spawn: {:?}", res);
            }
        }
    }

    for creep in api::game::creeps::values() {
        if creep.carry_total() == 0 {
            creep.say("no energy", false);
        } else {
            if let Some(c) = creep.room().controller() {
                if creep.pos().is_near_to(&c) {
                    let r = creep.upgrade_controller(&c);
                    if r == ReturnCode::NotInRange {
                        creep.move_to(&c);
                    } else if r != ReturnCode::Ok {
                        warn!("couldn't upgrade: {:?}", r);
                    }
                }
            }
        }
    }
}
