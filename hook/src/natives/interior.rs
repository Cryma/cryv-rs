use crate::types::NativeVector3;

pub fn _get_interior_heading(interior: i32) -> f32 {
    let value = native!(f32, 0xF49B58631D9E22D9, native_parameters!(interior));

    value
}

pub fn _get_interior_info(interior: i32, position: *mut NativeVector3, nameHash: *mut u32) -> () {
    let value = native!(
        (),
        0x252BDC06B73FA6EA,
        native_parameters!(interior, position, nameHash)
    );

    value
}

pub fn get_interior_group_id(interior: i32) -> i32 {
    let value = native!(i32, 0xE4A84ABF135EF91A, native_parameters!(interior));

    value
}

pub fn get_offset_from_interior_in_world_coords(
    interior: i32,
    x: f32,
    y: f32,
    z: f32,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x9E3B3E6D66F6E22F,
        native_parameters!(interior, x, y, z)
    );

    value
}

pub fn is_interior_scene() -> bool {
    let value = native!(bool, 0xBC72B5D7A1CBD54D, native_parameters!());

    value
}

pub fn is_valid_interior(interior: i32) -> bool {
    let value = native!(bool, 0x26B0E73D7EAAF4D3, native_parameters!(interior));

    value
}

pub fn clear_room_for_entity(entity: i32) -> () {
    let value = native!((), 0xB365FC0C4E27FFA7, native_parameters!(entity));

    value
}

pub fn force_room_for_entity(entity: i32, interior: i32, roomHashKey: u32) -> () {
    let value = native!(
        (),
        0x52923C4710DD9907,
        native_parameters!(entity, interior, roomHashKey)
    );

    value
}

pub fn get_room_key_from_entity(entity: i32) -> u32 {
    let value = native!(u32, 0x47C2A06D4F5F424B, native_parameters!(entity));

    value
}

pub fn get_key_for_entity_in_room(entity: i32) -> u32 {
    let value = native!(u32, 0x399685DB942336BC, native_parameters!(entity));

    value
}

pub fn get_interior_from_entity(entity: i32) -> i32 {
    let value = native!(i32, 0x2107BA504071A6BB, native_parameters!(entity));

    value
}

pub fn _0x82ebb79e258fa2b7(entity: i32, interior: i32) -> () {
    let value = native!((), 0x82EBB79E258FA2B7, native_parameters!(entity, interior));

    value
}

pub fn _clear_interior_for_entity(entity: i32) -> () {
    let value = native!((), 0x85D5422B2039A70D, native_parameters!(entity));

    value
}

pub fn _0x38c1cb1cb119a016(p0: u32, p1: u32) -> () {
    let value = native!((), 0x38C1CB1CB119A016, native_parameters!(p0, p1));

    value
}

pub fn force_room_for_game_viewport(interiorID: i32, roomHashKey: u32) -> () {
    let value = native!(
        (),
        0x920D853F3E17F1DA,
        native_parameters!(interiorID, roomHashKey)
    );

    value
}

pub fn _0xaf348afcb575a441(roomName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xAF348AFCB575A441,
        native_parameters!(roomName.as_ptr())
    );

    value
}

pub fn _0x405dc2aef6af95b9(roomHashKey: u32) -> () {
    let value = native!((), 0x405DC2AEF6AF95B9, native_parameters!(roomHashKey));

    value
}

pub fn get_room_key_for_game_viewport() -> u32 {
    let value = native!(u32, 0xA6575914D2A0B450, native_parameters!());

    value
}

pub fn clear_room_for_game_viewport() -> () {
    let value = native!((), 0x23B59D8912F94246, native_parameters!());

    value
}

pub fn _get_interior_from_gameplay_cam() -> i32 {
    let value = native!(i32, 0xE7D267EC6CA966C3, native_parameters!());

    value
}

pub fn get_interior_at_coords(x: f32, y: f32, z: f32) -> i32 {
    let value = native!(i32, 0xB0F7F8663821D9C3, native_parameters!(x, y, z));

    value
}

pub fn add_pickup_to_interior_room_by_name(pickup: i32, roomName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x3F6167F351168730,
        native_parameters!(pickup, roomName.as_ptr())
    );

    value
}

pub fn pin_interior_in_memory(interior: i32) -> () {
    let value = native!((), 0x2CA429C029CCF247, native_parameters!(interior));

    value
}

