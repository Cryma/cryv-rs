use crate::types::NativeVector3;

pub fn _localization_get_system_language() -> i32 {
    let value = native!(i32, 0x497420E022796B3F, native_parameters!());

    value
}

pub fn get_current_language() -> i32 {
    let value = native!(i32, 0x2BDD44CC428A7EAE, native_parameters!());

    value
}

pub fn _localization_get_system_date_format() -> i32 {
    let value = native!(i32, 0xA8AE43AEC1A61314, native_parameters!());

    value
}
