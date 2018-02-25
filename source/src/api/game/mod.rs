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

    get_from_js!(describe_exits(room_name: &str) -> {
        Game.cpu.describeExits(@{room_name})
    } -> stdweb::Object);
}

macro_rules! game_map_access {
    ($mod_name:ident, $type:path, $js_inner:expr) => (
        pub mod $mod_name {
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
    (construction_sites, ::api::objects::ConstructionSite, Game.constructionSites),
    (creeps, ::api::objects::Creep, Game.creeps),
    (flags, ::api::objects::Flag, Game.flags),
    (rooms, ::api::objects::Room, Game.rooms),
    (spawns, ::api::objects::StructureSpawn, Game.spawns),
    (structures, ::api::objects::Structure, Game.structures),
}
