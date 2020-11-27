use crate::utility::StreamedModel;
use hook::natives::{entity, misc, vehicle};
use hook::types::NativeVector3;
use shared::bevy::math::Vec3;
use shared::{EntityModel, EntityTransform};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityHandle {
    pub handle: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vehicle {
    pub color_primary: i32,
    pub color_secondary: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ped;

pub fn get_entity_transform(entity_id: i32) -> EntityTransform {
    let position = entity::get_entity_coords(entity_id, false);
    let rotation = entity::get_entity_rotation(entity_id, 2);
    let velocity = entity::get_entity_velocity(entity_id);

    EntityTransform {
        position: Vec3::new(position.x, position.y, position.z),
        rotation: Vec3::new(rotation.x, rotation.y, rotation.z),
        velocity: Vec3::new(velocity.x, velocity.y, velocity.z),
    }
}

pub fn get_entity_model(entity_id: i32) -> EntityModel {
    let model = entity::get_entity_model(entity_id);

    EntityModel { model }
}

pub fn create_vehicle_with_model_name(
    model: &str,
    position: NativeVector3,
    rotation: NativeVector3,
    color_primary: i32,
    color_secondary: i32,
) -> (EntityHandle, EntityModel, EntityTransform, Vehicle) {
    let model_cstring = std::ffi::CString::new(model).unwrap();
    let model = misc::get_hash_key(&model_cstring);

    create_vehicle_with_model_hash(model, position, rotation, color_primary, color_secondary)
}

pub fn create_vehicle_with_model_hash(
    model: u32,
    position: NativeVector3,
    rotation: NativeVector3,
    color_primary: i32,
    color_secondary: i32,
) -> (EntityHandle, EntityModel, EntityTransform, Vehicle) {
    let _streamed_model = StreamedModel::new(model);

    let handle = vehicle::create_vehicle(
        model, position.x, position.y, position.z, rotation.z, false, false, false,
    );

    entity::set_entity_rotation(handle, rotation.x, rotation.y, rotation.z, 2, true);
    vehicle::set_vehicle_dirt_level(handle, 0.0);
    vehicle::set_vehicle_colours(handle, color_primary, color_secondary);

    let handle = EntityHandle { handle };
    let model = EntityModel { model };

    let transform = EntityTransform {
        position: Vec3::new(position.x, position.y, position.z),
        rotation: Vec3::new(rotation.x, rotation.y, rotation.z),
        velocity: Vec3::default(),
    };

    let vehicle = Vehicle {
        color_primary,
        color_secondary,
    };

    (handle, model, transform, vehicle)
}
