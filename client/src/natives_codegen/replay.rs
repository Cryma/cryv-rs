use crate::natives::NativeVector3;

pub fn _0x7e2bd3ef6c205f09(p0: String, p1: bool) -> () {
    let p0_cstring = std::ffi::CString::new(p0).unwrap();
    let value = native!(
        (),
        0x7E2BD3EF6C205F09,
        native_parameters!(p0_cstring.as_ptr(), p1)
    );

    value
}

pub fn _is_interior_rendering_disabled() -> bool {
    let value = native!(bool, 0x95AB8B5C992C7B58, native_parameters!());

    value
}

pub fn _0x5ad3932daeb1e5d3() -> () {
    let value = native!((), 0x5AD3932DAEB1E5D3, native_parameters!());

    value
}

pub fn _0xe058175f8eafe79a(p0: bool) -> () {
    let value = native!((), 0xE058175F8EAFE79A, native_parameters!(p0));

    value
}

pub fn _reset_editor_values() -> () {
    let value = native!((), 0x3353D13F09307691, native_parameters!());

    value
}

pub fn _activate_rockstar_editor() -> () {
    let value = native!((), 0x49DA8145672B2725, native_parameters!());

    value
}
