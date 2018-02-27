use std::collections::HashMap;

use {bincode, screeps, base64};

pub mod cleanup;

#[derive(Serialize, Deserialize, Default)]
pub struct MemoryRoot {
    pub creeps: HashMap<i32, bool>,
}

/// Returns `true` if memory's loaded, `false` if we need to wait a tick for active segments
/// to change.
pub fn setup() -> Option<MemoryRoot> {
    // TODO: integrate lzma like lz-string?
    // TODO: grab/save this segment as serialized MemoryRoot with bincode with string encoding on
    // top of that
    // TODO: implement basic Memory API so it can be used for modification times.
    // TODO: base91 rust crate.
    let string = match screeps::raw_memory::get_segment(3) {
        Some(v) => v,
        None => {
            screeps::raw_memory::set_active_segments(&[3]);
            return None;
        }
    };
    let mem = base64::decode(&string)
        .map_err(|e| {
            error!(
                "base64 memory decoding failed! {}\n(bad memory: {:?})",
                e, string
            );
        })
        .and_then(|bytes| {
            bincode::deserialize(&bytes).map_err(|e| {
                error!(
                    "bincode memory decoding failed! {}\n(bad memory: {:?})",
                    e, string
                );
            })
        })
        .unwrap_or_else(|()| MemoryRoot::default());

    Some(mem)
}

pub fn save(mem: &MemoryRoot) {
    let string = {
        let bytes =
            bincode::serialize(mem).expect("expected bincode serialization to always succeed");
        base64::encode(&bytes)
    };
    screeps::raw_memory::set_segment(3, &string);
}
