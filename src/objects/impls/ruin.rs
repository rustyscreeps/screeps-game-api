use crate::{
    constants::StructureType,
    objects::{RawObjectId, Ruin},
    traits::TryInto,
};

simple_accessors! {
    impl Ruin {
        pub fn destroy_time() -> u32 = destroyTime;
    }
}

impl Ruin {
    /// Ruin.structure doesn't return complete object data, so instead of
    /// implementing that directly, this function exposes relevant
    /// properties of the ruin's structure directly in a tuple of the type,
    /// id, and owner
    pub fn structure_info(&self) -> (StructureType, RawObjectId, Option<String>) {
        (
            js_unwrap!(__structure_type_str_to_num(@{self.as_ref()}.structure.structureType)),
            RawObjectId::from_packed_js_val(
                js_unwrap!(object_id_to_packed(@{self.as_ref()}.structure.id)),
            )
            .expect(
                "expected ruin structure's JavaScript id to be a 12-byte number encoded in hex",
            ),
            (js! {
                var self = @{self.as_ref()};
                if (self.structure.owner) {
                    return self.structure.owner.username;
                } else {
                    return null;
                }
            })
            .try_into()
            .expect("expected ruin structure's owner.username to be a string"),
        )
    }
}
