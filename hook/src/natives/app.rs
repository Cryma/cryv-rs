use crate::types::NativeVector3;

pub fn app_data_valid() -> bool {
    let value = native!(bool, 0x846AA8E7D55EE5B6, native_parameters!());

    value
}

pub fn app_get_int(property: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0xD3A58A12C77D9D4B,
        native_parameters!(property.as_ptr())
    );

    value
}

pub fn app_get_float(property: &std::ffi::CString) -> f32 {
    let value = native!(
        f32,
        0x1514FB24C02C2322,
        native_parameters!(property.as_ptr())
    );

    value
}

pub fn app_get_string(property: &std::ffi::CString) -> String {
    let value = native!(
        *const i8,
        0x749B023950D2311C,
        native_parameters!(property.as_ptr())
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn app_set_int(property: &std::ffi::CString, value: i32) -> () {
    let value = native!(
        (),
        0x607E8E3D3E4F9611,
        native_parameters!(property.as_ptr(), value)
    );

    value
}

pub fn app_set_float(property: &std::ffi::CString, value: f32) -> () {
    let value = native!(
        (),
        0x25D7687C68E0DAA4,
        native_parameters!(property.as_ptr(), value)
    );

    value
}

pub fn app_set_string(property: &std::ffi::CString, value: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x3FF2FCEC4B7721B4,
        native_parameters!(property.as_ptr(), value.as_ptr())
    );

    value
}

pub fn app_set_app(appName: &std::ffi::CString) -> () {
    let value = native!((), 0xCFD0406ADAF90D2B, native_parameters!(appName.as_ptr()));

    value
}

pub fn app_set_block(blockName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x262AB456A3D21F93,
        native_parameters!(blockName.as_ptr())
    );

    value
}

pub fn app_clear_block() -> () {
    let value = native!((), 0x5FE1DF3342DB7DBA, native_parameters!());

    value
}

pub fn app_close_app() -> () {
    let value = native!((), 0xE41C65E07A5F05FC, native_parameters!());

    value
}

pub fn app_close_block() -> () {
    let value = native!((), 0xE8E3FCF72EAC0EF8, native_parameters!());

    value
}

pub fn app_has_linked_social_club_account() -> bool {
    let value = native!(bool, 0x71EEE69745088DA0, native_parameters!());

    value
}

pub fn app_has_synced_data(appName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xCA52279A7271517F,
        native_parameters!(appName.as_ptr())
    );

    value
}

pub fn app_save_data() -> () {
    let value = native!((), 0x95C5D356CDA6E85F, native_parameters!());

    value
}

pub fn app_get_deleted_file_status() -> i32 {
    let value = native!(i32, 0xC9853A2BE3DED1A6, native_parameters!());

    value
}

pub fn app_delete_app_data(appName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x44151AEA95C8A003,
        native_parameters!(appName.as_ptr())
    );

    value
}