pub fn unpin_interior(interior: i32) -> () {
    let value = native!((), 0x261CCE7EED010641, native_parameters!(interior));

    value
}

pub fn is_interior_ready(interior: i32) -> bool {
    let value = native!(bool, 0x6726BDCCC1932F0E, native_parameters!(interior));

    value
}

pub fn _0x4c2330e61d3deb56(interior: i32) -> u32 {
    let value = native!(u32, 0x4C2330E61D3DEB56, native_parameters!(interior));

    value
}

pub fn get_interior_at_coords_with_type(
    x: f32,
    y: f32,
    z: f32,
    interiorType: &std::ffi::CString,
) -> i32 {
    let value = native!(
        i32,
        0x05B7A89BD78797FC,
        native_parameters!(x, y, z, interiorType.as_ptr())
    );

    value
}

pub fn get_interior_at_coords_with_typehash(x: f32, y: f32, z: f32, typeHash: u32) -> i32 {
    let value = native!(
        i32,
        0xF0F77ADB9F67E79D,
        native_parameters!(x, y, z, typeHash)
    );

    value
}

pub fn _0x483aca1176ca93f1() -> () {
    let value = native!((), 0x483ACA1176CA93F1, native_parameters!());

    value
}

pub fn is_collision_marked_outside(x: f32, y: f32, z: f32) -> bool {
    let value = native!(bool, 0xEEA5AC2EDA7C33E8, native_parameters!(x, y, z));

    value
}

pub fn get_interior_from_collision(x: f32, y: f32, z: f32) -> i32 {
    let value = native!(i32, 0xEC4CF9FCB29A4424, native_parameters!(x, y, z));

    value
}

pub fn _0x7ecdf98587e92dec(p0: u32) -> () {
    let value = native!((), 0x7ECDF98587E92DEC, native_parameters!(p0));

    value
}

pub fn activate_interior_entity_set(interior: i32, entitySetName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x55E86AF2712B36A1,
        native_parameters!(interior, entitySetName.as_ptr())
    );

    value
}

pub fn deactivate_interior_entity_set(interior: i32, entitySetName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x420BD37289EEE162,
        native_parameters!(interior, entitySetName.as_ptr())
    );

    value
}

pub fn is_interior_entity_set_active(interior: i32, entitySetName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x35F7DD45E8C0A16D,
        native_parameters!(interior, entitySetName.as_ptr())
    );

    value
}

pub fn _set_interior_entity_set_color(
    interior: i32,
    entitySetName: &std::ffi::CString,
    color: i32,
) -> () {
    let value = native!(
        (),
        0xC1F1920BAF281317,
        native_parameters!(interior, entitySetName.as_ptr(), color)
    );

    value
}

pub fn refresh_interior(interior: i32) -> () {
    let value = native!((), 0x41F37C3427C75AE0, native_parameters!(interior));

    value
}

pub fn enable_exterior_cull_model_this_frame(mapObjectHash: u32) -> () {
    let value = native!((), 0xA97F257D0151A6AB, native_parameters!(mapObjectHash));

    value
}

pub fn _enable_script_cull_model_this_frame(mapObjectHash: u32) -> () {
    let value = native!((), 0x50C375537449F369, native_parameters!(mapObjectHash));

    value
}

pub fn disable_interior(interior: i32, toggle: bool) -> () {
    let value = native!((), 0x6170941419D7D8EC, native_parameters!(interior, toggle));

    value
}

pub fn is_interior_disabled(interior: i32) -> bool {
    let value = native!(bool, 0xBC5115A5A939DD15, native_parameters!(interior));

    value
}

pub fn cap_interior(interior: i32, toggle: bool) -> () {
    let value = native!((), 0xD9175F941610DB54, native_parameters!(interior, toggle));

    value
}

pub fn is_interior_capped(interior: i32) -> bool {
    let value = native!(bool, 0x92BAC8ACF88CEC26, native_parameters!(interior));

    value
}

pub fn _0x9e6542f0ce8e70a3(toggle: bool) -> () {
    let value = native!((), 0x9E6542F0CE8E70A3, native_parameters!(toggle));

    value
}

pub fn _0x7241ccb7d020db69(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x7241CCB7D020DB69, native_parameters!(entity, toggle));

    value
}
