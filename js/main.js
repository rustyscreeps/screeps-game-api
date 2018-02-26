function __part_num_to_str(num) {
    switch (num) {
        case 0: return MOVE;
        case 1: return WORK;
        case 2: return CARRY;
        case 3: return ATTACK;
        case 4: return RANGED_ATTACK;
        case 5: return HEAL;
        case 6: return TOUGH;
        case 7: return CLAIM;
    }
}

function __part_str_to_num(str) {
    switch (str) {
        case MOVE: return 0;
        case WORK: return 1;
        case CARRY: return 2;
        case ATTACK: return 3;
        case RANGED_ATTACK: return 4;
        case HEAL: return 5;
        case TOUGH: return 6;
        case CLAIM: return 7;
    }
}

function _hasActiveBodypart(body, type) {
    for (var i = body.length - 1; i >= 0; i--) {
        if (body[i].hits <= 0) {
            break;
        }
        if (body[i].type === type) {
            return true;
        }
    }
    return false;
}

// __initialize defined above when files are concatenated
// it's signature is 'function __initialize( __wasm_module, __load_asynchronously ) {'
__initialize(new WebAssembly.Module(require('compiled')), false);
