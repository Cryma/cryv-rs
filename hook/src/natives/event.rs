use crate::types::NativeVector3;

pub fn set_decision_maker(ped: i32, name: u32) -> () {
    let value = native!((), 0xB604A2942ADED0EE, native_parameters!(ped, name));

    value
}

pub fn clear_decision_maker_event_response(name: u32, eventType: i32) -> () {
    let value = native!((), 0x4FC9381A7AEE8968, native_parameters!(name, eventType));

    value
}

pub fn block_decision_maker_event(name: u32, eventType: i32) -> () {
    let value = native!((), 0xE42FCDFD0E4196F7, native_parameters!(name, eventType));

    value
}

pub fn unblock_decision_maker_event(name: u32, eventType: i32) -> () {
    let value = native!((), 0xD7CD9CF34F2C99E8, native_parameters!(name, eventType));

    value
}

pub fn add_shocking_event_at_position(
    eventType: i32,
    x: f32,
    y: f32,
    z: f32,
    duration: f32,
) -> u32 {
    let value = native!(
        u32,
        0xD9F8455409B525E9,
        native_parameters!(eventType, x, y, z, duration)
    );

    value
}

pub fn add_shocking_event_for_entity(eventType: i32, entity: i32, duration: f32) -> u32 {
    let value = native!(
        u32,
        0x7FD8F3BE76F89422,
        native_parameters!(eventType, entity, duration)
    );

    value
}

pub fn is_shocking_event_in_sphere(eventType: i32, x: f32, y: f32, z: f32, radius: f32) -> bool {
    let value = native!(
        bool,
        0x1374ABB7C15BAB92,
        native_parameters!(eventType, x, y, z, radius)
    );

    value
}

pub fn remove_shocking_event(event: u32) -> bool {
    let value = native!(bool, 0x2CDA538C44C6CCE5, native_parameters!(event));

    value
}

pub fn remove_all_shocking_events(p0: bool) -> () {
    let value = native!((), 0xEAABE8FDFA21274C, native_parameters!(p0));

    value
}

pub fn remove_shocking_event_spawn_blocking_areas() -> () {
    let value = native!((), 0x340F1415B68AEADE, native_parameters!());

    value
}

pub fn suppress_shocking_events_next_frame() -> () {
    let value = native!((), 0x2F9A292AD0A3BD89, native_parameters!());

    value
}

pub fn suppress_shocking_event_type_next_frame(eventType: i32) -> () {
    let value = native!((), 0x3FD2EC8BF1F1CF30, native_parameters!(eventType));

    value
}

pub fn suppress_agitation_events_next_frame() -> () {
    let value = native!((), 0x5F3B7749C112D552, native_parameters!());

    value
}
