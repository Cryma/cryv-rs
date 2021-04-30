use crate::types::NativeVector3;

pub fn get_allocated_stack_size() -> i32 {
    let value = native!(i32, 0x8B3CA62B1EF19B62, native_parameters!());

    value
}

pub fn get_number_of_free_stacks_of_this_size(stackSize: i32) -> i32 {
    let value = native!(i32, 0xFEAD16FC8F9DFC0F, native_parameters!(stackSize));

    value
}

pub fn set_random_seed(seed: i32) -> () {
    let value = native!((), 0x444D98F98C11F3EC, native_parameters!(seed));

    value
}

pub fn set_time_scale(timeScale: f32) -> () {
    let value = native!((), 0x1D408577D440E81E, native_parameters!(timeScale));

    value
}

pub fn set_mission_flag(toggle: bool) -> () {
    let value = native!((), 0xC4301E5121A0ED73, native_parameters!(toggle));

    value
}

pub fn get_mission_flag() -> bool {
    let value = native!(bool, 0xA33CDCCDA663159E, native_parameters!());

    value
}

pub fn set_random_event_flag(toggle: bool) -> () {
    let value = native!((), 0x971927086CFD2158, native_parameters!(toggle));

    value
}

pub fn get_random_event_flag() -> bool {
    let value = native!(bool, 0xD2D57F1D764117B1, native_parameters!());

    value
}

pub fn _get_global_char_buffer() -> String {
    let value = native!(*const i8, 0x24DA7D7667FD7B09, native_parameters!());
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _0x4dcdf92bf64236cd(p0: &std::ffi::CString, p1: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x4DCDF92BF64236CD,
        native_parameters!(p0.as_ptr(), p1.as_ptr())
    );

    value
}

pub fn _0x31125fd509d9043f(p0: &std::ffi::CString) -> () {
    let value = native!((), 0x31125FD509D9043F, native_parameters!(p0.as_ptr()));

    value
}

pub fn _0xebd3205a207939ed(p0: &std::ffi::CString) -> () {
    let value = native!((), 0xEBD3205A207939ED, native_parameters!(p0.as_ptr()));

    value
}

pub fn _0x97e7e2c04245115b(p0: u32) -> () {
    let value = native!((), 0x97E7E2C04245115B, native_parameters!(p0));

    value
}

pub fn _0x916ca67d26fd1e37(p0: &std::ffi::CString) -> () {
    let value = native!((), 0x916CA67D26FD1E37, native_parameters!(p0.as_ptr()));

    value
}

pub fn _0xeb078ca2b5e82add(p0: &std::ffi::CString, p1: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xEB078CA2B5E82ADD,
        native_parameters!(p0.as_ptr(), p1.as_ptr())
    );

    value
}

pub fn _0x703cc7f60cbb2b57(p0: &std::ffi::CString) -> () {
    let value = native!((), 0x703CC7F60CBB2B57, native_parameters!(p0.as_ptr()));

    value
}

pub fn _0x8951eb9c6906d3c8() -> () {
    let value = native!((), 0x8951EB9C6906D3C8, native_parameters!());

    value
}

pub fn _0xba4b8d83bdc75551(p0: &std::ffi::CString) -> () {
    let value = native!((), 0xBA4B8D83BDC75551, native_parameters!(p0.as_ptr()));

    value
}

pub fn _has_resumed_from_suspend() -> bool {
    let value = native!(bool, 0xE8B9C0EC9E183F35, native_parameters!());

    value
}

pub fn _0x65d2ebb47e1cec21(toggle: bool) -> () {
    let value = native!((), 0x65D2EBB47E1CEC21, native_parameters!(toggle));

    value
}

pub fn _0x6f2135b6129620c1(toggle: bool) -> () {
    let value = native!((), 0x6F2135B6129620C1, native_parameters!(toggle));

    value
}

pub fn _0x8d74e26f54b4e5c3(p0: &std::ffi::CString) -> () {
    let value = native!((), 0x8D74E26F54B4E5C3, native_parameters!(p0.as_ptr()));

    value
}

pub fn _get_base_element_metadata(p0: *mut u32, p1: *mut u32, p2: u32, p3: bool) -> bool {
    let value = native!(bool, 0xB335F761606DB47C, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn get_prev_weather_type_hash_name() -> u32 {
    let value = native!(u32, 0x564B884A05EC45A3, native_parameters!());

    value
}

pub fn get_next_weather_type_hash_name() -> u32 {
    let value = native!(u32, 0x711327CD09C8F162, native_parameters!());

    value
}

pub fn is_prev_weather_type(weatherType: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x44F28F86433B10A9,
        native_parameters!(weatherType.as_ptr())
    );

    value
}

pub fn is_next_weather_type(weatherType: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x2FAA3A30BEC0F25D,
        native_parameters!(weatherType.as_ptr())
    );

    value
}

pub fn set_weather_type_persist(weatherType: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x704983DF373B198F,
        native_parameters!(weatherType.as_ptr())
    );

    value
}

pub fn set_weather_type_now_persist(weatherType: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xED712CA327900C8A,
        native_parameters!(weatherType.as_ptr())
    );

    value
}

pub fn set_weather_type_now(weatherType: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x29B487C359E19889,
        native_parameters!(weatherType.as_ptr())
    );

    value
}

pub fn set_weather_type_overtime_persist(weatherType: &std::ffi::CString, time: f32) -> () {
    let value = native!(
        (),
        0xFB5045B7C42B75BF,
        native_parameters!(weatherType.as_ptr(), time)
    );

    value
}

pub fn set_random_weather_type() -> () {
    let value = native!((), 0x8B05F884CF7E8020, native_parameters!());

    value
}

pub fn clear_weather_type_persist() -> () {
    let value = native!((), 0xCCC39339BEF76CF5, native_parameters!());

    value
}

pub fn _0x0cf97f497fe7d048(p0: f32) -> () {
    let value = native!((), 0x0CF97F497FE7D048, native_parameters!(p0));

    value
}

pub fn _get_weather_type_transition(
    weatherType1: *mut u32,
    weatherType2: *mut u32,
    percentWeather2: *mut f32,
) -> () {
    let value = native!(
        (),
        0xF3BBE884A14BB413,
        native_parameters!(weatherType1, weatherType2, percentWeather2)
    );

    value
}

pub fn _set_weather_type_transition(
    weatherType1: u32,
    weatherType2: u32,
    percentWeather2: f32,
) -> () {
    let value = native!(
        (),
        0x578C752848ECFA0C,
        native_parameters!(weatherType1, weatherType2, percentWeather2)
    );

    value
}

pub fn set_override_weather(weatherType: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xA43D5C6FE51ADBEF,
        native_parameters!(weatherType.as_ptr())
    );

    value
}

pub fn _0x1178e104409fe58c(p0: u32, p1: u32) -> () {
    let value = native!((), 0x1178E104409FE58C, native_parameters!(p0, p1));

    value
}

pub fn clear_override_weather() -> () {
    let value = native!((), 0x338D2E3477711050, native_parameters!());

    value
}

pub fn water_override_set_shorewaveamplitude(amplitude: f32) -> () {
    let value = native!((), 0xB8F87EAD7533B176, native_parameters!(amplitude));

    value
}

pub fn water_override_set_shorewaveminamplitude(minAmplitude: f32) -> () {
    let value = native!((), 0xC3EAD29AB273ECE8, native_parameters!(minAmplitude));

    value
}

pub fn water_override_set_shorewavemaxamplitude(maxAmplitude: f32) -> () {
    let value = native!((), 0xA7A1127490312C36, native_parameters!(maxAmplitude));

    value
}

pub fn water_override_set_oceannoiseminamplitude(minAmplitude: f32) -> () {
    let value = native!((), 0x31727907B2C43C55, native_parameters!(minAmplitude));

    value
}

pub fn water_override_set_oceanwaveamplitude(amplitude: f32) -> () {
    let value = native!((), 0x405591EC8FD9096D, native_parameters!(amplitude));

    value
}

