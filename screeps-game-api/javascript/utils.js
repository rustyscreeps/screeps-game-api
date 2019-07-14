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
        default: throw new Error("unknown part integer encoding " + num);
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
        default: throw new Error("unknown part type " + str);
    }
}

function __look_num_to_str(num) {
    switch (num) {
        case 0: return LOOK_CREEPS;
        case 1: return LOOK_ENERGY;
        case 2: return LOOK_RESOURCES;
        case 3: return LOOK_SOURCES;
        case 4: return LOOK_MINERALS;
        case 5: return LOOK_STRUCTURES;
        case 6: return LOOK_FLAGS;
        case 7: return LOOK_CONSTRUCTION_SITES;
        case 8: return LOOK_NUKES;
        case 9: return LOOK_TERRAIN;
        case 10: return LOOK_TOMBSTONES;
        case 11: return LOOK_POWER_CREEPS;
        default: throw new Error("unknown look integer encoding " + num);
    }
}

function __structure_type_num_to_str(num) {
    switch (num) {
        case 0: return STRUCTURE_SPAWN;
        case 1: return STRUCTURE_EXTENSION;
        case 2: return STRUCTURE_ROAD;
        case 3: return STRUCTURE_WALL;
        case 4: return STRUCTURE_RAMPART;
        case 5: return STRUCTURE_KEEPER_LAIR;
        case 6: return STRUCTURE_PORTAL;
        case 7: return STRUCTURE_CONTROLLER;
        case 8: return STRUCTURE_LINK;
        case 9: return STRUCTURE_STORAGE;
        case 10: return STRUCTURE_TOWER;
        case 11: return STRUCTURE_OBSERVER;
        case 12: return STRUCTURE_POWER_BANK;
        case 13: return STRUCTURE_POWER_SPAWN;
        case 14: return STRUCTURE_EXTRACTOR;
        case 15: return STRUCTURE_LAB;
        case 16: return STRUCTURE_TERMINAL;
        case 17: return STRUCTURE_CONTAINER;
        case 18: return STRUCTURE_NUKER;
        default: throw new Error("unknown structure type integer encoding " + num);
    }
}

function __structure_type_str_to_num(str) {
    switch (str) {
        case STRUCTURE_SPAWN: return 0;
        case STRUCTURE_EXTENSION: return 1;
        case STRUCTURE_ROAD: return 2;
        case STRUCTURE_WALL: return 3;
        case STRUCTURE_RAMPART: return 4;
        case STRUCTURE_KEEPER_LAIR: return 5;
        case STRUCTURE_PORTAL: return 6;
        case STRUCTURE_CONTROLLER: return 7;
        case STRUCTURE_LINK: return 8;
        case STRUCTURE_STORAGE: return 9;
        case STRUCTURE_TOWER: return 10;
        case STRUCTURE_OBSERVER: return 11;
        case STRUCTURE_POWER_BANK: return 12;
        case STRUCTURE_POWER_SPAWN: return 13;
        case STRUCTURE_EXTRACTOR: return 14;
        case STRUCTURE_LAB: return 15;
        case STRUCTURE_TERMINAL: return 16;
        case STRUCTURE_CONTAINER: return 17;
        case STRUCTURE_NUKER: return 18;
        default: throw new Error("unknown structure type " + str);
    }
}

function __intershard_resource_type_num_to_str(num) {
    switch (num) {
        case 1: return SUBSCRIPTION_TOKEN;
        default: throw new Error("unknown inter-shard resource type integer encoding " + num);
    }
}

function __intershard_resource_type_str_to_num(str) {
    switch (str) {
        case SUBSCRIPTION_TOKEN: return 1;
        default: throw new Error("unknown inter-shard resource type " + str);
    }
}


