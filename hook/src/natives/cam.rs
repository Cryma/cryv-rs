use crate::types::NativeVector3;

pub fn render_script_cams(
    render: bool,
    ease: bool,
    easeTime: i32,
    p3: bool,
    p4: bool,
    p5: u32,
) -> () {
    let value = native!(
        (),
        0x07E5B515DB0636FC,
        native_parameters!(render, ease, easeTime, p3, p4, p5)
    );

    value
}

pub fn stop_rendering_script_cams_using_catch_up(render: bool, p1: f32, p2: i32, p3: u32) -> () {
    let value = native!(
        (),
        0xC819F3CBB62BF692,
        native_parameters!(render, p1, p2, p3)
    );

    value
}

pub fn create_cam(camName: &std::ffi::CString, p1: bool) -> i32 {
    let value = native!(
        i32,
        0xC3981DCE61D9E13F,
        native_parameters!(camName.as_ptr(), p1)
    );

    value
}

pub fn create_cam_with_params(
    camName: &std::ffi::CString,
    posX: f32,
    posY: f32,
    posZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    fov: f32,
    p8: bool,
    p9: i32,
) -> i32 {
    let value = native!(
        i32,
        0xB51194800B257161,
        native_parameters!(
            camName.as_ptr(),
            posX,
            posY,
            posZ,
            rotX,
            rotY,
            rotZ,
            fov,
            p8,
            p9
        )
    );

    value
}

pub fn create_camera(camHash: u32, p1: bool) -> i32 {
    let value = native!(i32, 0x5E3CF89C6BCCA67D, native_parameters!(camHash, p1));

    value
}

pub fn create_camera_with_params(
    camHash: u32,
    posX: f32,
    posY: f32,
    posZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    fov: f32,
    p8: bool,
    p9: u32,
) -> i32 {
    let value = native!(
        i32,
        0x6ABFA3E16460F22D,
        native_parameters!(camHash, posX, posY, posZ, rotX, rotY, rotZ, fov, p8, p9)
    );

    value
}

pub fn destroy_cam(cam: i32, bScriptHostCam: bool) -> () {
    let value = native!(
        (),
        0x865908C81A2C22E9,
        native_parameters!(cam, bScriptHostCam)
    );

    value
}

pub fn destroy_all_cams(bScriptHostCam: bool) -> () {
    let value = native!((), 0x8E5FB15663F79120, native_parameters!(bScriptHostCam));

    value
}

pub fn does_cam_exist(cam: i32) -> bool {
    let value = native!(bool, 0xA7A932170592B50E, native_parameters!(cam));

    value
}

pub fn set_cam_active(cam: i32, active: bool) -> () {
    let value = native!((), 0x026FB97D0A425F84, native_parameters!(cam, active));

    value
}

pub fn is_cam_active(cam: i32) -> bool {
    let value = native!(bool, 0xDFB2B516207D3534, native_parameters!(cam));

    value
}

pub fn is_cam_rendering(cam: i32) -> bool {
    let value = native!(bool, 0x02EC0AF5C5A49B7A, native_parameters!(cam));

    value
}

pub fn get_rendering_cam() -> i32 {
    let value = native!(i32, 0x5234F9F10919EABA, native_parameters!());

    value
}

pub fn get_cam_coord(cam: i32) -> NativeVector3 {
    let value = native!(NativeVector3, 0xBAC038F7459AE5AE, native_parameters!(cam));

    value
}

pub fn get_cam_rot(cam: i32, rotationOrder: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x7D304C1C955E3E12,
        native_parameters!(cam, rotationOrder)
    );

    value
}

pub fn get_cam_fov(cam: i32) -> f32 {
    let value = native!(f32, 0xC3330A45CCCDB26A, native_parameters!(cam));

    value
}

pub fn get_cam_near_clip(cam: i32) -> f32 {
    let value = native!(f32, 0xC520A34DAFBF24B1, native_parameters!(cam));

    value
}

pub fn get_cam_far_clip(cam: i32) -> f32 {
    let value = native!(f32, 0xB60A9CFEB21CA6AA, native_parameters!(cam));

    value
}

pub fn get_cam_far_dof(cam: i32) -> f32 {
    let value = native!(f32, 0x255F8DAFD540D397, native_parameters!(cam));

    value
}

pub fn set_cam_params(
    cam: i32,
    posX: f32,
    posY: f32,
    posZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    fieldOfView: f32,
    p8: u32,
    p9: i32,
    p10: i32,
    p11: i32,
) -> () {
    let value = native!(
        (),
        0xBFD8727AEA3CCEBA,
        native_parameters!(
            cam,
            posX,
            posY,
            posZ,
            rotX,
            rotY,
            rotZ,
            fieldOfView,
            p8,
            p9,
            p10,
            p11
        )
    );

    value
}

pub fn set_cam_coord(cam: i32, posX: f32, posY: f32, posZ: f32) -> () {
    let value = native!(
        (),
        0x4D41783FB745E42E,
        native_parameters!(cam, posX, posY, posZ)
    );

    value
}

pub fn set_cam_rot(cam: i32, rotX: f32, rotY: f32, rotZ: f32, rotationOrder: i32) -> () {
    let value = native!(
        (),
        0x85973643155D0B07,
        native_parameters!(cam, rotX, rotY, rotZ, rotationOrder)
    );

    value
}

pub fn set_cam_fov(cam: i32, fieldOfView: f32) -> () {
    let value = native!((), 0xB13C14F66A00D047, native_parameters!(cam, fieldOfView));

    value
}

pub fn set_cam_near_clip(cam: i32, nearClip: f32) -> () {
    let value = native!((), 0xC7848EFCCC545182, native_parameters!(cam, nearClip));

    value
}

pub fn set_cam_far_clip(cam: i32, farClip: f32) -> () {
    let value = native!((), 0xAE306F2A904BF86E, native_parameters!(cam, farClip));

    value
}