pub fn water_override_set_oceanwaveminamplitude(minAmplitude: f32) -> () {
    let value = native!((), 0xF751B16FB32ABC1D, native_parameters!(minAmplitude));

    value
}

pub fn water_override_set_oceanwavemaxamplitude(maxAmplitude: f32) -> () {
    let value = native!((), 0xB3E6360DDE733E82, native_parameters!(maxAmplitude));

    value
}

pub fn water_override_set_ripplebumpiness(bumpiness: f32) -> () {
    let value = native!((), 0x7C9C0B1EEB1F9072, native_parameters!(bumpiness));

    value
}

pub fn water_override_set_rippleminbumpiness(minBumpiness: f32) -> () {
    let value = native!((), 0x6216B116083A7CB4, native_parameters!(minBumpiness));

    value
}

pub fn water_override_set_ripplemaxbumpiness(maxBumpiness: f32) -> () {
    let value = native!((), 0x9F5E6BB6B34540DA, native_parameters!(maxBumpiness));

    value
}

pub fn water_override_set_rippledisturb(disturb: f32) -> () {
    let value = native!((), 0xB9854DFDE0D833D6, native_parameters!(disturb));

    value
}

pub fn water_override_set_strength(strength: f32) -> () {
    let value = native!((), 0xC54A08C85AE4D410, native_parameters!(strength));

    value
}

pub fn water_override_fade_in(p0: f32) -> () {
    let value = native!((), 0xA8434F1DFF41D6E7, native_parameters!(p0));

    value
}

pub fn water_override_fade_out(p0: f32) -> () {
    let value = native!((), 0xC3C221ADDDE31A11, native_parameters!(p0));

    value
}

pub fn set_wind(speed: f32) -> () {
    let value = native!((), 0xAC3A74E8384A9919, native_parameters!(speed));

    value
}

pub fn set_wind_speed(speed: f32) -> () {
    let value = native!((), 0xEE09ECEDBABE47FC, native_parameters!(speed));

    value
}

pub fn get_wind_speed() -> f32 {
    let value = native!(f32, 0xA8CF1CC0AFCD3F12, native_parameters!());

    value
}

pub fn set_wind_direction(direction: f32) -> () {
    let value = native!((), 0xEB0F4468467B4528, native_parameters!(direction));

    value
}

pub fn get_wind_direction() -> NativeVector3 {
    let value = native!(NativeVector3, 0x1F400FEF721170DA, native_parameters!());

    value
}

pub fn _set_rain_level(intensity: f32) -> () {
    let value = native!((), 0x643E26EA6E024D92, native_parameters!(intensity));

    value
}

pub fn get_rain_level() -> f32 {
    let value = native!(f32, 0x96695E368AD855F3, native_parameters!());

    value
}

pub fn _set_snow_level(level: f32) -> () {
    let value = native!((), 0x7F06937B0CDCBC1A, native_parameters!(level));

    value
}

pub fn get_snow_level() -> f32 {
    let value = native!(f32, 0xC5868A966E5BE3AE, native_parameters!());

    value
}

pub fn force_lightning_flash() -> () {
    let value = native!((), 0xF6062E089251C898, native_parameters!());

    value
}

pub fn _0x02deaac8f8ea7fe7(p0: &std::ffi::CString) -> () {
    let value = native!((), 0x02DEAAC8F8EA7FE7, native_parameters!(p0.as_ptr()));

    value
}

pub fn preload_cloud_hat(name: &std::ffi::CString) -> () {
    let value = native!((), 0x11B56FBBF7224868, native_parameters!(name.as_ptr()));

    value
}

pub fn load_cloud_hat(name: &std::ffi::CString, transitionTime: f32) -> () {
    let value = native!(
        (),
        0xFC4842A34657BFCB,
        native_parameters!(name.as_ptr(), transitionTime)
    );

    value
}

pub fn unload_cloud_hat(name: &std::ffi::CString, p1: f32) -> () {
    let value = native!(
        (),
        0xA74802FB8D0B7814,
        native_parameters!(name.as_ptr(), p1)
    );

    value
}

pub fn _clear_cloud_hat() -> () {
    let value = native!((), 0x957E790EA1727B64, native_parameters!());

    value
}

pub fn _set_cloud_hat_opacity(opacity: f32) -> () {
    let value = native!((), 0xF36199225D6D8C86, native_parameters!(opacity));

    value
}

pub fn _get_cloud_hat_opacity() -> f32 {
    let value = native!(f32, 0x20AC25E781AE4A84, native_parameters!());

    value
}

pub fn get_game_timer() -> i32 {
    let value = native!(i32, 0x9CD27B0045628463, native_parameters!());

    value
}

pub fn get_frame_time() -> f32 {
    let value = native!(f32, 0x15C40837039FFAF7, native_parameters!());

    value
}

pub fn _get_benchmark_time() -> f32 {
    let value = native!(f32, 0xE599A503B3837E1B, native_parameters!());

    value
}

pub fn get_frame_count() -> i32 {
    let value = native!(i32, 0xFC8202EFC642E6F2, native_parameters!());

    value
}

pub fn get_random_float_in_range(startRange: f32, endRange: f32) -> f32 {
    let value = native!(
        f32,
        0x313CE5879CEB6FCD,
        native_parameters!(startRange, endRange)
    );

    value
}

pub fn get_random_int_in_range(startRange: i32, endRange: i32) -> i32 {
    let value = native!(
        i32,
        0xD53343AA4FB7DD28,
        native_parameters!(startRange, endRange)
    );

    value
}

pub fn _get_random_int_in_range_2(startRange: i32, endRange: i32) -> i32 {
    let value = native!(
        i32,
        0xF2D49816A804D134,
        native_parameters!(startRange, endRange)
    );

    value
}

pub fn get_ground_z_for_3d_coord(
    x: f32,
    y: f32,
    z: f32,
    groundZ: *mut f32,
    ignoreWater: bool,
    p5: bool,
) -> bool {
    let value = native!(
        bool,
        0xC906A7DAB05C8D2B,
        native_parameters!(x, y, z, groundZ, ignoreWater, p5)
    );

    value
}

pub fn get_ground_z_and_normal_for_3d_coord(
    x: f32,
    y: f32,
    z: f32,
    groundZ: *mut f32,
    normal: *mut NativeVector3,
) -> bool {
    let value = native!(
        bool,
        0x8BDC7BFC57A81E76,
        native_parameters!(x, y, z, groundZ, normal)
    );

    value
}

pub fn _get_ground_z_for_3d_coord_2(
    x: f32,
    y: f32,
    z: f32,
    groundZ: *mut f32,
    p4: bool,
    p5: bool,
) -> bool {
    let value = native!(
        bool,
        0x9E82F0F362881B29,
        native_parameters!(x, y, z, groundZ, p4, p5)
    );

    value
}

pub fn asin(p0: f32) -> f32 {
    let value = native!(f32, 0xC843060B5765DCE7, native_parameters!(p0));

    value
}

pub fn acos(p0: f32) -> f32 {
    let value = native!(f32, 0x1D08B970013C34B6, native_parameters!(p0));

    value
}

pub fn tan(p0: f32) -> f32 {
    let value = native!(f32, 0x632106CC96E82E91, native_parameters!(p0));

    value
}

pub fn atan(p0: f32) -> f32 {
    let value = native!(f32, 0xA9D1795CD5043663, native_parameters!(p0));

    value
}

pub fn atan2(p0: f32, p1: f32) -> f32 {
    let value = native!(f32, 0x8927CBF9D22261A4, native_parameters!(p0, p1));

    value
}

pub fn get_distance_between_coords(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    useZ: bool,
) -> f32 {
    let value = native!(
        f32,
        0xF1B760881820C952,
        native_parameters!(x1, y1, z1, x2, y2, z2, useZ)
    );

    value
}

pub fn get_angle_between_2d_vectors(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let value = native!(f32, 0x186FC4BE848E1C92, native_parameters!(x1, y1, x2, y2));

    value
}

