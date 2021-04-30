use crate::types::NativeVector3;

pub fn does_entity_exist(entity: i32) -> bool {
    let value = native!(bool, 0x7239B21A38F536BA, native_parameters!(entity));

    value
}

pub fn does_entity_belong_to_this_script(entity: i32, p1: bool) -> bool {
    let value = native!(bool, 0xDDE6DF5AE89981D2, native_parameters!(entity, p1));

    value
}

pub fn does_entity_have_drawable(entity: i32) -> bool {
    let value = native!(bool, 0x060D6E96F8B8E48D, native_parameters!(entity));

    value
}

pub fn does_entity_have_physics(entity: i32) -> bool {
    let value = native!(bool, 0xDA95EA3317CC5064, native_parameters!(entity));

    value
}

pub fn has_entity_anim_finished(
    entity: i32,
    animDict: &std::ffi::CString,
    animName: &std::ffi::CString,
    p3: i32,
) -> bool {
    let value = native!(
        bool,
        0x20B711662962B472,
        native_parameters!(entity, animDict.as_ptr(), animName.as_ptr(), p3)
    );

    value
}

pub fn has_entity_been_damaged_by_any_object(entity: i32) -> bool {
    let value = native!(bool, 0x95EB9964FF5C5C65, native_parameters!(entity));

    value
}

pub fn has_entity_been_damaged_by_any_ped(entity: i32) -> bool {
    let value = native!(bool, 0x605F5A140F202491, native_parameters!(entity));

    value
}

pub fn has_entity_been_damaged_by_any_vehicle(entity: i32) -> bool {
    let value = native!(bool, 0xDFD5033FDBA0A9C8, native_parameters!(entity));

    value
}

pub fn has_entity_been_damaged_by_entity(entity1: i32, entity2: i32, p2: bool) -> bool {
    let value = native!(
        bool,
        0xC86D67D52A707CF8,
        native_parameters!(entity1, entity2, p2)
    );

    value
}

pub fn has_entity_clear_los_to_entity(entity1: i32, entity2: i32, traceType: i32) -> bool {
    let value = native!(
        bool,
        0xFCDFF7B72D23A1AC,
        native_parameters!(entity1, entity2, traceType)
    );

    value
}

pub fn _has_entity_clear_los_to_entity_2(entity1: i32, entity2: i32, traceType: i32) -> u32 {
    let value = native!(
        u32,
        0x394BDE2A7BBA031E,
        native_parameters!(entity1, entity2, traceType)
    );

    value
}

pub fn has_entity_clear_los_to_entity_in_front(entity1: i32, entity2: i32) -> bool {
    let value = native!(
        bool,
        0x0267D00AF114F17A,
        native_parameters!(entity1, entity2)
    );

    value
}

pub fn has_entity_collided_with_anything(entity: i32) -> bool {
    let value = native!(bool, 0x8BAD02F0368D9E14, native_parameters!(entity));

    value
}

pub fn get_last_material_hit_by_entity(entity: i32) -> u32 {
    let value = native!(u32, 0x5C3D0A935F535C4C, native_parameters!(entity));

    value
}

pub fn get_collision_normal_of_last_hit_for_entity(entity: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xE465D4AB7CA6AE72,
        native_parameters!(entity)
    );

    value
}

pub fn force_entity_ai_and_animation_update(entity: i32) -> () {
    let value = native!((), 0x40FDEDB72F8293B2, native_parameters!(entity));

    value
}

pub fn get_entity_anim_current_time(
    entity: i32,
    animDict: &std::ffi::CString,
    animName: &std::ffi::CString,
) -> f32 {
    let value = native!(
        f32,
        0x346D81500D088F42,
        native_parameters!(entity, animDict.as_ptr(), animName.as_ptr())
    );

    value
}

pub fn get_entity_anim_total_time(
    entity: i32,
    animDict: &std::ffi::CString,
    animName: &std::ffi::CString,
) -> f32 {
    let value = native!(
        f32,
        0x50BD2730B191E360,
        native_parameters!(entity, animDict.as_ptr(), animName.as_ptr())
    );

    value
}

pub fn get_anim_duration(animDict: &std::ffi::CString, animName: &std::ffi::CString) -> f32 {
    let value = native!(
        f32,
        0xFEDDF04D62B8D790,
        native_parameters!(animDict.as_ptr(), animName.as_ptr())
    );

    value
}

pub fn get_entity_attached_to(entity: i32) -> i32 {
    let value = native!(i32, 0x48C2BED9180FE123, native_parameters!(entity));

    value
}

pub fn get_entity_coords(entity: i32, alive: bool) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x3FEF770D40960D5A,
        native_parameters!(entity, alive)
    );

    value
}

pub fn get_entity_forward_vector(entity: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x0A794A5A57F8DF91,
        native_parameters!(entity)
    );

    value
}

pub fn get_entity_forward_x(entity: i32) -> f32 {
    let value = native!(f32, 0x8BB4EF4214E0E6D5, native_parameters!(entity));

    value
}

pub fn get_entity_forward_y(entity: i32) -> f32 {
    let value = native!(f32, 0x866A4A5FAE349510, native_parameters!(entity));

    value
}

