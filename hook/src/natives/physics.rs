use crate::types::NativeVector3;

pub fn add_rope(
    x: f32,
    y: f32,
    z: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    length: f32,
    ropeType: i32,
    maxLength: f32,
    minLength: f32,
    windingSpeed: f32,
    p11: bool,
    p12: bool,
    rigid: bool,
    p14: f32,
    breakWhenShot: bool,
    unkPtr: *mut u32,
) -> i32 {
    let value = native!(
        i32,
        0xE832D760399EB220,
        native_parameters!(
            x,
            y,
            z,
            rotX,
            rotY,
            rotZ,
            length,
            ropeType,
            maxLength,
            minLength,
            windingSpeed,
            p11,
            p12,
            rigid,
            p14,
            breakWhenShot,
            unkPtr
        )
    );

    value
}

pub fn delete_rope(ropeId: *mut i32) -> () {
    let value = native!((), 0x52B4829281364649, native_parameters!(ropeId));

    value
}

pub fn delete_child_rope(ropeId: i32) -> () {
    let value = native!((), 0xAA5D6B1888E4DB20, native_parameters!(ropeId));

    value
}

pub fn does_rope_exist(ropeId: *mut i32) -> bool {
    let value = native!(bool, 0xFD5448BE3111ED96, native_parameters!(ropeId));

    value
}

pub fn _0xa1ae736541b0fca3(ropeId: *mut i32, p1: bool) -> () {
    let value = native!((), 0xA1AE736541B0FCA3, native_parameters!(ropeId, p1));

    value
}

pub fn rope_draw_shadow_enabled(ropeId: *mut i32, toggle: bool) -> () {
    let value = native!((), 0xF159A63806BB5BA8, native_parameters!(ropeId, toggle));

    value
}

pub fn load_rope_data(ropeId: i32, rope_preset: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xCBB203C04D1ABD27,
        native_parameters!(ropeId, rope_preset.as_ptr())
    );

    value
}

pub fn pin_rope_vertex(ropeId: i32, vertex: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!(
        (),
        0x2B320CF14146B69A,
        native_parameters!(ropeId, vertex, x, y, z)
    );

    value
}

pub fn unpin_rope_vertex(ropeId: i32, vertex: i32) -> () {
    let value = native!((), 0x4B5AE2EEE4A8F180, native_parameters!(ropeId, vertex));

    value
}

pub fn get_rope_vertex_count(ropeId: i32) -> i32 {
    let value = native!(i32, 0x3655F544CD30F0B5, native_parameters!(ropeId));

    value
}

pub fn attach_entities_to_rope(
    ropeId: i32,
    ent1: i32,
    ent2: i32,
    ent1_x: f32,
    ent1_y: f32,
    ent1_z: f32,
    ent2_x: f32,
    ent2_y: f32,
    ent2_z: f32,
    length: f32,
    p10: bool,
    p11: bool,
    p12: *mut u32,
    p13: *mut u32,
) -> () {
    let value = native!(
        (),
        0x3D95EC8B6D940AC3,
        native_parameters!(
            ropeId, ent1, ent2, ent1_x, ent1_y, ent1_z, ent2_x, ent2_y, ent2_z, length, p10, p11,
            p12, p13
        )
    );

    value
}

pub fn attach_rope_to_entity(ropeId: i32, entity: i32, x: f32, y: f32, z: f32, p5: bool) -> () {
    let value = native!(
        (),
        0x4B490A6832559A65,
        native_parameters!(ropeId, entity, x, y, z, p5)
    );

    value
}

pub fn detach_rope_from_entity(ropeId: i32, entity: i32) -> () {
    let value = native!((), 0xBCF3026912A8647D, native_parameters!(ropeId, entity));

    value
}

pub fn rope_set_update_pinverts(ropeId: i32) -> () {
    let value = native!((), 0xC8D667EE52114ABA, native_parameters!(ropeId));

    value
}

pub fn rope_set_update_order(ropeId: i32, p1: u32) -> () {
    let value = native!((), 0xDC57A637A20006ED, native_parameters!(ropeId, p1));

    value
}

pub fn _0x36ccb9be67b970fd(ropeId: i32, p1: bool) -> () {
    let value = native!((), 0x36CCB9BE67B970FD, native_parameters!(ropeId, p1));

    value
}

pub fn _0x84de3b5fb3e666f0(ropeId: *mut i32) -> bool {
    let value = native!(bool, 0x84DE3B5FB3E666F0, native_parameters!(ropeId));

    value
}

pub fn get_rope_last_vertex_coord(ropeId: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x21BB0FBD3E217C2D,
        native_parameters!(ropeId)
    );

    value
}

pub fn get_rope_vertex_coord(ropeId: i32, vertex: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xEA61CA8E80F09E4D,
        native_parameters!(ropeId, vertex)
    );

    value
}

pub fn start_rope_winding(ropeId: i32) -> () {
    let value = native!((), 0x1461C72C889E343E, native_parameters!(ropeId));

    value
}

pub fn stop_rope_winding(ropeId: i32) -> () {
    let value = native!((), 0xCB2D4AB84A19AA7C, native_parameters!(ropeId));

    value
}