pub fn get_heading_from_vector_2d(dx: f32, dy: f32) -> f32 {
    let value = native!(f32, 0x2FFB6B224F4B2926, native_parameters!(dx, dy));

    value
}

pub fn _0x7f8f6405f4777af6(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
    p8: f32,
    p9: bool,
) -> f32 {
    let value = native!(
        f32,
        0x7F8F6405F4777AF6,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn _0x21c235bc64831e5a(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
    p8: f32,
    p9: bool,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x21C235BC64831E5A,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn _0xf56dfb7b61be7276(
    p0: f32,
    p1: f32,
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
    p12: *mut f32,
) -> bool {
    let value = native!(
        bool,
        0xF56DFB7B61BE7276,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12)
    );

    value
}

pub fn _0xa0ad167e4b39d9a2(
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
) -> u32 {
    let value = native!(
        u32,
        0xA0AD167E4B39D9A2,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13)
    );

    value
}

pub fn set_bit(address: *mut i32, offset: i32) -> () {
    let value = native!((), 0x933D6A9EEC1BACD0, native_parameters!(address, offset));

    value
}

pub fn clear_bit(address: *mut i32, offset: i32) -> () {
    let value = native!((), 0xE80492A9AC099A93, native_parameters!(address, offset));

    value
}

pub fn get_hash_key(string: &std::ffi::CString) -> u32 {
    let value = native!(u32, 0xD24D37CC275948CC, native_parameters!(string.as_ptr()));

    value
}

pub fn slerp_near_quaternion(
    t: f32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    x1: f32,
    y1: f32,
    z1: f32,
    w1: f32,
    outX: *mut f32,
    outY: *mut f32,
    outZ: *mut f32,
    outW: *mut f32,
) -> () {
    let value = native!(
        (),
        0xF2F6A2FA49278625,
        native_parameters!(t, x, y, z, w, x1, y1, z1, w1, outX, outY, outZ, outW)
    );

    value
}

pub fn is_area_occupied(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: bool,
    p7: bool,
    p8: bool,
    p9: bool,
    p10: bool,
    p11: u32,
    p12: bool,
) -> bool {
    let value = native!(
        bool,
        0xA61B4DF533DCB56E,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12)
    );

    value
}

pub fn _0x39455bf4f4f55186(
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
) -> u32 {
    let value = native!(
        u32,
        0x39455BF4F4F55186,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12)
    );

    value
}

pub fn is_position_occupied(
    x: f32,
    y: f32,
    z: f32,
    range: f32,
    p4: bool,
    checkVehicles: bool,
    checkPeds: bool,
    p7: bool,
    p8: bool,
    ignoreEntity: i32,
    p10: bool,
) -> bool {
    let value = native!(
        bool,
        0xADCDE75E1C60F32D,
        native_parameters!(
            x,
            y,
            z,
            range,
            p4,
            checkVehicles,
            checkPeds,
            p7,
            p8,
            ignoreEntity,
            p10
        )
    );

    value
}

