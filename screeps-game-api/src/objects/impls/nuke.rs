use objects::Nuke;

simple_accessors! {
    Nuke;
    // id from HasID
    (lauch_room_name -> lauchRoomName -> String),
    (time_to_land -> timeToLand -> u32),
}
