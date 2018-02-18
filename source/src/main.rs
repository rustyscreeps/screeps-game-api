#[macro_use]
extern crate stdweb;

fn main() {
    stdweb::initialize();

    js! {
        module.exports.loop = @{game_loop};
    }
}

fn game_loop() {}
