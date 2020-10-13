use crate::types::NativeVector3;

pub fn get_zone_at_coords(x: f32, y: f32, z: f32) -> i32 {
    let value = native!(i32, 0x27040C25DE6CB2F4, native_parameters!(x, y, z));

    value
}

pub fn get_zone_from_name_id(zoneName: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0x98CD1D2934B76CC1,
        native_parameters!(zoneName.as_ptr())
    );

    value
}

pub fn get_zone_popschedule(zoneId: i32) -> i32 {
    let value = native!(i32, 0x4334BC40AA0CB4BB, native_parameters!(zoneId));

    value
}

pub fn get_name_of_zone(x: f32, y: f32, z: f32) -> String {
    let value = native!(*const i8, 0xCD90657D4C30E1CA, native_parameters!(x, y, z));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn set_zone_enabled(zoneId: i32, toggle: bool) -> () {
    let value = native!((), 0xBA5ECEEA120E5611, native_parameters!(zoneId, toggle));

    value
}

pub fn get_zone_scumminess(zoneId: i32) -> i32 {
    let value = native!(i32, 0x5F7B268D15BA0739, native_parameters!(zoneId));

    value
}

pub fn override_popschedule_vehicle_model(scheduleId: i32, vehicleHash: u32) -> () {
    let value = native!(
        (),
        0x5F7D596BAC2E7777,
        native_parameters!(scheduleId, vehicleHash)
    );

    value
}

pub fn clear_popschedule_override_vehicle_model(scheduleId: i32) -> () {
    let value = native!((), 0x5C0DE367AA0D911C, native_parameters!(scheduleId));

    value
}

pub fn get_hash_of_map_area_at_coords(x: f32, y: f32, z: f32) -> u32 {
    let value = native!(u32, 0x7EE64D51E8498728, native_parameters!(x, y, z));

    value
}
