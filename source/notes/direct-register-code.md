
/*
idea for thing which we aren't implementing yet:
var register = new Array(1024);
var next_idx = 0;

function register_load_item(item) {
    while (register[next_idx] !== undefined) {
        next_idx += 17;
        next_idx %= 1024;
    }
    var stored_idx = next_idx;
    register[stored_idx] = item;
    next_idx += 1;
    return stored_idx;
}

function register_load_array(items) {
    var indices = new Array(items.length);
    for (var i = 0; i < items.length; i++) {
        indices[i] = register_load_item(items[i]);
    }
    return indices;
}

function register_drop_item(index) {
    register[index] = undefined;
}
use stdweb::{Value, unstable::TryInto};

/// Direct references to JS register.
struct Direct {
    register_index: u16,
}


impl Direct {
    #[inline]
    pub fn load_single(register_index: Value) {
        Direct {
            register_index: register_index.try_into().expect("expected register index to be a number"),
        }
    }

    pub fn load_arr()
}

 */
