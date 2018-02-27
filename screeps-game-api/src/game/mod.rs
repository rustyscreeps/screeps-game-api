// TODO: split these out into separate files once we add documentation.
//
// Right now, they can all fit in here because they're pretty small.
pub mod cpu {
    use std::collections;
    get_from_js!(limit -> { Game.cpu.limit } -> f64);
    get_from_js!(tick_limit -> { Game.cpu.tickLimit } -> f64);
    get_from_js!(bucket -> { Game.cpu.bucket } -> f64);
    get_from_js!(shard_limits -> { Game.cpu.shardLimits } -> collections::HashMap<String, f64>);
    get_from_js!(get_used -> { Game.cpu.getUsed() } -> f64);
}
pub mod map {
    use stdweb;
    use {RoomPosition, Terrain};

    get_from_js!(describe_exits(room_name: &str) -> {
        Game.cpu.describeExits(@{room_name})
    } -> stdweb::Object);

    get_from_js!(get_terrain_at(pos: &RoomPosition) -> {
        __terrain_type_str_to_num(Game.map.getTerrainAt(@{pos.as_ref()}))
    } -> Terrain);

    get_from_js!(get_world_size -> {
        Game.map.getWorldSize()
    } -> i32);

    get_from_js!(is_room_available(room_name: &str) -> {
        Game.map.isRoomAvailable(@{room_name})
    } -> bool);
}

macro_rules! game_map_access {
    ($mod_name:ident, $type:path, $js_inner:expr) => (
        pub mod $mod_name {
            use objects;
            get_from_js!(names -> { Object.keys($js_inner) } -> Vec<String>);
            get_from_js!(values -> { Object.values($js_inner) } -> Vec<$type>);
            get_from_js!(get(name: &str) -> { $js_inner[@{name}]} -> $type);
        }
    );
    ($(
        ($mod:ident, $type:path, $js:expr)
    ),* $(,)*) => {
        $(
            game_map_access!($mod, $type, $js);
        )*
    };
}

game_map_access! {
    (construction_sites, objects::ConstructionSite, Game.constructionSites),
    (creeps, objects::Creep, Game.creeps),
    (flags, objects::Flag, Game.flags),
    (rooms, objects::Room, Game.rooms),
    (spawns, objects::StructureSpawn, Game.spawns),
    (structures, objects::Structure, Game.structures),
}
