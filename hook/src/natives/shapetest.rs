use crate::types::NativeVector3;

pub fn start_shape_test_los_probe(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    flags: i32,
    entity: i32,
    p8: i32,
) -> i32 {
    let value = native!(
        i32,
        0x7EE9F5D83DD4F90E,
        native_parameters!(x1, y1, z1, x2, y2, z2, flags, entity, p8)
    );

    value
}

pub fn start_expensive_synchronous_shape_test_los_probe(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    flags: i32,
    entity: i32,
    p8: i32,
) -> i32 {
    let value = native!(
        i32,
        0x377906D8A31E5586,
        native_parameters!(x1, y1, z1, x2, y2, z2, flags, entity, p8)
    );

    value
}

pub fn start_shape_test_bounding_box(entity: i32, flags1: i32, flags2: i32) -> i32 {
    let value = native!(
        i32,
        0x052837721A854EC7,
        native_parameters!(entity, flags1, flags2)
    );

    value
}

pub fn start_shape_test_box(
    x: f32,
    y: f32,
    z: f32,
    x1: f32,
    y2: f32,
    z2: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    p9: u32,
    flags: i32,
    entity: i32,
    p12: u32,
) -> i32 {
    let value = native!(
        i32,
        0xFE466162C4401D18,
        native_parameters!(x, y, z, x1, y2, z2, rotX, rotY, rotZ, p9, flags, entity, p12)
    );

    value
}

pub fn start_shape_test_bound(entity: i32, flags1: i32, flags2: i32) -> i32 {
    let value = native!(
        i32,
        0x37181417CE7C8900,
        native_parameters!(entity, flags1, flags2)
    );

    value
}

pub fn start_shape_test_capsule(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    radius: f32,
    flags: i32,
    entity: i32,
    p9: i32,
) -> i32 {
    let value = native!(
        i32,
        0x28579D1B8F8AAC80,
        native_parameters!(x1, y1, z1, x2, y2, z2, radius, flags, entity, p9)
    );

    value
}

pub fn start_shape_test_swept_sphere(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    radius: f32,
    flags: i32,
    entity: i32,
    p9: u32,
) -> i32 {
    let value = native!(
        i32,
        0xE6AC6C45FBE83004,
        native_parameters!(x1, y1, z1, x2, y2, z2, radius, flags, entity, p9)
    );

    value
}

pub fn _start_shape_test_surrounding_coords(
    pVec1: *mut NativeVector3,
    pVec2: *mut NativeVector3,
    flag: i32,
    entity: i32,
    flag2: i32,
) -> i32 {
    let value = native!(
        i32,
        0xFF6BE494C7987F34,
        native_parameters!(pVec1, pVec2, flag, entity, flag2)
    );

    value
}

pub fn get_shape_test_result(
    shapeTestHandle: i32,
    hit: *mut bool,
    endCoords: *mut NativeVector3,
    surfaceNormal: *mut NativeVector3,
    entityHit: *mut i32,
) -> i32 {
    let value = native!(
        i32,
        0x3D87450E15D98694,
        native_parameters!(shapeTestHandle, hit, endCoords, surfaceNormal, entityHit)
    );

    value
}

pub fn get_shape_test_result_including_material(
    shapeTestHandle: i32,
    hit: *mut bool,
    endCoords: *mut NativeVector3,
    surfaceNormal: *mut NativeVector3,
    materialHash: *mut u32,
    entityHit: *mut i32,
) -> i32 {
    let value = native!(
        i32,
        0x65287525D951F6BE,
        native_parameters!(
            shapeTestHandle,
            hit,
            endCoords,
            surfaceNormal,
            materialHash,
            entityHit
        )
    );

    value
}

pub fn release_script_guid_from_entity(entityHit: i32) -> () {
    let value = native!((), 0x2B3334BCA57CD799, native_parameters!(entityHit));

    value
}