pub fn _0xaabd62873ffb1a33(p0: u32, p1: u32) -> () {
    let value = native!((), 0xAABD62873FFB1A33, native_parameters!(p0, p1));

    value
}

pub fn set_cam_motion_blur_strength(cam: i32, strength: f32) -> () {
    let value = native!((), 0x6F0F77FBA9A8F2E6, native_parameters!(cam, strength));

    value
}

pub fn set_cam_near_dof(cam: i32, nearDOF: f32) -> () {
    let value = native!((), 0x3FA4BF0A7AB7DE2C, native_parameters!(cam, nearDOF));

    value
}

pub fn set_cam_far_dof(cam: i32, farDOF: f32) -> () {
    let value = native!((), 0xEDD91296CD01AEE0, native_parameters!(cam, farDOF));

    value
}

pub fn set_cam_dof_strength(cam: i32, dofStrength: f32) -> () {
    let value = native!((), 0x5EE29B4D7D5DF897, native_parameters!(cam, dofStrength));

    value
}

pub fn set_cam_dof_planes(cam: i32, p1: f32, p2: f32, p3: f32, p4: f32) -> () {
    let value = native!(
        (),
        0x3CF48F6F96E749DC,
        native_parameters!(cam, p1, p2, p3, p4)
    );

    value
}

pub fn set_cam_use_shallow_dof_mode(cam: i32, toggle: bool) -> () {
    let value = native!((), 0x16A96863A17552BB, native_parameters!(cam, toggle));

    value
}

pub fn set_use_hi_dof() -> () {
    let value = native!((), 0xA13B0222F3D94A94, native_parameters!());

    value
}

pub fn _0xf55e4046f6f831dc(p0: u32, p1: f32) -> () {
    let value = native!((), 0xF55E4046F6F831DC, native_parameters!(p0, p1));

    value
}

pub fn _0xe111a7c0d200cbc5(p0: u32, p1: f32) -> () {
    let value = native!((), 0xE111A7C0D200CBC5, native_parameters!(p0, p1));

    value
}

pub fn _set_cam_dof_fnumber_of_lens(camera: i32, p1: f32) -> () {
    let value = native!((), 0x7DD234D6F3914C5B, native_parameters!(camera, p1));

    value
}

pub fn _set_cam_dof_focal_length_multiplier(camera: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0x47B595D60664CFFA,
        native_parameters!(camera, multiplier)
    );

    value
}

pub fn _set_cam_dof_focus_distance_bias(camera: i32, p1: f32) -> () {
    let value = native!((), 0xC669EEA5D031B7DE, native_parameters!(camera, p1));

    value
}

pub fn _set_cam_dof_max_near_in_focus_distance(camera: i32, p1: f32) -> () {
    let value = native!((), 0xC3654A441402562D, native_parameters!(camera, p1));

    value
}

pub fn _set_cam_dof_max_near_in_focus_distance_blend_level(camera: i32, p1: f32) -> () {
    let value = native!((), 0x2C654B4943BDDF7C, native_parameters!(camera, p1));

    value
}

pub fn attach_cam_to_entity(
    cam: i32,
    entity: i32,
    xOffset: f32,
    yOffset: f32,
    zOffset: f32,
    isRelative: bool,
) -> () {
    let value = native!(
        (),
        0xFEDB7D269E8C60E3,
        native_parameters!(cam, entity, xOffset, yOffset, zOffset, isRelative)
    );

    value
}

pub fn attach_cam_to_ped_bone(
    cam: i32,
    ped: i32,
    boneIndex: i32,
    x: f32,
    y: f32,
    z: f32,
    heading: bool,
) -> () {
    let value = native!(
        (),
        0x61A3DBA14AB7F411,
        native_parameters!(cam, ped, boneIndex, x, y, z, heading)
    );

    value
}