pub fn get_entity_heading(entity: i32) -> f32 {
    let value = native!(f32, 0xE83D4F9BA2A38914, native_parameters!(entity));

    value
}

pub fn _get_entity_physics_heading(entity: i32) -> f32 {
    let value = native!(f32, 0x846BF6291198A71E, native_parameters!(entity));

    value
}

pub fn get_entity_health(entity: i32) -> i32 {
    let value = native!(i32, 0xEEF059FAD016D209, native_parameters!(entity));

    value
}

pub fn get_entity_max_health(entity: i32) -> i32 {
    let value = native!(i32, 0x15D757606D170C3C, native_parameters!(entity));

    value
}

pub fn set_entity_max_health(entity: i32, value: i32) -> () {
    let value = native!((), 0x166E7CF68597D8B5, native_parameters!(entity, value));

    value
}

pub fn get_entity_height(
    entity: i32,
    X: f32,
    Y: f32,
    Z: f32,
    atTop: bool,
    inWorldCoords: bool,
) -> f32 {
    let value = native!(
        f32,
        0x5A504562485944DD,
        native_parameters!(entity, X, Y, Z, atTop, inWorldCoords)
    );

    value
}

pub fn get_entity_height_above_ground(entity: i32) -> f32 {
    let value = native!(f32, 0x1DD55701034110E5, native_parameters!(entity));

    value
}

pub fn get_entity_matrix(
    entity: i32,
    forwardVector: *mut NativeVector3,
    rightVector: *mut NativeVector3,
    upVector: *mut NativeVector3,
    position: *mut NativeVector3,
) -> () {
    let value = native!(
        (),
        0xECB2FC7235A7D137,
        native_parameters!(entity, forwardVector, rightVector, upVector, position)
    );

    value
}

pub fn get_entity_model(entity: i32) -> u32 {
    let value = native!(u32, 0x9F47B058362C84B5, native_parameters!(entity));

    value
}

pub fn get_offset_from_entity_given_world_coords(
    entity: i32,
    posX: f32,
    posY: f32,
    posZ: f32,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x2274BC1C4885E333,
        native_parameters!(entity, posX, posY, posZ)
    );

    value
}

pub fn get_offset_from_entity_in_world_coords(
    entity: i32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x1899F328B0E12848,
        native_parameters!(entity, offsetX, offsetY, offsetZ)
    );

    value
}

pub fn get_entity_pitch(entity: i32) -> f32 {
    let value = native!(f32, 0xD45DC2893621E1FE, native_parameters!(entity));

    value
}

pub fn get_entity_quaternion(
    entity: i32,
    x: *mut f32,
    y: *mut f32,
    z: *mut f32,
    w: *mut f32,
) -> () {
    let value = native!(
        (),
        0x7B3703D2D32DFA18,
        native_parameters!(entity, x, y, z, w)
    );

    value
}

pub fn get_entity_roll(entity: i32) -> f32 {
    let value = native!(f32, 0x831E0242595560DF, native_parameters!(entity));

    value
}

pub fn get_entity_rotation(entity: i32, rotationOrder: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xAFBD61CC738D9EB9,
        native_parameters!(entity, rotationOrder)
    );

    value
}

pub fn get_entity_rotation_velocity(entity: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x213B91045D09B983,
        native_parameters!(entity)
    );

    value
}

pub fn get_entity_script(entity: i32, script: *mut u32) -> String {
    let value = native!(
        *const i8,
        0xA6E9C38DB51D7748,
        native_parameters!(entity, script)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_entity_speed(entity: i32) -> f32 {
    let value = native!(f32, 0xD5037BA82E12416F, native_parameters!(entity));

    value
}

pub fn get_entity_speed_vector(entity: i32, relative: bool) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x9A8D700A51CB7B0D,
        native_parameters!(entity, relative)
    );

    value
}

pub fn get_entity_upright_value(entity: i32) -> f32 {
    let value = native!(f32, 0x95EED5A694951F9F, native_parameters!(entity));

    value
}

pub fn get_entity_velocity(entity: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x4805D2B1D8CF94A9,
        native_parameters!(entity)
    );

    value
}

pub fn get_object_index_from_entity_index(entity: i32) -> i32 {
    let value = native!(i32, 0xD7E3B9735C0F89D6, native_parameters!(entity));

    value
}

pub fn get_ped_index_from_entity_index(entity: i32) -> i32 {
    let value = native!(i32, 0x04A2A40C73395041, native_parameters!(entity));

    value
}

pub fn get_vehicle_index_from_entity_index(entity: i32) -> i32 {
    let value = native!(i32, 0x4B53F92932ADFAC0, native_parameters!(entity));

    value
}

pub fn get_world_position_of_entity_bone(entity: i32, boneIndex: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x44A8FCB8ED227738,
        native_parameters!(entity, boneIndex)
    );

    value
}

pub fn get_nearest_player_to_entity(entity: i32) -> i32 {
    let value = native!(i32, 0x7196842CB375CDB3, native_parameters!(entity));

    value
}

pub fn get_nearest_player_to_entity_on_team(entity: i32, team: i32) -> i32 {
    let value = native!(i32, 0x4DC9A62F844D9337, native_parameters!(entity, team));

    value
}