pub fn is_point_obscured_by_a_mission_entity(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: u32,
) -> bool {
    let value = native!(
        bool,
        0xE54E209C35FFA18D,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn clear_area(
    X: f32,
    Y: f32,
    Z: f32,
    radius: f32,
    p4: bool,
    ignoreCopCars: bool,
    ignoreObjects: bool,
    p7: bool,
) -> () {
    let value = native!(
        (),
        0xA56F01F3765B93A0,
        native_parameters!(X, Y, Z, radius, p4, ignoreCopCars, ignoreObjects, p7)
    );

    value
}

pub fn clear_area_leave_vehicle_health(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    p4: bool,
    p5: bool,
    p6: bool,
    p7: bool,
) -> () {
    let value = native!(
        (),
        0x957838AAF91BD12D,
        native_parameters!(x, y, z, radius, p4, p5, p6, p7)
    );

    value
}

pub fn clear_area_of_vehicles(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    p4: bool,
    p5: bool,
    p6: bool,
    p7: bool,
    p8: bool,
    p9: bool,
) -> () {
    let value = native!(
        (),
        0x01C7B9B38428AEB6,
        native_parameters!(x, y, z, radius, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn clear_angled_area_of_vehicles(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    width: f32,
    p7: bool,
    p8: bool,
    p9: bool,
    p10: bool,
    p11: bool,
    p12: u32,
) -> () {
    let value = native!(
        (),
        0x11DB3500F042A8AA,
        native_parameters!(x1, y1, z1, x2, y2, z2, width, p7, p8, p9, p10, p11, p12)
    );

    value
}

pub fn clear_area_of_objects(x: f32, y: f32, z: f32, radius: f32, flags: i32) -> () {
    let value = native!(
        (),
        0xDD9B9B385AAC7F5B,
        native_parameters!(x, y, z, radius, flags)
    );

    value
}

pub fn clear_area_of_peds(x: f32, y: f32, z: f32, radius: f32, flags: i32) -> () {
    let value = native!(
        (),
        0xBE31FD6CE464AC59,
        native_parameters!(x, y, z, radius, flags)
    );

    value
}

pub fn clear_area_of_cops(x: f32, y: f32, z: f32, radius: f32, flags: i32) -> () {
    let value = native!(
        (),
        0x04F8FC8FCF58F88D,
        native_parameters!(x, y, z, radius, flags)
    );

    value
}

pub fn clear_area_of_projectiles(x: f32, y: f32, z: f32, radius: f32, flags: i32) -> () {
    let value = native!(
        (),
        0x0A1CB9094635D1A6,
        native_parameters!(x, y, z, radius, flags)
    );

    value
}

pub fn _0x7ec6f9a478a6a512() -> () {
    let value = native!((), 0x7EC6F9A478A6A512, native_parameters!());

    value
}

pub fn set_save_menu_active(ignoreVehicle: bool) -> () {
    let value = native!((), 0xC9BF75D28165FF77, native_parameters!(ignoreVehicle));

    value
}

pub fn _0x397baa01068baa96() -> i32 {
    let value = native!(i32, 0x397BAA01068BAA96, native_parameters!());

    value
}

pub fn set_credits_active(toggle: bool) -> () {
    let value = native!((), 0xB938B7E6D3C0620C, native_parameters!(toggle));

    value
}

pub fn _0xb51b9ab9ef81868c(toggle: bool) -> () {
    let value = native!((), 0xB51B9AB9EF81868C, native_parameters!(toggle));

    value
}

pub fn have_credits_reached_end() -> bool {
    let value = native!(bool, 0x075F1D57402C93BA, native_parameters!());

    value
}

pub fn terminate_all_scripts_with_this_name(scriptName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x9DC711BC69C548DF,
        native_parameters!(scriptName.as_ptr())
    );

    value
}

pub fn network_set_script_is_safe_for_network_game() -> () {
    let value = native!((), 0x9243BAC96D64C050, native_parameters!());

    value
}

pub fn add_hospital_restart(x: f32, y: f32, z: f32, p3: f32, p4: u32) -> i32 {
    let value = native!(i32, 0x1F464EF988465A81, native_parameters!(x, y, z, p3, p4));

    value
}

pub fn disable_hospital_restart(hospitalIndex: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0xC8535819C450EBA8,
        native_parameters!(hospitalIndex, toggle)
    );

    value
}

pub fn add_police_restart(p0: f32, p1: f32, p2: f32, p3: f32, p4: u32) -> u32 {
    let value = native!(
        u32,
        0x452736765B31FC4B,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn disable_police_restart(policeIndex: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0x23285DED6EBD7EA3,
        native_parameters!(policeIndex, toggle)
    );

    value
}

pub fn _set_restart_custom_position(x: f32, y: f32, z: f32, heading: f32) -> () {
    let value = native!((), 0x706B5EDCAA7FA663, native_parameters!(x, y, z, heading));

    value
}

pub fn _clear_restart_custom_position() -> () {
    let value = native!((), 0xA2716D40842EAF79, native_parameters!());

    value
}

pub fn pause_death_arrest_restart(toggle: bool) -> () {
    let value = native!((), 0x2C2B3493FBF51C71, native_parameters!(toggle));

    value
}

pub fn ignore_next_restart(toggle: bool) -> () {
    let value = native!((), 0x21FFB63D8C615361, native_parameters!(toggle));

    value
}

pub fn set_fade_out_after_death(toggle: bool) -> () {
    let value = native!((), 0x4A18E01DF2C87B86, native_parameters!(toggle));

    value
}

pub fn set_fade_out_after_arrest(toggle: bool) -> () {
    let value = native!((), 0x1E0B4DC0D990A4E7, native_parameters!(toggle));

    value
}

pub fn set_fade_in_after_death_arrest(toggle: bool) -> () {
    let value = native!((), 0xDA66D2796BA33F12, native_parameters!(toggle));

    value
}

pub fn set_fade_in_after_load(toggle: bool) -> () {
    let value = native!((), 0xF3D78F59DFE18D79, native_parameters!(toggle));

    value
}

pub fn register_save_house(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: *mut u32,
    p5: u32,
    p6: u32,
) -> u32 {
    let value = native!(
        u32,
        0xC0714D0A7EEECA54,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn set_save_house(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x4F548CABEAE553BC, native_parameters!(p0, p1, p2));

    value
}

pub fn override_save_house(
    p0: bool,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: bool,
    p6: f32,
    p7: f32,
) -> bool {
    let value = native!(
        bool,
        0x1162EA8AE9D24EEA,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn _0xa4a0065e39c9f25c(
    p0: *mut NativeVector3,
    p1: *mut f32,
    fadeInAfterLoad: *mut bool,
    p3: *mut bool,
) -> bool {
    let value = native!(
        bool,
        0xA4A0065E39C9F25C,
        native_parameters!(p0, p1, fadeInAfterLoad, p3)
    );

    value
}

pub fn do_auto_save() -> () {
    let value = native!((), 0x50EEAAD86232EE55, native_parameters!());

    value
}

pub fn get_is_auto_save_off() -> bool {
    let value = native!(bool, 0x6E04F06094C87047, native_parameters!());

    value
}

pub fn is_auto_save_in_progress() -> bool {
    let value = native!(bool, 0x69240733738C19A0, native_parameters!());

    value
}

pub fn _0x2107a3773771186d() -> bool {
    let value = native!(bool, 0x2107A3773771186D, native_parameters!());

    value
}

pub fn _0x06462a961e94b67c() -> () {
    let value = native!((), 0x06462A961E94B67C, native_parameters!());

    value
}

pub fn begin_replay_stats(p0: u32, p1: u32) -> () {
    let value = native!((), 0xE0E500246FF73D66, native_parameters!(p0, p1));

    value
}

pub fn add_replay_stat_value(value: u32) -> () {
    let value = native!((), 0x69FE6DC87BD2A5E9, native_parameters!(value));

    value
}

pub fn end_replay_stats() -> () {
    let value = native!((), 0xA23E821FBDF8A5F2, native_parameters!());

    value
}

pub fn _0xd642319c54aadeb6() -> u32 {
    let value = native!(u32, 0xD642319C54AADEB6, native_parameters!());

    value
}

pub fn _0x5b1f2e327b6b6fe1() -> u32 {
    let value = native!(u32, 0x5B1F2E327B6B6FE1, native_parameters!());

    value
}

pub fn get_replay_stat_mission_type() -> i32 {
    let value = native!(i32, 0x2B626A0150E4D449, native_parameters!());

    value
}

pub fn get_replay_stat_count() -> i32 {
    let value = native!(i32, 0xDC9274A7EF6B2867, native_parameters!());

    value
}

pub fn get_replay_stat_at_index(index: i32) -> i32 {
    let value = native!(i32, 0x8098C8D6597AAE18, native_parameters!(index));

    value
}

pub fn clear_replay_stats() -> () {
    let value = native!((), 0x1B1AB132A16FDA55, native_parameters!());

    value
}

pub fn _0x72de52178c291cb5() -> u32 {
    let value = native!(u32, 0x72DE52178C291CB5, native_parameters!());

    value
}

pub fn _0x44a0bdc559b35f6e() -> bool {
    let value = native!(bool, 0x44A0BDC559B35F6E, native_parameters!());

    value
}

pub fn _0xeb2104e905c6f2e9() -> u32 {
    let value = native!(u32, 0xEB2104E905C6F2E9, native_parameters!());

    value
}

pub fn _0x2b5e102e4a42f2bf() -> u32 {
    let value = native!(u32, 0x2B5E102E4A42F2BF, native_parameters!());

    value
}

pub fn is_memory_card_in_use() -> bool {
    let value = native!(bool, 0x8A75CE2956274ADD, native_parameters!());

    value
}

pub fn shoot_single_bullet_between_coords(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    damage: i32,
    p7: bool,
    weaponHash: u32,
    ownerPed: i32,
    isAudible: bool,
    isInvisible: bool,
    speed: f32,
) -> () {
    let value = native!(
        (),
        0x867654CBC7606F2C,
        native_parameters!(
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
            damage,
            p7,
            weaponHash,
            ownerPed,
            isAudible,
            isInvisible,
            speed
        )
    );

    value
}

pub fn shoot_single_bullet_between_coords_ignore_entity(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    damage: i32,
    p7: bool,
    weaponHash: u32,
    ownerPed: i32,
    isAudible: bool,
    isInvisible: bool,
    speed: f32,
    entity: i32,
    p14: u32,
) -> () {
    let value = native!(
        (),
        0xE3A7742E0B7A2F8B,
        native_parameters!(
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
            damage,
            p7,
            weaponHash,
            ownerPed,
            isAudible,
            isInvisible,
            speed,
            entity,
            p14
        )
    );

    value
}

pub fn shoot_single_bullet_between_coords_ignore_entity_new(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    damage: i32,
    p7: bool,
    weaponHash: u32,
    ownerPed: i32,
    isAudible: bool,
    isInvisible: bool,
    speed: f32,
    entity: i32,
    p14: bool,
    p15: bool,
    p16: bool,
    p17: bool,
    p18: u32,
    p19: u32,
    p20: u32,
) -> () {
    let value = native!(
        (),
        0xBFE5756E7407064A,
        native_parameters!(
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
            damage,
            p7,
            weaponHash,
            ownerPed,
            isAudible,
            isInvisible,
            speed,
            entity,
            p14,
            p15,
            p16,
            p17,
            p18,
            p19,
            p20
        )
    );

    value
}

pub fn get_model_dimensions(
    modelHash: u32,
    minimum: *mut NativeVector3,
    maximum: *mut NativeVector3,
) -> () {
    let value = native!(
        (),
        0x03E8D3D5F549087A,
        native_parameters!(modelHash, minimum, maximum)
    );

    value
}

pub fn set_fake_wanted_level(fakeWantedLevel: i32) -> () {
    let value = native!((), 0x1454F2448DE30163, native_parameters!(fakeWantedLevel));

    value
}

pub fn get_fake_wanted_level() -> i32 {
    let value = native!(i32, 0x4C9296CBCD1B971E, native_parameters!());

    value
}

pub fn is_bit_set(address: i32, offset: i32) -> bool {
    let value = native!(
        bool,
        0xA921AA820C25702F,
        native_parameters!(address, offset)
    );

    value
}

pub fn using_mission_creator(toggle: bool) -> () {
    let value = native!((), 0xF14878FC50BEC6EE, native_parameters!(toggle));

    value
}

pub fn allow_mission_creator_warp(toggle: bool) -> () {
    let value = native!((), 0xDEA36202FC3382DF, native_parameters!(toggle));

    value
}

pub fn set_minigame_in_progress(toggle: bool) -> () {
    let value = native!((), 0x19E00D7322C6F85B, native_parameters!(toggle));

    value
}

pub fn is_minigame_in_progress() -> bool {
    let value = native!(bool, 0x2B4A15E44DE0F478, native_parameters!());

    value
}

pub fn is_this_a_minigame_script() -> bool {
    let value = native!(bool, 0x7B30F65D7B710098, native_parameters!());

    value
}

pub fn is_sniper_inverted() -> bool {
    let value = native!(bool, 0x61A23B7EDA9BDA24, native_parameters!());

    value
}

pub fn should_use_metric_measurements() -> bool {
    let value = native!(bool, 0xD3D15555431AB793, native_parameters!());

    value
}

pub fn get_profile_setting(profileSetting: i32) -> i32 {
    let value = native!(i32, 0xC488FF2356EA7791, native_parameters!(profileSetting));

    value
}

pub fn are_strings_equal(string1: &std::ffi::CString, string2: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x0C515FAB3FF9EA92,
        native_parameters!(string1.as_ptr(), string2.as_ptr())
    );

    value
}

pub fn compare_strings(
    str1: &std::ffi::CString,
    str2: &std::ffi::CString,
    matchCase: bool,
    maxLength: i32,
) -> i32 {
    let value = native!(
        i32,
        0x1E34710ECD4AB0EB,
        native_parameters!(str1.as_ptr(), str2.as_ptr(), matchCase, maxLength)
    );

    value
}

pub fn absi(value: i32) -> i32 {
    let value = native!(i32, 0xF0D31AD191A74F87, native_parameters!(value));

    value
}

pub fn absf(value: f32) -> f32 {
    let value = native!(f32, 0x73D57CFFDD12C355, native_parameters!(value));

    value
}

pub fn is_sniper_bullet_in_area(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
    let value = native!(
        bool,
        0xFEFCF11B01287125,
        native_parameters!(x1, y1, z1, x2, y2, z2)
    );

    value
}

pub fn is_projectile_in_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    ownedByPlayer: bool,
) -> bool {
    let value = native!(
        bool,
        0x5270A8FBC098C3F8,
        native_parameters!(x1, y1, z1, x2, y2, z2, ownedByPlayer)
    );

    value
}

pub fn is_projectile_type_in_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    type_esc: i32,
    ownedByPlayer: bool,
) -> bool {
    let value = native!(
        bool,
        0x2E0DC353342C4A6D,
        native_parameters!(x1, y1, z1, x2, y2, z2, type_esc, ownedByPlayer)
    );

    value
}

pub fn is_projectile_type_in_angled_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    width: f32,
    p7: u32,
    ownedByPlayer: bool,
) -> bool {
    let value = native!(
        bool,
        0xF0BC12401061DEA0,
        native_parameters!(x1, y1, z1, x2, y2, z2, width, p7, ownedByPlayer)
    );

    value
}

pub fn is_projectile_type_within_distance(
    x: f32,
    y: f32,
    z: f32,
    projectileHash: u32,
    radius: f32,
    ownedByPlayer: bool,
) -> bool {
    let value = native!(
        bool,
        0x34318593248C8FB2,
        native_parameters!(x, y, z, projectileHash, radius, ownedByPlayer)
    );

    value
}

pub fn get_coords_of_projectile_type_in_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    projectileHash: u32,
    projectilePos: *mut NativeVector3,
    ownedByPlayer: bool,
) -> bool {
    let value = native!(
        bool,
        0x8D7A43EC6A5FEA45,
        native_parameters!(
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
            projectileHash,
            projectilePos,
            ownedByPlayer
        )
    );

    value
}

pub fn get_coords_of_projectile_type_within_distance(
    ped: i32,
    weaponHash: u32,
    radius: f32,
    entity: *mut i32,
    ownedByPlayer: bool,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xDFB4138EEFED7B81,
        native_parameters!(ped, weaponHash, radius, entity, ownedByPlayer)
    );

    value
}

pub fn _get_projectile_near_ped(
    ped: i32,
    weaponhash: u32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: bool,
) -> bool {
    let value = native!(
        bool,
        0x82FDE6A57EE4EE44,
        native_parameters!(ped, weaponhash, p2, p3, p4, p5)
    );

    value
}

pub fn is_bullet_in_angled_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    width: f32,
    ownedByPlayer: bool,
) -> bool {
    let value = native!(
        bool,
        0x1A8B5F3C01E2B477,
        native_parameters!(x1, y1, z1, x2, y2, z2, width, ownedByPlayer)
    );

    value
}