pub fn _attach_cam_to_ped_bone_2(
    cam: i32,
    ped: i32,
    boneIndex: i32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
    p8: f32,
    p9: bool,
) -> () {
    let value = native!(
        (),
        0x149916F50C34A40D,
        native_parameters!(cam, ped, boneIndex, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn _0x202a5ed9ce01d6e7(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
    p8: u32,
) -> () {
    let value = native!(
        (),
        0x202A5ED9CE01D6E7,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn _attach_cam_to_vehicle_bone(
    cam: i32,
    vehicle: i32,
    boneIndex: i32,
    relativeRotation: bool,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    fixedDirection: bool,
) -> () {
    let value = native!(
        (),
        0x8DB3F12A02CAEF72,
        native_parameters!(
            cam,
            vehicle,
            boneIndex,
            relativeRotation,
            rotX,
            rotY,
            rotZ,
            offsetX,
            offsetY,
            offsetZ,
            fixedDirection
        )
    );

    value
}

pub fn detach_cam(cam: i32) -> () {
    let value = native!((), 0xA2FABBE87F4BAD82, native_parameters!(cam));

    value
}

pub fn set_cam_inherit_roll_vehicle(cam: i32, p1: bool) -> () {
    let value = native!((), 0x45F1DE9C34B93AE6, native_parameters!(cam, p1));

    value
}

pub fn point_cam_at_coord(cam: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xF75497BB865F0803, native_parameters!(cam, x, y, z));

    value
}

pub fn point_cam_at_entity(cam: i32, entity: i32, p2: f32, p3: f32, p4: f32, p5: bool) -> () {
    let value = native!(
        (),
        0x5640BFF86B16E8DC,
        native_parameters!(cam, entity, p2, p3, p4, p5)
    );

    value
}

pub fn point_cam_at_ped_bone(
    cam: i32,
    ped: i32,
    boneIndex: i32,
    x: f32,
    y: f32,
    z: f32,
    p6: bool,
) -> () {
    let value = native!(
        (),
        0x68B2B5F33BA63C41,
        native_parameters!(cam, ped, boneIndex, x, y, z, p6)
    );

    value
}

pub fn stop_cam_pointing(cam: i32) -> () {
    let value = native!((), 0xF33AB75780BA57DE, native_parameters!(cam));

    value
}

pub fn set_cam_affects_aiming(cam: i32, toggle: bool) -> () {
    let value = native!((), 0x8C1DC7770C51DC8D, native_parameters!(cam, toggle));

    value
}

pub fn _0x661b5c8654add825(cam: i32, p1: bool) -> () {
    let value = native!((), 0x661B5C8654ADD825, native_parameters!(cam, p1));

    value
}

pub fn _0xa2767257a320fc82(p0: u32, p1: bool) -> () {
    let value = native!((), 0xA2767257A320FC82, native_parameters!(p0, p1));

    value
}

pub fn _0x271017b9ba825366(p0: u32, p1: bool) -> () {
    let value = native!((), 0x271017B9BA825366, native_parameters!(p0, p1));

    value
}

pub fn set_cam_debug_name(camera: i32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x1B93E0107865DD40,
        native_parameters!(camera, name.as_ptr())
    );

    value
}

pub fn add_cam_spline_node(
    camera: i32,
    x: f32,
    y: f32,
    z: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    length: i32,
    p8: i32,
    p9: i32,
) -> () {
    let value = native!(
        (),
        0x8609C75EC438FB3B,
        native_parameters!(camera, x, y, z, xRot, yRot, zRot, length, p8, p9)
    );

    value
}

pub fn add_cam_spline_node_using_camera_frame(cam: i32, cam2: i32, p2: i32, p3: i32) -> () {
    let value = native!(
        (),
        0x0A9F2A468B328E74,
        native_parameters!(cam, cam2, p2, p3)
    );

    value
}

pub fn add_cam_spline_node_using_camera(cam: i32, cam2: i32, p2: i32, p3: i32) -> () {
    let value = native!(
        (),
        0x0FB82563989CF4FB,
        native_parameters!(cam, cam2, p2, p3)
    );

    value
}

pub fn add_cam_spline_node_using_gameplay_frame(cam: i32, p1: i32, p2: i32) -> () {
    let value = native!((), 0x609278246A29CA34, native_parameters!(cam, p1, p2));

    value
}

pub fn set_cam_spline_phase(cam: i32, p1: f32) -> () {
    let value = native!((), 0x242B5874F0A4E052, native_parameters!(cam, p1));

    value
}

pub fn get_cam_spline_phase(cam: i32) -> f32 {
    let value = native!(f32, 0xB5349E36C546509A, native_parameters!(cam));

    value
}

pub fn get_cam_spline_node_phase(cam: i32) -> f32 {
    let value = native!(f32, 0xD9D0E694C8282C96, native_parameters!(cam));

    value
}

pub fn set_cam_spline_duration(cam: i32, timeDuration: i32) -> () {
    let value = native!(
        (),
        0x1381539FEE034CDA,
        native_parameters!(cam, timeDuration)
    );

    value
}

pub fn set_cam_spline_smoothing_style(cam: i32, smoothingStyle: i32) -> () {
    let value = native!(
        (),
        0xD1B0F412F109EA5D,
        native_parameters!(cam, smoothingStyle)
    );

    value
}

pub fn get_cam_spline_node_index(cam: i32) -> i32 {
    let value = native!(i32, 0xB22B17DF858716A6, native_parameters!(cam));

    value
}

pub fn set_cam_spline_node_ease(cam: i32, p1: i32, p2: i32, p3: f32) -> () {
    let value = native!((), 0x83B8201ED82A9A2D, native_parameters!(cam, p1, p2, p3));

    value
}

pub fn set_cam_spline_node_velocity_scale(cam: i32, p1: i32, scale: f32) -> () {
    let value = native!((), 0xA6385DEB180F319F, native_parameters!(cam, p1, scale));

    value
}

pub fn override_cam_spline_velocity(cam: i32, p1: i32, p2: f32, p3: f32) -> () {
    let value = native!((), 0x40B62FA033EB0346, native_parameters!(cam, p1, p2, p3));

    value
}

pub fn override_cam_spline_motion_blur(cam: i32, p1: i32, p2: f32, p3: f32) -> () {
    let value = native!((), 0x7DCF7C708D292D55, native_parameters!(cam, p1, p2, p3));

    value
}

pub fn set_cam_spline_node_extra_flags(cam: i32, p1: i32, flags: i32) -> () {
    let value = native!((), 0x7BF1A54AE67AC070, native_parameters!(cam, p1, flags));

    value
}

pub fn is_cam_spline_paused(p0: u32) -> bool {
    let value = native!(bool, 0x0290F35C0AD97864, native_parameters!(p0));

    value
}

pub fn set_cam_active_with_interp(
    camTo: i32,
    camFrom: i32,
    duration: i32,
    easeLocation: i32,
    easeRotation: i32,
) -> () {
    let value = native!(
        (),
        0x9FBDA379383A52A4,
        native_parameters!(camTo, camFrom, duration, easeLocation, easeRotation)
    );

    value
}

pub fn is_cam_interpolating(cam: i32) -> bool {
    let value = native!(bool, 0x036F97C908C2B52C, native_parameters!(cam));

    value
}

pub fn shake_cam(cam: i32, type_esc: &std::ffi::CString, amplitude: f32) -> () {
    let value = native!(
        (),
        0x6A25241C340D3822,
        native_parameters!(cam, type_esc.as_ptr(), amplitude)
    );

    value
}

pub fn animated_shake_cam(
    cam: i32,
    p1: &std::ffi::CString,
    p2: &std::ffi::CString,
    p3: &std::ffi::CString,
    amplitude: f32,
) -> () {
    let value = native!(
        (),
        0xA2746EEAE3E577CD,
        native_parameters!(cam, p1.as_ptr(), p2.as_ptr(), p3.as_ptr(), amplitude)
    );

    value
}

pub fn is_cam_shaking(cam: i32) -> bool {
    let value = native!(bool, 0x6B24BFE83A2BE47B, native_parameters!(cam));

    value
}

pub fn set_cam_shake_amplitude(cam: i32, amplitude: f32) -> () {
    let value = native!((), 0xD93DB43B82BC0D00, native_parameters!(cam, amplitude));

    value
}

pub fn stop_cam_shaking(cam: i32, p1: bool) -> () {
    let value = native!((), 0xBDECF64367884AC3, native_parameters!(cam, p1));

    value
}

pub fn shake_script_global(p0: &std::ffi::CString, p1: f32) -> () {
    let value = native!((), 0xF4C8CF9E353AFECA, native_parameters!(p0.as_ptr(), p1));

    value
}

pub fn animated_shake_script_global(
    p0: &std::ffi::CString,
    p1: &std::ffi::CString,
    p2: &std::ffi::CString,
    p3: f32,
) -> () {
    let value = native!(
        (),
        0xC2EAE3FB8CDBED31,
        native_parameters!(p0.as_ptr(), p1.as_ptr(), p2.as_ptr(), p3)
    );

    value
}

pub fn is_script_global_shaking() -> bool {
    let value = native!(bool, 0xC912AF078AF19212, native_parameters!());

    value
}

pub fn stop_script_global_shaking(p0: bool) -> () {
    let value = native!((), 0x1C9D7949FA533490, native_parameters!(p0));

    value
}

pub fn _0x5d96cfb59da076a0(vehicle: i32, p1: i32, p2: f32) -> () {
    let value = native!((), 0x5D96CFB59DA076A0, native_parameters!(vehicle, p1, p2));

    value
}

pub fn play_cam_anim(
    cam: i32,
    animName: &std::ffi::CString,
    animDictionary: &std::ffi::CString,
    x: f32,
    y: f32,
    z: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    p9: bool,
    p10: i32,
) -> bool {
    let value = native!(
        bool,
        0x9A2D0FB2E7852392,
        native_parameters!(
            cam,
            animName.as_ptr(),
            animDictionary.as_ptr(),
            x,
            y,
            z,
            xRot,
            yRot,
            zRot,
            p9,
            p10
        )
    );

    value
}

pub fn is_cam_playing_anim(
    cam: i32,
    animName: &std::ffi::CString,
    animDictionary: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0xC90621D8A0CEECF2,
        native_parameters!(cam, animName.as_ptr(), animDictionary.as_ptr())
    );

    value
}

pub fn set_cam_anim_current_phase(cam: i32, phase: f32) -> () {
    let value = native!((), 0x4145A4C44FF3B5A6, native_parameters!(cam, phase));

    value
}

pub fn get_cam_anim_current_phase(cam: i32) -> f32 {
    let value = native!(f32, 0xA10B2DB49E92A6B0, native_parameters!(cam));

    value
}

pub fn play_synchronized_cam_anim(
    p0: u32,
    p1: u32,
    animName: &std::ffi::CString,
    animDictionary: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0xE32EFE9AB4A9AA0C,
        native_parameters!(p0, p1, animName.as_ptr(), animDictionary.as_ptr())
    );

    value
}

pub fn set_fly_cam_horizontal_response(cam: i32, p1: f32, p2: f32, p3: f32) -> () {
    let value = native!((), 0x503F5920162365B2, native_parameters!(cam, p1, p2, p3));

    value
}

pub fn _set_fly_cam_vertical_speed_multiplier(cam: i32, p1: f32, p2: f32, p3: f32) -> () {
    let value = native!((), 0xE827B9382CFB41BA, native_parameters!(cam, p1, p2, p3));

    value
}

pub fn set_fly_cam_max_height(cam: i32, height: f32) -> () {
    let value = native!((), 0xF9D02130ECDD1D77, native_parameters!(cam, height));

    value
}

pub fn set_fly_cam_coord_and_constrain(cam: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xC91C6C55199308CA, native_parameters!(cam, x, y, z));

    value
}

