use crate::types::NativeVector3;

pub fn is_control_enabled(padIndex: i32, control: i32) -> bool {
    let value = native!(
        bool,
        0x1CEA6BFDF248E5D9,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn is_control_pressed(padIndex: i32, control: i32) -> bool {
    let value = native!(
        bool,
        0xF3A21BCD95725A4A,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn is_control_released(padIndex: i32, control: i32) -> bool {
    let value = native!(
        bool,
        0x648EE3E7F38877DD,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn is_control_just_pressed(padIndex: i32, control: i32) -> bool {
    let value = native!(
        bool,
        0x580417101DDB492F,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn is_control_just_released(padIndex: i32, control: i32) -> bool {
    let value = native!(
        bool,
        0x50F940259D3841E6,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn get_control_value(padIndex: i32, control: i32) -> i32 {
    let value = native!(
        i32,
        0xD95E79E8686D2C27,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn get_control_normal(padIndex: i32, control: i32) -> f32 {
    let value = native!(
        f32,
        0xEC3C9B8D5327B563,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn _0x5b73c77d9eb66e24(p0: bool) -> () {
    let value = native!((), 0x5B73C77D9EB66E24, native_parameters!(p0));

    value
}

pub fn get_control_unbound_normal(padIndex: i32, control: i32) -> f32 {
    let value = native!(
        f32,
        0x5B84D09CEC5209C5,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn _set_control_normal(padIndex: i32, control: i32, amount: f32) -> bool {
    let value = native!(
        bool,
        0xE8A25867FBA3B05E,
        native_parameters!(padIndex, control, amount)
    );

    value
}

pub fn is_disabled_control_pressed(padIndex: i32, control: i32) -> bool {
    let value = native!(
        bool,
        0xE2587F8CBBD87B1D,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn is_disabled_control_released(padIndex: i32, control: i32) -> bool {
    let value = native!(
        bool,
        0xFB6C4072E9A32E92,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn is_disabled_control_just_pressed(padIndex: i32, control: i32) -> bool {
    let value = native!(
        bool,
        0x91AEF906BCA88877,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn is_disabled_control_just_released(padIndex: i32, control: i32) -> bool {
    let value = native!(
        bool,
        0x305C8DCD79DA8B0F,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn get_disabled_control_normal(padIndex: i32, control: i32) -> f32 {
    let value = native!(
        f32,
        0x11E65974A982637C,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn get_disabled_control_unbound_normal(padIndex: i32, control: i32) -> f32 {
    let value = native!(
        f32,
        0x4F8A26A890FD62FB,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn _0xd7d22f5592aed8ba(p0: i32) -> i32 {
    let value = native!(i32, 0xD7D22F5592AED8BA, native_parameters!(p0));

    value
}

pub fn _is_using_keyboard(padIndex: i32) -> bool {
    let value = native!(bool, 0xA571D46727E2B718, native_parameters!(padIndex));

    value
}

pub fn _is_using_keyboard_2(padIndex: i32) -> bool {
    let value = native!(bool, 0x13337B38DB572509, native_parameters!(padIndex));

    value
}

pub fn _set_cursor_location(x: f32, y: f32) -> bool {
    let value = native!(bool, 0xFC695459D4D0E219, native_parameters!(x, y));

    value
}

pub fn _0x23f09eadc01449d6(padIndex: i32) -> bool {
    let value = native!(bool, 0x23F09EADC01449D6, native_parameters!(padIndex));

    value
}

pub fn _0x6cd79468a1e595c6(padIndex: i32) -> bool {
    let value = native!(bool, 0x6CD79468A1E595C6, native_parameters!(padIndex));

    value
}

pub fn get_control_instructional_button(padIndex: i32, control: i32, p2: bool) -> String {
    let value = native!(
        *const i8,
        0x0499D7B09FC9B407,
        native_parameters!(padIndex, control, p2)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_control_group_instructional_button(
    padIndex: i32,
    controlGroup: i32,
    p2: bool,
) -> String {
    let value = native!(
        *const i8,
        0x80C2FD58D720C801,
        native_parameters!(padIndex, controlGroup, p2)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn set_control_light_effect_color(padIndex: i32, red: i32, green: i32, blue: i32) -> () {
    let value = native!(
        (),
        0x8290252FFF36ACB5,
        native_parameters!(padIndex, red, green, blue)
    );

    value
}

pub fn _0xcb0360efefb2580d(padIndex: i32) -> () {
    let value = native!((), 0xCB0360EFEFB2580D, native_parameters!(padIndex));

    value
}

pub fn set_pad_shake(padIndex: i32, duration: i32, frequency: i32) -> () {
    let value = native!(
        (),
        0x48B3886C1358D0D5,
        native_parameters!(padIndex, duration, frequency)
    );

    value
}

pub fn _0x14d29bb12d47f68c(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x14D29BB12D47F68C,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn stop_pad_shake(padIndex: i32) -> () {
    let value = native!((), 0x38C16A305E8CDC8D, native_parameters!(padIndex));

    value
}

pub fn _0xf239400e16c23e08(p0: u32, p1: u32) -> () {
    let value = native!((), 0xF239400E16C23E08, native_parameters!(p0, p1));

    value
}

pub fn _0xa0cefcea390aab9b(p0: u32) -> () {
    let value = native!((), 0xA0CEFCEA390AAB9B, native_parameters!(p0));

    value
}

pub fn is_look_inverted() -> bool {
    let value = native!(bool, 0x77B612531280010D, native_parameters!());

    value
}

pub fn _0xe1615ec03b3bb4fd() -> bool {
    let value = native!(bool, 0xE1615EC03B3BB4FD, native_parameters!());

    value
}

pub fn get_local_player_aim_state() -> i32 {
    let value = native!(i32, 0xBB41AFBBBC0A0287, native_parameters!());

    value
}

pub fn _get_local_player_aim_state_2() -> i32 {
    let value = native!(i32, 0x59B9A7AF4C95133C, native_parameters!());

    value
}

pub fn _0x25aaa32bdc98f2a3() -> u32 {
    let value = native!(u32, 0x25AAA32BDC98F2A3, native_parameters!());

    value
}

pub fn get_is_using_alternate_driveby() -> bool {
    let value = native!(bool, 0x0F70731BACCFBB96, native_parameters!());

    value
}

pub fn get_allow_movement_while_zoomed() -> bool {
    let value = native!(bool, 0xFC859E2374407556, native_parameters!());

    value
}

pub fn set_playerpad_shakes_when_controller_disabled(toggle: bool) -> () {
    let value = native!((), 0x798FDEB5B1575088, native_parameters!(toggle));

    value
}

pub fn set_input_exclusive(padIndex: i32, control: i32) -> () {
    let value = native!(
        (),
        0xEDE476E5EE29EDB1,
        native_parameters!(padIndex, control)
    );

    value
}

pub fn disable_control_action(padIndex: i32, control: i32, disable: bool) -> () {
    let value = native!(
        (),
        0xFE99B66D079CF6BC,
        native_parameters!(padIndex, control, disable)
    );

    value
}

pub fn enable_control_action(padIndex: i32, control: i32, enable: bool) -> () {
    let value = native!(
        (),
        0x351220255D64C155,
        native_parameters!(padIndex, control, enable)
    );

    value
}

pub fn disable_all_control_actions(padIndex: i32) -> () {
    let value = native!((), 0x5F4B6931816E599B, native_parameters!(padIndex));

    value
}

pub fn enable_all_control_actions(padIndex: i32) -> () {
    let value = native!((), 0xA5FFE9B05F199DE7, native_parameters!(padIndex));

    value
}

pub fn _switch_to_input_mapping_scheme(name: &std::ffi::CString) -> bool {
    let value = native!(bool, 0x3D42B92563939375, native_parameters!(name.as_ptr()));

    value
}

pub fn _switch_to_input_mapping_scheme_2(name: &std::ffi::CString) -> bool {
    let value = native!(bool, 0x4683149ED1DDE7A1, native_parameters!(name.as_ptr()));

    value
}

pub fn _reset_input_mapping_scheme() -> () {
    let value = native!((), 0x643ED62D5EA3BEBD, native_parameters!());

    value
}

pub fn _0x7f4724035fdca1dd(padIndex: i32) -> () {
    let value = native!((), 0x7F4724035FDCA1DD, native_parameters!(padIndex));

    value
}
