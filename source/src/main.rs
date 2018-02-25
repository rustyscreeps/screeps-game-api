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
mod api;

fn main() {
    stdweb::initialize();
    logging::setup_logging(1);

    js! {
        module.exports.loop = @{game_loop};
    }
}

fn game_loop() {
    info!("hello, world!");
    info!("CPU usage: {:?}", api::game::cpu::get_used());
}
