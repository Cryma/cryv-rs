use crate::types::NativeVector3;

pub fn create_mobile_phone(phoneType: i32) -> () {
    let value = native!((), 0xA4E8E696C532FBC7, native_parameters!(phoneType));

    value
}

pub fn destroy_mobile_phone() -> () {
    let value = native!((), 0x3BC861DF703E5097, native_parameters!());

    value
}

pub fn set_mobile_phone_scale(scale: f32) -> () {
    let value = native!((), 0xCBDD322A73D6D932, native_parameters!(scale));

    value
}

pub fn set_mobile_phone_rotation(rotX: f32, rotY: f32, rotZ: f32, p3: u32) -> () {
    let value = native!(
        (),
        0xBB779C0CA917E865,
        native_parameters!(rotX, rotY, rotZ, p3)
    );

    value
}

pub fn get_mobile_phone_rotation(rotation: *mut NativeVector3, p1: i32) -> () {
    let value = native!((), 0x1CEFB61F193070AE, native_parameters!(rotation, p1));

    value
}

pub fn set_mobile_phone_position(posX: f32, posY: f32, posZ: f32) -> () {
    let value = native!((), 0x693A5C6D6734085B, native_parameters!(posX, posY, posZ));

    value
}

pub fn get_mobile_phone_position(position: *mut NativeVector3) -> () {
    let value = native!((), 0x584FDFDA48805B86, native_parameters!(position));

    value
}

pub fn script_is_moving_mobile_phone_offscreen(toggle: bool) -> () {
    let value = native!((), 0xF511F759238A5122, native_parameters!(toggle));

    value
}

pub fn can_phone_be_seen_on_screen() -> bool {
    let value = native!(bool, 0xC4E2813898C97A4B, native_parameters!());

    value
}

pub fn _set_mobile_phone_unk(toggle: bool) -> () {
    let value = native!((), 0x375A706A5C2FD084, native_parameters!(toggle));

    value
}

pub fn _cell_cam_move_finger(direction: i32) -> () {
    let value = native!((), 0x95C9E72F3D7DEC9B, native_parameters!(direction));

    value
}

pub fn _cell_cam_set_lean(toggle: bool) -> () {
    let value = native!((), 0x44E44169EF70138E, native_parameters!(toggle));

    value
}

pub fn cell_cam_activate(p0: bool, p1: bool) -> () {
    let value = native!((), 0xFDE8F069C542D126, native_parameters!(p0, p1));

    value
}

pub fn _cell_cam_disable_this_frame(toggle: bool) -> () {
    let value = native!((), 0x015C49A93E3E086E, native_parameters!(toggle));

    value
}

pub fn _0xa2ccbe62cd4c91a4(toggle: *mut i32) -> () {
    let value = native!((), 0xA2CCBE62CD4C91A4, native_parameters!(toggle));

    value
}

pub fn _0x1b0b4aeed5b9b41c(p0: f32) -> () {
    let value = native!((), 0x1B0B4AEED5B9B41C, native_parameters!(p0));

    value
}

pub fn _0x53f4892d18ec90a4(p0: f32) -> () {
    let value = native!((), 0x53F4892D18EC90A4, native_parameters!(p0));

    value
}

pub fn _0x3117d84efa60f77b(p0: f32) -> () {
    let value = native!((), 0x3117D84EFA60F77B, native_parameters!(p0));

    value
}

pub fn _0x15e69e2802c24b8d(p0: f32) -> () {
    let value = native!((), 0x15E69E2802C24B8D, native_parameters!(p0));

    value
}

pub fn _0xac2890471901861c(p0: f32) -> () {
    let value = native!((), 0xAC2890471901861C, native_parameters!(p0));

    value
}

pub fn _0xd6ade981781fca09(p0: f32) -> () {
    let value = native!((), 0xD6ADE981781FCA09, native_parameters!(p0));

    value
}

pub fn _0xf1e22dc13f5eebad(p0: f32) -> () {
    let value = native!((), 0xF1E22DC13F5EEBAD, native_parameters!(p0));

    value
}

pub fn _0x466da42c89865553(p0: f32) -> () {
    let value = native!((), 0x466DA42C89865553, native_parameters!(p0));

    value
}

pub fn cell_cam_is_char_visible_no_face_check(entity: i32) -> bool {
    let value = native!(bool, 0x439E9BC95B7E7FBE, native_parameters!(entity));

    value
}

pub fn get_mobile_phone_render_id(renderId: *mut i32) -> () {
    let value = native!((), 0xB4A53E05F68B6FA1, native_parameters!(renderId));

    value
}