pub fn get_entity_type(entity: i32) -> i32 {
    let value = native!(i32, 0x8ACD366038D14505, native_parameters!(entity));

    value
}

pub fn get_entity_population_type(entity: i32) -> i32 {
    let value = native!(i32, 0xF6F5161F4534EDFF, native_parameters!(entity));

    value
}

pub fn is_an_entity(handle: i32) -> bool {
    let value = native!(bool, 0x731EC8A916BD11A1, native_parameters!(handle));

    value
}

pub fn is_entity_a_ped(entity: i32) -> bool {
    let value = native!(bool, 0x524AC5ECEA15343E, native_parameters!(entity));

    value
}

pub fn is_entity_a_mission_entity(entity: i32) -> bool {
    let value = native!(bool, 0x0A7B270912999B3C, native_parameters!(entity));

    value
}

pub fn is_entity_a_vehicle(entity: i32) -> bool {
    let value = native!(bool, 0x6AC7003FA6E5575E, native_parameters!(entity));

    value
}

pub fn is_entity_an_object(entity: i32) -> bool {
    let value = native!(bool, 0x8D68C8FD0FACA94E, native_parameters!(entity));

    value
}

pub fn is_entity_at_coord(
    entity: i32,
    xPos: f32,
    yPos: f32,
    zPos: f32,
    xSize: f32,
    ySize: f32,
    zSize: f32,
    p7: bool,
    p8: bool,
    p9: i32,
) -> bool {
    let value = native!(
        bool,
        0x20B60995556D004F,
        native_parameters!(entity, xPos, yPos, zPos, xSize, ySize, zSize, p7, p8, p9)
    );

    value
}

pub fn is_entity_at_entity(
    entity1: i32,
    entity2: i32,
    xSize: f32,
    ySize: f32,
    zSize: f32,
    p5: bool,
    p6: bool,
    p7: i32,
) -> bool {
    let value = native!(
        bool,
        0x751B70C3D034E187,
        native_parameters!(entity1, entity2, xSize, ySize, zSize, p5, p6, p7)
    );

    value
}

pub fn is_entity_attached(entity: i32) -> bool {
    let value = native!(bool, 0xB346476EF1A64897, native_parameters!(entity));

    value
}

pub fn is_entity_attached_to_any_object(entity: i32) -> bool {
    let value = native!(bool, 0xCF511840CEEDE0CC, native_parameters!(entity));

    value
}

pub fn is_entity_attached_to_any_ped(entity: i32) -> bool {
    let value = native!(bool, 0xB1632E9A5F988D11, native_parameters!(entity));

    value
}

pub fn is_entity_attached_to_any_vehicle(entity: i32) -> bool {
    let value = native!(bool, 0x26AA915AD89BFB4B, native_parameters!(entity));

    value
}

pub fn is_entity_attached_to_entity(from: i32, to: i32) -> bool {
    let value = native!(bool, 0xEFBE71898A993728, native_parameters!(from, to));

    value
}

pub fn is_entity_dead(entity: i32, p1: bool) -> bool {
    let value = native!(bool, 0x5F9532F3B5CC2551, native_parameters!(entity, p1));

    value
}

pub fn is_entity_in_air(entity: i32) -> bool {
    let value = native!(bool, 0x886E37EC497200B6, native_parameters!(entity));

    value
}

pub fn is_entity_in_angled_area(
    entity: i32,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    width: f32,
    debug: bool,
    includeZ: bool,
    p10: u32,
) -> bool {
    let value = native!(
        bool,
        0x51210CED3DA1C78A,
        native_parameters!(entity, x1, y1, z1, x2, y2, z2, width, debug, includeZ, p10)
    );

    value
}

pub fn is_entity_in_area(
    entity: i32,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    p7: bool,
    p8: bool,
    p9: u32,
) -> bool {
    let value = native!(
        bool,
        0x54736AA40E271165,
        native_parameters!(entity, x1, y1, z1, x2, y2, z2, p7, p8, p9)
    );

    value
}

pub fn is_entity_in_zone(entity: i32, zone: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xB6463CF6AF527071,
        native_parameters!(entity, zone.as_ptr())
    );

    value
}

pub fn is_entity_in_water(entity: i32) -> bool {
    let value = native!(bool, 0xCFB0A0D8EDD145A3, native_parameters!(entity));

    value
}

pub fn get_entity_submerged_level(entity: i32) -> f32 {
    let value = native!(f32, 0xE81AFC1BC4CC41CE, native_parameters!(entity));

    value
}

pub fn _0x694e00132f2823ed(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x694E00132F2823ED, native_parameters!(entity, toggle));

    value
}

pub fn is_entity_on_screen(entity: i32) -> bool {
    let value = native!(bool, 0xE659E47AF827484B, native_parameters!(entity));

    value
}

pub fn is_entity_playing_anim(
    entity: i32,
    animDict: &std::ffi::CString,
    animName: &std::ffi::CString,
    taskFlag: i32,
) -> bool {
    let value = native!(
        bool,
        0x1F0B79228E461EC9,
        native_parameters!(entity, animDict.as_ptr(), animName.as_ptr(), taskFlag)
    );

    value
}

pub fn is_entity_static(entity: i32) -> bool {
    let value = native!(bool, 0x1218E6886D3D8327, native_parameters!(entity));

    value
}