pub fn is_bullet_in_area(x: f32, y: f32, z: f32, radius: f32, ownedByPlayer: bool) -> bool {
    let value = native!(
        bool,
        0x3F2023999AD51C1F,
        native_parameters!(x, y, z, radius, ownedByPlayer)
    );

    value
}

pub fn is_bullet_in_box(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    ownedByPlayer: bool,
) -> bool {
    let value = native!(
        bool,
        0xDE0F6D7450D37351,
        native_parameters!(x1, y1, z1, x2, y2, z2, ownedByPlayer)
    );

    value
}

pub fn has_bullet_impacted_in_area(x: f32, y: f32, z: f32, p3: f32, p4: bool, p5: bool) -> bool {
    let value = native!(
        bool,
        0x9870ACFB89A90995,
        native_parameters!(x, y, z, p3, p4, p5)
    );

    value
}

pub fn has_bullet_impacted_in_box(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: bool,
    p7: bool,
) -> bool {
    let value = native!(
        bool,
        0xDC8C5D7CFEAB8394,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn is_orbis_version() -> bool {
    let value = native!(bool, 0xA72BC0B675B1519E, native_parameters!());

    value
}

pub fn is_durango_version() -> bool {
    let value = native!(bool, 0x4D982ADB1978442D, native_parameters!());

    value
}

pub fn is_xbox360_version() -> bool {
    let value = native!(bool, 0xF6201B4DAF662A9D, native_parameters!());

    value
}

pub fn is_ps3_version() -> bool {
    let value = native!(bool, 0xCCA1072C29D096C2, native_parameters!());

    value
}

pub fn is_pc_version() -> bool {
    let value = native!(bool, 0x48AF36444B965238, native_parameters!());

    value
}

pub fn is_aussie_version() -> bool {
    let value = native!(bool, 0x9F1935CA1F724008, native_parameters!());

    value
}

pub fn is_string_null(string: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xF22B6C47C6EAB066,
        native_parameters!(string.as_ptr())
    );

    value
}

pub fn is_string_null_or_empty(string: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xCA042B6957743895,
        native_parameters!(string.as_ptr())
    );

    value
}

pub fn string_to_int(string: &std::ffi::CString, outInteger: *mut i32) -> bool {
    let value = native!(
        bool,
        0x5A5F40FE637EB584,
        native_parameters!(string.as_ptr(), outInteger)
    );

    value
}

pub fn set_bits_in_range(var: *mut i32, rangeStart: i32, rangeEnd: i32, p3: i32) -> () {
    let value = native!(
        (),
        0x8EF07E15701D61ED,
        native_parameters!(var, rangeStart, rangeEnd, p3)
    );

    value
}

pub fn get_bits_in_range(var: i32, rangeStart: i32, rangeEnd: i32) -> i32 {
    let value = native!(
        i32,
        0x53158863FCC0893A,
        native_parameters!(var, rangeStart, rangeEnd)
    );

    value
}

pub fn add_stunt_jump(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    x3: f32,
    y3: f32,
    z3: f32,
    x4: f32,
    y4: f32,
    z4: f32,
    camX: f32,
    camY: f32,
    camZ: f32,
    p15: i32,
    p16: i32,
    p17: i32,
) -> i32 {
    let value = native!(
        i32,
        0x1A992DA297A4630C,
        native_parameters!(
            x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, camX, camY, camZ, p15, p16, p17
        )
    );

    value
}

pub fn add_stunt_jump_angled(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    radius1: f32,
    x3: f32,
    y3: f32,
    z3: f32,
    x4: f32,
    y4: f32,
    z4: f32,
    radius2: f32,
    camX: f32,
    camY: f32,
    camZ: f32,
    p17: i32,
    p18: i32,
    p19: i32,
) -> i32 {
    let value = native!(
        i32,
        0xBBE5D803A5360CBF,
        native_parameters!(
            x1, y1, z1, x2, y2, z2, radius1, x3, y3, z3, x4, y4, z4, radius2, camX, camY, camZ,
            p17, p18, p19
        )
    );

    value
}

