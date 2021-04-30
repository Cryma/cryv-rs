use crate::types::NativeVector3;

pub fn task_pause(ped: i32, ms: i32) -> () {
    let value = native!((), 0xE73A266DB0CA9042, native_parameters!(ped, ms));

    value
}

pub fn task_stand_still(ped: i32, time: i32) -> () {
    let value = native!((), 0x919BE13EED931959, native_parameters!(ped, time));

    value
}

pub fn task_jump(ped: i32, unused: bool, p2: u32, p3: u32) -> () {
    let value = native!(
        (),
        0x0AE4086104E067B1,
        native_parameters!(ped, unused, p2, p3)
    );

    value
}

pub fn task_cower(ped: i32, duration: i32) -> () {
    let value = native!((), 0x3EB1FE9E8E908E15, native_parameters!(ped, duration));

    value
}

pub fn task_hands_up(ped: i32, duration: i32, facingPed: i32, p3: i32, p4: bool) -> () {
    let value = native!(
        (),
        0xF2EAB31979A7F910,
        native_parameters!(ped, duration, facingPed, p3, p4)
    );

    value
}

pub fn update_task_hands_up_duration(ped: i32, duration: i32) -> () {
    let value = native!((), 0xA98FCAFD7893C834, native_parameters!(ped, duration));

    value
}

pub fn task_open_vehicle_door(ped: i32, vehicle: i32, timeOut: i32, seat: i32, speed: f32) -> () {
    let value = native!(
        (),
        0x965791A9A488A062,
        native_parameters!(ped, vehicle, timeOut, seat, speed)
    );

    value
}

pub fn task_enter_vehicle(
    ped: i32,
    vehicle: i32,
    timeout: i32,
    seat: i32,
    speed: f32,
    flag: i32,
    p6: u32,
) -> () {
    let value = native!(
        (),
        0xC20E50AA46D09CA8,
        native_parameters!(ped, vehicle, timeout, seat, speed, flag, p6)
    );

    value
}

pub fn task_leave_vehicle(ped: i32, vehicle: i32, flags: i32) -> () {
    let value = native!(
        (),
        0xD3DBCE61A490BE02,
        native_parameters!(ped, vehicle, flags)
    );

    value
}

pub fn task_get_off_boat(ped: i32, boat: i32) -> () {
    let value = native!((), 0x9C00E77AF14B2DFF, native_parameters!(ped, boat));

    value
}

pub fn task_sky_dive(ped: i32, p1: bool) -> () {
    let value = native!((), 0x601736CFE536B0A0, native_parameters!(ped, p1));

    value
}

pub fn task_parachute(ped: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xD2F1C53C97EE81AB, native_parameters!(ped, p1, p2));

    value
}

pub fn task_parachute_to_target(ped: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xB33E291AFA6BD03A, native_parameters!(ped, x, y, z));

    value
}

pub fn set_parachute_task_target(ped: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xC313379AF0FCEDA7, native_parameters!(ped, x, y, z));

    value
}

pub fn set_parachute_task_thrust(ped: i32, thrust: f32) -> () {
    let value = native!((), 0x0729BAC1B8C64317, native_parameters!(ped, thrust));

    value
}

pub fn task_rappel_from_heli(ped: i32, p1: f32) -> () {
    let value = native!((), 0x09693B0312F91649, native_parameters!(ped, p1));

    value
}

pub fn task_vehicle_drive_to_coord(
    ped: i32,
    vehicle: i32,
    x: f32,
    y: f32,
    z: f32,
    speed: f32,
    p6: u32,
    vehicleModel: u32,
    drivingMode: i32,
    stopRange: f32,
    p10: f32,
) -> () {
    let value = native!(
        (),
        0xE2A2AA2F659D77A7,
        native_parameters!(
            ped,
            vehicle,
            x,
            y,
            z,
            speed,
            p6,
            vehicleModel,
            drivingMode,
            stopRange,
            p10
        )
    );

    value
}

pub fn task_vehicle_drive_to_coord_longrange(
    ped: i32,
    vehicle: i32,
    x: f32,
    y: f32,
    z: f32,
    speed: f32,
    driveMode: i32,
    stopRange: f32,
) -> () {
    let value = native!(
        (),
        0x158BB33F920D360C,
        native_parameters!(ped, vehicle, x, y, z, speed, driveMode, stopRange)
    );

    value
}

pub fn task_vehicle_drive_wander(ped: i32, vehicle: i32, speed: f32, drivingStyle: i32) -> () {
    let value = native!(
        (),
        0x480142959D337D00,
        native_parameters!(ped, vehicle, speed, drivingStyle)
    );

    value
}

pub fn task_follow_to_offset_of_entity(
    ped: i32,
    entity: i32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    movementSpeed: f32,
    timeout: i32,
    stoppingRange: f32,
    persistFollowing: bool,
) -> () {
    let value = native!(
        (),
        0x304AE42E357B8C7E,
        native_parameters!(
            ped,
            entity,
            offsetX,
            offsetY,
            offsetZ,
            movementSpeed,
            timeout,
            stoppingRange,
            persistFollowing
        )
    );

    value
}

pub fn task_go_straight_to_coord(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    speed: f32,
    timeout: i32,
    targetHeading: f32,
    distanceToSlide: f32,
) -> () {
    let value = native!(
        (),
        0xD76B57B44F1E6F8B,
        native_parameters!(ped, x, y, z, speed, timeout, targetHeading, distanceToSlide)
    );

    value
}

pub fn task_go_straight_to_coord_relative_to_entity(
    entity1: i32,
    entity2: i32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: u32,
) -> () {
    let value = native!(
        (),
        0x61E360B7E040D12E,
        native_parameters!(entity1, entity2, p2, p3, p4, p5, p6)
    );

    value
}

pub fn task_achieve_heading(ped: i32, heading: f32, timeout: i32) -> () {
    let value = native!(
        (),
        0x93B93A37987F1F3D,
        native_parameters!(ped, heading, timeout)
    );

    value
}

pub fn task_flush_route() -> () {
    let value = native!((), 0x841142A1376E9006, native_parameters!());

    value
}

pub fn task_extend_route(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x1E7889778264843A, native_parameters!(x, y, z));

    value
}

pub fn task_follow_point_route(ped: i32, speed: f32, unknown: i32) -> () {
    let value = native!(
        (),
        0x595583281858626E,
        native_parameters!(ped, speed, unknown)
    );

    value
}

pub fn task_go_to_entity(
    entity: i32,
    target: i32,
    duration: i32,
    distance: f32,
    speed: f32,
    p5: f32,
    p6: i32,
) -> () {
    let value = native!(
        (),
        0x6A071245EB0D1882,
        native_parameters!(entity, target, duration, distance, speed, p5, p6)
    );

    value
}

pub fn task_smart_flee_coord(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    distance: f32,
    time: i32,
    p6: bool,
    p7: bool,
) -> () {
    let value = native!(
        (),
        0x94587F17E9C365D5,
        native_parameters!(ped, x, y, z, distance, time, p6, p7)
    );

    value
}

pub fn task_smart_flee_ped(
    ped: i32,
    fleeTarget: i32,
    distance: f32,
    fleeTime: u32,
    p4: bool,
    p5: bool,
) -> () {
    let value = native!(
        (),
        0x22B0D0E37CCB840D,
        native_parameters!(ped, fleeTarget, distance, fleeTime, p4, p5)
    );

    value
}

pub fn task_react_and_flee_ped(ped: i32, fleeTarget: i32) -> () {
    let value = native!((), 0x72C896464915D1B1, native_parameters!(ped, fleeTarget));

    value
}

pub fn task_shocking_event_react(ped: i32, eventHandle: i32) -> () {
    let value = native!((), 0x452419CBD838065B, native_parameters!(ped, eventHandle));

    value
}

pub fn task_wander_in_area(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    minimalLength: f32,
    timeBetweenWalks: f32,
) -> () {
    let value = native!(
        (),
        0xE054346CA3A0F315,
        native_parameters!(ped, x, y, z, radius, minimalLength, timeBetweenWalks)
    );

    value
}

pub fn task_wander_standard(ped: i32, p1: f32, p2: i32) -> () {
    let value = native!((), 0xBB9CE077274F6A1B, native_parameters!(ped, p1, p2));

    value
}