pub fn is_entity_touching_entity(entity: i32, targetEntity: i32) -> bool {
    let value = native!(
        bool,
        0x17FFC1B2BA35A494,
        native_parameters!(entity, targetEntity)
    );

    value
}

pub fn is_entity_touching_model(entity: i32, modelHash: u32) -> bool {
    let value = native!(
        bool,
        0x0F42323798A58C8C,
        native_parameters!(entity, modelHash)
    );

    value
}

pub fn is_entity_upright(entity: i32, angle: f32) -> bool {
    let value = native!(bool, 0x5333F526F6AB19AA, native_parameters!(entity, angle));

    value
}

pub fn is_entity_upsidedown(entity: i32) -> bool {
    let value = native!(bool, 0x1DBD58820FA61D71, native_parameters!(entity));

    value
}

pub fn is_entity_visible(entity: i32) -> bool {
    let value = native!(bool, 0x47D6F43D77935C75, native_parameters!(entity));

    value
}

pub fn is_entity_visible_to_script(entity: i32) -> bool {
    let value = native!(bool, 0xD796CB5BA8F20E32, native_parameters!(entity));

    value
}

pub fn is_entity_occluded(entity: i32) -> bool {
    let value = native!(bool, 0xE31C2C72B8692B64, native_parameters!(entity));

    value
}

pub fn would_entity_be_occluded(entityModelHash: u32, x: f32, y: f32, z: f32, p4: bool) -> bool {
    let value = native!(
        bool,
        0xEE5D2A122E09EC42,
        native_parameters!(entityModelHash, x, y, z, p4)
    );

    value
}

pub fn is_entity_waiting_for_world_collision(entity: i32) -> bool {
    let value = native!(bool, 0xD05BFF0C0A12C68F, native_parameters!(entity));

    value
}

pub fn apply_force_to_entity_center_of_mass(
    entity: i32,
    forceType: i32,
    x: f32,
    y: f32,
    z: f32,
    p5: bool,
    isDirectionRel: bool,
    isForceRel: bool,
    p8: bool,
) -> () {
    let value = native!(
        (),
        0x18FF00FC7EFF559E,
        native_parameters!(
            entity,
            forceType,
            x,
            y,
            z,
            p5,
            isDirectionRel,
            isForceRel,
            p8
        )
    );

    value
}

pub fn apply_force_to_entity(
    entity: i32,
    forceFlags: i32,
    x: f32,
    y: f32,
    z: f32,
    offX: f32,
    offY: f32,
    offZ: f32,
    boneIndex: i32,
    isDirectionRel: bool,
    ignoreUpVec: bool,
    isForceRel: bool,
    p12: bool,
    p13: bool,
) -> () {
    let value = native!(
        (),
        0xC5F68BE9613E2D18,
        native_parameters!(
            entity,
            forceFlags,
            x,
            y,
            z,
            offX,
            offY,
            offZ,
            boneIndex,
            isDirectionRel,
            ignoreUpVec,
            isForceRel,
            p12,
            p13
        )
    );

    value
}

pub fn attach_entity_to_entity(
    entity1: i32,
    entity2: i32,
    boneIndex: i32,
    xPos: f32,
    yPos: f32,
    zPos: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    p9: bool,
    useSoftPinning: bool,
    collision: bool,
    isPed: bool,
    vertexIndex: i32,
    fixedRot: bool,
) -> () {
    let value = native!(
        (),
        0x6B9BBD38AB0796DF,
        native_parameters!(
            entity1,
            entity2,
            boneIndex,
            xPos,
            yPos,
            zPos,
            xRot,
            yRot,
            zRot,
            p9,
            useSoftPinning,
            collision,
            isPed,
            vertexIndex,
            fixedRot
        )
    );

    value
}

pub fn _attach_entity_bone_to_entity_bone(
    entity1: i32,
    entity2: i32,
    boneIndex1: i32,
    boneIndex2: i32,
    p4: bool,
    p5: bool,
) -> () {
    let value = native!(
        (),
        0x5C48B75732C8456C,
        native_parameters!(entity1, entity2, boneIndex1, boneIndex2, p4, p5)
    );

    value
}

pub fn _attach_entity_bone_to_entity_bone_physically(
    entity1: i32,
    entity2: i32,
    boneIndex1: i32,
    boneIndex2: i32,
    p4: bool,
    p5: bool,
) -> () {
    let value = native!(
        (),
        0xFD1695C5D3B05439,
        native_parameters!(entity1, entity2, boneIndex1, boneIndex2, p4, p5)
    );

    value
}

pub fn attach_entity_to_entity_physically(
    entity1: i32,
    entity2: i32,
    boneIndex1: i32,
    boneIndex2: i32,
    xPos1: f32,
    yPos1: f32,
    zPos1: f32,
    xPos2: f32,
    yPos2: f32,
    zPos2: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    breakForce: f32,
    fixedRot: bool,
    p15: bool,
    collision: bool,
    p17: bool,
    p18: i32,
) -> () {
    let value = native!(
        (),
        0xC3675780C92F90F9,
        native_parameters!(
            entity1, entity2, boneIndex1, boneIndex2, xPos1, yPos1, zPos1, xPos2, yPos2, zPos2,
            xRot, yRot, zRot, breakForce, fixedRot, p15, collision, p17, p18
        )
    );

    value
}