pub fn _0xc8b5c4a79cc18b94(cam: i32) -> () {
    let value = native!((), 0xC8B5C4A79CC18B94, native_parameters!(cam));

    value
}

pub fn _0x5c48a1d6e3b33179(cam: i32) -> bool {
    let value = native!(bool, 0x5C48A1D6E3B33179, native_parameters!(cam));

    value
}

pub fn is_screen_faded_out() -> bool {
    let value = native!(bool, 0xB16FCE9DDC7BA182, native_parameters!());

    value
}

pub fn is_screen_faded_in() -> bool {
    let value = native!(bool, 0x5A859503B0C08678, native_parameters!());

    value
}

pub fn is_screen_fading_out() -> bool {
    let value = native!(bool, 0x797AC7CB535BA28F, native_parameters!());

    value
}

pub fn is_screen_fading_in() -> bool {
    let value = native!(bool, 0x5C544BC6C57AC575, native_parameters!());

    value
}

pub fn do_screen_fade_in(duration: i32) -> () {
    let value = native!((), 0xD4E8E24955024033, native_parameters!(duration));

    value
}

pub fn do_screen_fade_out(duration: i32) -> () {
    let value = native!((), 0x891B5B39AC6302AF, native_parameters!(duration));

    value
}

pub fn set_widescreen_borders(p0: bool, p1: i32) -> () {
    let value = native!((), 0xDCD4EA924F42D01A, native_parameters!(p0, p1));

    value
}

pub fn _0x4879e4fe39074cdf() -> bool {
    let value = native!(bool, 0x4879E4FE39074CDF, native_parameters!());

    value
}

pub fn get_gameplay_cam_coord() -> NativeVector3 {
    let value = native!(NativeVector3, 0x14D6F5678D8F1B37, native_parameters!());

    value
}

pub fn get_gameplay_cam_rot(rotationOrder: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x837765A25378F0BB,
        native_parameters!(rotationOrder)
    );

    value
}

pub fn get_gameplay_cam_fov() -> f32 {
    let value = native!(f32, 0x65019750A0324133, native_parameters!());

    value
}

pub fn _0x487a82c650eb7799(p0: f32) -> () {
    let value = native!((), 0x487A82C650EB7799, native_parameters!(p0));

    value
}