pub fn task_wander_specific(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x6919A2F136426098, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn task_vehicle_park(
    ped: i32,
    vehicle: i32,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    mode: i32,
    radius: f32,
    keepEngineOn: bool,
) -> () {
    let value = native!(
        (),
        0x0F3E34E968EA374E,
        native_parameters!(ped, vehicle, x, y, z, heading, mode, radius, keepEngineOn)
    );

    value
}

pub fn task_stealth_kill(killer: i32, target: i32, actionType: u32, p3: f32, p4: u32) -> () {
    let value = native!(
        (),
        0xAA5DC05579D60BD9,
        native_parameters!(killer, target, actionType, p3, p4)
    );

    value
}

pub fn task_plant_bomb(ped: i32, x: f32, y: f32, z: f32, heading: f32) -> () {
    let value = native!(
        (),
        0x965FEC691D55E9BF,
        native_parameters!(ped, x, y, z, heading)
    );

    value
}

pub fn task_follow_nav_mesh_to_coord(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    speed: f32,
    timeout: i32,
    stoppingRange: f32,
    persistFollowing: bool,
    unk: f32,
) -> () {
    let value = native!(
        (),
        0x15D3A79D4E44B913,
        native_parameters!(
            ped,
            x,
            y,
            z,
            speed,
            timeout,
            stoppingRange,
            persistFollowing,
            unk
        )
    );

    value
}

pub fn task_follow_nav_mesh_to_coord_advanced(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    speed: f32,
    timeout: i32,
    unkFloat: f32,
    unkInt: i32,
    unkX: f32,
    unkY: f32,
    unkZ: f32,
    unk_40000f: f32,
) -> () {
    let value = native!(
        (),
        0x17F58B88D085DBAC,
        native_parameters!(
            ped, x, y, z, speed, timeout, unkFloat, unkInt, unkX, unkY, unkZ, unk_40000f
        )
    );

    value
}

pub fn set_ped_path_can_use_climbovers(ped: i32, Toggle: bool) -> () {
    let value = native!((), 0x8E06A6FE76C9EFF4, native_parameters!(ped, Toggle));

    value
}

pub fn set_ped_path_can_use_ladders(ped: i32, Toggle: bool) -> () {
    let value = native!((), 0x77A5B103C87F476E, native_parameters!(ped, Toggle));

    value
}

pub fn set_ped_path_can_drop_from_height(ped: i32, Toggle: bool) -> () {
    let value = native!((), 0xE361C5C71C431A4F, native_parameters!(ped, Toggle));

    value
}

pub fn set_ped_path_climb_cost_modifier(ped: i32, modifier: f32) -> () {
    let value = native!((), 0x88E32DB8C1A4AA4B, native_parameters!(ped, modifier));

    value
}

pub fn set_ped_path_may_enter_water(ped: i32, mayEnterWater: bool) -> () {
    let value = native!(
        (),
        0xF35425A4204367EC,
        native_parameters!(ped, mayEnterWater)
    );

    value
}

pub fn set_ped_path_prefer_to_avoid_water(ped: i32, avoidWater: bool) -> () {
    let value = native!((), 0x38FE1EC73743793C, native_parameters!(ped, avoidWater));

    value
}

pub fn set_ped_path_avoid_fire(ped: i32, avoidFire: bool) -> () {
    let value = native!((), 0x4455517B28441E60, native_parameters!(ped, avoidFire));

    value
}

pub fn set_global_min_bird_flight_height(height: f32) -> () {
    let value = native!((), 0x6C6B148586F934F7, native_parameters!(height));

    value
}

pub fn get_navmesh_route_distance_remaining(
    ped: i32,
    distanceRemaining: *mut f32,
    isPathReady: *mut bool,
) -> i32 {
    let value = native!(
        i32,
        0xC6F5C0BCDC74D62D,
        native_parameters!(ped, distanceRemaining, isPathReady)
    );

    value
}

pub fn get_navmesh_route_result(ped: i32) -> i32 {
    let value = native!(i32, 0x632E831F382A0FA8, native_parameters!(ped));

    value
}

pub fn _0x3e38e28a1d80ddf6(ped: i32) -> bool {
    let value = native!(bool, 0x3E38E28A1D80DDF6, native_parameters!(ped));

    value
}

pub fn task_go_to_coord_any_means(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    speed: f32,
    p5: u32,
    p6: bool,
    walkingStyle: i32,
    p8: f32,
) -> () {
    let value = native!(
        (),
        0x5BC448CB78FA3E88,
        native_parameters!(ped, x, y, z, speed, p5, p6, walkingStyle, p8)
    );

    value
}

pub fn task_go_to_coord_any_means_extra_params(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    speed: f32,
    p5: u32,
    p6: bool,
    walkingStyle: i32,
    p8: f32,
    p9: u32,
    p10: u32,
    p11: u32,
    p12: u32,
) -> () {
    let value = native!(
        (),
        0x1DD45F9ECFDB1BC9,
        native_parameters!(
            ped,
            x,
            y,
            z,
            speed,
            p5,
            p6,
            walkingStyle,
            p8,
            p9,
            p10,
            p11,
            p12
        )
    );

    value
}

pub fn task_go_to_coord_any_means_extra_params_with_cruise_speed(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    speed: f32,
    p5: u32,
    p6: bool,
    walkingStyle: i32,
    p8: f32,
    p9: u32,
    p10: u32,
    p11: u32,
    p12: u32,
    p13: u32,
) -> () {
    let value = native!(
        (),
        0xB8ECD61F531A7B02,
        native_parameters!(
            ped,
            x,
            y,
            z,
            speed,
            p5,
            p6,
            walkingStyle,
            p8,
            p9,
            p10,
            p11,
            p12,
            p13
        )
    );

    value
}

pub fn task_play_anim(
    ped: i32,
    animDictionary: &std::ffi::CString,
    animationName: &std::ffi::CString,
    blendInSpeed: f32,
    blendOutSpeed: f32,
    duration: i32,
    flag: i32,
    playbackRate: f32,
    lockX: bool,
    lockY: bool,
    lockZ: bool,
) -> () {
    let value = native!(
        (),
        0xEA47FE3719165B94,
        native_parameters!(
            ped,
            animDictionary.as_ptr(),
            animationName.as_ptr(),
            blendInSpeed,
            blendOutSpeed,
            duration,
            flag,
            playbackRate,
            lockX,
            lockY,
            lockZ
        )
    );

    value
}

pub fn task_play_anim_advanced(
    ped: i32,
    animDict: &std::ffi::CString,
    animName: &std::ffi::CString,
    posX: f32,
    posY: f32,
    posZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    animEnterSpeed: f32,
    animExitSpeed: f32,
    duration: i32,
    flag: u32,
    animTime: f32,
    p14: u32,
    p15: u32,
) -> () {
    let value = native!(
        (),
        0x83CDB10EA29B370B,
        native_parameters!(
            ped,
            animDict.as_ptr(),
            animName.as_ptr(),
            posX,
            posY,
            posZ,
            rotX,
            rotY,
            rotZ,
            animEnterSpeed,
            animExitSpeed,
            duration,
            flag,
            animTime,
            p14,
            p15
        )
    );

    value
}

pub fn stop_anim_task(
    ped: i32,
    animDictionary: &std::ffi::CString,
    animationName: &std::ffi::CString,
    p3: f32,
) -> () {
    let value = native!(
        (),
        0x97FF36A1D40EA00A,
        native_parameters!(ped, animDictionary.as_ptr(), animationName.as_ptr(), p3)
    );

    value
}

pub fn task_scripted_animation(
    ped: i32,
    p1: *mut u32,
    p2: *mut u32,
    p3: *mut u32,
    p4: f32,
    p5: f32,
) -> () {
    let value = native!(
        (),
        0x126EF75F1E17ABE5,
        native_parameters!(ped, p1, p2, p3, p4, p5)
    );

    value
}

pub fn play_entity_scripted_anim(
    p0: u32,
    p1: *mut u32,
    p2: *mut u32,
    p3: *mut u32,
    p4: f32,
    p5: f32,
) -> () {
    let value = native!(
        (),
        0x77A1EEC547E7FCF1,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn stop_anim_playback(ped: i32, p1: i32, p2: bool) -> () {
    let value = native!((), 0xEE08C992D238C5D1, native_parameters!(ped, p1, p2));

    value
}

pub fn set_anim_weight(p0: u32, p1: f32, p2: u32, p3: u32, p4: bool) -> () {
    let value = native!(
        (),
        0x207F1A47C0342F48,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn set_anim_rate(p0: u32, p1: f32, p2: u32, p3: bool) -> () {
    let value = native!((), 0x032D49C5E359C847, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn set_anim_looped(p0: u32, p1: bool, p2: u32, p3: bool) -> () {
    let value = native!((), 0x70033C3CC29A1FF4, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn task_play_phone_gesture_animation(
    ped: i32,
    animDict: &std::ffi::CString,
    animation: &std::ffi::CString,
    boneMaskType: &std::ffi::CString,
    p4: f32,
    p5: f32,
    p6: bool,
    p7: bool,
) -> () {
    let value = native!(
        (),
        0x8FBB6758B3B3E9EC,
        native_parameters!(
            ped,
            animDict.as_ptr(),
            animation.as_ptr(),
            boneMaskType.as_ptr(),
            p4,
            p5,
            p6,
            p7
        )
    );

    value
}

pub fn task_stop_phone_gesture_animation(ped: i32, p1: u32) -> () {
    let value = native!((), 0x3FA00D4F4641BFAE, native_parameters!(ped, p1));

    value
}

pub fn is_playing_phone_gesture_anim(ped: i32) -> bool {
    let value = native!(bool, 0xB8EBB1E9D3588C10, native_parameters!(ped));

    value
}

pub fn get_phone_gesture_anim_current_time(ped: i32) -> f32 {
    let value = native!(f32, 0x47619ABE8B268C60, native_parameters!(ped));

    value
}

pub fn get_phone_gesture_anim_total_time(ped: i32) -> f32 {
    let value = native!(f32, 0x1EE0F68A7C25DEC6, native_parameters!(ped));

    value
}

pub fn task_vehicle_play_anim(
    vehicle: i32,
    animationSet: &std::ffi::CString,
    animationName: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0x69F5C3BD0F3EBD89,
        native_parameters!(vehicle, animationSet.as_ptr(), animationName.as_ptr())
    );

    value
}

pub fn task_look_at_coord(
    entity: i32,
    x: f32,
    y: f32,
    z: f32,
    duration: i32,
    p5: u32,
    p6: u32,
) -> () {
    let value = native!(
        (),
        0x6FA46612594F7973,
        native_parameters!(entity, x, y, z, duration, p5, p6)
    );

    value
}

pub fn task_look_at_entity(
    ped: i32,
    lookAt: i32,
    duration: i32,
    unknown1: i32,
    unknown2: i32,
) -> () {
    let value = native!(
        (),
        0x69F4BE8C8CC4796C,
        native_parameters!(ped, lookAt, duration, unknown1, unknown2)
    );

    value
}

pub fn task_clear_look_at(ped: i32) -> () {
    let value = native!((), 0x0F804F1DB19B9689, native_parameters!(ped));

    value
}

pub fn open_sequence_task(taskSequenceId: *mut i32) -> () {
    let value = native!((), 0xE8854A4326B9E12B, native_parameters!(taskSequenceId));

    value
}

pub fn close_sequence_task(taskSequenceId: i32) -> () {
    let value = native!((), 0x39E72BC99E6360CB, native_parameters!(taskSequenceId));

    value
}

pub fn task_perform_sequence(ped: i32, taskSequenceId: i32) -> () {
    let value = native!(
        (),
        0x5ABA3986D90D8A3B,
        native_parameters!(ped, taskSequenceId)
    );

    value
}

pub fn task_perform_sequence_locally(ped: i32, taskSequenceId: i32) -> () {
    let value = native!(
        (),
        0x8C33220C8D78CA0D,
        native_parameters!(ped, taskSequenceId)
    );

    value
}

pub fn clear_sequence_task(taskSequenceId: *mut i32) -> () {
    let value = native!((), 0x3841422E9C488D8C, native_parameters!(taskSequenceId));

    value
}

pub fn set_sequence_to_repeat(taskSequenceId: i32, repeat: bool) -> () {
    let value = native!(
        (),
        0x58C70CF3A41E4AE7,
        native_parameters!(taskSequenceId, repeat)
    );

    value
}

pub fn get_sequence_progress(ped: i32) -> i32 {
    let value = native!(i32, 0x00A9010CFE1E3533, native_parameters!(ped));

    value
}

pub fn get_is_task_active(ped: i32, taskIndex: i32) -> bool {
    let value = native!(bool, 0xB0760331C7AA4155, native_parameters!(ped, taskIndex));

    value
}

pub fn get_script_task_status(ped: i32, taskHash: u32) -> i32 {
    let value = native!(i32, 0x77F1BEB8863288D5, native_parameters!(ped, taskHash));

    value
}

pub fn get_active_vehicle_mission_type(vehicle: i32) -> i32 {
    let value = native!(i32, 0x534AEBA6E5ED4CAB, native_parameters!(vehicle));

    value
}

pub fn task_leave_any_vehicle(ped: i32, p1: i32, flags: i32) -> () {
    let value = native!((), 0x504D54DF3F6F2247, native_parameters!(ped, p1, flags));

    value
}

pub fn task_aim_gun_scripted(ped: i32, scriptTask: u32, p2: bool, p3: bool) -> () {
    let value = native!(
        (),
        0x7A192BE16D373D00,
        native_parameters!(ped, scriptTask, p2, p3)
    );

    value
}

pub fn task_aim_gun_scripted_with_target(
    p0: u32,
    p1: u32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: u32,
    p6: bool,
    p7: bool,
) -> () {
    let value = native!(
        (),
        0x8605AF0DE8B3A5AC,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn update_task_aim_gun_scripted_target(
    p0: i32,
    p1: i32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: bool,
) -> () {
    let value = native!(
        (),
        0x9724FB59A3E72AD0,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn get_clip_set_for_scripted_gun_task(p0: i32) -> String {
    let value = native!(*const i8, 0x3A8CADC7D37AACC5, native_parameters!(p0));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn task_aim_gun_at_entity(ped: i32, entity: i32, duration: i32, p3: bool) -> () {
    let value = native!(
        (),
        0x9B53BB6E8943AF53,
        native_parameters!(ped, entity, duration, p3)
    );

    value
}

pub fn task_turn_ped_to_face_entity(ped: i32, entity: i32, duration: i32) -> () {
    let value = native!(
        (),
        0x5AD23D40115353AC,
        native_parameters!(ped, entity, duration)
    );

    value
}

pub fn task_aim_gun_at_coord(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    time: i32,
    p5: bool,
    p6: bool,
) -> () {
    let value = native!(
        (),
        0x6671F3EEC681BDA1,
        native_parameters!(ped, x, y, z, time, p5, p6)
    );

    value
}

pub fn task_shoot_at_coord(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    duration: i32,
    firingPattern: u32,
) -> () {
    let value = native!(
        (),
        0x46A6CC01E0826106,
        native_parameters!(ped, x, y, z, duration, firingPattern)
    );

    value
}

pub fn task_shuffle_to_next_vehicle_seat(ped: i32, vehicle: i32, p2: u32) -> () {
    let value = native!((), 0x7AA80209BDA643EB, native_parameters!(ped, vehicle, p2));

    value
}

pub fn clear_ped_tasks(ped: i32) -> () {
    let value = native!((), 0xE1EF3C1216AFF2CD, native_parameters!(ped));

    value
}

pub fn clear_ped_secondary_task(ped: i32) -> () {
    let value = native!((), 0x176CECF6F920D707, native_parameters!(ped));

    value
}

pub fn task_everyone_leave_vehicle(vehicle: i32) -> () {
    let value = native!((), 0x7F93691AB4B92272, native_parameters!(vehicle));

    value
}

pub fn task_goto_entity_offset(
    ped: i32,
    p1: u32,
    p2: u32,
    x: f32,
    y: f32,
    z: f32,
    duration: i32,
) -> () {
    let value = native!(
        (),
        0xE39B4FF4FDEBDE27,
        native_parameters!(ped, p1, p2, x, y, z, duration)
    );

    value
}

pub fn task_goto_entity_offset_xy(
    p0: i32,
    oed: i32,
    duration: i32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: bool,
) -> () {
    let value = native!(
        (),
        0x338E7EF52B6095A9,
        native_parameters!(p0, oed, duration, p3, p4, p5, p6, p7)
    );

    value
}

pub fn task_turn_ped_to_face_coord(ped: i32, x: f32, y: f32, z: f32, duration: i32) -> () {
    let value = native!(
        (),
        0x1DDA930A0AC38571,
        native_parameters!(ped, x, y, z, duration)
    );

    value
}

pub fn task_vehicle_temp_action(driver: i32, vehicle: i32, action: i32, time: i32) -> () {
    let value = native!(
        (),
        0xC429DCEEB339E129,
        native_parameters!(driver, vehicle, action, time)
    );

    value
}

pub fn task_vehicle_mission(
    driver: i32,
    vehicle: i32,
    vehicleTarget: i32,
    missionType: i32,
    p4: f32,
    p5: u32,
    p6: f32,
    p7: f32,
    DriveAgainstTraffic: bool,
) -> () {
    let value = native!(
        (),
        0x659427E0EF36BCDE,
        native_parameters!(
            driver,
            vehicle,
            vehicleTarget,
            missionType,
            p4,
            p5,
            p6,
            p7,
            DriveAgainstTraffic
        )
    );

    value
}

pub fn task_vehicle_mission_ped_target(
    ped: i32,
    vehicle: i32,
    pedTarget: i32,
    missionType: i32,
    maxSpeed: f32,
    drivingStyle: i32,
    minDistance: f32,
    p7: f32,
    DriveAgainstTraffic: bool,
) -> () {
    let value = native!(
        (),
        0x9454528DF15D657A,
        native_parameters!(
            ped,
            vehicle,
            pedTarget,
            missionType,
            maxSpeed,
            drivingStyle,
            minDistance,
            p7,
            DriveAgainstTraffic
        )
    );

    value
}

pub fn task_vehicle_mission_coors_target(
    ped: i32,
    vehicle: i32,
    x: f32,
    y: f32,
    z: f32,
    p5: i32,
    p6: i32,
    p7: i32,
    p8: f32,
    p9: f32,
    DriveAgainstTraffic: bool,
) -> () {
    let value = native!(
        (),
        0xF0AF20AA7731F8C3,
        native_parameters!(
            ped,
            vehicle,
            x,
            y,
            z,
            p5,
            p6,
            p7,
            p8,
            p9,
            DriveAgainstTraffic
        )
    );

    value
}

pub fn task_vehicle_escort(
    ped: i32,
    vehicle: i32,
    targetVehicle: i32,
    mode: i32,
    speed: f32,
    drivingStyle: i32,
    minDistance: f32,
    p7: i32,
    noRoadsDistance: f32,
) -> () {
    let value = native!(
        (),
        0x0FA6E4B75F302400,
        native_parameters!(
            ped,
            vehicle,
            targetVehicle,
            mode,
            speed,
            drivingStyle,
            minDistance,
            p7,
            noRoadsDistance
        )
    );

    value
}

pub fn task_vehicle_follow(
    driver: i32,
    vehicle: i32,
    targetEntity: i32,
    speed: f32,
    drivingStyle: i32,
    minDistance: i32,
) -> () {
    let value = native!(
        (),
        0xFC545A9F0626E3B6,
        native_parameters!(
            driver,
            vehicle,
            targetEntity,
            speed,
            drivingStyle,
            minDistance
        )
    );

    value
}

pub fn task_vehicle_chase(driver: i32, targetEnt: i32) -> () {
    let value = native!(
        (),
        0x3C08A8E30363B353,
        native_parameters!(driver, targetEnt)
    );

    value
}

pub fn task_vehicle_heli_protect(
    pilot: i32,
    vehicle: i32,
    entityToFollow: i32,
    targetSpeed: f32,
    p4: i32,
    radius: f32,
    altitude: i32,
    p7: i32,
) -> () {
    let value = native!(
        (),
        0x1E09C32048FEFD1C,
        native_parameters!(
            pilot,
            vehicle,
            entityToFollow,
            targetSpeed,
            p4,
            radius,
            altitude,
            p7
        )
    );

    value
}

pub fn set_task_vehicle_chase_behavior_flag(ped: i32, flag: i32, set: bool) -> () {
    let value = native!((), 0xCC665AAC360D31E7, native_parameters!(ped, flag, set));

    value
}

pub fn set_task_vehicle_chase_ideal_pursuit_distance(ped: i32, distance: f32) -> () {
    let value = native!((), 0x639B642FACBE4EDD, native_parameters!(ped, distance));

    value
}

pub fn task_heli_chase(pilot: i32, entityToFollow: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!(
        (),
        0xAC83B1DB38D0ADA0,
        native_parameters!(pilot, entityToFollow, x, y, z)
    );

    value
}

pub fn task_plane_chase(pilot: i32, entityToFollow: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!(
        (),
        0x2D2386F273FF7A25,
        native_parameters!(pilot, entityToFollow, x, y, z)
    );

    value
}

pub fn task_plane_land(
    pilot: i32,
    plane: i32,
    runwayStartX: f32,
    runwayStartY: f32,
    runwayStartZ: f32,
    runwayEndX: f32,
    runwayEndY: f32,
    runwayEndZ: f32,
) -> () {
    let value = native!(
        (),
        0xBF19721FA34D32C0,
        native_parameters!(
            pilot,
            plane,
            runwayStartX,
            runwayStartY,
            runwayStartZ,
            runwayEndX,
            runwayEndY,
            runwayEndZ
        )
    );

    value
}

pub fn _0x6100b3cefd43452e(p0: u32) -> () {
    let value = native!((), 0x6100B3CEFD43452E, native_parameters!(p0));

    value
}

pub fn _clear_vehicle_tasks(vehicle: i32) -> () {
    let value = native!((), 0xDBBC7A2432524127, native_parameters!(vehicle));

    value
}

pub fn _0x53ddc75bc3ac0a90(vehicle: i32) -> () {
    let value = native!((), 0x53DDC75BC3AC0A90, native_parameters!(vehicle));

    value
}

pub fn task_plane_goto_precise_vtol(
    ped: i32,
    vehicle: i32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
    p8: u32,
    p9: u32,
) -> () {
    let value = native!(
        (),
        0xF7F9DCCA89E7505B,
        native_parameters!(ped, vehicle, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn task_submarine_goto_and_stop(
    p0: u32,
    submarine: i32,
    x: f32,
    y: f32,
    z: f32,
    p5: u32,
) -> () {
    let value = native!(
        (),
        0xC22B40579A498CA4,
        native_parameters!(p0, submarine, x, y, z, p5)
    );

    value
}

pub fn task_heli_mission(
    pilot: i32,
    aircraft: i32,
    targetVehicle: i32,
    targetPed: i32,
    destinationX: f32,
    destinationY: f32,
    destinationZ: f32,
    missionFlag: i32,
    maxSpeed: f32,
    landingRadius: f32,
    targetHeading: f32,
    unk1: i32,
    unk2: i32,
    unk3: u32,
    landingFlags: i32,
) -> () {
    let value = native!(
        (),
        0xDAD029E187A2BEB4,
        native_parameters!(
            pilot,
            aircraft,
            targetVehicle,
            targetPed,
            destinationX,
            destinationY,
            destinationZ,
            missionFlag,
            maxSpeed,
            landingRadius,
            targetHeading,
            unk1,
            unk2,
            unk3,
            landingFlags
        )
    );

    value
}

pub fn task_heli_escort_heli(pilot: i32, heli1: i32, heli2: i32, p3: f32, p4: f32, p5: f32) -> () {
    let value = native!(
        (),
        0xB385523325077210,
        native_parameters!(pilot, heli1, heli2, p3, p4, p5)
    );

    value
}

pub fn task_plane_mission(
    pilot: i32,
    aircraft: i32,
    targetVehicle: i32,
    targetPed: i32,
    destinationX: f32,
    destinationY: f32,
    destinationZ: f32,
    missionFlag: i32,
    angularDrag: f32,
    unk: f32,
    targetHeading: f32,
    maxZ: f32,
    minZ: f32,
    p13: u32,
) -> () {
    let value = native!(
        (),
        0x23703CD154E83B88,
        native_parameters!(
            pilot,
            aircraft,
            targetVehicle,
            targetPed,
            destinationX,
            destinationY,
            destinationZ,
            missionFlag,
            angularDrag,
            unk,
            targetHeading,
            maxZ,
            minZ,
            p13
        )
    );

    value
}

pub fn task_plane_taxi(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32, p6: u32) -> () {
    let value = native!(
        (),
        0x92C360B5F15D2302,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn task_boat_mission(
    pedDriver: i32,
    boat: i32,
    p2: u32,
    p3: u32,
    x: f32,
    y: f32,
    z: f32,
    p7: u32,
    maxSpeed: f32,
    drivingStyle: i32,
    p10: f32,
    p11: u32,
) -> () {
    let value = native!(
        (),
        0x15C86013127CE63F,
        native_parameters!(
            pedDriver,
            boat,
            p2,
            p3,
            x,
            y,
            z,
            p7,
            maxSpeed,
            drivingStyle,
            p10,
            p11
        )
    );

    value
}

pub fn task_drive_by(
    driverPed: i32,
    targetPed: i32,
    targetVehicle: i32,
    targetX: f32,
    targetY: f32,
    targetZ: f32,
    distanceToShoot: f32,
    pedAccuracy: i32,
    p8: bool,
    firingPattern: u32,
) -> () {
    let value = native!(
        (),
        0x2F8AF0E82773A171,
        native_parameters!(
            driverPed,
            targetPed,
            targetVehicle,
            targetX,
            targetY,
            targetZ,
            distanceToShoot,
            pedAccuracy,
            p8,
            firingPattern
        )
    );

    value
}

pub fn set_driveby_task_target(
    shootingPed: i32,
    targetPed: i32,
    targetVehicle: i32,
    x: f32,
    y: f32,
    z: f32,
) -> () {
    let value = native!(
        (),
        0xE5B302114D8162EE,
        native_parameters!(shootingPed, targetPed, targetVehicle, x, y, z)
    );

    value
}

pub fn clear_driveby_task_underneath_driving_task(ped: i32) -> () {
    let value = native!((), 0xC35B5CDB2824CF69, native_parameters!(ped));

    value
}

pub fn is_driveby_task_underneath_driving_task(ped: i32) -> bool {
    let value = native!(bool, 0x8785E6E40C7A8818, native_parameters!(ped));

    value
}

pub fn control_mounted_weapon(ped: i32) -> bool {
    let value = native!(bool, 0xDCFE42068FE0135A, native_parameters!(ped));

    value
}

pub fn set_mounted_weapon_target(
    shootingPed: i32,
    targetPed: i32,
    targetVehicle: i32,
    x: f32,
    y: f32,
    z: f32,
    p6: u32,
    p7: u32,
) -> () {
    let value = native!(
        (),
        0xCCD892192C6D2BB9,
        native_parameters!(shootingPed, targetPed, targetVehicle, x, y, z, p6, p7)
    );

    value
}

pub fn is_mounted_weapon_task_underneath_driving_task(ped: i32) -> bool {
    let value = native!(bool, 0xA320EF046186FA3B, native_parameters!(ped));

    value
}

pub fn task_use_mobile_phone(ped: i32, p1: i32, p2: u32) -> () {
    let value = native!((), 0xBD2A8EC3AF4DE7DB, native_parameters!(ped, p1, p2));

    value
}

pub fn task_use_mobile_phone_timed(ped: i32, duration: i32) -> () {
    let value = native!((), 0x5EE02954A14C69DB, native_parameters!(ped, duration));

    value
}

pub fn task_chat_to_ped(
    ped: i32,
    target: i32,
    p2: u32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
) -> () {
    let value = native!(
        (),
        0x8C338E0263E4FD19,
        native_parameters!(ped, target, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn task_warp_ped_into_vehicle(ped: i32, vehicle: i32, seat: i32) -> () {
    let value = native!(
        (),
        0x9A7D091411C5F684,
        native_parameters!(ped, vehicle, seat)
    );

    value
}

pub fn task_shoot_at_entity(entity: i32, target: i32, duration: i32, firingPattern: u32) -> () {
    let value = native!(
        (),
        0x08DA95E8298AE772,
        native_parameters!(entity, target, duration, firingPattern)
    );

    value
}

pub fn task_climb(ped: i32, unused: bool) -> () {
    let value = native!((), 0x89D9FCC2435112F1, native_parameters!(ped, unused));

    value
}

pub fn task_climb_ladder(ped: i32, p1: i32) -> () {
    let value = native!((), 0xB6C987F9285A3814, native_parameters!(ped, p1));

    value
}

pub fn task_rappel_down_wall(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
    p8: u32,
    p9: u32,
    p10: u32,
) -> () {
    let value = native!(
        (),
        0xEAF66ACDDC794793,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10)
    );

    value
}

pub fn _0x9d252648778160df(p0: u32) -> u32 {
    let value = native!(u32, 0x9D252648778160DF, native_parameters!(p0));

    value
}

pub fn clear_ped_tasks_immediately(ped: i32) -> () {
    let value = native!((), 0xAAA34F8A7CB32098, native_parameters!(ped));

    value
}

pub fn task_perform_sequence_from_progress(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x89221B16730234F0, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn set_next_desired_move_state(p0: f32) -> () {
    let value = native!((), 0xF1B9F16E89E2C93A, native_parameters!(p0));

    value
}

pub fn set_ped_desired_move_blend_ratio(ped: i32, p1: f32) -> () {
    let value = native!((), 0x1E982AC8716912C5, native_parameters!(ped, p1));

    value
}

pub fn get_ped_desired_move_blend_ratio(ped: i32) -> f32 {
    let value = native!(f32, 0x8517D4A6CA8513ED, native_parameters!(ped));

    value
}

pub fn task_goto_entity_aiming(
    ped: i32,
    target: i32,
    distanceToStopAt: f32,
    StartAimingDist: f32,
) -> () {
    let value = native!(
        (),
        0xA9DA48FAB8A76C12,
        native_parameters!(ped, target, distanceToStopAt, StartAimingDist)
    );

    value
}

pub fn task_set_decision_maker(ped: i32, p1: u32) -> () {
    let value = native!((), 0xEB8517DDA73720DA, native_parameters!(ped, p1));

    value
}

pub fn task_set_sphere_defensive_area(p0: u32, p1: f32, p2: f32, p3: f32, p4: f32) -> () {
    let value = native!(
        (),
        0x933C06518B52A9A4,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn task_clear_defensive_area(p0: u32) -> () {
    let value = native!((), 0x95A6C46A31D1917D, native_parameters!(p0));

    value
}

pub fn task_ped_slide_to_coord(ped: i32, x: f32, y: f32, z: f32, heading: f32, p5: f32) -> () {
    let value = native!(
        (),
        0xD04FE6765D990A06,
        native_parameters!(ped, x, y, z, heading, p5)
    );

    value
}

pub fn task_ped_slide_to_coord_hdg_rate(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    p5: f32,
    p6: f32,
) -> () {
    let value = native!(
        (),
        0x5A4A6A6D3DC64F52,
        native_parameters!(ped, x, y, z, heading, p5, p6)
    );

    value
}

pub fn add_cover_point(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: bool,
) -> u32 {
    let value = native!(
        u32,
        0xD5C12A75C7B9497F,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn remove_cover_point(coverpoint: u32) -> () {
    let value = native!((), 0xAE287C923D891715, native_parameters!(coverpoint));

    value
}

pub fn does_scripted_cover_point_exist_at_coords(x: f32, y: f32, z: f32) -> bool {
    let value = native!(bool, 0xA98B8E3C088E5A31, native_parameters!(x, y, z));

    value
}

pub fn get_scripted_cover_point_coords(coverpoint: u32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x594A1028FC2A3E85,
        native_parameters!(coverpoint)
    );

    value
}

pub fn task_combat_ped(ped: i32, targetPed: i32, p2: i32, p3: i32) -> () {
    let value = native!(
        (),
        0xF166E48407BAC484,
        native_parameters!(ped, targetPed, p2, p3)
    );

    value
}

pub fn task_combat_ped_timed(p0: u32, ped: i32, p2: i32, p3: u32) -> () {
    let value = native!((), 0x944F30DCB7096BDE, native_parameters!(p0, ped, p2, p3));

    value
}

pub fn task_seek_cover_from_pos(ped: i32, x: f32, y: f32, z: f32, duration: i32, p5: bool) -> () {
    let value = native!(
        (),
        0x75AC2B60386D89F2,
        native_parameters!(ped, x, y, z, duration, p5)
    );

    value
}

pub fn task_seek_cover_from_ped(ped: i32, target: i32, duration: i32, p3: bool) -> () {
    let value = native!(
        (),
        0x84D32B3BEC531324,
        native_parameters!(ped, target, duration, p3)
    );

    value
}

pub fn task_seek_cover_to_cover_point(
    p0: u32,
    p1: u32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: u32,
    p6: bool,
) -> () {
    let value = native!(
        (),
        0xD43D95C7A869447F,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn task_seek_cover_to_coords(
    ped: i32,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    p7: u32,
    p8: bool,
) -> () {
    let value = native!(
        (),
        0x39246A6958EF072C,
        native_parameters!(ped, x1, y1, z1, x2, y2, z2, p7, p8)
    );

    value
}

pub fn task_put_ped_directly_into_cover(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    timeout: u32,
    p5: bool,
    p6: f32,
    p7: bool,
    p8: bool,
    p9: u32,
    p10: bool,
) -> () {
    let value = native!(
        (),
        0x4172393E6BE1FECE,
        native_parameters!(ped, x, y, z, timeout, p5, p6, p7, p8, p9, p10)
    );

    value
}

pub fn task_exit_cover(p0: u32, p1: u32, p2: f32, p3: f32, p4: f32) -> () {
    let value = native!(
        (),
        0x79B258E397854D29,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn task_put_ped_directly_into_melee(
    ped: i32,
    meleeTarget: i32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: bool,
) -> () {
    let value = native!(
        (),
        0x1C6CD14A876FFE39,
        native_parameters!(ped, meleeTarget, p2, p3, p4, p5)
    );

    value
}

pub fn task_toggle_duck(p0: bool, p1: bool) -> () {
    let value = native!((), 0xAC96609B9995EDF8, native_parameters!(p0, p1));

    value
}

pub fn task_guard_current_position(p0: i32, p1: f32, p2: f32, p3: bool) -> () {
    let value = native!((), 0x4A58A47A72E3FCB4, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn task_guard_assigned_defensive_area(
    p0: u32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: u32,
) -> () {
    let value = native!(
        (),
        0xD2A207EEBDF9889B,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn task_guard_sphere_defensive_area(
    p0: i32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: u32,
    p7: f32,
    p8: f32,
    p9: f32,
    p10: f32,
) -> () {
    let value = native!(
        (),
        0xC946FE14BE0EB5E2,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10)
    );

    value
}

pub fn task_stand_guard(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    scenarioName: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0xAE032F8BBA959E90,
        native_parameters!(ped, x, y, z, heading, scenarioName.as_ptr())
    );

    value
}

pub fn set_drive_task_cruise_speed(driver: i32, cruiseSpeed: f32) -> () {
    let value = native!(
        (),
        0x5C9B84BD7D31D908,
        native_parameters!(driver, cruiseSpeed)
    );

    value
}

pub fn set_drive_task_max_cruise_speed(p0: u32, p1: f32) -> () {
    let value = native!((), 0x404A5AA9B9F0B746, native_parameters!(p0, p1));

    value
}

pub fn set_drive_task_driving_style(ped: i32, drivingStyle: i32) -> () {
    let value = native!(
        (),
        0xDACE1BE37D88AF67,
        native_parameters!(ped, drivingStyle)
    );

    value
}

pub fn add_cover_blocking_area(
    playerX: f32,
    playerY: f32,
    playerZ: f32,
    radiusX: f32,
    radiusY: f32,
    radiusZ: f32,
    p6: bool,
    p7: bool,
    p8: bool,
    p9: bool,
) -> () {
    let value = native!(
        (),
        0x45C597097DD7CB81,
        native_parameters!(playerX, playerY, playerZ, radiusX, radiusY, radiusZ, p6, p7, p8, p9)
    );

    value
}

pub fn remove_all_cover_blocking_areas() -> () {
    let value = native!((), 0xDB6708C0B46F56D8, native_parameters!());

    value
}

pub fn _0xfa83ca6776038f64(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xFA83CA6776038F64, native_parameters!(x, y, z));

    value
}

pub fn _0x1f351cf1c6475734(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
    p8: u32,
    p9: u32,
) -> () {
    let value = native!(
        (),
        0x1F351CF1C6475734,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn task_start_scenario_in_place(
    ped: i32,
    scenarioName: &std::ffi::CString,
    unkDelay: i32,
    playEnterAnim: bool,
) -> () {
    let value = native!(
        (),
        0x142A02425FF02BD9,
        native_parameters!(ped, scenarioName.as_ptr(), unkDelay, playEnterAnim)
    );

    value
}

pub fn task_start_scenario_at_position(
    ped: i32,
    scenarioName: &std::ffi::CString,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    duration: i32,
    sittingScenario: bool,
    teleport: bool,
) -> () {
    let value = native!(
        (),
        0xFA4EFC79F69D4F07,
        native_parameters!(
            ped,
            scenarioName.as_ptr(),
            x,
            y,
            z,
            heading,
            duration,
            sittingScenario,
            teleport
        )
    );

    value
}

pub fn task_use_nearest_scenario_to_coord(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    distance: f32,
    duration: i32,
) -> () {
    let value = native!(
        (),
        0x277F471BA9DB000B,
        native_parameters!(ped, x, y, z, distance, duration)
    );

    value
}

pub fn task_use_nearest_scenario_to_coord_warp(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    p5: u32,
) -> () {
    let value = native!(
        (),
        0x58E2E0F23F6B76C3,
        native_parameters!(ped, x, y, z, radius, p5)
    );

    value
}

pub fn task_use_nearest_scenario_chain_to_coord(
    p0: u32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: u32,
) -> () {
    let value = native!(
        (),
        0x9FDA1B3D7E7028B3,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn task_use_nearest_scenario_chain_to_coord_warp(
    p0: u32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: u32,
) -> () {
    let value = native!(
        (),
        0x97A28E63F0BA5631,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn does_scenario_exist_in_area(x: f32, y: f32, z: f32, radius: f32, b: bool) -> bool {
    let value = native!(
        bool,
        0x5A59271FFADD33C1,
        native_parameters!(x, y, z, radius, b)
    );

    value
}

pub fn does_scenario_of_type_exist_in_area(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: *mut u32,
    p4: f32,
    p5: bool,
) -> bool {
    let value = native!(
        bool,
        0x0A9D0C2A3BBC86C1,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn is_scenario_occupied(p0: f32, p1: f32, p2: f32, p3: f32, p4: bool) -> bool {
    let value = native!(
        bool,
        0x788756D73AC2E07C,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn ped_has_use_scenario_task(ped: i32) -> bool {
    let value = native!(bool, 0x295E3CCEC879CCD7, native_parameters!(ped));

    value
}

pub fn play_anim_on_running_scenario(
    ped: i32,
    animDict: &std::ffi::CString,
    animName: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0x748040460F8DF5DC,
        native_parameters!(ped, animDict.as_ptr(), animName.as_ptr())
    );

    value
}

pub fn does_scenario_group_exist(scenarioGroup: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xF9034C136C9E00D3,
        native_parameters!(scenarioGroup.as_ptr())
    );

    value
}

pub fn is_scenario_group_enabled(scenarioGroup: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x367A09DED4E05B99,
        native_parameters!(scenarioGroup.as_ptr())
    );

    value
}

pub fn set_scenario_group_enabled(scenarioGroup: &std::ffi::CString, p1: bool) -> () {
    let value = native!(
        (),
        0x02C8E5B49848664E,
        native_parameters!(scenarioGroup.as_ptr(), p1)
    );

    value
}

pub fn reset_scenario_groups_enabled() -> () {
    let value = native!((), 0xDD902D0349AFAD3A, native_parameters!());

    value
}

pub fn set_exclusive_scenario_group(scenarioGroup: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x535E97E1F7FC0C6A,
        native_parameters!(scenarioGroup.as_ptr())
    );

    value
}

pub fn reset_exclusive_scenario_group() -> () {
    let value = native!((), 0x4202BBCB8684563D, native_parameters!());

    value
}

pub fn is_scenario_type_enabled(scenarioType: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x3A815DB3EA088722,
        native_parameters!(scenarioType.as_ptr())
    );

    value
}

pub fn set_scenario_type_enabled(scenarioType: &std::ffi::CString, toggle: bool) -> () {
    let value = native!(
        (),
        0xEB47EC4E34FB7EE1,
        native_parameters!(scenarioType.as_ptr(), toggle)
    );

    value
}

pub fn reset_scenario_types_enabled() -> () {
    let value = native!((), 0x0D40EE2A7F2B2D6D, native_parameters!());

    value
}

pub fn is_ped_active_in_scenario(ped: i32) -> bool {
    let value = native!(bool, 0xAA135F9482C82CC3, native_parameters!(ped));

    value
}

pub fn is_ped_playing_base_clip_in_scenario(ped: i32) -> bool {
    let value = native!(bool, 0x621C6E4729388E41, native_parameters!(ped));

    value
}

pub fn set_ped_can_play_ambient_idles(ped: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x8FD89A6240813FD0, native_parameters!(ped, p1, p2));

    value
}

pub fn task_combat_hated_targets_in_area(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    p5: u32,
) -> () {
    let value = native!(
        (),
        0x4CF5F55DAC3280A0,
        native_parameters!(ped, x, y, z, radius, p5)
    );

    value
}

pub fn task_combat_hated_targets_around_ped(ped: i32, radius: f32, p2: i32) -> () {
    let value = native!((), 0x7BF835BB9E2698C8, native_parameters!(ped, radius, p2));

    value
}

pub fn task_combat_hated_targets_around_ped_timed(p0: u32, p1: f32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x2BBA30B854534A0C, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn task_throw_projectile(ped: i32, x: f32, y: f32, z: f32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0x7285951DBF6B5A51,
        native_parameters!(ped, x, y, z, p4, p5)
    );

    value
}

pub fn task_swap_weapon(ped: i32, p1: bool) -> () {
    let value = native!((), 0xA21C51255B205245, native_parameters!(ped, p1));

    value
}

pub fn task_reload_weapon(ped: i32, unused: bool) -> () {
    let value = native!((), 0x62D2916F56B9CD2D, native_parameters!(ped, unused));

    value
}

pub fn is_ped_getting_up(ped: i32) -> bool {
    let value = native!(bool, 0x2A74E1D5F2F00EEC, native_parameters!(ped));

    value
}

pub fn task_writhe(ped: i32, target: i32, time: i32, p3: i32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0xCDDC2B77CE54AC6E,
        native_parameters!(ped, target, time, p3, p4, p5)
    );

    value
}

pub fn is_ped_in_writhe(ped: i32) -> bool {
    let value = native!(bool, 0xDEB6D52126E7D640, native_parameters!(ped));

    value
}

pub fn open_patrol_route(patrolRoute: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xA36BFB5EE89F3D82,
        native_parameters!(patrolRoute.as_ptr())
    );

    value
}

pub fn close_patrol_route() -> () {
    let value = native!((), 0xB043ECA801B8CBC1, native_parameters!());

    value
}

pub fn add_patrol_route_node(
    p0: i32,
    p1: &std::ffi::CString,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    p8: i32,
) -> () {
    let value = native!(
        (),
        0x8EDF950167586B7C,
        native_parameters!(p0, p1.as_ptr(), x1, y1, z1, x2, y2, z2, p8)
    );

    value
}

pub fn add_patrol_route_link(p0: u32, p1: u32) -> () {
    let value = native!((), 0x23083260DEC3A551, native_parameters!(p0, p1));

    value
}

pub fn create_patrol_route() -> () {
    let value = native!((), 0xAF8A443CCC8018DC, native_parameters!());

    value
}

pub fn delete_patrol_route(patrolRoute: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x7767DD9D65E91319,
        native_parameters!(patrolRoute.as_ptr())
    );

    value
}

pub fn task_patrol(ped: i32, p1: &std::ffi::CString, p2: u32, p3: bool, p4: bool) -> () {
    let value = native!(
        (),
        0xBDA5DF49D080FE4E,
        native_parameters!(ped, p1.as_ptr(), p2, p3, p4)
    );

    value
}

pub fn task_stay_in_cover(ped: i32) -> () {
    let value = native!((), 0xE5DA8615A6180789, native_parameters!(ped));

    value
}

pub fn add_vehicle_subtask_attack_coord(ped: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x5CF0D8F9BBA0DD75, native_parameters!(ped, x, y, z));

    value
}

pub fn add_vehicle_subtask_attack_ped(ped: i32, ped2: i32) -> () {
    let value = native!((), 0x85F462BADC7DA47F, native_parameters!(ped, ped2));

    value
}

pub fn task_vehicle_shoot_at_ped(ped: i32, target: i32, p2: f32) -> () {
    let value = native!((), 0x10AB107B887214D8, native_parameters!(ped, target, p2));

    value
}

pub fn task_vehicle_aim_at_ped(ped: i32, target: i32) -> () {
    let value = native!((), 0xE41885592B08B097, native_parameters!(ped, target));

    value
}

pub fn task_vehicle_shoot_at_coord(ped: i32, x: f32, y: f32, z: f32, p4: f32) -> () {
    let value = native!((), 0x5190796ED39C9B6D, native_parameters!(ped, x, y, z, p4));

    value
}

pub fn task_vehicle_aim_at_coord(ped: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x447C1E9EF844BC0F, native_parameters!(ped, x, y, z));

    value
}

pub fn task_vehicle_goto_navmesh(
    ped: i32,
    vehicle: i32,
    x: f32,
    y: f32,
    z: f32,
    speed: f32,
    behaviorFlag: i32,
    stoppingRange: f32,
) -> () {
    let value = native!(
        (),
        0x195AEEB13CEFE2EE,
        native_parameters!(ped, vehicle, x, y, z, speed, behaviorFlag, stoppingRange)
    );

    value
}

pub fn task_go_to_coord_while_aiming_at_coord(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    aimAtX: f32,
    aimAtY: f32,
    aimAtZ: f32,
    moveSpeed: f32,
    p8: bool,
    p9: f32,
    p10: f32,
    p11: bool,
    flags: u32,
    p13: bool,
    firingPattern: u32,
) -> () {
    let value = native!(
        (),
        0x11315AB3385B8AC0,
        native_parameters!(
            ped,
            x,
            y,
            z,
            aimAtX,
            aimAtY,
            aimAtZ,
            moveSpeed,
            p8,
            p9,
            p10,
            p11,
            flags,
            p13,
            firingPattern
        )
    );

    value
}

pub fn task_go_to_coord_while_aiming_at_entity(
    p0: u32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: u32,
    p5: f32,
    p6: bool,
    p7: f32,
    p8: f32,
    p9: bool,
    p10: u32,
    p11: bool,
    p12: u32,
    p13: u32,
) -> () {
    let value = native!(
        (),
        0xB2A16444EAD9AE47,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13)
    );

    value
}

pub fn task_go_to_coord_and_aim_at_hated_entities_near_coord(
    pedHandle: i32,
    goToLocationX: f32,
    goToLocationY: f32,
    goToLocationZ: f32,
    focusLocationX: f32,
    focusLocationY: f32,
    focusLocationZ: f32,
    speed: f32,
    shootAtEnemies: bool,
    distanceToStopAt: f32,
    noRoadsDistance: f32,
    unkTrue: bool,
    unkFlag: i32,
    aimingFlag: i32,
    firingPattern: u32,
) -> () {
    let value = native!(
        (),
        0xA55547801EB331FC,
        native_parameters!(
            pedHandle,
            goToLocationX,
            goToLocationY,
            goToLocationZ,
            focusLocationX,
            focusLocationY,
            focusLocationZ,
            speed,
            shootAtEnemies,
            distanceToStopAt,
            noRoadsDistance,
            unkTrue,
            unkFlag,
            aimingFlag,
            firingPattern
        )
    );

    value
}

pub fn task_go_to_entity_while_aiming_at_coord(
    p0: u32,
    p1: u32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: bool,
    p7: f32,
    p8: f32,
    p9: bool,
    p10: bool,
    p11: u32,
) -> () {
    let value = native!(
        (),
        0x04701832B739DCE5,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11)
    );

    value
}

pub fn task_go_to_entity_while_aiming_at_entity(
    ped: i32,
    entityToWalkTo: i32,
    entityToAimAt: i32,
    speed: f32,
    shootatEntity: bool,
    p5: f32,
    p6: f32,
    p7: bool,
    p8: bool,
    firingPattern: u32,
) -> () {
    let value = native!(
        (),
        0x97465886D35210E9,
        native_parameters!(
            ped,
            entityToWalkTo,
            entityToAimAt,
            speed,
            shootatEntity,
            p5,
            p6,
            p7,
            p8,
            firingPattern
        )
    );

    value
}

pub fn set_high_fall_task(ped: i32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x8C825BDC7741D37C, native_parameters!(ped, p1, p2, p3));

    value
}

pub fn request_waypoint_recording(name: &std::ffi::CString) -> () {
    let value = native!((), 0x9EEFB62EB27B5792, native_parameters!(name.as_ptr()));

    value
}

pub fn get_is_waypoint_recording_loaded(name: &std::ffi::CString) -> bool {
    let value = native!(bool, 0xCB4E8BE8A0063C5D, native_parameters!(name.as_ptr()));

    value
}

pub fn remove_waypoint_recording(name: &std::ffi::CString) -> () {
    let value = native!((), 0xFF1B8B4AA1C25DC8, native_parameters!(name.as_ptr()));

    value
}

pub fn waypoint_recording_get_num_points(name: &std::ffi::CString, points: *mut i32) -> bool {
    let value = native!(
        bool,
        0x5343532C01A07234,
        native_parameters!(name.as_ptr(), points)
    );

    value
}

pub fn waypoint_recording_get_coord(
    name: &std::ffi::CString,
    point: i32,
    coord: *mut NativeVector3,
) -> bool {
    let value = native!(
        bool,
        0x2FB897405C90B361,
        native_parameters!(name.as_ptr(), point, coord)
    );

    value
}

pub fn waypoint_recording_get_speed_at_point(name: &std::ffi::CString, point: i32) -> f32 {
    let value = native!(
        f32,
        0x005622AEBC33ACA9,
        native_parameters!(name.as_ptr(), point)
    );

    value
}

pub fn waypoint_recording_get_closest_waypoint(
    name: &std::ffi::CString,
    x: f32,
    y: f32,
    z: f32,
    point: *mut i32,
) -> bool {
    let value = native!(
        bool,
        0xB629A298081F876F,
        native_parameters!(name.as_ptr(), x, y, z, point)
    );

    value
}

pub fn task_follow_waypoint_recording(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x0759591819534F7B,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn is_waypoint_playback_going_on_for_ped(p0: u32) -> bool {
    let value = native!(bool, 0xE03B3F2D3DC59B64, native_parameters!(p0));

    value
}

pub fn get_ped_waypoint_progress(ped: i32) -> i32 {
    let value = native!(i32, 0x2720AAA75001E094, native_parameters!(ped));

    value
}

pub fn get_ped_waypoint_distance(p0: u32) -> f32 {
    let value = native!(f32, 0xE6A877C64CAF1BC5, native_parameters!(p0));

    value
}

pub fn set_ped_waypoint_route_offset(p0: u32, p1: u32, p2: u32, p3: u32) -> u32 {
    let value = native!(u32, 0xED98E10B0AFCE4B4, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn get_waypoint_distance_along_route(p0: &std::ffi::CString, p1: i32) -> f32 {
    let value = native!(f32, 0xA5B769058763E497, native_parameters!(p0.as_ptr(), p1));

    value
}

pub fn waypoint_playback_get_is_paused(p0: u32) -> bool {
    let value = native!(bool, 0x701375A7D43F01CB, native_parameters!(p0));

    value
}

pub fn waypoint_playback_pause(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x0F342546AA06FED5, native_parameters!(p0, p1, p2));

    value
}

pub fn waypoint_playback_resume(p0: u32, p1: bool, p2: u32, p3: u32) -> () {
    let value = native!((), 0x244F70C84C547D2D, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn waypoint_playback_override_speed(p0: u32, p1: f32, p2: bool) -> () {
    let value = native!((), 0x7D7D2B47FA788E85, native_parameters!(p0, p1, p2));

    value
}

pub fn waypoint_playback_use_default_speed(p0: u32) -> () {
    let value = native!((), 0x6599D834B12D0800, native_parameters!(p0));

    value
}

pub fn use_waypoint_recording_as_assisted_movement_route(
    name: &std::ffi::CString,
    p1: bool,
    p2: f32,
    p3: f32,
) -> () {
    let value = native!(
        (),
        0x5A353B8E6B1095B5,
        native_parameters!(name.as_ptr(), p1, p2, p3)
    );

    value
}

pub fn waypoint_playback_start_aiming_at_ped(p0: u32, p1: u32, p2: bool) -> () {
    let value = native!((), 0x20E330937C399D29, native_parameters!(p0, p1, p2));

    value
}

pub fn waypoint_playback_start_aiming_at_coord(p0: u32, p1: f32, p2: f32, p3: f32, p4: bool) -> () {
    let value = native!(
        (),
        0x8968400D900ED8B3,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn waypoint_playback_start_shooting_at_ped(p0: u32, p1: u32, p2: bool, p3: u32) -> () {
    let value = native!((), 0xE70BA7B90F8390DC, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn waypoint_playback_start_shooting_at_coord(
    p0: u32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: bool,
    p5: u32,
) -> () {
    let value = native!(
        (),
        0x057A25CFCC9DB671,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn waypoint_playback_stop_aiming_or_shooting(p0: u32) -> () {
    let value = native!((), 0x47EFA040EBB8E2EA, native_parameters!(p0));

    value
}

pub fn assisted_movement_request_route(route: &std::ffi::CString) -> () {
    let value = native!((), 0x817268968605947A, native_parameters!(route.as_ptr()));

    value
}

pub fn assisted_movement_remove_route(route: &std::ffi::CString) -> () {
    let value = native!((), 0x3548536485DD792B, native_parameters!(route.as_ptr()));

    value
}

pub fn assisted_movement_is_route_loaded(route: &std::ffi::CString) -> bool {
    let value = native!(bool, 0x60F9A4393A21F741, native_parameters!(route.as_ptr()));

    value
}

pub fn assisted_movement_set_route_properties(route: &std::ffi::CString, props: i32) -> () {
    let value = native!(
        (),
        0xD5002D78B7162E1B,
        native_parameters!(route.as_ptr(), props)
    );

    value
}

pub fn assisted_movement_override_load_distance_this_frame(dist: f32) -> () {
    let value = native!((), 0x13945951E16EF912, native_parameters!(dist));

    value
}

pub fn task_vehicle_follow_waypoint_recording(
    ped: i32,
    vehicle: i32,
    WPRecording: &std::ffi::CString,
    p3: i32,
    p4: i32,
    p5: i32,
    p6: i32,
    p7: f32,
    p8: bool,
    p9: f32,
) -> () {
    let value = native!(
        (),
        0x3123FAA6DB1CF7ED,
        native_parameters!(
            ped,
            vehicle,
            WPRecording.as_ptr(),
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
            p9
        )
    );

    value
}

pub fn is_waypoint_playback_going_on_for_vehicle(vehicle: i32) -> bool {
    let value = native!(bool, 0xF5134943EA29868C, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_waypoint_progress(vehicle: i32) -> i32 {
    let value = native!(i32, 0x9824CFF8FC66E159, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_waypoint_target_point(vehicle: i32) -> i32 {
    let value = native!(i32, 0x416B62AC8B9E5BBD, native_parameters!(vehicle));

    value
}

pub fn vehicle_waypoint_playback_pause(vehicle: i32) -> () {
    let value = native!((), 0x8A4E6AC373666BC5, native_parameters!(vehicle));

    value
}

pub fn vehicle_waypoint_playback_resume(vehicle: i32) -> () {
    let value = native!((), 0xDC04FCAA7839D492, native_parameters!(vehicle));

    value
}

pub fn vehicle_waypoint_playback_use_default_speed(vehicle: i32) -> () {
    let value = native!((), 0x5CEB25A7D2848963, native_parameters!(vehicle));

    value
}

pub fn vehicle_waypoint_playback_override_speed(vehicle: i32, speed: f32) -> () {
    let value = native!((), 0x121F0593E0A431D7, native_parameters!(vehicle, speed));

    value
}

pub fn task_set_blocking_of_non_temporary_events(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x90D2156198831D69, native_parameters!(ped, toggle));

    value
}

pub fn task_force_motion_state(ped: i32, state: u32, p2: bool) -> () {
    let value = native!((), 0x4F056E1AFFEF17AB, native_parameters!(ped, state, p2));

    value
}

pub fn task_move_network_by_name(
    ped: i32,
    task: &std::ffi::CString,
    multiplier: f32,
    p3: bool,
    animDict: &std::ffi::CString,
    flags: i32,
) -> () {
    let value = native!(
        (),
        0x2D537BA194896636,
        native_parameters!(ped, task.as_ptr(), multiplier, p3, animDict.as_ptr(), flags)
    );

    value
}

pub fn task_move_network_advanced_by_name(
    ped: i32,
    p1: &std::ffi::CString,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
    p8: u32,
    p9: f32,
    p10: bool,
    animDict: &std::ffi::CString,
    flags: i32,
) -> () {
    let value = native!(
        (),
        0xD5B35BEA41919ACB,
        native_parameters!(
            ped,
            p1.as_ptr(),
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
            p9,
            p10,
            animDict.as_ptr(),
            flags
        )
    );

    value
}

pub fn _task_move_network_by_name_with_init_params(
    ped: i32,
    p1: &std::ffi::CString,
    data: *mut u32,
    p3: f32,
    p4: bool,
    animDict: &std::ffi::CString,
    flags: i32,
) -> () {
    let value = native!(
        (),
        0x3D45B0B355C5E0C9,
        native_parameters!(ped, p1.as_ptr(), data, p3, p4, animDict.as_ptr(), flags)
    );

    value
}

pub fn _0x29682e2ccf21e9b5(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
    p8: u32,
    p9: u32,
    p10: u32,
    p11: u32,
    p12: u32,
    p13: u32,
) -> () {
    let value = native!(
        (),
        0x29682E2CCF21E9B5,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13)
    );

    value
}

pub fn is_task_move_network_active(ped: i32) -> bool {
    let value = native!(bool, 0x921CE12C489C4C41, native_parameters!(ped));

    value
}

pub fn is_task_move_network_ready_for_transition(ped: i32) -> bool {
    let value = native!(bool, 0x30ED88D5E0C56A37, native_parameters!(ped));

    value
}

pub fn request_task_move_network_state_transition(ped: i32, name: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xD01015C7316AE176,
        native_parameters!(ped, name.as_ptr())
    );

    value
}

pub fn _0xab13a5565480b6d9(ped: i32, p1: &std::ffi::CString) -> u32 {
    let value = native!(
        u32,
        0xAB13A5565480B6D9,
        native_parameters!(ped, p1.as_ptr())
    );

    value
}

pub fn get_task_move_network_state(ped: i32) -> String {
    let value = native!(*const i8, 0x717E4D1F2048376D, native_parameters!(ped));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _0x8423541e8b3a1589(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x8423541E8B3A1589, native_parameters!(p0, p1, p2));

    value
}

pub fn set_task_move_network_signal_float(
    ped: i32,
    signalName: &std::ffi::CString,
    value: f32,
) -> () {
    let value = native!(
        (),
        0xD5BB4025AE449A4E,
        native_parameters!(ped, signalName.as_ptr(), value)
    );

    value
}

pub fn _set_task_move_network_signal_float_2(
    ped: i32,
    signalName: &std::ffi::CString,
    value: f32,
) -> () {
    let value = native!(
        (),
        0x373EF409B82697A3,
        native_parameters!(ped, signalName.as_ptr(), value)
    );

    value
}

pub fn _0x8634cef2522d987b(ped: i32, p1: &std::ffi::CString, value: f32) -> () {
    let value = native!(
        (),
        0x8634CEF2522D987B,
        native_parameters!(ped, p1.as_ptr(), value)
    );

    value
}

pub fn set_task_move_network_signal_bool(
    ped: i32,
    signalName: &std::ffi::CString,
    value: bool,
) -> () {
    let value = native!(
        (),
        0xB0A6CFD2C69C1088,
        native_parameters!(ped, signalName.as_ptr(), value)
    );

    value
}

pub fn _get_task_move_network_signal_float(ped: i32, signalName: &std::ffi::CString) -> f32 {
    let value = native!(
        f32,
        0x44AB0B3AFECCE242,
        native_parameters!(ped, signalName.as_ptr())
    );

    value
}

pub fn get_task_move_network_signal_bool(ped: i32, signalName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xA7FFBA498E4AAF67,
        native_parameters!(ped, signalName.as_ptr())
    );

    value
}

pub fn get_task_move_network_event(ped: i32, eventName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xB4F47213DF45A64C,
        native_parameters!(ped, eventName.as_ptr())
    );

    value
}

pub fn _0x0ffb3c758e8c07b9(ped: i32, p1: bool) -> u32 {
    let value = native!(u32, 0x0FFB3C758E8C07B9, native_parameters!(ped, p1));

    value
}

pub fn is_move_blend_ratio_still(ped: i32) -> bool {
    let value = native!(bool, 0x349CE7B56DAFD95C, native_parameters!(ped));

    value
}

pub fn is_move_blend_ratio_walking(ped: i32) -> bool {
    let value = native!(bool, 0xF133BBBE91E1691F, native_parameters!(ped));

    value
}

pub fn is_move_blend_ratio_running(ped: i32) -> bool {
    let value = native!(bool, 0xD4D8636C0199A939, native_parameters!(ped));

    value
}

pub fn is_move_blend_ratio_sprinting(ped: i32) -> bool {
    let value = native!(bool, 0x24A2AD74FA9814E2, native_parameters!(ped));

    value
}

pub fn is_ped_still(ped: i32) -> bool {
    let value = native!(bool, 0xAC29253EEF8F0180, native_parameters!(ped));

    value
}

pub fn is_ped_walking(ped: i32) -> bool {
    let value = native!(bool, 0xDE4C184B2B9B071A, native_parameters!(ped));

    value
}

pub fn is_ped_running(ped: i32) -> bool {
    let value = native!(bool, 0xC5286FFC176F28A2, native_parameters!(ped));

    value
}

pub fn is_ped_sprinting(ped: i32) -> bool {
    let value = native!(bool, 0x57E457CD2C0FC168, native_parameters!(ped));

    value
}

pub fn is_ped_strafing(ped: i32) -> bool {
    let value = native!(bool, 0xE45B7F222DE47E09, native_parameters!(ped));

    value
}

pub fn task_synchronized_scene(
    ped: i32,
    scene: i32,
    animDictionary: &std::ffi::CString,
    animationName: &std::ffi::CString,
    speed: f32,
    speedMultiplier: f32,
    duration: i32,
    flag: i32,
    playbackRate: f32,
    p9: u32,
) -> () {
    let value = native!(
        (),
        0xEEA929141F699854,
        native_parameters!(
            ped,
            scene,
            animDictionary.as_ptr(),
            animationName.as_ptr(),
            speed,
            speedMultiplier,
            duration,
            flag,
            playbackRate,
            p9
        )
    );

    value
}

pub fn task_agitated_action(ped: i32, ped2: i32) -> () {
    let value = native!((), 0x19D1B791CB3670FE, native_parameters!(ped, ped2));

    value
}

pub fn task_sweep_aim_entity(
    ped: i32,
    anim: &std::ffi::CString,
    p2: &std::ffi::CString,
    p3: &std::ffi::CString,
    p4: &std::ffi::CString,
    p5: i32,
    vehicle: i32,
    p7: f32,
    p8: f32,
) -> () {
    let value = native!(
        (),
        0x2047C02158D6405A,
        native_parameters!(
            ped,
            anim.as_ptr(),
            p2.as_ptr(),
            p3.as_ptr(),
            p4.as_ptr(),
            p5,
            vehicle,
            p7,
            p8
        )
    );

    value
}

pub fn update_task_sweep_aim_entity(ped: i32, entity: i32) -> () {
    let value = native!((), 0xE4973DBDBE6E44B3, native_parameters!(ped, entity));

    value
}

pub fn task_sweep_aim_position(
    p0: u32,
    p1: *mut u32,
    p2: *mut u32,
    p3: *mut u32,
    p4: *mut u32,
    p5: u32,
    p6: f32,
    p7: f32,
    p8: f32,
    p9: f32,
    p10: f32,
) -> () {
    let value = native!(
        (),
        0x7AFE8FDC10BC07D2,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10)
    );

    value
}

pub fn update_task_sweep_aim_position(p0: u32, p1: f32, p2: f32, p3: f32) -> () {
    let value = native!((), 0xBB106883F5201FC4, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn task_arrest_ped(ped: i32, target: i32) -> () {
    let value = native!((), 0xF3B9A78A178572B1, native_parameters!(ped, target));

    value
}

pub fn is_ped_running_arrest_task(ped: i32) -> bool {
    let value = native!(bool, 0x3DC52677769B4AE0, native_parameters!(ped));

    value
}

pub fn is_ped_being_arrested(ped: i32) -> bool {
    let value = native!(bool, 0x90A09F3A45FED688, native_parameters!(ped));

    value
}

pub fn uncuff_ped(ped: i32) -> () {
    let value = native!((), 0x67406F2C8F87FC4F, native_parameters!(ped));

    value
}

pub fn is_ped_cuffed(ped: i32) -> bool {
    let value = native!(bool, 0x74E559B3BC910685, native_parameters!(ped));

    value
}