pub fn process_entity_attachments(entity: i32) -> () {
    let value = native!((), 0xF4080490ADC51C6F, native_parameters!(entity));

    value
}

pub fn get_entity_bone_index_by_name(entity: i32, boneName: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0xFB71170B7E76ACBA,
        native_parameters!(entity, boneName.as_ptr())
    );

    value
}

pub fn clear_entity_last_damage_entity(entity: i32) -> () {
    let value = native!((), 0xA72CD9CA74A5ECBA, native_parameters!(entity));

    value
}

pub fn delete_entity(entity: *mut i32) -> () {
    let value = native!((), 0xAE3CBE5BF394C9C9, native_parameters!(entity));

    value
}

pub fn detach_entity(entity: i32, dynamic: bool, collision: bool) -> () {
    let value = native!(
        (),
        0x961AC54BF0613F5D,
        native_parameters!(entity, dynamic, collision)
    );

    value
}

pub fn freeze_entity_position(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x428CA6DBD1094446, native_parameters!(entity, toggle));

    value
}

pub fn _set_entity_cleanup_by_engine(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x3910051CCECDB00C, native_parameters!(entity, toggle));

    value
}

pub fn play_entity_anim(
    entity: i32,
    animName: &std::ffi::CString,
    animDict: &std::ffi::CString,
    p3: f32,
    loop_esc: bool,
    stayInAnim: bool,
    p6: bool,
    delta: f32,
    bitset: u32,
) -> bool {
    let value = native!(
        bool,
        0x7FB218262B810701,
        native_parameters!(
            entity,
            animName.as_ptr(),
            animDict.as_ptr(),
            p3,
            loop_esc,
            stayInAnim,
            p6,
            delta,
            bitset
        )
    );

    value
}

pub fn play_synchronized_entity_anim(
    entity: i32,
    syncedScene: i32,
    animation: &std::ffi::CString,
    propName: &std::ffi::CString,
    p4: f32,
    p5: f32,
    p6: u32,
    p7: f32,
) -> bool {
    let value = native!(
        bool,
        0xC77720A12FE14A86,
        native_parameters!(
            entity,
            syncedScene,
            animation.as_ptr(),
            propName.as_ptr(),
            p4,
            p5,
            p6,
            p7
        )
    );

    value
}

pub fn play_synchronized_map_entity_anim(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: u32,
    p5: u32,
    p6: *mut u32,
    p7: *mut u32,
    p8: f32,
    p9: f32,
    p10: u32,
    p11: f32,
) -> bool {
    let value = native!(
        bool,
        0xB9C54555ED30FBC4,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11)
    );

    value
}

pub fn stop_synchronized_map_entity_anim(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: u32,
    p5: f32,
) -> bool {
    let value = native!(
        bool,
        0x11E79CAB7183B6F5,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn stop_entity_anim(
    entity: i32,
    animation: &std::ffi::CString,
    animGroup: &std::ffi::CString,
    p3: f32,
) -> u32 {
    let value = native!(
        u32,
        0x28004F88151E03E0,
        native_parameters!(entity, animation.as_ptr(), animGroup.as_ptr(), p3)
    );

    value
}

pub fn stop_synchronized_entity_anim(entity: i32, p1: f32, p2: bool) -> bool {
    let value = native!(bool, 0x43D3807C077261E3, native_parameters!(entity, p1, p2));

    value
}

pub fn has_anim_event_fired(entity: i32, actionHash: u32) -> bool {
    let value = native!(
        bool,
        0xEAF4CD9EA3E7E922,
        native_parameters!(entity, actionHash)
    );

    value
}

pub fn find_anim_event_phase(
    animDictionary: &std::ffi::CString,
    animName: &std::ffi::CString,
    p2: &std::ffi::CString,
    p3: *mut u32,
    p4: *mut u32,
) -> bool {
    let value = native!(
        bool,
        0x07F1BE2BCCAA27A7,
        native_parameters!(
            animDictionary.as_ptr(),
            animName.as_ptr(),
            p2.as_ptr(),
            p3,
            p4
        )
    );

    value
}

pub fn set_entity_anim_current_time(
    entity: i32,
    animDictionary: &std::ffi::CString,
    animName: &std::ffi::CString,
    time: f32,
) -> () {
    let value = native!(
        (),
        0x4487C259F0F70977,
        native_parameters!(entity, animDictionary.as_ptr(), animName.as_ptr(), time)
    );

    value
}

pub fn set_entity_anim_speed(
    entity: i32,
    animDictionary: &std::ffi::CString,
    animName: &std::ffi::CString,
    speedMultiplier: f32,
) -> () {
    let value = native!(
        (),
        0x28D1A16553C51776,
        native_parameters!(
            entity,
            animDictionary.as_ptr(),
            animName.as_ptr(),
            speedMultiplier
        )
    );

    value
}

pub fn set_entity_as_mission_entity(entity: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xAD738C3085FE7E11, native_parameters!(entity, p1, p2));

    value
}

pub fn set_entity_as_no_longer_needed(entity: *mut i32) -> () {
    let value = native!((), 0xB736A491E64A32CF, native_parameters!(entity));

    value
}

pub fn set_ped_as_no_longer_needed(ped: *mut i32) -> () {
    let value = native!((), 0x2595DD4236549CE3, native_parameters!(ped));

    value
}

pub fn set_vehicle_as_no_longer_needed(vehicle: *mut i32) -> () {
    let value = native!((), 0x629BFA74418D6239, native_parameters!(vehicle));

    value
}

pub fn set_object_as_no_longer_needed(object: *mut i32) -> () {
    let value = native!((), 0x3AE22DEB5BA5A3E6, native_parameters!(object));

    value
}

pub fn set_entity_can_be_damaged(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x1760FFA8AB074D66, native_parameters!(entity, toggle));

    value
}