pub fn _0xfb80ab299d2ee1bd(toggle: bool) -> () {
    let value = native!((), 0xFB80AB299D2EE1BD, native_parameters!(toggle));

    value
}

pub fn delete_stunt_jump(p0: i32) -> () {
    let value = native!((), 0xDC518000E39DAE1F, native_parameters!(p0));

    value
}

pub fn enable_stunt_jump_set(p0: i32) -> () {
    let value = native!((), 0xE369A5783B866016, native_parameters!(p0));

    value
}

pub fn disable_stunt_jump_set(p0: i32) -> () {
    let value = native!((), 0xA5272EBEDD4747F6, native_parameters!(p0));

    value
}

pub fn set_stunt_jumps_can_trigger(toggle: bool) -> () {
    let value = native!((), 0xD79185689F8FD5DF, native_parameters!(toggle));

    value
}

pub fn is_stunt_jump_in_progress() -> bool {
    let value = native!(bool, 0x7A3F19700A4D0525, native_parameters!());

    value
}

pub fn is_stunt_jump_message_showing() -> bool {
    let value = native!(bool, 0x2272B0A1343129F4, native_parameters!());

    value
}

pub fn get_num_successful_stunt_jumps() -> i32 {
    let value = native!(i32, 0x996DD1E1E02F1008, native_parameters!());

    value
}

pub fn get_total_successful_stunt_jumps() -> i32 {
    let value = native!(i32, 0x6856EC3D35C81EA4, native_parameters!());

    value
}

pub fn cancel_stunt_jump() -> () {
    let value = native!((), 0xE6B7B0ACD4E4B75E, native_parameters!());

    value
}

pub fn set_game_paused(toggle: bool) -> () {
    let value = native!((), 0x577D1284D6873711, native_parameters!(toggle));

    value
}

pub fn set_this_script_can_be_paused(toggle: bool) -> () {
    let value = native!((), 0xAA391C728106F7AF, native_parameters!(toggle));

    value
}

pub fn set_this_script_can_remove_blips_created_by_any_script(toggle: bool) -> () {
    let value = native!((), 0xB98236CAAECEF897, native_parameters!(toggle));

    value
}

pub fn _has_button_combination_just_been_entered(hash: u32, amount: i32) -> bool {
    let value = native!(bool, 0x071E2A839DE82D90, native_parameters!(hash, amount));

    value
}

pub fn _has_cheat_string_just_been_entered(hash: u32) -> bool {
    let value = native!(bool, 0x557E43C447E700A8, native_parameters!(hash));

    value
}

pub fn _0xfa3ffb0eebc288a3(p0: bool) -> () {
    let value = native!((), 0xFA3FFB0EEBC288A3, native_parameters!(p0));

    value
}

pub fn set_instance_priority_mode(p0: i32) -> () {
    let value = native!((), 0x9BAE5AD2508DF078, native_parameters!(p0));

    value
}

pub fn set_instance_priority_hint(flag: i32) -> () {
    let value = native!((), 0xC5F0A8EBD3F361CE, native_parameters!(flag));

    value
}

pub fn is_frontend_fading() -> bool {
    let value = native!(bool, 0x7EA2B6AF97ECA6ED, native_parameters!());

    value
}

pub fn populate_now() -> () {
    let value = native!((), 0x7472BB270D7B4F3E, native_parameters!());

    value
}

pub fn get_index_of_current_level() -> i32 {
    let value = native!(i32, 0xCBAD6729F7B1F4FC, native_parameters!());

    value
}

pub fn set_gravity_level(level: i32) -> () {
    let value = native!((), 0x740E14FAD5842351, native_parameters!(level));

    value
}

pub fn start_save_data(p0: *mut u32, p1: u32, p2: bool) -> () {
    let value = native!((), 0xA9575F812C6A7997, native_parameters!(p0, p1, p2));

    value
}

pub fn stop_save_data() -> () {
    let value = native!((), 0x74E20C9145FB66FD, native_parameters!());

    value
}

pub fn get_size_of_save_data(p0: bool) -> i32 {
    let value = native!(i32, 0xA09F896CE912481F, native_parameters!(p0));

    value
}

pub fn register_int_to_save(p0: *mut u32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x34C9EE5986258415,
        native_parameters!(p0, name.as_ptr())
    );

    value
}

pub fn _register_int64_to_save(p0: *mut u32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xA735353C77334EA0,
        native_parameters!(p0, name.as_ptr())
    );

    value
}

pub fn register_enum_to_save(p0: *mut u32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x10C2FA78D0E128A1,
        native_parameters!(p0, name.as_ptr())
    );

    value
}

pub fn register_float_to_save(p0: *mut u32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x7CAEC29ECB5DFEBB,
        native_parameters!(p0, name.as_ptr())
    );

    value
}

pub fn register_bool_to_save(p0: *mut u32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xC8F4131414C835A1,
        native_parameters!(p0, name.as_ptr())
    );

    value
}

pub fn register_text_label_to_save(p0: *mut u32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xEDB1232C5BEAE62F,
        native_parameters!(p0, name.as_ptr())
    );

    value
}

pub fn _register_text_label_to_save_2(p0: *mut u32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x6F7794F28C6B2535,
        native_parameters!(p0, name.as_ptr())
    );

    value
}

pub fn _0x48f069265a0e4bec(p0: *mut u32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x48F069265A0E4BEC,
        native_parameters!(p0, name.as_ptr())
    );

    value
}

pub fn _0x8269816f6cfd40f8(p0: *mut u32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x8269816F6CFD40F8,
        native_parameters!(p0, name.as_ptr())
    );

    value
}

pub fn _0xfaa457ef263e8763(p0: *mut u32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xFAA457EF263E8763,
        native_parameters!(p0, name.as_ptr())
    );

    value
}

pub fn start_save_struct_with_size(p0: *mut u32, size: i32, structName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xBF737600CDDBEADD,
        native_parameters!(p0, size, structName.as_ptr())
    );

    value
}

pub fn stop_save_struct() -> () {
    let value = native!((), 0xEB1774DF12BB9F12, native_parameters!());

    value
}

pub fn start_save_array_with_size(p0: *mut u32, size: i32, arrayName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x60FE567DF1B1AF9D,
        native_parameters!(p0, size, arrayName.as_ptr())
    );

    value
}

pub fn stop_save_array() -> () {
    let value = native!((), 0x04456F95153C6BE4, native_parameters!());

    value
}

pub fn _copy_memory(dst: *mut u32, src: *mut u32, size: i32) -> () {
    let value = native!((), 0x213AEB2B90CBA7AC, native_parameters!(dst, src, size));

    value
}

pub fn enable_dispatch_service(dispatchService: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0xDC0F817884CDD856,
        native_parameters!(dispatchService, toggle)
    );

    value
}

pub fn block_dispatch_service_resource_creation(dispatchService: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0x9B2BD3773123EA2F,
        native_parameters!(dispatchService, toggle)
    );

    value
}

pub fn _get_num_dispatched_units_for_player(dispatchService: i32) -> i32 {
    let value = native!(i32, 0xEB4A0C2D56441717, native_parameters!(dispatchService));

    value
}

pub fn create_incident(
    dispatchService: i32,
    x: f32,
    y: f32,
    z: f32,
    numUnits: i32,
    radius: f32,
    outIncidentID: *mut i32,
    p7: u32,
    p8: u32,
) -> bool {
    let value = native!(
        bool,
        0x3F892CAF67444AE7,
        native_parameters!(
            dispatchService,
            x,
            y,
            z,
            numUnits,
            radius,
            outIncidentID,
            p7,
            p8
        )
    );

    value
}