pub fn _0x0225778816fdc28c(p0: f32) -> () {
    let value = native!((), 0x0225778816FDC28C, native_parameters!(p0));

    value
}

pub fn get_gameplay_cam_relative_heading() -> f32 {
    let value = native!(f32, 0x743607648ADD4587, native_parameters!());

    value
}

pub fn set_gameplay_cam_relative_heading(heading: f32) -> () {
    let value = native!((), 0xB4EC2312F4E5B1F1, native_parameters!(heading));

    value
}

pub fn get_gameplay_cam_relative_pitch() -> f32 {
    let value = native!(f32, 0x3A6867B4845BEDA2, native_parameters!());

    value
}

pub fn set_gameplay_cam_relative_pitch(angle: f32, scalingFactor: f32) -> () {
    let value = native!(
        (),
        0x6D0858B8EDFD2B7D,
        native_parameters!(angle, scalingFactor)
    );

    value
}

pub fn _set_gameplay_cam_relative_rotation(roll: f32, pitch: f32, yaw: f32) -> () {
    let value = native!((), 0x48608C3464F58AB4, native_parameters!(roll, pitch, yaw));

    value
}

pub fn _0x28b022a17b068a3a(p0: f32, p1: f32) -> () {
    let value = native!((), 0x28B022A17B068A3A, native_parameters!(p0, p1));

    value
}

pub fn _set_gameplay_cam_raw_yaw(yaw: f32) -> () {
    let value = native!((), 0x103991D4A307D472, native_parameters!(yaw));

    value
}

pub fn _set_gameplay_cam_raw_pitch(pitch: f32) -> () {
    let value = native!((), 0x759E13EBC1C15C5A, native_parameters!(pitch));

    value
}

pub fn _0x469f2ecdec046337(p0: bool) -> () {
    let value = native!((), 0x469F2ECDEC046337, native_parameters!(p0));

    value
}

pub fn shake_gameplay_cam(shakeName: &std::ffi::CString, intensity: f32) -> () {
    let value = native!(
        (),
        0xFD55E49555E017CF,
        native_parameters!(shakeName.as_ptr(), intensity)
    );

    value
}

pub fn is_gameplay_cam_shaking() -> bool {
    let value = native!(bool, 0x016C090630DF1F89, native_parameters!());

    value
}

pub fn set_gameplay_cam_shake_amplitude(amplitude: f32) -> () {
    let value = native!((), 0xA87E00932DB4D85D, native_parameters!(amplitude));

    value
}

pub fn stop_gameplay_cam_shaking(p0: bool) -> () {
    let value = native!((), 0x0EF93E9F3D08C178, native_parameters!(p0));

    value
}

pub fn set_gameplay_cam_follow_ped_this_update(ped: i32) -> () {
    let value = native!((), 0x8BBACBF51DA047A8, native_parameters!(ped));

    value
}

pub fn is_gameplay_cam_rendering() -> bool {
    let value = native!(bool, 0x39B5D1B10383F0C8, native_parameters!());

    value
}

pub fn _0x3044240d2e0fa842() -> bool {
    let value = native!(bool, 0x3044240D2E0FA842, native_parameters!());

    value
}

pub fn _0x705a276ebff3133d() -> bool {
    let value = native!(bool, 0x705A276EBFF3133D, native_parameters!());

    value
}

pub fn _0xdb90c6cca48940f1(p0: bool) -> () {
    let value = native!((), 0xDB90C6CCA48940F1, native_parameters!(p0));

    value
}

pub fn _enable_crosshair_this_frame() -> () {
    let value = native!((), 0xEA7F0AD7E9BA676F, native_parameters!());

    value
}

pub fn is_gameplay_cam_looking_behind() -> bool {
    let value = native!(bool, 0x70FDA869F3317EA9, native_parameters!());

    value
}

pub fn _disable_cam_collision_for_entity(entity: i32) -> () {
    let value = native!((), 0x2AED6301F67007D5, native_parameters!(entity));

    value
}

pub fn disable_cam_collision_for_object(entity: i32) -> () {
    let value = native!((), 0x49482F9FCD825AAA, native_parameters!(entity));

    value
}

pub fn _0xa7092afe81944852() -> () {
    let value = native!((), 0xA7092AFE81944852, native_parameters!());

    value
}

pub fn _0xfd3151cd37ea2245(entity: i32) -> () {
    let value = native!((), 0xFD3151CD37EA2245, native_parameters!(entity));

    value
}

pub fn _0xb1381b97f70c7b30() -> () {
    let value = native!((), 0xB1381B97F70C7B30, native_parameters!());

    value
}

pub fn _0xdd79df9f4d26e1c9() -> () {
    let value = native!((), 0xDD79DF9F4D26E1C9, native_parameters!());

    value
}

pub fn is_sphere_visible(x: f32, y: f32, z: f32, radius: f32) -> bool {
    let value = native!(
        bool,
        0xE33D59DA70B58FDF,
        native_parameters!(x, y, z, radius)
    );

    value
}

pub fn is_follow_ped_cam_active() -> bool {
    let value = native!(bool, 0xC6D3D26810C8E0F9, native_parameters!());

    value
}

pub fn set_follow_ped_cam_this_update(camName: &std::ffi::CString, p1: i32) -> bool {
    let value = native!(
        bool,
        0x44A113DD6FFC48D1,
        native_parameters!(camName.as_ptr(), p1)
    );

    value
}

pub fn _0x271401846bd26e92(p0: bool, p1: bool) -> () {
    let value = native!((), 0x271401846BD26E92, native_parameters!(p0, p1));

    value
}

pub fn _0xc8391c309684595a() -> () {
    let value = native!((), 0xC8391C309684595A, native_parameters!());

    value
}

pub fn _clamp_gameplay_cam_yaw(minimum: f32, maximum: f32) -> () {
    let value = native!((), 0x8F993D26E0CA5E8E, native_parameters!(minimum, maximum));

    value
}