pub fn _get_entity_can_be_damaged(entity: i32) -> bool {
    let value = native!(bool, 0xD95CC5D2AB15A09F, native_parameters!(entity));

    value
}

pub fn set_entity_can_be_damaged_by_relationship_group(
    entity: i32,
    bCanBeDamaged: bool,
    relGroup: i32,
) -> () {
    let value = native!(
        (),
        0xE22D8FDE858B8119,
        native_parameters!(entity, bCanBeDamaged, relGroup)
    );

    value
}

pub fn _0x352e2b5cf420bf3b(p0: u32, p1: u32) -> () {
    let value = native!((), 0x352E2B5CF420BF3B, native_parameters!(p0, p1));

    value
}

pub fn set_entity_can_be_targeted_without_los(entity: i32, toggle: bool) -> () {
    let value = native!((), 0xD3997889736FD899, native_parameters!(entity, toggle));

    value
}

pub fn set_entity_collision(entity: i32, toggle: bool, keepPhysics: bool) -> () {
    let value = native!(
        (),
        0x1A9205C1B9EE827F,
        native_parameters!(entity, toggle, keepPhysics)
    );

    value
}

pub fn get_entity_collision_disabled(entity: i32) -> bool {
    let value = native!(bool, 0xCCF1E97BEFDAE480, native_parameters!(entity));

    value
}

pub fn set_entity_completely_disable_collision(entity: i32, toggle: bool, keepPhysics: bool) -> () {
    let value = native!(
        (),
        0x9EBC85ED0FFFE51C,
        native_parameters!(entity, toggle, keepPhysics)
    );

    value
}

pub fn set_entity_coords(
    entity: i32,
    xPos: f32,
    yPos: f32,
    zPos: f32,
    xAxis: bool,
    yAxis: bool,
    zAxis: bool,
    clearArea: bool,
) -> () {
    let value = native!(
        (),
        0x06843DA7060A026B,
        native_parameters!(entity, xPos, yPos, zPos, xAxis, yAxis, zAxis, clearArea)
    );

    value
}

pub fn set_entity_coords_without_plants_reset(
    entity: i32,
    xPos: f32,
    yPos: f32,
    zPos: f32,
    alive: bool,
    deadFlag: bool,
    ragdollFlag: bool,
    clearArea: bool,
) -> () {
    let value = native!(
        (),
        0x621873ECE1178967,
        native_parameters!(
            entity,
            xPos,
            yPos,
            zPos,
            alive,
            deadFlag,
            ragdollFlag,
            clearArea
        )
    );

    value
}

pub fn set_entity_coords_no_offset(
    entity: i32,
    xPos: f32,
    yPos: f32,
    zPos: f32,
    xAxis: bool,
    yAxis: bool,
    zAxis: bool,
) -> () {
    let value = native!(
        (),
        0x239A3351AC1DA385,
        native_parameters!(entity, xPos, yPos, zPos, xAxis, yAxis, zAxis)
    );

    value
}

pub fn set_entity_dynamic(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x1718DE8E3F2823CA, native_parameters!(entity, toggle));

    value
}

pub fn set_entity_heading(entity: i32, heading: f32) -> () {
    let value = native!((), 0x8E2530AA8ADA980E, native_parameters!(entity, heading));

    value
}

pub fn set_entity_health(entity: i32, health: i32, p2: i32) -> () {
    let value = native!(
        (),
        0x6B76DC1F3AE6E6A3,
        native_parameters!(entity, health, p2)
    );

    value
}

pub fn set_entity_invincible(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x3882114BDE571AD4, native_parameters!(entity, toggle));

    value
}

pub fn set_entity_is_target_priority(entity: i32, p1: bool, p2: f32) -> () {
    let value = native!((), 0xEA02E132F5C68722, native_parameters!(entity, p1, p2));

    value
}

pub fn set_entity_lights(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x7CFBA6A80BDF3874, native_parameters!(entity, toggle));

    value
}

pub fn set_entity_load_collision_flag(entity: i32, toggle: bool, p2: u32) -> () {
    let value = native!(
        (),
        0x0DC7CABAB1E9B67E,
        native_parameters!(entity, toggle, p2)
    );

    value
}

pub fn has_collision_loaded_around_entity(entity: i32) -> bool {
    let value = native!(bool, 0xE9676F61BC0B3321, native_parameters!(entity));

    value
}

