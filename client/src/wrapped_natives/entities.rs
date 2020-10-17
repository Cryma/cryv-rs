use hook::natives::*;

pub fn delete_entity(handle: &mut i32) {
    entity::set_entity_as_no_longer_needed(handle);
    entity::set_entity_as_mission_entity(*handle, false, true);

    entity::delete_entity(handle);
}