pub fn create_incident_with_entity(
    dispatchService: i32,
    ped: i32,
    numUnits: i32,
    radius: f32,
    outIncidentID: *mut i32,
    p5: u32,
    p6: u32,
) -> bool {
    let value = native!(
        bool,
        0x05983472F0494E60,
        native_parameters!(
            dispatchService,
            ped,
            numUnits,
            radius,
            outIncidentID,
            p5,
            p6
        )
    );

    value
}

pub fn delete_incident(incidentId: i32) -> () {
    let value = native!((), 0x556C1AA270D5A207, native_parameters!(incidentId));

    value
}

pub fn is_incident_valid(incidentId: i32) -> bool {
    let value = native!(bool, 0xC8BC6461E629BEAA, native_parameters!(incidentId));

    value
}

pub fn set_incident_requested_units(incidentId: i32, dispatchService: i32, numUnits: i32) -> () {
    let value = native!(
        (),
        0xB08B85D860E7BA3C,
        native_parameters!(incidentId, dispatchService, numUnits)
    );

    value
}

pub fn _set_incident_unk(incidentId: i32, p1: f32) -> () {
    let value = native!((), 0xD261BA3E7E998072, native_parameters!(incidentId, p1));

    value
}

pub fn find_spawn_point_in_direction(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    distance: f32,
    spawnPoint: *mut NativeVector3,
) -> bool {
    let value = native!(
        bool,
        0x6874E2190B0C1972,
        native_parameters!(x1, y1, z1, x2, y2, z2, distance, spawnPoint)
    );

    value
}

pub fn add_pop_multiplier_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    p6: f32,
    p7: f32,
    p8: bool,
    p9: bool,
) -> i32 {
    let value = native!(
        i32,
        0x67F6413D3220E18D,
        native_parameters!(x1, y1, z1, x2, y2, z2, p6, p7, p8, p9)
    );

    value
}

pub fn does_pop_multiplier_area_exist(id: i32) -> bool {
    let value = native!(bool, 0x1327E2FE9746BAEE, native_parameters!(id));

    value
}

pub fn remove_pop_multiplier_area(id: i32, p1: bool) -> () {
    let value = native!((), 0xB129E447A2EDA4BF, native_parameters!(id, p1));

    value
}

pub fn _is_pop_multiplier_area_unk(id: i32) -> bool {
    let value = native!(bool, 0x1312F4B242609CE3, native_parameters!(id));

    value
}

pub fn add_pop_multiplier_sphere(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    pedMultiplier: f32,
    vehicleMultiplier: f32,
    p6: bool,
    p7: bool,
) -> i32 {
    let value = native!(
        i32,
        0x32C7A7E8C43A1F80,
        native_parameters!(x, y, z, radius, pedMultiplier, vehicleMultiplier, p6, p7)
    );

    value
}

pub fn does_pop_multiplier_sphere_exist(id: i32) -> bool {
    let value = native!(bool, 0x171BAFB3C60389F4, native_parameters!(id));

    value
}

pub fn remove_pop_multiplier_sphere(id: i32, p1: bool) -> () {
    let value = native!((), 0xE6869BECDD8F2403, native_parameters!(id, p1));

    value
}

pub fn enable_tennis_mode(ped: i32, toggle: bool, p2: bool) -> () {
    let value = native!((), 0x28A04B411933F8A6, native_parameters!(ped, toggle, p2));

    value
}

pub fn is_tennis_mode(ped: i32) -> bool {
    let value = native!(bool, 0x5D5479D115290C3F, native_parameters!(ped));

    value
}

pub fn play_tennis_swing_anim(
    ped: i32,
    animDict: &std::ffi::CString,
    animName: &std::ffi::CString,
    p3: f32,
    p4: f32,
    p5: bool,
) -> () {
    let value = native!(
        (),
        0xE266ED23311F24D4,
        native_parameters!(ped, animDict.as_ptr(), animName.as_ptr(), p3, p4, p5)
    );

    value
}

pub fn get_tennis_swing_anim_complete(ped: i32) -> bool {
    let value = native!(bool, 0x17DF68D720AA77F8, native_parameters!(ped));

    value
}

pub fn _0x19bfed045c647c49(ped: i32) -> bool {
    let value = native!(bool, 0x19BFED045C647C49, native_parameters!(ped));

    value
}

pub fn _0xe95b0c7d5ba3b96b(ped: i32) -> bool {
    let value = native!(bool, 0xE95B0C7D5BA3B96B, native_parameters!(ped));

    value
}

pub fn play_tennis_dive_anim(ped: i32, p1: i32, p2: f32, p3: f32, p4: f32, p5: bool) -> () {
    let value = native!(
        (),
        0x8FA9C42FC5D7C64B,
        native_parameters!(ped, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _0x54f157e0336a3822(ped: i32, p1: &std::ffi::CString, p2: f32) -> () {
    let value = native!(
        (),
        0x54F157E0336A3822,
        native_parameters!(ped, p1.as_ptr(), p2)
    );

    value
}

pub fn _reset_dispatch_spawn_location() -> () {
    let value = native!((), 0x5896F2BD5683A4E1, native_parameters!());

    value
}

pub fn set_dispatch_spawn_location(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xD10F442036302D50, native_parameters!(x, y, z));

    value
}

pub fn reset_dispatch_ideal_spawn_distance() -> () {
    let value = native!((), 0x77A84429DD9F0A15, native_parameters!());

    value
}

pub fn set_dispatch_ideal_spawn_distance(p0: f32) -> () {
    let value = native!((), 0x6FE601A64180D423, native_parameters!(p0));

    value
}

pub fn reset_dispatch_time_between_spawn_attempts(p0: u32) -> () {
    let value = native!((), 0xEB2DB0CAD13154B3, native_parameters!(p0));

    value
}

pub fn set_dispatch_time_between_spawn_attempts(p0: u32, p1: f32) -> () {
    let value = native!((), 0x44F7CBC1BEB3327D, native_parameters!(p0, p1));

    value
}

pub fn set_dispatch_time_between_spawn_attempts_multiplier(p0: u32, p1: f32) -> () {
    let value = native!((), 0x48838ED9937A15D1, native_parameters!(p0, p1));

    value
}

pub fn _add_dispatch_spawn_blocking_angled_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    width: f32,
) -> u32 {
    let value = native!(
        u32,
        0x918C7B2D2FF3928B,
        native_parameters!(x1, y1, z1, x2, y2, z2, width)
    );

    value
}

pub fn _add_dispatch_spawn_blocking_area(x1: f32, y1: f32, x2: f32, y2: f32) -> u32 {
    let value = native!(u32, 0x2D4259F1FEB81DA9, native_parameters!(x1, y1, x2, y2));

    value
}

pub fn remove_dispatch_spawn_blocking_area(p0: u32) -> () {
    let value = native!((), 0x264AC28B01B353A5, native_parameters!(p0));

    value
}

pub fn reset_dispatch_spawn_blocking_areas() -> () {
    let value = native!((), 0xAC7BFD5C1D83EA75, native_parameters!());

    value
}

pub fn _0xd9f692d349249528() -> () {
    let value = native!((), 0xD9F692D349249528, native_parameters!());

    value
}

pub fn _0xe532ec1a63231b4f(p0: i32, p1: i32) -> () {
    let value = native!((), 0xE532EC1A63231B4F, native_parameters!(p0, p1));

    value
}

pub fn _add_tactical_analysis_point(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xB8721407EE9C3FF6, native_parameters!(x, y, z));

    value
}

pub fn _clear_tactical_analysis_points() -> () {
    let value = native!((), 0xB3CD58CCA6CDA852, native_parameters!());

    value
}

pub fn set_riot_mode_enabled(toggle: bool) -> () {
    let value = native!((), 0x2587A48BC88DFADF, native_parameters!(toggle));

    value
}