pub fn set_entity_max_speed(entity: i32, speed: f32) -> () {
    let value = native!((), 0x0E46A3FCBDE2A1B1, native_parameters!(entity, speed));

    value
}

pub fn set_entity_only_damaged_by_player(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x79F020FF9EDC0748, native_parameters!(entity, toggle));

    value
}

pub fn set_entity_only_damaged_by_relationship_group(entity: i32, p1: bool, p2: u32) -> () {
    let value = native!((), 0x7022BD828FA0B082, native_parameters!(entity, p1, p2));

    value
}

pub fn set_entity_proofs(
    entity: i32,
    bulletProof: bool,
    fireProof: bool,
    explosionProof: bool,
    collisionProof: bool,
    meleeProof: bool,
    p6: bool,
    p7: bool,
    drownProof: bool,
) -> () {
    let value = native!(
        (),
        0xFAEE099C6F890BB8,
        native_parameters!(
            entity,
            bulletProof,
            fireProof,
            explosionProof,
            collisionProof,
            meleeProof,
            p6,
            p7,
            drownProof
        )
    );

    value
}

pub fn _get_entity_proofs(
    entity: i32,
    bulletProof: *mut bool,
    fireProof: *mut bool,
    explosionProof: *mut bool,
    collisionProof: *mut bool,
    meleeProof: *mut bool,
    steamProof: *mut bool,
    p7: *mut bool,
    drownProof: *mut bool,
) -> bool {
    let value = native!(
        bool,
        0xBE8CD9BE829BBEBF,
        native_parameters!(
            entity,
            bulletProof,
            fireProof,
            explosionProof,
            collisionProof,
            meleeProof,
            steamProof,
            p7,
            drownProof
        )
    );

    value
}

pub fn set_entity_quaternion(entity: i32, x: f32, y: f32, z: f32, w: f32) -> () {
    let value = native!(
        (),
        0x77B21BE7AC540F07,
        native_parameters!(entity, x, y, z, w)
    );

    value
}

pub fn set_entity_records_collisions(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x0A50A1EEDAD01E65, native_parameters!(entity, toggle));

    value
}

pub fn set_entity_rotation(
    entity: i32,
    pitch: f32,
    roll: f32,
    yaw: f32,
    rotationOrder: i32,
    p5: bool,
) -> () {
    let value = native!(
        (),
        0x8524A8B0171D5E07,
        native_parameters!(entity, pitch, roll, yaw, rotationOrder, p5)
    );

    value
}

pub fn set_entity_visible(entity: i32, toggle: bool, unk: bool) -> () {
    let value = native!(
        (),
        0xEA1C610A04DB6BBB,
        native_parameters!(entity, toggle, unk)
    );

    value
}

pub fn _0xc34bc448da29f5e9(entity: i32, toggle: bool) -> () {
    let value = native!((), 0xC34BC448DA29F5E9, native_parameters!(entity, toggle));

    value
}

pub fn _0xe66377cddada4810(entity: i32, p1: bool) -> () {
    let value = native!((), 0xE66377CDDADA4810, native_parameters!(entity, p1));

    value
}

pub fn set_entity_velocity(entity: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x1C99BB7B6E96D16F, native_parameters!(entity, x, y, z));

    value
}

pub fn set_entity_has_gravity(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x4A4722448F18EEF5, native_parameters!(entity, toggle));

    value
}

pub fn set_entity_lod_dist(entity: i32, value: i32) -> () {
    let value = native!((), 0x5927F96A78577363, native_parameters!(entity, value));

    value
}

pub fn get_entity_lod_dist(entity: i32) -> i32 {
    let value = native!(i32, 0x4159C2762B5791D6, native_parameters!(entity));

    value
}

pub fn set_entity_alpha(entity: i32, alphaLevel: i32, skin: bool) -> () {
    let value = native!(
        (),
        0x44A0870B7E92D7C0,
        native_parameters!(entity, alphaLevel, skin)
    );

    value
}

pub fn get_entity_alpha(entity: i32) -> i32 {
    let value = native!(i32, 0x5A47B3B5E63E94C6, native_parameters!(entity));

    value
}

pub fn reset_entity_alpha(entity: i32) -> () {
    let value = native!((), 0x9B1E824FFBB7027A, native_parameters!(entity));

    value
}

pub fn _0x490861b88f4fd846(p0: u32) -> () {
    let value = native!((), 0x490861B88F4FD846, native_parameters!(p0));

    value
}

pub fn _0xcea7c8e1b48ff68c(p0: u32, p1: u32) -> () {
    let value = native!((), 0xCEA7C8E1B48FF68C, native_parameters!(p0, p1));

    value
}

pub fn _0x5c3b791d580e0bc2(entity: i32, p1: f32) -> () {
    let value = native!((), 0x5C3B791D580E0BC2, native_parameters!(entity, p1));

    value
}

pub fn set_entity_always_prerender(entity: i32, toggle: bool) -> () {
    let value = native!((), 0xACAD101E1FB66689, native_parameters!(entity, toggle));

    value
}

pub fn set_entity_render_scorched(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x730F5F8D3F0F2050, native_parameters!(entity, toggle));

    value
}

pub fn set_entity_trafficlight_override(entity: i32, state: i32) -> () {
    let value = native!((), 0x57C5DB656185EAC4, native_parameters!(entity, state));

    value
}