pub fn _clamp_gameplay_cam_pitch(minimum: f32, maximum: f32) -> () {
    let value = native!((), 0xA516C198B7DCA1E1, native_parameters!(minimum, maximum));

    value
}

pub fn _animate_gameplay_cam_zoom(p0: f32, distance: f32) -> () {
    let value = native!((), 0xDF2E1F7742402E81, native_parameters!(p0, distance));

    value
}

pub fn set_in_vehicle_cam_state_this_update(p0: i32, p1: i32) -> () {
    let value = native!((), 0xE9EA16D6E54CDCA4, native_parameters!(p0, p1));

    value
}

pub fn _disable_first_person_cam_this_frame() -> () {
    let value = native!((), 0xDE2EF5DA284CC8DF, native_parameters!());

    value
}

pub fn _0x59424bd75174c9b1() -> () {
    let value = native!((), 0x59424BD75174C9B1, native_parameters!());

    value
}

pub fn _0x9f97da93681f87ea() -> () {
    let value = native!((), 0x9F97DA93681F87EA, native_parameters!());

    value
}

pub fn get_follow_ped_cam_zoom_level() -> i32 {
    let value = native!(i32, 0x33E6C8EFD0CD93E9, native_parameters!());

    value
}

pub fn get_follow_ped_cam_view_mode() -> i32 {
    let value = native!(i32, 0x8D4D46230B2C353A, native_parameters!());

    value
}

pub fn set_follow_ped_cam_view_mode(viewMode: i32) -> () {
    let value = native!((), 0x5A4F9EDF1673F704, native_parameters!(viewMode));

    value
}

pub fn is_follow_vehicle_cam_active() -> bool {
    let value = native!(bool, 0xCBBDE6D335D6D496, native_parameters!());

    value
}

pub fn _0x91ef6ee6419e5b97(p0: bool) -> () {
    let value = native!((), 0x91EF6EE6419E5B97, native_parameters!(p0));

    value
}

pub fn _0x9dfe13ecdc1ec196(p0: bool, p1: bool) -> () {
    let value = native!((), 0x9DFE13ECDC1EC196, native_parameters!(p0, p1));

    value
}

pub fn _0x79c0e43eb9b944e2(hash: u32) -> bool {
    let value = native!(bool, 0x79C0E43EB9B944E2, native_parameters!(hash));

    value
}

pub fn get_follow_vehicle_cam_zoom_level() -> i32 {
    let value = native!(i32, 0xEE82280AB767B690, native_parameters!());

    value
}

pub fn set_follow_vehicle_cam_zoom_level(zoomLevel: i32) -> () {
    let value = native!((), 0x19464CB6E4078C8A, native_parameters!(zoomLevel));

    value
}

pub fn get_follow_vehicle_cam_view_mode() -> i32 {
    let value = native!(i32, 0xA4FF579AC0E3AAAE, native_parameters!());

    value
}

pub fn set_follow_vehicle_cam_view_mode(viewMode: i32) -> () {
    let value = native!((), 0xAC253D7842768F48, native_parameters!(viewMode));

    value
}

pub fn _0xee778f8c7e1142e2(p0: u32) -> u32 {
    let value = native!(u32, 0xEE778F8C7E1142E2, native_parameters!(p0));

    value
}

pub fn _0x2a2173e46daecd12(p0: u32, p1: u32) -> () {
    let value = native!((), 0x2A2173E46DAECD12, native_parameters!(p0, p1));

    value
}

pub fn _0x19cafa3c87f7c2ff() -> u32 {
    let value = native!(u32, 0x19CAFA3C87F7C2FF, native_parameters!());

    value
}

pub fn _use_stunt_camera_this_frame() -> () {
    let value = native!((), 0x6493CF69859B116A, native_parameters!());

    value
}

pub fn _set_gameplay_cam_hash(camName: &std::ffi::CString) -> () {
    let value = native!((), 0x425A920FDB9A0DDA, native_parameters!(camName.as_ptr()));

    value
}

pub fn _0x0aa27680a0bd43fa() -> () {
    let value = native!((), 0x0AA27680A0BD43FA, native_parameters!());

    value
}

pub fn _set_follow_turret_seat_cam(seatIndex: i32) -> () {
    let value = native!((), 0x5C90CAB09951A12F, native_parameters!(seatIndex));

    value
}

pub fn is_aim_cam_active() -> bool {
    let value = native!(bool, 0x68EDDA28A5976D07, native_parameters!());

    value
}

pub fn _is_aim_cam_third_person_active() -> bool {
    let value = native!(bool, 0x74BD83EA840F6BC9, native_parameters!());

    value
}

pub fn is_first_person_aim_cam_active() -> bool {
    let value = native!(bool, 0x5E346D934122613F, native_parameters!());

    value
}

pub fn disable_aim_cam_this_update() -> () {
    let value = native!((), 0x1A31FE0049E542F6, native_parameters!());

    value
}

pub fn get_first_person_aim_cam_zoom_factor() -> f32 {
    let value = native!(f32, 0x7EC52CC40597D170, native_parameters!());

    value
}

pub fn set_first_person_aim_cam_zoom_factor(zoomFactor: f32) -> () {
    let value = native!((), 0x70894BD0915C5BCA, native_parameters!(zoomFactor));

    value
}

pub fn _0xced08cbe8ebb97c7(p0: f32, p1: f32) -> () {
    let value = native!((), 0xCED08CBE8EBB97C7, native_parameters!(p0, p1));

    value
}

pub fn _0x2f7f2b26dd3f18ee(p0: f32, p1: f32) -> () {
    let value = native!((), 0x2F7F2B26DD3F18EE, native_parameters!(p0, p1));

    value
}

pub fn _set_first_person_cam_pitch_range(p0: f32, p1: f32) -> () {
    let value = native!((), 0xBCFC632DB7673BF0, native_parameters!(p0, p1));

    value
}

pub fn set_first_person_aim_cam_near_clip_this_update(p0: f32) -> () {
    let value = native!((), 0x0AF7B437918103B3, native_parameters!(p0));

    value
}

