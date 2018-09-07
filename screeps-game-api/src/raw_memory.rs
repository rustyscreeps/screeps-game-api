get_from_js!(active_segments -> { Object.keys(RawMemory.segments).map(Number) } -> Vec<i32>);
get_from_js!(get_segment(id: i32) -> { RawMemory.segments[@{id}] } -> Option<String>);
pub fn set_segment(id: i32, data: &str) {
    js! {
        RawMemory.segments[@{id}] = @{data};
    }
}
get_from_js!(foreign_segment_username -> {
    RawMemory.forignSegment && RawMemory.forignSegment.username
} -> Option<String>);
get_from_js!(foreign_segment_id -> {
    RawMemory.forignSegment && RawMemory.forignSegment.id
} -> Option<i32>);
get_from_js!(foreign_segment_data -> {
    RawMemory.forignSegment && RawMemory.forignSegment.data
} -> Option<String>);

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

pub fn set_active_foreign_segment(username: &str, id: Option<i32>) {
    match id {
        Some(id) => js! { RawMemory.setActiveForeignSegment(@{username}, @{id}); },
        None => js! { RawMemory.setActiveForeignSegment(@{username}); },
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

get_from_js!(get() -> {RawMemory.get()} -> String);

pub fn set(value: String) {
    js!{
        RawMemory.set(@{value});
    }
}