pub fn _0x78e8e3a640178255(entity: i32) -> () {
    let value = native!((), 0x78E8E3A640178255, native_parameters!(entity));

    value
}

pub fn create_model_swap(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    originalModel: u32,
    newModel: u32,
    p6: bool,
) -> () {
    let value = native!(
        (),
        0x92C47782FDA8B2A3,
        native_parameters!(x, y, z, radius, originalModel, newModel, p6)
    );

    value
}

pub fn remove_model_swap(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    originalModel: u32,
    newModel: u32,
    p6: bool,
) -> () {
    let value = native!(
        (),
        0x033C0F9A64E229AE,
        native_parameters!(x, y, z, radius, originalModel, newModel, p6)
    );

    value
}

pub fn create_model_hide(x: f32, y: f32, z: f32, radius: f32, modelHash: u32, p5: bool) -> () {
    let value = native!(
        (),
        0x8A97BCA30A0CE478,
        native_parameters!(x, y, z, radius, modelHash, p5)
    );

    value
}

pub fn create_model_hide_excluding_script_objects(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    modelHash: u32,
    p5: bool,
) -> () {
    let value = native!(
        (),
        0x3A52AE588830BF7F,
        native_parameters!(x, y, z, radius, modelHash, p5)
    );

    value
}

pub fn remove_model_hide(x: f32, y: f32, z: f32, radius: f32, modelHash: u32, p5: bool) -> () {
    let value = native!(
        (),
        0xD9E3006FB3CBD765,
        native_parameters!(x, y, z, radius, modelHash, p5)
    );

    value
}

pub fn create_forced_object(x: f32, y: f32, z: f32, p3: u32, modelHash: u32, p5: bool) -> () {
    let value = native!(
        (),
        0x150E808B375A385A,
        native_parameters!(x, y, z, p3, modelHash, p5)
    );

    value
}

pub fn remove_forced_object(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x61B6775E83C0DB6F,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn set_entity_no_collision_entity(entity1: i32, entity2: i32, thisFrameOnly: bool) -> () {
    let value = native!(
        (),
        0xA53ED5520C07654A,
        native_parameters!(entity1, entity2, thisFrameOnly)
    );

    value
}

pub fn set_entity_motion_blur(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x295D82A8559F9150, native_parameters!(entity, toggle));

    value
}

pub fn set_can_auto_vault_on_entity(entity: i32, toggle: bool) -> () {
    let value = native!((), 0xE12ABE5E3A389A6C, native_parameters!(entity, toggle));

    value
}

pub fn set_can_climb_on_entity(entity: i32, toggle: bool) -> () {
    let value = native!((), 0xA80AE305E0A3044F, native_parameters!(entity, toggle));

    value
}

pub fn _0xdc6f8601faf2e893(entity: i32, toggle: bool) -> () {
    let value = native!((), 0xDC6F8601FAF2E893, native_parameters!(entity, toggle));

    value
}

pub fn _set_entity_decals_disabled(entity: i32, p1: bool) -> () {
    let value = native!((), 0x2C2E3DC128F44309, native_parameters!(entity, p1));

    value
}

pub fn _0x1a092bb0c3808b96(entity: i32, p1: bool) -> () {
    let value = native!((), 0x1A092BB0C3808B96, native_parameters!(entity, p1));

    value
}

pub fn _get_entity_bone_rotation(entity: i32, boneIndex: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xCE6294A232D03786,
        native_parameters!(entity, boneIndex)
    );

    value
}

pub fn _get_entity_bone_position_2(entity: i32, boneIndex: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x46F8696933A63C9B,
        native_parameters!(entity, boneIndex)
    );

    value
}

pub fn _get_entity_bone_rotation_local(entity: i32, boneIndex: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xBD8D32550E5CEBFE,
        native_parameters!(entity, boneIndex)
    );

    value
}

pub fn _get_entity_bone_count(entity: i32) -> i32 {
    let value = native!(i32, 0xB328DCC3A3AA401B, native_parameters!(entity));

    value
}

pub fn _enable_entity_unk(entity: i32) -> () {
    let value = native!((), 0x6CE177D014502E8A, native_parameters!(entity));

    value
}

pub fn _0xb17bc6453f6cf5ac(p0: u32, p1: u32) -> () {
    let value = native!((), 0xB17BC6453F6CF5AC, native_parameters!(p0, p1));

    value
}

pub fn _0x68b562e124cc0aef(pickup: i32, pickup2: i32) -> () {
    let value = native!((), 0x68B562E124CC0AEF, native_parameters!(pickup, pickup2));

    value
}

pub fn _0x36f32de87082343e(p0: u32, p1: u32) -> () {
    let value = native!((), 0x36F32DE87082343E, native_parameters!(p0, p1));

    value
}

pub fn _get_entity_pickup(entity: i32, modelHash: u32) -> i32 {
    let value = native!(
        i32,
        0x1F922734E259BD26,
        native_parameters!(entity, modelHash)
    );

    value
}

pub fn _0xd7b80e7c3befc396(pickup: i32, toggle: bool) -> () {
    let value = native!((), 0xD7B80E7C3BEFC396, native_parameters!(pickup, toggle));

    value
}
