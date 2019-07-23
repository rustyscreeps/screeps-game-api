//! See [http://docs.screeps.com/api/#Game.shard]
//!
//! [http://docs.screeps.com/api/#Game.shard]: http://docs.screeps.com/api/#Game.shard
use crate::macros::*;

/// See [http://docs.screeps.com/api/#Game.shard]
///
/// [http://docs.screeps.com/api/#Game.shard]: http://docs.screeps.com/api/#Game.shard
pub fn name() -> String {
    js_unwrap!(Game.shard.name)
}

/// See [http://docs.screeps.com/api/#Game.shard]
///
/// [http://docs.screeps.com/api/#Game.shard]: http://docs.screeps.com/api/#Game.shard
pub fn shard_type() -> String {
    js_unwrap!(Game.shard.type)
}

/// See [http://docs.screeps.com/api/#Game.shard]
///
/// [http://docs.screeps.com/api/#Game.shard]: http://docs.screeps.com/api/#Game.shard
pub fn ptr() -> bool {
    js_unwrap!(Game.shard.ptr)
}
