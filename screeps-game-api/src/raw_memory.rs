//! Interface for Screeps `RawMemory` global object.
#[derive(Deserialize, Debug)]
pub struct ForeignSegment {
    username: String,
    id: String,
    data: String,
}

js_deserializable!(ForeignSegment);

get_from_js!(get_active_segments -> {
    Object.keys(RawMemory.segments).map(Number)
} -> Vec<i32>);

/// Sets active segments (max 10 ids).
pub fn set_active_segments(ids: &[i32]) {
    assert!(
        ids.len() <= 10,
        "can't set more than 10 active segments at a time"
    );
    js! {
        RawMemory.setActiveSegments(@{ids});
    }
}

get_from_js!(get_segment(id: u32) -> {
    RawMemory.segments[@{id}]
} -> Option<String>);

pub fn set_segment(id: i32, data: &str) {
    js! {
        RawMemory.segments[@{id}] = @{data};
    }
}

get_from_js!(get_foreign_segment -> {
    RawMemory.foreignSegment
} -> ForeignSegment);

/// Implements `RawMemory.setActiveForeignSegment`
///
/// To use the default public segment of `username` (as set with
/// [`set_default_public_segment`]), Use `None` instead of `Some(id)`.
///
/// To clear the foreign segment, pass the empty string `""` as a username.
///
pub fn set_active_foreign_segment(username: &str, id: Option<u32>) {
    if username == "" {
        js! { RawMemory.setActiveForeignSegment(null); }
    } else {
        match id {
            Some(id) => js! { RawMemory.setActiveForeignSegment(@{username}, @{id}); },
            None => js! { RawMemory.setActiveForeignSegment(@{username}); },
        };
    };
}

pub fn set_default_public_segment(id: i32) {
    js! {
        RawMemory.setDefaultPublicSegment(@{id});
    }
}

pub fn set_public_segments(ids: &[i32]) {
    js! {
        RawMemory.setPublicSegments(@{ids});
    }
}

get_from_js!(get -> {RawMemory.get()} -> String);

pub fn set(value: &str) {
    js!{
        RawMemory.set(@{value});
    }
}
