use crate::types::NativeVector3;

pub fn wait(ms: i32) -> () {
    let value = native!((), 0x4EDE34FBADD967A6, native_parameters!(ms));

    value
}

pub fn start_new_script(scriptName: &std::ffi::CString, stackSize: i32) -> i32 {
    let value = native!(
        i32,
        0xE81651AD79516E48,
        native_parameters!(scriptName.as_ptr(), stackSize)
    );

    value
}

pub fn start_new_script_with_args(
    scriptName: &std::ffi::CString,
    args: *mut u32,
    argCount: i32,
    stackSize: i32,
) -> i32 {
    let value = native!(
        i32,
        0xB8BA7F44DF1575E1,
        native_parameters!(scriptName.as_ptr(), args, argCount, stackSize)
    );

    value
}

pub fn start_new_script_with_name_hash(scriptHash: u32, stackSize: i32) -> i32 {
    let value = native!(
        i32,
        0xEB1C67C3A5333A92,
        native_parameters!(scriptHash, stackSize)
    );

    value
}

pub fn start_new_script_with_name_hash_and_args(
    scriptHash: u32,
    args: *mut u32,
    argCount: i32,
    stackSize: i32,
) -> i32 {
    let value = native!(
        i32,
        0xC4BB298BD441BE78,
        native_parameters!(scriptHash, args, argCount, stackSize)
    );

    value
}

pub fn timera() -> i32 {
    let value = native!(i32, 0x83666F9FB8FEBD4B, native_parameters!());

    value
}

pub fn timerb() -> i32 {
    let value = native!(i32, 0xC9D9444186B5A374, native_parameters!());

    value
}

pub fn settimera(value: i32) -> () {
    let value = native!((), 0xC1B1E9A034A63A62, native_parameters!(value));

    value
}

pub fn settimerb(value: i32) -> () {
    let value = native!((), 0x5AE11BC36633DE4E, native_parameters!(value));

    value
}

pub fn timestep() -> f32 {
    let value = native!(f32, 0x0000000050597EE2, native_parameters!());

    value
}

pub fn sin(value: f32) -> f32 {
    let value = native!(f32, 0x0BADBFA3B172435F, native_parameters!(value));

    value
}

pub fn cos(value: f32) -> f32 {
    let value = native!(f32, 0xD0FFB162F40A139C, native_parameters!(value));

    value
}

pub fn sqrt(value: f32) -> f32 {
    let value = native!(f32, 0x71D93B57D07F9804, native_parameters!(value));

    value
}

pub fn pow(base: f32, exponent: f32) -> f32 {
    let value = native!(f32, 0xE3621CC40F31FE2E, native_parameters!(base, exponent));

    value
}

pub fn _log10(value: f32) -> f32 {
    let value = native!(f32, 0xE816E655DE37FE20, native_parameters!(value));

    value
}

pub fn vmag(x: f32, y: f32, z: f32) -> f32 {
    let value = native!(f32, 0x652D2EEEF1D3E62C, native_parameters!(x, y, z));

    value
}

pub fn vmag2(x: f32, y: f32, z: f32) -> f32 {
    let value = native!(f32, 0xA8CEACB4F35AE058, native_parameters!(x, y, z));

    value
}

pub fn vdist(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 {
    let value = native!(
        f32,
        0x2A488C176D52CCA5,
        native_parameters!(x1, y1, z1, x2, y2, z2)
    );

    value
}

pub fn vdist2(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 {
    let value = native!(
        f32,
        0xB7A628320EFF8E47,
        native_parameters!(x1, y1, z1, x2, y2, z2)
    );

    value
}

pub fn shift_left(value: i32, bitShift: i32) -> i32 {
    let value = native!(i32, 0xEDD95A39E5544DE8, native_parameters!(value, bitShift));

    value
}

pub fn shift_right(value: i32, bitShift: i32) -> i32 {
    let value = native!(i32, 0x97EF1E5BCE9DC075, native_parameters!(value, bitShift));

    value
}

pub fn floor(value: f32) -> i32 {
    let value = native!(i32, 0xF34EE736CF047844, native_parameters!(value));

    value
}

pub fn ceil(value: f32) -> i32 {
    let value = native!(i32, 0x11E019C8F43ACC8A, native_parameters!(value));

    value
}

pub fn round(value: f32) -> i32 {
    let value = native!(i32, 0xF2DB717A73826179, native_parameters!(value));

    value
}

pub fn to_float(value: i32) -> f32 {
    let value = native!(f32, 0xBBDA792448DB5A89, native_parameters!(value));

    value
}

pub fn set_thread_priority(priority: i32) -> () {
    let value = native!((), 0x42B65DEEF2EDF2A1, native_parameters!(priority));

    value
}