pub fn set_third_person_aim_cam_near_clip_this_update(p0: f32) -> () {
    let value = native!((), 0x42156508606DE65E, native_parameters!(p0));

    value
}

pub fn _0x4008edf7d6e48175(p0: bool) -> () {
    let value = native!((), 0x4008EDF7D6E48175, native_parameters!(p0));

    value
}

pub fn _0x380b4968d1e09e55() -> () {
    let value = native!((), 0x380B4968D1E09E55, native_parameters!());

    value
}

pub fn get_final_rendered_cam_coord() -> NativeVector3 {
    let value = native!(NativeVector3, 0xA200EB1EE790F448, native_parameters!());

    value
}

pub fn get_final_rendered_cam_rot(rotationOrder: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x5B4E4C817FCC2DFB,
        native_parameters!(rotationOrder)
    );

    value
}

pub fn get_final_rendered_in_when_friendly_rot(player: i32, rotationOrder: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x26903D9CD1175F2C,
        native_parameters!(player, rotationOrder)
    );

    value
}

pub fn get_final_rendered_cam_fov() -> f32 {
    let value = native!(f32, 0x80EC114669DAEFF4, native_parameters!());

    value
}

pub fn get_final_rendered_in_when_friendly_fov(player: i32) -> f32 {
    let value = native!(f32, 0x5F35F6732C3FBBA0, native_parameters!(player));

    value
}

pub fn get_final_rendered_cam_near_clip() -> f32 {
    let value = native!(f32, 0xD0082607100D7193, native_parameters!());

    value
}

pub fn get_final_rendered_cam_far_clip() -> f32 {
    let value = native!(f32, 0xDFC8CBC606FDB0FC, native_parameters!());

    value
}

pub fn get_final_rendered_cam_near_dof() -> f32 {
    let value = native!(f32, 0xA03502FC581F7D9B, native_parameters!());

    value
}

pub fn get_final_rendered_cam_far_dof() -> f32 {
    let value = native!(f32, 0x9780F32BCAF72431, native_parameters!());

    value
}

pub fn get_final_rendered_cam_motion_blur_strength() -> f32 {
    let value = native!(f32, 0x162F9D995753DC19, native_parameters!());

    value
}

pub fn set_gameplay_coord_hint(
    x: f32,
    y: f32,
    z: f32,
    duration: i32,
    blendOutDuration: i32,
    blendInDuration: i32,
    unk: i32,
) -> () {
    let value = native!(
        (),
        0xD51ADCD2D8BC0FB3,
        native_parameters!(x, y, z, duration, blendOutDuration, blendInDuration, unk)
    );

    value
}

pub fn set_gameplay_ped_hint(
    p0: i32,
    x1: f32,
    y1: f32,
    z1: f32,
    p4: bool,
    p5: u32,
    p6: u32,
    p7: u32,
) -> () {
    let value = native!(
        (),
        0x2B486269ACD548D3,
        native_parameters!(p0, x1, y1, z1, p4, p5, p6, p7)
    );

    value
}

pub fn set_gameplay_vehicle_hint(
    vehicle: i32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    p4: bool,
    time: i32,
    easeInTime: i32,
    easeOutTime: i32,
) -> () {
    let value = native!(
        (),
        0xA2297E18F3E71C2E,
        native_parameters!(
            vehicle,
            offsetX,
            offsetY,
            offsetZ,
            p4,
            time,
            easeInTime,
            easeOutTime
        )
    );

    value
}

