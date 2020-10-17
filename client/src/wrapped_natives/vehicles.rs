use crate::utility::StreamedModel;
use hook::natives::*;

// TODO: Use proper structs for position and rotation
// Preferably through a library
pub fn create_vehicle_with_model_name(
    model: &str,
    position: (f32, f32, f32),
    rotation: (f32, f32, f32),
) -> i32 {
    let model_cstring = std::ffi::CString::new(model).unwrap();
    let hash = misc::get_hash_key(&model_cstring);

    create_vehicle_with_model_hash(hash, position, rotation)
}

pub fn create_vehicle_with_model_hash(
    model: u32,
    position: (f32, f32, f32),
    rotation: (f32, f32, f32),
) -> i32 {
    let _streamed_model = StreamedModel::new(model);

    let handle = vehicle::create_vehicle(
        model, position.0, position.1, position.2, position.2, false, false, false,
    );

    entity::set_entity_rotation(handle, rotation.0, rotation.1, rotation.2, 2, true);
    vehicle::set_vehicle_dirt_level(handle, 0.0);

    handle
}
