//! Interface for Screeps [`RawMemory`] global object.
//!
//! [`RawMemory`]: https://docs.screeps.com/api/#RawMemory

use serde::Deserialize;

use crate::macros::*;

#[derive(Deserialize, Debug)]
pub struct ForeignSegment {
    username: String,
    id: String,
    data: String,
}

js_deserializable!(ForeignSegment);

pub fn get_active_segments() -> Vec<u32> {
    js_unwrap!(Object.keys(RawMemory.segments).map(Number))
}

/// Sets active segments (max 10 ids).
pub fn set_active_segments(ids: &[u32]) {
    assert!(
        ids.len() <= 10,
        "can't set more than 10 active segments at a time"
    );
    js! { @(no_return)
        RawMemory.setActiveSegments(@{ids});
    }
}

pub fn get_segment(id: u32) -> Option<String> {
    js_unwrap!(RawMemory.segments[@{id}])
}

pub fn set_segment(id: u32, data: &str) {
    js! { @(no_return)
        RawMemory.segments[@{id}] = @{data};
    }
}

/// This drops the reference to a segment; it doesn't affect the content of the
/// segment.
///
/// This is the equivalent of doing `delete RawMemory.segments[id]`. Again, this
/// only deletes the local view of the segment, not the serialized one. It may
/// be used to `set_segment` a new segment that wasn't part of the original 10
/// active segments.
pub fn drop_segment(id: u32) {
    js! { @(no_return)
        delete RawMemory.segments[@{id}];
    }
}

pub fn get_foreign_segment() -> ForeignSegment {
    js_unwrap!(RawMemory.foreignSegment)
}

/// Implements `RawMemory.setActiveForeignSegment`
///
/// To use the default public segment of `username` (as set with
/// [`set_default_public_segment`]), Use `None` instead of `Some(id)`.
///
/// To clear the foreign segment, pass the empty string `""` as a username.
pub fn set_active_foreign_segment(username: &str, id: Option<u32>) {
    if username == "" {
        js! { @(no_return)
            RawMemory.setActiveForeignSegment(null);
        }
    } else {
        match id {
            Some(id) => js! { @(no_return)
                RawMemory.setActiveForeignSegment(@{username}, @{id});
            },
            None => js! { @(no_return)
                RawMemory.setActiveForeignSegment(@{username});
            },
        };
    };
}

pub fn set_default_public_segment(id: u32) {
    js! { @(no_return)
        RawMemory.setDefaultPublicSegment(@{id});
    }
}

pub fn set_public_segments(ids: &[u32]) {
    js! { @(no_return)
        RawMemory.setPublicSegments(@{ids});
    }
}

pub fn get() -> String {
    js_unwrap!(RawMemory.get())
}

pub fn set(value: &str) {
    js! { @(no_return)
        RawMemory.set(@{value});
    }
}