pub fn display_onscreen_keyboard_with_longer_initial_string(
    p0: i32,
    windowTitle: &std::ffi::CString,
    p2: *mut u32,
    defaultText: &std::ffi::CString,
    defaultConcat1: &std::ffi::CString,
    defaultConcat2: &std::ffi::CString,
    defaultConcat3: &std::ffi::CString,
    defaultConcat4: &std::ffi::CString,
    defaultConcat5: &std::ffi::CString,
    defaultConcat6: &std::ffi::CString,
    defaultConcat7: &std::ffi::CString,
    maxInputLength: i32,
) -> () {
    let value = native!(
        (),
        0xCA78CFA0366592FE,
        native_parameters!(
            p0,
            windowTitle.as_ptr(),
            p2,
            defaultText.as_ptr(),
            defaultConcat1.as_ptr(),
            defaultConcat2.as_ptr(),
            defaultConcat3.as_ptr(),
            defaultConcat4.as_ptr(),
            defaultConcat5.as_ptr(),
            defaultConcat6.as_ptr(),
            defaultConcat7.as_ptr(),
            maxInputLength
        )
    );

    value
}

pub fn display_onscreen_keyboard(
    p0: i32,
    windowTitle: &std::ffi::CString,
    p2: &std::ffi::CString,
    defaultText: &std::ffi::CString,
    defaultConcat1: &std::ffi::CString,
    defaultConcat2: &std::ffi::CString,
    defaultConcat3: &std::ffi::CString,
    maxInputLength: i32,
) -> () {
    let value = native!(
        (),
        0x00DC833F2568DBF6,
        native_parameters!(
            p0,
            windowTitle.as_ptr(),
            p2.as_ptr(),
            defaultText.as_ptr(),
            defaultConcat1.as_ptr(),
            defaultConcat2.as_ptr(),
            defaultConcat3.as_ptr(),
            maxInputLength
        )
    );

    value
}

pub fn update_onscreen_keyboard() -> i32 {
    let value = native!(i32, 0x0CF2B696BBF945AE, native_parameters!());

    value
}

pub fn get_onscreen_keyboard_result() -> String {
    let value = native!(*const i8, 0x8362B09B91893647, native_parameters!());
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _cancel_onscreen_keyboard() -> () {
    let value = native!((), 0x58A39BE597CE99CD, native_parameters!());

    value
}

pub fn next_onscreen_keyboard_result_will_display_using_these_fonts(p0: i32) -> () {
    let value = native!((), 0x3ED1438C1F5C6612, native_parameters!(p0));

    value
}

pub fn _remove_stealth_kill(hash: u32, p1: bool) -> () {
    let value = native!((), 0xA6A12939F16D85BE, native_parameters!(hash, p1));

    value
}

pub fn _0x1eae0a6e978894a2(p0: i32, p1: bool) -> () {
    let value = native!((), 0x1EAE0A6E978894A2, native_parameters!(p0, p1));

    value
}

pub fn set_explosive_ammo_this_frame(player: i32) -> () {
    let value = native!((), 0xA66C71C98D5F2CFB, native_parameters!(player));

    value
}

pub fn set_fire_ammo_this_frame(player: i32) -> () {
    let value = native!((), 0x11879CDD803D30F4, native_parameters!(player));

    value
}

pub fn set_explosive_melee_this_frame(player: i32) -> () {
    let value = native!((), 0xFF1BED81BFDC0FE0, native_parameters!(player));

    value
}

pub fn set_super_jump_this_frame(player: i32) -> () {
    let value = native!((), 0x57FFF03E423A4C0B, native_parameters!(player));

    value
}

pub fn _set_beast_mode_active(player: i32) -> () {
    let value = native!((), 0x438822C279B73B93, native_parameters!(player));

    value
}

pub fn _set_force_player_to_jump(player: i32) -> () {
    let value = native!((), 0xA1183BCFEE0F93D1, native_parameters!(player));

    value
}

pub fn _0x6fddf453c0c756ec() -> bool {
    let value = native!(bool, 0x6FDDF453C0C756EC, native_parameters!());

    value
}

pub fn _0xfb00ca71da386228() -> () {
    let value = native!((), 0xFB00CA71DA386228, native_parameters!());

    value
}

pub fn are_profile_settings_valid() -> bool {
    let value = native!(bool, 0x5AA3BEFA29F03AD4, native_parameters!());

    value
}

pub fn _0xe3d969d2785ffb5e() -> () {
    let value = native!((), 0xE3D969D2785FFB5E, native_parameters!());

    value
}

pub fn force_game_state_playing() -> () {
    let value = native!((), 0xC0AA53F866B3134D, native_parameters!());

    value
}

pub fn script_race_init(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x0A60017F841A54F2, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn script_race_shutdown() -> () {
    let value = native!((), 0x1FF6BF9A63E5757F, native_parameters!());

    value
}

pub fn _0x1bb299305c3e8c13(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x1BB299305C3E8C13, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn script_race_get_player_split_time(player: i32, p1: *mut i32, p2: *mut i32) -> bool {
    let value = native!(bool, 0x8EF5573A1F801A5C, native_parameters!(player, p1, p2));

    value
}

pub fn _start_benchmark_recording() -> () {
    let value = native!((), 0x92790862E36C2ADA, native_parameters!());

    value
}

pub fn _stop_benchmark_recording() -> () {
    let value = native!((), 0xC7DB36C24634F52B, native_parameters!());

    value
}

pub fn _reset_benchmark_recording() -> () {
    let value = native!((), 0x437138B6A830166A, native_parameters!());

    value
}

pub fn _save_benchmark_recording() -> () {
    let value = native!((), 0x37DEB0AA183FB6D8, native_parameters!());

    value
}

pub fn _ui_is_singleplayer_pause_menu_active() -> bool {
    let value = native!(bool, 0xEA2F2061875EED90, native_parameters!());

    value
}

pub fn _landing_menu_is_active() -> bool {
    let value = native!(bool, 0x3BBBD13E5041A79E, native_parameters!());

    value
}

pub fn _is_command_line_benchmark_value_set() -> bool {
    let value = native!(bool, 0xA049A5BE0F04F2F8, native_parameters!());

    value
}

pub fn _get_benchmark_iterations_from_command_line() -> i32 {
    let value = native!(i32, 0x4750FC27570311EC, native_parameters!());

    value
}

pub fn _get_benchmark_pass_from_command_line() -> i32 {
    let value = native!(i32, 0x1B2366C3F2A5C8DF, native_parameters!());

    value
}

pub fn _restart_game() -> () {
    let value = native!((), 0xE574A662ACAEFBB1, native_parameters!());

    value
}

pub fn _force_social_club_update() -> () {
    let value = native!((), 0xEB6891F03362FB12, native_parameters!());

    value
}

pub fn _has_async_install_finished() -> bool {
    let value = native!(bool, 0x14832BF2ABA53FC5, native_parameters!());

    value
}

pub fn _cleanup_async_install() -> () {
    let value = native!((), 0xC79AE21974B01FB2, native_parameters!());

    value
}

pub fn _is_in_power_saving_mode() -> bool {
    let value = native!(bool, 0x684A41975F077262, native_parameters!());

    value
}

pub fn _get_power_saving_mode_duration() -> i32 {
    let value = native!(i32, 0xABB2FA71C83A1B72, native_parameters!());

    value
}

pub fn _set_player_is_in_animal_form(toggle: bool) -> () {
    let value = native!((), 0x4EBB7E87AA0DBED4, native_parameters!(toggle));

    value
}

pub fn get_is_player_in_animal_form() -> bool {
    let value = native!(bool, 0x9689123E3F213AA5, native_parameters!());

    value
}

pub fn _set_player_rockstar_editor_disabled(toggle: bool) -> () {
    let value = native!((), 0x9D8D44ADBBA61EF2, native_parameters!(toggle));

    value
}

pub fn _0x23227df0b2115469() -> () {
    let value = native!((), 0x23227DF0B2115469, native_parameters!());

    value
}

pub fn _0xd10282b6e3751ba0() -> u32 {
    let value = native!(u32, 0xD10282B6E3751BA0, native_parameters!());

    value
}

pub fn _0x693478acbd7f18e7() -> () {
    let value = native!((), 0x693478ACBD7F18E7, native_parameters!());

    value
}
