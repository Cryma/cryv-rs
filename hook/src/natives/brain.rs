use crate::types::NativeVector3;

pub fn add_script_to_random_ped(name: &std::ffi::CString, model: u32, p2: f32, p3: f32) -> () {
    let value = native!(
        (),
        0x4EE5367468A65CCC,
        native_parameters!(name.as_ptr(), model, p2, p3)
    );

    value
}

pub fn register_object_script_brain(
    scriptName: &std::ffi::CString,
    modelHash: u32,
    p2: i32,
    activationRange: f32,
    p4: i32,
    p5: i32,
) -> () {
    let value = native!(
        (),
        0x0BE84C318BA6EC22,
        native_parameters!(scriptName.as_ptr(), modelHash, p2, activationRange, p4, p5)
    );

    value
}

pub fn is_object_within_brain_activation_range(object: i32) -> bool {
    let value = native!(bool, 0xCCBA154209823057, native_parameters!(object));

    value
}

pub fn register_world_point_script_brain(
    scriptName: &std::ffi::CString,
    activationRange: f32,
    p2: i32,
) -> () {
    let value = native!(
        (),
        0x3CDC7136613284BD,
        native_parameters!(scriptName.as_ptr(), activationRange, p2)
    );

    value
}

pub fn is_world_point_within_brain_activation_range() -> bool {
    let value = native!(bool, 0xC5042CC6F5E3D450, native_parameters!());

    value
}

pub fn enable_script_brain_set(brainSet: i32) -> () {
    let value = native!((), 0x67AA4D73F0CFA86B, native_parameters!(brainSet));

    value
}

pub fn disable_script_brain_set(brainSet: i32) -> () {
    let value = native!((), 0x14D8518E9760F08F, native_parameters!(brainSet));

    value
}

pub fn _0x0b40ed49d7d6ff84() -> () {
    let value = native!((), 0x0B40ED49D7D6FF84, native_parameters!());

    value
}

pub fn _0x4d953df78ebf8158() -> () {
    let value = native!((), 0x4D953DF78EBF8158, native_parameters!());

    value
}

pub fn _0x6d6840cee8845831(action: &std::ffi::CString) -> () {
    let value = native!((), 0x6D6840CEE8845831, native_parameters!(action.as_ptr()));

    value
}

pub fn _0x6e91b04e08773030(action: &std::ffi::CString) -> () {
    let value = native!((), 0x6E91B04E08773030, native_parameters!(action.as_ptr()));

    value
}