pub fn set_gameplay_object_hint(
    p0: u32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: bool,
    p5: u32,
    p6: u32,
    p7: u32,
) -> () {
    let value = native!(
        (),
        0x83E87508A2CA2AC6,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn set_gameplay_entity_hint(
    entity: i32,
    xOffset: f32,
    yOffset: f32,
    zOffset: f32,
    p4: bool,
    p5: i32,
    p6: i32,
    p7: i32,
    p8: u32,
) -> () {
    let value = native!(
        (),
        0x189E955A8313E298,
        native_parameters!(entity, xOffset, yOffset, zOffset, p4, p5, p6, p7, p8)
    );

    value
}

pub fn is_gameplay_hint_active() -> bool {
    let value = native!(bool, 0xE520FF1AD2785B40, native_parameters!());

    value
}

pub fn stop_gameplay_hint(p0: bool) -> () {
    let value = native!((), 0xF46C581C61718916, native_parameters!(p0));

    value
}

pub fn _0xccd078c2665d2973(p0: bool) -> () {
    let value = native!((), 0xCCD078C2665D2973, native_parameters!(p0));

    value
}

pub fn _0x247acbc4abbc9d1c(p0: bool) -> () {
    let value = native!((), 0x247ACBC4ABBC9D1C, native_parameters!(p0));

    value
}

pub fn _0xbf72910d0f26f025() -> u32 {
    let value = native!(u32, 0xBF72910D0F26F025, native_parameters!());

    value
}

pub fn set_gameplay_hint_fov(FOV: f32) -> () {
    let value = native!((), 0x513403FB9C56211F, native_parameters!(FOV));

    value
}

pub fn set_gameplay_hint_follow_distance_scalar(value: f32) -> () {
    let value = native!((), 0xF8BDBF3D573049A1, native_parameters!(value));

    value
}

pub fn set_gameplay_hint_base_orbit_pitch_offset(value: f32) -> () {
    let value = native!((), 0xD1F8363DFAD03848, native_parameters!(value));

    value
}

pub fn _set_gameplay_hint_anim_offsetx(xOffset: f32) -> () {
    let value = native!((), 0x5D7B620DAE436138, native_parameters!(xOffset));

    value
}

pub fn _set_gameplay_hint_anim_offsety(yOffset: f32) -> () {
    let value = native!((), 0xC92717EF615B6704, native_parameters!(yOffset));

    value
}

pub fn _set_gameplay_hint_anim_closeup(toggle: bool) -> () {
    let value = native!((), 0xE3433EADAAF7EE40, native_parameters!(toggle));

    value
}

pub fn set_cinematic_button_active(p0: bool) -> () {
    let value = native!((), 0x51669F7D1FB53D9F, native_parameters!(p0));

    value
}

pub fn is_cinematic_cam_rendering() -> bool {
    let value = native!(bool, 0xB15162CB5826E9E8, native_parameters!());

    value
}

pub fn shake_cinematic_cam(p0: &std::ffi::CString, p1: f32) -> () {
    let value = native!((), 0xDCE214D9ED58F3CF, native_parameters!(p0.as_ptr(), p1));

    value
}

pub fn is_cinematic_cam_shaking() -> bool {
    let value = native!(bool, 0xBBC08F6B4CB8FF0A, native_parameters!());

    value
}

pub fn set_cinematic_cam_shake_amplitude(p0: f32) -> () {
    let value = native!((), 0xC724C701C30B2FE7, native_parameters!(p0));

    value
}

pub fn stop_cinematic_cam_shaking(p0: bool) -> () {
    let value = native!((), 0x2238E588E588A6D7, native_parameters!(p0));

    value
}

pub fn _disable_vehicle_first_person_cam_this_frame() -> () {
    let value = native!((), 0xADFF1B2A555F5FBA, native_parameters!());

    value
}

pub fn _0x62ecfcfdee7885d6() -> () {
    let value = native!((), 0x62ECFCFDEE7885D6, native_parameters!());

    value
}

pub fn _invalidate_vehicle_idle_cam() -> () {
    let value = native!((), 0x9E4CFFF989258472, native_parameters!());

    value
}

pub fn invalidate_idle_cam() -> () {
    let value = native!((), 0xF4F2C0D4EE209E20, native_parameters!());

    value
}

pub fn is_cinematic_idle_cam_rendering() -> bool {
    let value = native!(bool, 0xCA9D2AA3E326D720, native_parameters!());

    value
}

pub fn _is_in_vehicle_cam_disabled() -> bool {
    let value = native!(bool, 0x4F32C0D5A90A9B40, native_parameters!());

    value
}

pub fn create_cinematic_shot(p0: u32, p1: i32, p2: u32, entity: i32) -> () {
    let value = native!(
        (),
        0x741B0129D4560F31,
        native_parameters!(p0, p1, p2, entity)
    );

    value
}

pub fn is_cinematic_shot_active(p0: u32) -> bool {
    let value = native!(bool, 0xCC9F3371A7C28BC9, native_parameters!(p0));

    value
}

pub fn stop_cinematic_shot(p0: u32) -> () {
    let value = native!((), 0x7660C6E75D3A078E, native_parameters!(p0));

    value
}

pub fn force_cinematic_rendering_this_update(p0: bool) -> () {
    let value = native!((), 0xA41BCD7213805AAC, native_parameters!(p0));

    value
}

pub fn _0xdc9da9e8789f5246() -> () {
    let value = native!((), 0xDC9DA9E8789F5246, native_parameters!());

    value
}

pub fn set_cinematic_mode_active(toggle: bool) -> () {
    let value = native!((), 0xDCF0754AC3D6FD4E, native_parameters!(toggle));

    value
}

pub fn _0x1f2300cb7fa7b7f6() -> u32 {
    let value = native!(u32, 0x1F2300CB7FA7B7F6, native_parameters!());

    value
}

pub fn _0x17fca7199a530203() -> u32 {
    let value = native!(u32, 0x17FCA7199A530203, native_parameters!());

    value
}

pub fn _0xd7360051c885628b() -> u32 {
    let value = native!(u32, 0xD7360051C885628B, native_parameters!());

    value
}

pub fn _is_cinematic_cam_active() -> bool {
    let value = native!(bool, 0xF5F1E89A970B7796, native_parameters!());

    value
}

pub fn _0x7b8a361c1813fbef() -> () {
    let value = native!((), 0x7B8A361C1813FBEF, native_parameters!());

    value
}

pub fn stop_cutscene_cam_shaking() -> () {
    let value = native!((), 0xDB629FFD9285FA06, native_parameters!());

    value
}

pub fn _0x324c5aa411da7737(p0: u32) -> () {
    let value = native!((), 0x324C5AA411DA7737, native_parameters!(p0));

    value
}

pub fn _0x12ded8ca53d47ea5(p0: f32) -> () {
    let value = native!((), 0x12DED8CA53D47EA5, native_parameters!(p0));

    value
}

pub fn get_focus_ped_on_screen(
    p0: f32,
    p1: i32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: i32,
    p8: i32,
) -> i32 {
    let value = native!(
        i32,
        0x89215EC747DF244A,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn _0x5a43c76f7fc7ba5f() -> () {
    let value = native!((), 0x5A43C76F7FC7BA5F, native_parameters!());

    value
}

pub fn _set_cam_effect(p0: i32) -> () {
    let value = native!((), 0x80C8B1846639BB19, native_parameters!(p0));

    value
}

pub fn _0x5c41e6babc9e2112(p0: u32) -> () {
    let value = native!((), 0x5C41E6BABC9E2112, native_parameters!(p0));

    value
}

pub fn _set_gameplay_cam_vehicle_camera(vehicleName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x21E253A7F8DA5DFB,
        native_parameters!(vehicleName.as_ptr())
    );

    value
}

pub fn _set_gameplay_cam_vehicle_camera_name(vehicleModel: u32) -> () {
    let value = native!((), 0x11FA5D3479C7DD47, native_parameters!(vehicleModel));

    value
}

pub fn _0xeaf0fa793d05c592() -> u32 {
    let value = native!(u32, 0xEAF0FA793D05C592, native_parameters!());

    value
}

pub fn _0x62374889a4d59f72() -> () {
    let value = native!((), 0x62374889A4D59F72, native_parameters!());

    value
}

pub fn _replay_free_cam_get_max_range() -> f32 {
    let value = native!(f32, 0x8BFCEB5EA1B161B6, native_parameters!());

    value
}
