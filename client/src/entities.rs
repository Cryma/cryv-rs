use crate::utility::StreamedModel;
use hook::natives::{entity, misc, vehicle};
use hook::types::NativeVector3;

// The name is prefixed with 'CryV' as it would inevitably conflict with 'Entity' from bevy_ecs
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CryVEntity {
    pub handle: i32,
    pub model: u32,
    pub position: NativeVector3, // TODO: Use proper vector type
    pub rotation: NativeVector3, // TODO: Should probably use a quaternion
}

impl Default for CryVEntity {
    fn default() -> Self {
        CryVEntity {
            handle: 0,
            model: 0,
            position: NativeVector3::default(),
            rotation: NativeVector3::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vehicle {
    pub color_primary: i32,
    pub color_secondary: i32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ped;

pub fn create_vehicle_with_model_name(
    model: &str,
    position: NativeVector3,
    rotation: NativeVector3,
    color_primary: i32,
    color_secondary: i32,
) -> (CryVEntity, Vehicle) {
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
) -> (CryVEntity, Vehicle) {
    let _streamed_model = StreamedModel::new(model);

    let handle = vehicle::create_vehicle(
        model, position.x, position.y, position.z, rotation.z, false, false, false,
    );

    entity::set_entity_rotation(handle, rotation.x, rotation.y, rotation.z, 2, true);
    vehicle::set_vehicle_dirt_level(handle, 0.0);
    vehicle::set_vehicle_colours(handle, color_primary, color_secondary);

    let entity = CryVEntity {
        handle,
        model,
        position,
        rotation,
    };

    let vehicle = Vehicle {
        color_primary,
        color_secondary,
    };

    (entity, vehicle)
}