pub fn start_rope_unwinding_front(ropeId: i32) -> () {
    let value = native!((), 0x538D1179EC1AA9A9, native_parameters!(ropeId));

    value
}

pub fn stop_rope_unwinding_front(ropeId: i32) -> () {
    let value = native!((), 0xFFF3A50779EFBBB3, native_parameters!(ropeId));

    value
}

pub fn rope_convert_to_simple(ropeId: i32) -> () {
    let value = native!((), 0x5389D48EFA2F079A, native_parameters!(ropeId));

    value
}

pub fn rope_load_textures() -> () {
    let value = native!((), 0x9B9039DBF2D258C1, native_parameters!());

    value
}

pub fn rope_are_textures_loaded() -> bool {
    let value = native!(bool, 0xF2D0E6A75CC05597, native_parameters!());

    value
}

pub fn rope_unload_textures() -> () {
    let value = native!((), 0x6CE36C35C1AC8163, native_parameters!());

    value
}

pub fn _does_rope_belong_to_this_script(ropeId: i32) -> bool {
    let value = native!(bool, 0x271C9D3ACA5D6409, native_parameters!(ropeId));

    value
}

pub fn _0xbc0ce682d4d05650(
    ropeId: i32,
    p1: i32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
    p8: f32,
    p9: f32,
    p10: f32,
    p11: f32,
    p12: f32,
    p13: f32,
) -> () {
    let value = native!(
        (),
        0xBC0CE682D4D05650,
        native_parameters!(ropeId, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13)
    );

    value
}

pub fn _0xb1b6216ca2e7b55e(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xB1B6216CA2E7B55E, native_parameters!(p0, p1, p2));

    value
}

pub fn _0xb743f735c03d7810(ropeId: i32, p1: i32) -> () {
    let value = native!((), 0xB743F735C03D7810, native_parameters!(ropeId, p1));

    value
}

pub fn rope_get_distance_between_ends(ropeId: i32) -> f32 {
    let value = native!(f32, 0x73040398DFF9A4A6, native_parameters!(ropeId));

    value
}

pub fn rope_force_length(ropeId: i32, length: f32) -> () {
    let value = native!((), 0xD009F759A723DB1B, native_parameters!(ropeId, length));

    value
}

pub fn rope_reset_length(ropeId: i32, length: f32) -> () {
    let value = native!((), 0xC16DE94D9BEA14A0, native_parameters!(ropeId, length));

    value
}

pub fn apply_impulse_to_cloth(
    posX: f32,
    posY: f32,
    posZ: f32,
    vecX: f32,
    vecY: f32,
    vecZ: f32,
    impulse: f32,
) -> () {
    let value = native!(
        (),
        0xE37F721824571784,
        native_parameters!(posX, posY, posZ, vecX, vecY, vecZ, impulse)
    );

    value
}

pub fn set_damping(entity: i32, vertex: i32, value: f32) -> () {
    let value = native!(
        (),
        0xEEA3B200A6FEB65B,
        native_parameters!(entity, vertex, value)
    );

    value
}

pub fn activate_physics(entity: i32) -> () {
    let value = native!((), 0x710311ADF0E20730, native_parameters!(entity));

    value
}

pub fn set_cgoffset(entity: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xD8FA3908D7B86904, native_parameters!(entity, x, y, z));

    value
}

pub fn get_cgoffset(entity: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x8214A4B5A7A33612,
        native_parameters!(entity)
    );

    value
}

pub fn set_cg_at_boundcenter(entity: i32) -> () {
    let value = native!((), 0xBE520D9761FF811F, native_parameters!(entity));

    value
}

pub fn break_entity_glass(
    entity: i32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
    p8: f32,
    p9: u32,
    p10: bool,
) -> () {
    let value = native!(
        (),
        0x2E648D16F6E308F3,
        native_parameters!(entity, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10)
    );

    value
}

pub fn _get_has_object_frag_inst(object: i32) -> bool {
    let value = native!(bool, 0x0C112765300C7E1E, native_parameters!(object));

    value
}

pub fn set_disable_breaking(object: i32, toggle: bool) -> () {
    let value = native!((), 0x5CEC1A84620E7D5B, native_parameters!(object, toggle));

    value
}

pub fn _0xcc6e963682533882(object: i32) -> () {
    let value = native!((), 0xCC6E963682533882, native_parameters!(object));

    value
}

pub fn set_disable_frag_damage(object: i32, toggle: bool) -> () {
    let value = native!((), 0x01BA3AED21C16CFB, native_parameters!(object, toggle));

    value
}

pub fn _set_entity_proof_unk(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x15F944730C832252, native_parameters!(entity, toggle));

    value
}

pub fn _0x9ebd751e5787baf2(p0: bool) -> () {
    let value = native!((), 0x9EBD751E5787BAF2, native_parameters!(p0));

    value
}

pub fn _set_launch_control_enabled(toggle: bool) -> () {
    let value = native!((), 0xAA6A6098851C396F, native_parameters!(toggle));

    value
}
