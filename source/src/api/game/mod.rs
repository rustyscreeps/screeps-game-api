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

pub mod construction_sites {
    use api::objects::ConstructionSite;
    get_from_js!(names -> { Object.keys(Game.constructionSites) } -> Vec<String>);
    get_from_js!(get(name: &str) -> { Game.constructionSites[@{name}]} -> ConstructionSite);
}

pub mod creeps {
    use api::objects::Creep;
    get_from_js!(names -> { Object.keys(Game.creeps) } -> Vec<String>);
    get_from_js!(get(name: &str) -> { Game.creeps[@{name}] } -> Creep);
}
