use crate::types::NativeVector3;

pub fn get_water_height(x: f32, y: f32, z: f32, height: *mut f32) -> bool {
    let value = native!(
        bool,
        0xF6829842C06AE524,
        native_parameters!(x, y, z, height)
    );

    value
}

pub fn get_water_height_no_waves(x: f32, y: f32, z: f32, height: *mut f32) -> bool {
    let value = native!(
        bool,
        0x8EE6B53CE13A9794,
        native_parameters!(x, y, z, height)
    );

    value
}

pub fn test_probe_against_water(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    result: *mut NativeVector3,
) -> bool {
    let value = native!(
        bool,
        0xFFA5D878809819DB,
        native_parameters!(x1, y1, z1, x2, y2, z2, result)
    );

    value
}

pub fn test_probe_against_all_water(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
) -> bool {
    let value = native!(
        bool,
        0x8974647ED222EA5F,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn test_vertical_probe_against_all_water(
    x: f32,
    y: f32,
    z: f32,
    p3: u32,
    height: *mut f32,
) -> bool {
    let value = native!(
        bool,
        0x2B3451FA1E3142E2,
        native_parameters!(x, y, z, p3, height)
    );

    value
}

pub fn modify_water(x: f32, y: f32, radius: f32, height: f32) -> () {
    let value = native!(
        (),
        0xC443FD757C3BA637,
        native_parameters!(x, y, radius, height)
    );

    value
}

pub fn _add_current_rise(x: f32, y: f32, z: f32, radius: f32, unk: f32) -> i32 {
    let value = native!(
        i32,
        0xFDBF4CDBC07E1706,
        native_parameters!(x, y, z, radius, unk)
    );

    value
}

pub fn _remove_current_rise(p0: i32) -> () {
    let value = native!((), 0xB1252E3E59A82AAF, native_parameters!(p0));

    value
}

pub fn set_deep_ocean_scaler(intensity: f32) -> () {
    let value = native!((), 0xB96B00E976BE977F, native_parameters!(intensity));

    value
}

pub fn get_deep_ocean_scaler() -> f32 {
    let value = native!(f32, 0x2B2A2CC86778B619, native_parameters!());

    value
}

pub fn _0x547237aa71ab44de(p0: f32) -> () {
    let value = native!((), 0x547237AA71AB44DE, native_parameters!(p0));

    value
}

pub fn reset_deep_ocean_scaler() -> () {
    let value = native!((), 0x5E5E99285AE812DB, native_parameters!());

    value
}
