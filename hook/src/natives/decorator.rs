use crate::types::NativeVector3;

pub fn decor_set_time(entity: i32, propertyName: &std::ffi::CString, timestamp: i32) -> bool {
    let value = native!(
        bool,
        0x95AED7B8E39ECAA4,
        native_parameters!(entity, propertyName.as_ptr(), timestamp)
    );

    value
}

pub fn decor_set_bool(entity: i32, propertyName: &std::ffi::CString, value: bool) -> bool {
    let value = native!(
        bool,
        0x6B1E8E2ED1335B71,
        native_parameters!(entity, propertyName.as_ptr(), value)
    );

    value
}

pub fn decor_set_float(entity: i32, propertyName: &std::ffi::CString, value: f32) -> bool {
    let value = native!(
        bool,
        0x211AB1DD8D0F363A,
        native_parameters!(entity, propertyName.as_ptr(), value)
    );

    value
}

pub fn decor_set_int(entity: i32, propertyName: &std::ffi::CString, value: i32) -> bool {
    let value = native!(
        bool,
        0x0CE3AA5E1CA19E10,
        native_parameters!(entity, propertyName.as_ptr(), value)
    );

    value
}

pub fn decor_get_bool(entity: i32, propertyName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xDACE671663F2F5DB,
        native_parameters!(entity, propertyName.as_ptr())
    );

    value
}

pub fn decor_get_float(entity: i32, propertyName: &std::ffi::CString) -> f32 {
    let value = native!(
        f32,
        0x6524A2F114706F43,
        native_parameters!(entity, propertyName.as_ptr())
    );

    value
}

pub fn decor_get_int(entity: i32, propertyName: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0xA06C969B02A97298,
        native_parameters!(entity, propertyName.as_ptr())
    );

    value
}

pub fn decor_exist_on(entity: i32, propertyName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x05661B80A8C9165F,
        native_parameters!(entity, propertyName.as_ptr())
    );

    value
}

pub fn decor_remove(entity: i32, propertyName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x00EE9F297C738720,
        native_parameters!(entity, propertyName.as_ptr())
    );

    value
}

pub fn decor_register(propertyName: &std::ffi::CString, type_esc: i32) -> () {
    let value = native!(
        (),
        0x9FD90732F56403CE,
        native_parameters!(propertyName.as_ptr(), type_esc)
    );

    value
}

pub fn decor_is_registered_as_type(propertyName: &std::ffi::CString, type_esc: i32) -> bool {
    let value = native!(
        bool,
        0x4F14F9F870D6FBC8,
        native_parameters!(propertyName.as_ptr(), type_esc)
    );

    value
}

pub fn decor_register_lock() -> () {
    let value = native!((), 0xA9D14EEA259F9248, native_parameters!());

    value
}