function __resource_type_num_to_str(num) {
    switch (num) {
        case 1: return RESOURCE_ENERGY;
        case 2: return RESOURCE_POWER;
        case 3: return RESOURCE_HYDROGEN;
        case 4: return RESOURCE_OXYGEN;
        case 5: return RESOURCE_UTRIUM;
        case 6: return RESOURCE_LEMERGIUM;
        case 7: return RESOURCE_KEANIUM;
        case 8: return RESOURCE_ZYNTHIUM;
        case 9: return RESOURCE_CATALYST;
        case 10: return RESOURCE_GHODIUM;
        case 11: return RESOURCE_HYDROXIDE;
        case 12: return RESOURCE_ZYNTHIUM_KEANITE;
        case 13: return RESOURCE_UTRIUM_LEMERGITE;
        case 14: return RESOURCE_UTRIUM_HYDRIDE;
        case 15: return RESOURCE_UTRIUM_OXIDE;
        case 16: return RESOURCE_KEANIUM_HYDRIDE;
        case 17: return RESOURCE_KEANIUM_OXIDE;
        case 18: return RESOURCE_LEMERGIUM_HYDRIDE;
        case 19: return RESOURCE_LEMERGIUM_OXIDE;
        case 20: return RESOURCE_ZYNTHIUM_HYDRIDE;
        case 21: return RESOURCE_ZYNTHIUM_OXIDE;
        case 22: return RESOURCE_GHODIUM_HYDRIDE;
        case 23: return RESOURCE_GHODIUM_OXIDE;
        case 24: return RESOURCE_UTRIUM_ACID;
        case 25: return RESOURCE_UTRIUM_ALKALIDE;
        case 26: return RESOURCE_KEANIUM_ACID;
        case 27: return RESOURCE_KEANIUM_ALKALIDE;
        case 28: return RESOURCE_LEMERGIUM_ACID;
        case 29: return RESOURCE_LEMERGIUM_ALKALIDE;
        case 30: return RESOURCE_ZYNTHIUM_ACID;
        case 31: return RESOURCE_ZYNTHIUM_ALKALIDE;
        case 32: return RESOURCE_GHODIUM_ACID;
        case 33: return RESOURCE_GHODIUM_ALKALIDE;
        case 34: return RESOURCE_CATALYZED_UTRIUM_ACID;
        case 35: return RESOURCE_CATALYZED_UTRIUM_ALKALIDE;
        case 36: return RESOURCE_CATALYZED_KEANIUM_ACID;
        case 37: return RESOURCE_CATALYZED_KEANIUM_ALKALIDE;
        case 38: return RESOURCE_CATALYZED_LEMERGIUM_ACID;
        case 39: return RESOURCE_CATALYZED_LEMERGIUM_ALKALIDE;
        case 40: return RESOURCE_CATALYZED_ZYNTHIUM_ACID;
        case 41: return RESOURCE_CATALYZED_ZYNTHIUM_ALKALIDE;
        case 42: return RESOURCE_CATALYZED_GHODIUM_ACID;
        case 43: return RESOURCE_CATALYZED_GHODIUM_ALKALIDE;
        case 44: return RESOURCE_OPS;
        default: throw new Error("unknown resource type integer encoding " + num);
    }
}

function __resource_type_str_to_num(str) {
    switch (str) {
        case RESOURCE_ENERGY: return 1;
        case RESOURCE_POWER: return 2;
        case RESOURCE_HYDROGEN: return 3;
        case RESOURCE_OXYGEN: return 4;
        case RESOURCE_UTRIUM: return 5;
        case RESOURCE_LEMERGIUM: return 6;
        case RESOURCE_KEANIUM: return 7;
        case RESOURCE_ZYNTHIUM: return 8;
        case RESOURCE_CATALYST: return 9;
        case RESOURCE_GHODIUM: return 10;
        case RESOURCE_HYDROXIDE: return 11;
        case RESOURCE_ZYNTHIUM_KEANITE: return 12;
        case RESOURCE_UTRIUM_LEMERGITE: return 13;
        case RESOURCE_UTRIUM_HYDRIDE: return 14;
        case RESOURCE_UTRIUM_OXIDE: return 15;
        case RESOURCE_KEANIUM_HYDRIDE: return 16;
        case RESOURCE_KEANIUM_OXIDE: return 17;
        case RESOURCE_LEMERGIUM_HYDRIDE: return 18;
        case RESOURCE_LEMERGIUM_OXIDE: return 19;
        case RESOURCE_ZYNTHIUM_HYDRIDE: return 20;
        case RESOURCE_ZYNTHIUM_OXIDE: return 21;
        case RESOURCE_GHODIUM_HYDRIDE: return 22;
        case RESOURCE_GHODIUM_OXIDE: return 23;
        case RESOURCE_UTRIUM_ACID: return 24;
        case RESOURCE_UTRIUM_ALKALIDE: return 25;
        case RESOURCE_KEANIUM_ACID: return 26;
        case RESOURCE_KEANIUM_ALKALIDE: return 27;
        case RESOURCE_LEMERGIUM_ACID: return 28;
        case RESOURCE_LEMERGIUM_ALKALIDE: return 29;
        case RESOURCE_ZYNTHIUM_ACID: return 30;
        case RESOURCE_ZYNTHIUM_ALKALIDE: return 31;
        case RESOURCE_GHODIUM_ACID: return 32;
        case RESOURCE_GHODIUM_ALKALIDE: return 33;
        case RESOURCE_CATALYZED_UTRIUM_ACID: return 34;
        case RESOURCE_CATALYZED_UTRIUM_ALKALIDE: return 35;
        case RESOURCE_CATALYZED_KEANIUM_ACID: return 36;
        case RESOURCE_CATALYZED_KEANIUM_ALKALIDE: return 37;
        case RESOURCE_CATALYZED_LEMERGIUM_ACID: return 38;
        case RESOURCE_CATALYZED_LEMERGIUM_ALKALIDE: return 39;
        case RESOURCE_CATALYZED_ZYNTHIUM_ACID: return 40;
        case RESOURCE_CATALYZED_ZYNTHIUM_ALKALIDE: return 41;
        case RESOURCE_CATALYZED_GHODIUM_ACID: return 42;
        case RESOURCE_CATALYZED_GHODIUM_ALKALIDE: return 43;
        case RESOURCE_OPS: return 44;
        default: throw new Error("unknown resource type " + str);
    }
}

function __order_type_str_to_num(str) {
    switch (str) {
        case ORDER_SELL: return 0;
        case ORDER_BUY: return 1;
        default: throw new Error("unknown order type " + str);
    }
}

function __order_type_num_to_str(num) {
    switch (num) {
        case 0: return ORDER_SELL;
        case 1: return ORDER_BUY;
        default: throw new Error("unknown order type " + num);
    }
}
