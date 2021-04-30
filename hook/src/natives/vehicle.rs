use crate::types::NativeVector3;

pub fn create_vehicle(
    modelHash: u32,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    isNetwork: bool,
    bScriptHostVeh: bool,
    p7: bool,
) -> i32 {
    let value = native!(
        i32,
        0xAF35D0D2583051B0,
        native_parameters!(modelHash, x, y, z, heading, isNetwork, bScriptHostVeh, p7)
    );

    value
}

pub fn delete_vehicle(vehicle: *mut i32) -> () {
    let value = native!((), 0xEA386986E786A54F, native_parameters!(vehicle));

    value
}

pub fn _0x7d6f9a3ef26136a0(vehicle: i32, toggle: bool, p2: bool) -> () {
    let value = native!(
        (),
        0x7D6F9A3EF26136A0,
        native_parameters!(vehicle, toggle, p2)
    );

    value
}

pub fn _set_vehicle_can_be_locked_on(vehicle: i32, canBeLockedOn: bool, unk: bool) -> () {
    let value = native!(
        (),
        0x1DDA078D12879EEE,
        native_parameters!(vehicle, canBeLockedOn, unk)
    );

    value
}

pub fn set_vehicle_allow_no_passengers_lockon(veh: i32, toggle: bool) -> () {
    let value = native!((), 0x5D14D4154BFE7B2C, native_parameters!(veh, toggle));

    value
}

pub fn get_vehicle_homing_lockon_state(vehicle: i32) -> i32 {
    let value = native!(i32, 0xE6B0E8CFC3633BF0, native_parameters!(vehicle));

    value
}

pub fn _0x6eaaefc76acc311f(p0: u32) -> u32 {
    let value = native!(u32, 0x6EAAEFC76ACC311F, native_parameters!(p0));

    value
}

pub fn _0x407dc5e97db1a4d3(p0: u32, p1: u32) -> () {
    let value = native!((), 0x407DC5E97DB1A4D3, native_parameters!(p0, p1));

    value
}

pub fn is_vehicle_model(vehicle: i32, model: u32) -> bool {
    let value = native!(bool, 0x423E8DE37D934D89, native_parameters!(vehicle, model));

    value
}

pub fn does_script_vehicle_generator_exist(vehicleGenerator: i32) -> bool {
    let value = native!(
        bool,
        0xF6086BC836400876,
        native_parameters!(vehicleGenerator)
    );

    value
}

pub fn create_script_vehicle_generator(
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    p4: f32,
    p5: f32,
    modelHash: u32,
    p7: i32,
    p8: i32,
    p9: i32,
    p10: i32,
    p11: bool,
    p12: bool,
    p13: bool,
    p14: bool,
    p15: bool,
    p16: i32,
) -> i32 {
    let value = native!(
        i32,
        0x9DEF883114668116,
        native_parameters!(
            x, y, z, heading, p4, p5, modelHash, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16
        )
    );

    value
}

pub fn delete_script_vehicle_generator(vehicleGenerator: i32) -> () {
    let value = native!((), 0x22102C9ABFCF125D, native_parameters!(vehicleGenerator));

    value
}

pub fn set_script_vehicle_generator(vehicleGenerator: i32, enabled: bool) -> () {
    let value = native!(
        (),
        0xD9D620E0AC6DC4B0,
        native_parameters!(vehicleGenerator, enabled)
    );

    value
}

pub fn set_all_vehicle_generators_active_in_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    p6: bool,
    p7: bool,
) -> () {
    let value = native!(
        (),
        0xC12321827687FE4D,
        native_parameters!(x1, y1, z1, x2, y2, z2, p6, p7)
    );

    value
}

pub fn set_all_vehicle_generators_active() -> () {
    let value = native!((), 0x34AD89078831A4BC, native_parameters!());

    value
}

pub fn set_all_low_priority_vehicle_generators_active(active: bool) -> () {
    let value = native!((), 0x608207E7A8FB787C, native_parameters!(active));

    value
}

pub fn _0x9a75585fb2e54fad(x: f32, y: f32, z: f32, radius: f32) -> () {
    let value = native!((), 0x9A75585FB2E54FAD, native_parameters!(x, y, z, radius));

    value
}

pub fn _0x0a436b8643716d14() -> () {
    let value = native!((), 0x0A436B8643716D14, native_parameters!());

    value
}

pub fn set_vehicle_on_ground_properly(vehicle: i32, p1: f32) -> bool {
    let value = native!(bool, 0x49733E92263139D1, native_parameters!(vehicle, p1));

    value
}

pub fn set_vehicle_use_cutscene_wheel_compression(p0: i32, p1: bool, p2: bool, p3: bool) -> u32 {
    let value = native!(u32, 0xE023E8AC4EF7C117, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn is_vehicle_stuck_on_roof(vehicle: i32) -> bool {
    let value = native!(bool, 0xB497F06B288DCFDF, native_parameters!(vehicle));

    value
}

pub fn add_vehicle_upsidedown_check(vehicle: i32) -> () {
    let value = native!((), 0xB72E26D81006005B, native_parameters!(vehicle));

    value
}

pub fn remove_vehicle_upsidedown_check(vehicle: i32) -> () {
    let value = native!((), 0xC53EB42A499A7E90, native_parameters!(vehicle));

    value
}

pub fn is_vehicle_stopped(vehicle: i32) -> bool {
    let value = native!(bool, 0x5721B434AD84D57A, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_number_of_passengers(vehicle: i32) -> i32 {
    let value = native!(i32, 0x24CB2137731FFE89, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_max_number_of_passengers(vehicle: i32) -> i32 {
    let value = native!(i32, 0xA7C4F2C6E744A550, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_model_number_of_seats(modelHash: u32) -> i32 {
    let value = native!(i32, 0x2AD93716F184EDA4, native_parameters!(modelHash));

    value
}

pub fn is_seat_warp_only(vehicle: i32, seatIndex: i32) -> bool {
    let value = native!(
        bool,
        0xF7F203E31F96F6A1,
        native_parameters!(vehicle, seatIndex)
    );

    value
}

pub fn is_turret_seat(vehicle: i32, seatIndex: i32) -> bool {
    let value = native!(
        bool,
        0xE33FFA906CE74880,
        native_parameters!(vehicle, seatIndex)
    );

    value
}

pub fn _does_vehicle_allow_rappel(vehicle: i32) -> bool {
    let value = native!(bool, 0x4E417C547182C84D, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_density_multiplier_this_frame(multiplier: f32) -> () {
    let value = native!((), 0x245A6883D966D537, native_parameters!(multiplier));

    value
}

pub fn set_random_vehicle_density_multiplier_this_frame(multiplier: f32) -> () {
    let value = native!((), 0xB3B3359379FE77D3, native_parameters!(multiplier));

    value
}

pub fn set_parked_vehicle_density_multiplier_this_frame(multiplier: f32) -> () {
    let value = native!((), 0xEAE6DCC7EEE3DB1D, native_parameters!(multiplier));

    value
}

pub fn set_disable_random_trains_this_frame(toggle: bool) -> () {
    let value = native!((), 0xD4B8E3D1917BC86B, native_parameters!(toggle));

    value
}

pub fn set_ambient_vehicle_range_multiplier_this_frame(value: f32) -> () {
    let value = native!((), 0x90B6DA738A9A25DA, native_parameters!(value));

    value
}

pub fn set_far_draw_vehicles(toggle: bool) -> () {
    let value = native!((), 0x26324F33423F3CC3, native_parameters!(toggle));

    value
}

pub fn set_number_of_parked_vehicles(value: i32) -> () {
    let value = native!((), 0xCAA15F13EBD417FF, native_parameters!(value));

    value
}

pub fn set_vehicle_doors_locked(vehicle: i32, doorLockStatus: i32) -> () {
    let value = native!(
        (),
        0xB664292EAECF7FA6,
        native_parameters!(vehicle, doorLockStatus)
    );

    value
}

pub fn set_vehicle_individual_doors_locked(vehicle: i32, doorIndex: i32, destroyType: i32) -> () {
    let value = native!(
        (),
        0xBE70724027F85BCD,
        native_parameters!(vehicle, doorIndex, destroyType)
    );

    value
}

pub fn set_vehicle_has_muted_sirens(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xD8050E0EB60CF274, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_doors_locked_for_player(vehicle: i32, player: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0x517AAF684BB50CD1,
        native_parameters!(vehicle, player, toggle)
    );

    value
}

pub fn get_vehicle_doors_locked_for_player(vehicle: i32, player: i32) -> bool {
    let value = native!(
        bool,
        0xF6AF6CB341349015,
        native_parameters!(vehicle, player)
    );

    value
}

pub fn set_vehicle_doors_locked_for_all_players(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xA2F80B8D040727CC, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_doors_locked_for_non_script_players(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x9737A37136F07E75, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_doors_locked_for_team(vehicle: i32, team: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0xB81F6D4A8F5EEBA8,
        native_parameters!(vehicle, team, toggle)
    );

    value
}

pub fn _set_vehicle_doors_locked_for_unk(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x203B527D1B77904C, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x76d26a22750e849e(vehicle: i32) -> () {
    let value = native!((), 0x76D26A22750E849E, native_parameters!(vehicle));

    value
}

pub fn explode_vehicle(vehicle: i32, isAudible: bool, isInvisible: bool) -> () {
    let value = native!(
        (),
        0xBA71116ADF5B514C,
        native_parameters!(vehicle, isAudible, isInvisible)
    );

    value
}

pub fn set_vehicle_out_of_control(vehicle: i32, killDriver: bool, explodeOnImpact: bool) -> () {
    let value = native!(
        (),
        0xF19D095E42D430CC,
        native_parameters!(vehicle, killDriver, explodeOnImpact)
    );

    value
}

pub fn set_vehicle_timed_explosion(vehicle: i32, ped: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0x2E0A74E1002380B1,
        native_parameters!(vehicle, ped, toggle)
    );

    value
}

pub fn add_vehicle_phone_explosive_device(vehicle: i32) -> () {
    let value = native!((), 0x99AD4CCCB128CBC9, native_parameters!(vehicle));

    value
}

pub fn _clear_vehicle_phone_explosive_device() -> () {
    let value = native!((), 0xAA3F739ABDDCF21F, native_parameters!());

    value
}

pub fn has_vehicle_phone_explosive_device() -> bool {
    let value = native!(bool, 0x6ADAABD3068C5235, native_parameters!());

    value
}

pub fn detonate_vehicle_phone_explosive_device() -> () {
    let value = native!((), 0xEF49CF0270307CBE, native_parameters!());

    value
}

pub fn set_taxi_lights(vehicle: i32, state: bool) -> () {
    let value = native!((), 0x598803E85E8448D9, native_parameters!(vehicle, state));

    value
}

pub fn is_taxi_light_on(vehicle: i32) -> bool {
    let value = native!(bool, 0x7504C0F113AB50FC, native_parameters!(vehicle));

    value
}

pub fn is_vehicle_in_garage_area(garageName: &std::ffi::CString, vehicle: i32) -> bool {
    let value = native!(
        bool,
        0xCEE4490CD57BB3C2,
        native_parameters!(garageName.as_ptr(), vehicle)
    );

    value
}

pub fn set_vehicle_colours(vehicle: i32, colorPrimary: i32, colorSecondary: i32) -> () {
    let value = native!(
        (),
        0x4F1D4BE3A7F24601,
        native_parameters!(vehicle, colorPrimary, colorSecondary)
    );

    value
}

pub fn set_vehicle_fullbeam(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x8B7FD87F0DDB421E, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_is_racing(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x07116E24E9D1929D, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_custom_primary_colour(vehicle: i32, r: i32, g: i32, b: i32) -> () {
    let value = native!((), 0x7141766F91D15BEA, native_parameters!(vehicle, r, g, b));

    value
}

pub fn get_vehicle_custom_primary_colour(
    vehicle: i32,
    r: *mut i32,
    g: *mut i32,
    b: *mut i32,
) -> () {
    let value = native!((), 0xB64CF2CCA9D95F52, native_parameters!(vehicle, r, g, b));

    value
}

pub fn clear_vehicle_custom_primary_colour(vehicle: i32) -> () {
    let value = native!((), 0x55E1D2758F34E437, native_parameters!(vehicle));

    value
}

pub fn get_is_vehicle_primary_colour_custom(vehicle: i32) -> bool {
    let value = native!(bool, 0xF095C0405307B21B, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_custom_secondary_colour(vehicle: i32, r: i32, g: i32, b: i32) -> () {
    let value = native!((), 0x36CED73BFED89754, native_parameters!(vehicle, r, g, b));

    value
}

pub fn get_vehicle_custom_secondary_colour(
    vehicle: i32,
    r: *mut i32,
    g: *mut i32,
    b: *mut i32,
) -> () {
    let value = native!((), 0x8389CD56CA8072DC, native_parameters!(vehicle, r, g, b));

    value
}

pub fn clear_vehicle_custom_secondary_colour(vehicle: i32) -> () {
    let value = native!((), 0x5FFBDEEC3E8E2009, native_parameters!(vehicle));

    value
}

pub fn get_is_vehicle_secondary_colour_custom(vehicle: i32) -> bool {
    let value = native!(bool, 0x910A32E7AAD2656C, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_enveff_scale(vehicle: i32, fade: f32) -> () {
    let value = native!((), 0x3AFDC536C3D01674, native_parameters!(vehicle, fade));

    value
}

pub fn get_vehicle_enveff_scale(vehicle: i32) -> f32 {
    let value = native!(f32, 0xA82819CAC9C4C403, native_parameters!(vehicle));

    value
}

pub fn set_can_respray_vehicle(vehicle: i32, state: bool) -> () {
    let value = native!((), 0x52BBA29D5EC69356, native_parameters!(vehicle, state));

    value
}

pub fn _0xab31ef4de6800ce9(p0: u32, p1: u32) -> () {
    let value = native!((), 0xAB31EF4DE6800CE9, native_parameters!(p0, p1));

    value
}

pub fn _0x1b212b26dd3c04df(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x1B212B26DD3C04DF, native_parameters!(vehicle, toggle));

    value
}

pub fn force_submarine_surface_mode(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x33506883545AC0DF, native_parameters!(vehicle, toggle));

    value
}

pub fn _0xc67db108a9ade3be(p0: u32, p1: u32) -> () {
    let value = native!((), 0xC67DB108A9ADE3BE, native_parameters!(p0, p1));

    value
}

pub fn set_submarine_crush_depths(
    vehicle: i32,
    p1: bool,
    depth1: f32,
    depth2: f32,
    depth3: f32,
) -> () {
    let value = native!(
        (),
        0xC59872A5134879C7,
        native_parameters!(vehicle, p1, depth1, depth2, depth3)
    );

    value
}

pub fn _get_submarine_is_below_first_crush_depth(submarine: i32) -> bool {
    let value = native!(bool, 0x3E71D0B300B7AA79, native_parameters!(submarine));

    value
}

pub fn _get_submarine_crush_depth_warning_state(submarine: i32) -> i32 {
    let value = native!(i32, 0x093D6DDCA5B8FBAE, native_parameters!(submarine));

    value
}

pub fn _0xed5ede9e676643c9(p0: u32, p1: u32) -> () {
    let value = native!((), 0xED5EDE9E676643C9, native_parameters!(p0, p1));

    value
}

pub fn set_boat_anchor(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x75DBEC174AEEAD10, native_parameters!(vehicle, toggle));

    value
}

pub fn can_anchor_boat_here(vehicle: i32) -> bool {
    let value = native!(bool, 0x26C10ECBDA5D043B, native_parameters!(vehicle));

    value
}

pub fn _can_anchor_boat_here_2(vehicle: i32) -> bool {
    let value = native!(bool, 0x24F4121D07579880, native_parameters!(vehicle));

    value
}

pub fn _set_boat_frozen_when_anchored(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xE3EBAAE484798530, native_parameters!(vehicle, toggle));

    value
}

pub fn _0xb28b1fe5bfadd7f5(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0xB28B1FE5BFADD7F5, native_parameters!(vehicle, p1));

    value
}

pub fn _set_boat_movement_resistance(vehicle: i32, value: f32) -> () {
    let value = native!((), 0xE842A9398079BD82, native_parameters!(vehicle, value));

    value
}

pub fn _is_boat_anchored_and_frozen(vehicle: i32) -> bool {
    let value = native!(bool, 0xB0AD1238A709B1A2, native_parameters!(vehicle));

    value
}

pub fn set_boat_sinks_when_wrecked(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x8F719973E1445BA2, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_boat_is_sinking(p0: u32) -> () {
    let value = native!((), 0xBD32E46AA95C1DD2, native_parameters!(p0));

    value
}

pub fn set_vehicle_siren(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xF4924635A19EB37D, native_parameters!(vehicle, toggle));

    value
}

pub fn is_vehicle_siren_on(vehicle: i32) -> bool {
    let value = native!(bool, 0x4C9BF537BE2634B2, native_parameters!(vehicle));

    value
}

pub fn is_vehicle_siren_audio_on(vehicle: i32) -> bool {
    let value = native!(bool, 0xB5CC40FBCB586380, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_strong(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x3E8C8727991A8A0B, native_parameters!(vehicle, toggle));

    value
}

pub fn remove_vehicle_stuck_check(vehicle: i32) -> () {
    let value = native!((), 0x8386BFB614D06749, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_colours(vehicle: i32, colorPrimary: *mut i32, colorSecondary: *mut i32) -> () {
    let value = native!(
        (),
        0xA19435F193E081AC,
        native_parameters!(vehicle, colorPrimary, colorSecondary)
    );

    value
}

pub fn is_vehicle_seat_free(vehicle: i32, seatIndex: i32, isTaskRunning: bool) -> bool {
    let value = native!(
        bool,
        0x22AC59A870E6A669,
        native_parameters!(vehicle, seatIndex, isTaskRunning)
    );

    value
}

pub fn get_ped_in_vehicle_seat(vehicle: i32, seatIndex: i32, p2: bool) -> i32 {
    let value = native!(
        i32,
        0xBB40DD2270B65366,
        native_parameters!(vehicle, seatIndex, p2)
    );

    value
}

pub fn get_last_ped_in_vehicle_seat(vehicle: i32, seatIndex: i32) -> i32 {
    let value = native!(
        i32,
        0x83F969AA1EE2A664,
        native_parameters!(vehicle, seatIndex)
    );

    value
}

pub fn get_vehicle_lights_state(vehicle: i32, lightsOn: *mut bool, highbeamsOn: *mut bool) -> bool {
    let value = native!(
        bool,
        0xB91B4C20085BD12F,
        native_parameters!(vehicle, lightsOn, highbeamsOn)
    );

    value
}

pub fn is_vehicle_tyre_burst(vehicle: i32, wheelID: i32, completely: bool) -> bool {
    let value = native!(
        bool,
        0xBA291848A0815CA9,
        native_parameters!(vehicle, wheelID, completely)
    );

    value
}

pub fn set_vehicle_forward_speed(vehicle: i32, speed: f32) -> () {
    let value = native!((), 0xAB54A438726D25D5, native_parameters!(vehicle, speed));

    value
}

pub fn _0x6501129c9e0ffa05(p0: u32, p1: u32) -> () {
    let value = native!((), 0x6501129C9E0FFA05, native_parameters!(p0, p1));

    value
}

pub fn bring_vehicle_to_halt(vehicle: i32, distance: f32, duration: i32, unknown: bool) -> () {
    let value = native!(
        (),
        0x260BE8F09E326A20,
        native_parameters!(vehicle, distance, duration, unknown)
    );

    value
}

pub fn _0xdce97bdf8a0eabc8(vehicle: i32, p1: u32) -> () {
    let value = native!((), 0xDCE97BDF8A0EABC8, native_parameters!(vehicle, p1));

    value
}

pub fn _0x9849de24fcf23ccc(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x9849DE24FCF23CCC, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x8664170ef165c4a6(p0: u32, p1: u32) -> () {
    let value = native!((), 0x8664170EF165C4A6, native_parameters!(p0, p1));

    value
}

pub fn _stop_bring_vehicle_to_halt(vehicle: i32) -> () {
    let value = native!((), 0x7C06330BFDDA182E, native_parameters!(vehicle));

    value
}

pub fn _is_vehicle_being_halted(vehicle: i32) -> bool {
    let value = native!(bool, 0xC69BB1D832A710EF, native_parameters!(vehicle));

    value
}

pub fn set_forklift_fork_height(vehicle: i32, height: f32) -> () {
    let value = native!((), 0x37EBBF3117BD6A25, native_parameters!(vehicle, height));

    value
}

pub fn is_entity_attached_to_handler_frame(vehicle: i32, entity: i32) -> bool {
    let value = native!(
        bool,
        0x57715966069157AD,
        native_parameters!(vehicle, entity)
    );

    value
}

pub fn is_any_entity_attached_to_handler_frame(vehicle: i32) -> bool {
    let value = native!(bool, 0x62CA17B74C435651, native_parameters!(vehicle));

    value
}

pub fn _find_vehicle_carrying_this_entity(entity: i32) -> i32 {
    let value = native!(i32, 0x375E7FC44F21C8AB, native_parameters!(entity));

    value
}

pub fn _is_handler_frame_above_container(vehicle: i32, entity: i32) -> bool {
    let value = native!(
        bool,
        0x89D630CF5EA96D23,
        native_parameters!(vehicle, entity)
    );

    value
}

pub fn _0x6a98c2ecf57fa5d4(vehicle: i32, entity: i32) -> () {
    let value = native!((), 0x6A98C2ECF57FA5D4, native_parameters!(vehicle, entity));

    value
}

pub fn detach_container_from_handler_frame(vehicle: i32) -> () {
    let value = native!((), 0x7C0043FDFF6436BC, native_parameters!(vehicle));

    value
}

pub fn _0x8aa9180de2fedd45(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x8AA9180DE2FEDD45, native_parameters!(vehicle, p1));

    value
}

pub fn set_boat_disable_avoidance(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x0A6A279F3AA4FD70, native_parameters!(vehicle, p1));

    value
}

pub fn is_heli_landing_area_blocked(vehicle: i32) -> bool {
    let value = native!(bool, 0x634148744F385576, native_parameters!(vehicle));

    value
}

pub fn _0x107a473d7a6647a9(vehicle: i32) -> () {
    let value = native!((), 0x107A473D7A6647A9, native_parameters!(vehicle));

    value
}

pub fn set_heli_turbulence_scalar(vehicle: i32, p1: f32) -> () {
    let value = native!((), 0xE6F13851780394DA, native_parameters!(vehicle, p1));

    value
}

pub fn set_car_boot_open(vehicle: i32) -> () {
    let value = native!((), 0xFC40CBF7B90CA77C, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_tyre_burst(vehicle: i32, index: i32, onRim: bool, p3: f32) -> () {
    let value = native!(
        (),
        0xEC6A202EE4960385,
        native_parameters!(vehicle, index, onRim, p3)
    );

    value
}

pub fn set_vehicle_doors_shut(vehicle: i32, closeInstantly: bool) -> () {
    let value = native!(
        (),
        0x781B3D62BB013EF5,
        native_parameters!(vehicle, closeInstantly)
    );

    value
}

pub fn set_vehicle_tyres_can_burst(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xEB9DC3C7D8596C46, native_parameters!(vehicle, toggle));

    value
}

pub fn get_vehicle_tyres_can_burst(vehicle: i32) -> bool {
    let value = native!(bool, 0x678B9BB8C3F58FEB, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_wheels_can_break(vehicle: i32, enabled: bool) -> () {
    let value = native!((), 0x29B18B4FD460CA8F, native_parameters!(vehicle, enabled));

    value
}

pub fn set_vehicle_door_open(vehicle: i32, doorIndex: i32, loose: bool, openInstantly: bool) -> () {
    let value = native!(
        (),
        0x7C65DAC73C35C862,
        native_parameters!(vehicle, doorIndex, loose, openInstantly)
    );

    value
}

pub fn _0x3b458ddb57038f08(vehicle: i32, doorIndex: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0x3B458DDB57038F08,
        native_parameters!(vehicle, doorIndex, toggle)
    );

    value
}

pub fn _0xa247f9ef01d8082e(p0: u32) -> () {
    let value = native!((), 0xA247F9EF01D8082E, native_parameters!(p0));

    value
}

pub fn remove_vehicle_window(vehicle: i32, windowIndex: i32) -> () {
    let value = native!(
        (),
        0xA711568EEDB43069,
        native_parameters!(vehicle, windowIndex)
    );

    value
}

pub fn roll_down_windows(vehicle: i32) -> () {
    let value = native!((), 0x85796B0549DDE156, native_parameters!(vehicle));

    value
}

pub fn roll_down_window(vehicle: i32, windowIndex: i32) -> () {
    let value = native!(
        (),
        0x7AD9E6CE657D69E3,
        native_parameters!(vehicle, windowIndex)
    );

    value
}

pub fn roll_up_window(vehicle: i32, windowIndex: i32) -> () {
    let value = native!(
        (),
        0x602E548F46E24D59,
        native_parameters!(vehicle, windowIndex)
    );

    value
}

pub fn smash_vehicle_window(vehicle: i32, index: i32) -> () {
    let value = native!((), 0x9E5B5E4D2CCD2259, native_parameters!(vehicle, index));

    value
}

pub fn fix_vehicle_window(vehicle: i32, index: i32) -> () {
    let value = native!((), 0x772282EBEB95E682, native_parameters!(vehicle, index));

    value
}

pub fn pop_out_vehicle_windscreen(vehicle: i32) -> () {
    let value = native!((), 0x6D645D59FB5F5AD3, native_parameters!(vehicle));

    value
}

pub fn _eject_jb700_roof(vehicle: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xE38CB9D7D39FDBCC, native_parameters!(vehicle, x, y, z));

    value
}

pub fn set_vehicle_lights(vehicle: i32, state: i32) -> () {
    let value = native!((), 0x34E710FF01247C5A, native_parameters!(vehicle, state));

    value
}

pub fn set_vehicle_use_player_light_settings(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xC45C27EF50F36ADC, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_vehicle_lights_mode(vehicle: i32, p1: i32) -> () {
    let value = native!((), 0x1FD09E7390A74D54, native_parameters!(vehicle, p1));

    value
}

pub fn set_vehicle_alarm(vehicle: i32, state: bool) -> () {
    let value = native!((), 0xCDE5E70C1DDB954C, native_parameters!(vehicle, state));

    value
}

pub fn start_vehicle_alarm(vehicle: i32) -> () {
    let value = native!((), 0xB8FF7AB45305C345, native_parameters!(vehicle));

    value
}

pub fn is_vehicle_alarm_activated(vehicle: i32) -> bool {
    let value = native!(bool, 0x4319E335B71FFF34, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_interiorlight(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xBC2042F090AF6AD3, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x8821196d91fa2de5(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x8821196D91FA2DE5, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_light_multiplier(vehicle: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0xB385454F8791F57C,
        native_parameters!(vehicle, multiplier)
    );

    value
}

pub fn attach_vehicle_to_trailer(vehicle: i32, trailer: i32, radius: f32) -> () {
    let value = native!(
        (),
        0x3C7D42D58F770B54,
        native_parameters!(vehicle, trailer, radius)
    );

    value
}

pub fn attach_vehicle_on_to_trailer(
    vehicle: i32,
    trailer: i32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    coordsX: f32,
    coordsY: f32,
    coordsZ: f32,
    rotationX: f32,
    rotationY: f32,
    rotationZ: f32,
    disableCollisions: f32,
) -> () {
    let value = native!(
        (),
        0x16B5E274BDE402F8,
        native_parameters!(
            vehicle,
            trailer,
            offsetX,
            offsetY,
            offsetZ,
            coordsX,
            coordsY,
            coordsZ,
            rotationX,
            rotationY,
            rotationZ,
            disableCollisions
        )
    );

    value
}

pub fn stabilise_entity_attached_to_heli(vehicle: i32, entity: i32, p2: f32) -> () {
    let value = native!(
        (),
        0x374706271354CB18,
        native_parameters!(vehicle, entity, p2)
    );

    value
}

pub fn detach_vehicle_from_trailer(vehicle: i32) -> () {
    let value = native!((), 0x90532EDF0D2BDD86, native_parameters!(vehicle));

    value
}

pub fn is_vehicle_attached_to_trailer(vehicle: i32) -> bool {
    let value = native!(bool, 0xE7CF3C4F9F489F0C, native_parameters!(vehicle));

    value
}

pub fn set_trailer_inverse_mass_scale(vehicle: i32, p1: f32) -> () {
    let value = native!((), 0x2A8F319B392E7B3F, native_parameters!(vehicle, p1));

    value
}

pub fn set_trailer_legs_raised(vehicle: i32) -> () {
    let value = native!((), 0x95CF53B3D687F9FA, native_parameters!(vehicle));

    value
}

pub fn _set_trailer_legs_lowered(p0: u32) -> () {
    let value = native!((), 0x878C75C09FBDB942, native_parameters!(p0));

    value
}

pub fn set_vehicle_tyre_fixed(vehicle: i32, tyreIndex: i32) -> () {
    let value = native!(
        (),
        0x6E13FC662B882D1D,
        native_parameters!(vehicle, tyreIndex)
    );

    value
}

pub fn set_vehicle_number_plate_text(vehicle: i32, plateText: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x95A88F0B409CDA47,
        native_parameters!(vehicle, plateText.as_ptr())
    );

    value
}

pub fn get_vehicle_number_plate_text(vehicle: i32) -> String {
    let value = native!(*const i8, 0x7CE1CCB9B293020E, native_parameters!(vehicle));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_number_of_vehicle_number_plates() -> i32 {
    let value = native!(i32, 0x4C4D6B2644F458CB, native_parameters!());

    value
}

pub fn set_vehicle_number_plate_text_index(vehicle: i32, plateIndex: i32) -> () {
    let value = native!(
        (),
        0x9088EB5A43FFB0A1,
        native_parameters!(vehicle, plateIndex)
    );

    value
}

pub fn get_vehicle_number_plate_text_index(vehicle: i32) -> i32 {
    let value = native!(i32, 0xF11BC2DD9A3E7195, native_parameters!(vehicle));

    value
}

pub fn set_random_trains(toggle: bool) -> () {
    let value = native!((), 0x80D9F74197EA47D9, native_parameters!(toggle));

    value
}

pub fn create_mission_train(variation: i32, x: f32, y: f32, z: f32, direction: bool) -> i32 {
    let value = native!(
        i32,
        0x63C6CCA8E68AE8C8,
        native_parameters!(variation, x, y, z, direction)
    );

    value
}

pub fn switch_train_track(trackId: i32, state: bool) -> () {
    let value = native!((), 0xFD813BB7DB977F20, native_parameters!(trackId, state));

    value
}

pub fn set_train_track_spawn_frequency(trackIndex: i32, frequency: i32) -> () {
    let value = native!(
        (),
        0x21973BBF8D17EDFA,
        native_parameters!(trackIndex, frequency)
    );

    value
}

pub fn delete_all_trains() -> () {
    let value = native!((), 0x736A718577F39C7D, native_parameters!());

    value
}

pub fn set_train_speed(train: i32, speed: f32) -> () {
    let value = native!((), 0xAA0BC91BE0B796E3, native_parameters!(train, speed));

    value
}

pub fn set_train_cruise_speed(train: i32, speed: f32) -> () {
    let value = native!((), 0x16469284DB8C62B5, native_parameters!(train, speed));

    value
}

pub fn set_random_boats(toggle: bool) -> () {
    let value = native!((), 0x84436EC293B1415F, native_parameters!(toggle));

    value
}

pub fn set_garbage_trucks(toggle: bool) -> () {
    let value = native!((), 0x2AFD795EEAC8D30D, native_parameters!(toggle));

    value
}

pub fn does_vehicle_have_stuck_vehicle_check(vehicle: i32) -> bool {
    let value = native!(bool, 0x57E4C39DE5EE8470, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_recording_id(recording: i32, script: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0x21543C612379DB3C,
        native_parameters!(recording, script.as_ptr())
    );

    value
}

pub fn request_vehicle_recording(recording: i32, script: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xAF514CABE74CBF15,
        native_parameters!(recording, script.as_ptr())
    );

    value
}

pub fn has_vehicle_recording_been_loaded(recording: i32, script: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x300D614A4C785FC4,
        native_parameters!(recording, script.as_ptr())
    );

    value
}

pub fn remove_vehicle_recording(recording: i32, script: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xF1160ACCF98A3FC8,
        native_parameters!(recording, script.as_ptr())
    );

    value
}

pub fn get_position_of_vehicle_recording_id_at_time(id: i32, time: f32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x92523B76657A517D,
        native_parameters!(id, time)
    );

    value
}

pub fn get_position_of_vehicle_recording_at_time(
    recording: i32,
    time: f32,
    script: &std::ffi::CString,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xD242728AA6F0FBA2,
        native_parameters!(recording, time, script.as_ptr())
    );

    value
}

pub fn get_rotation_of_vehicle_recording_id_at_time(id: i32, time: f32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xF0F2103EFAF8CBA7,
        native_parameters!(id, time)
    );

    value
}

pub fn get_rotation_of_vehicle_recording_at_time(
    recording: i32,
    time: f32,
    script: &std::ffi::CString,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x2058206FBE79A8AD,
        native_parameters!(recording, time, script.as_ptr())
    );

    value
}

pub fn get_total_duration_of_vehicle_recording_id(id: i32) -> f32 {
    let value = native!(f32, 0x102D125411A7B6E6, native_parameters!(id));

    value
}

pub fn get_total_duration_of_vehicle_recording(recording: i32, script: &std::ffi::CString) -> f32 {
    let value = native!(
        f32,
        0x0E48D1C262390950,
        native_parameters!(recording, script.as_ptr())
    );

    value
}

pub fn get_position_in_recording(vehicle: i32) -> f32 {
    let value = native!(f32, 0x2DACD605FC681475, native_parameters!(vehicle));

    value
}

pub fn get_time_position_in_recording(vehicle: i32) -> f32 {
    let value = native!(f32, 0x5746F3A7AB7FE544, native_parameters!(vehicle));

    value
}

pub fn start_playback_recorded_vehicle(
    vehicle: i32,
    recording: i32,
    script: &std::ffi::CString,
    p3: bool,
) -> () {
    let value = native!(
        (),
        0x3F878F92B3A7A071,
        native_parameters!(vehicle, recording, script.as_ptr(), p3)
    );

    value
}

pub fn start_playback_recorded_vehicle_with_flags(
    vehicle: i32,
    recording: i32,
    script: &std::ffi::CString,
    flags: i32,
    time: i32,
    drivingStyle: i32,
) -> () {
    let value = native!(
        (),
        0x7D80FD645D4DA346,
        native_parameters!(
            vehicle,
            recording,
            script.as_ptr(),
            flags,
            time,
            drivingStyle
        )
    );

    value
}

pub fn force_playback_recorded_vehicle_update(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x1F2E4E06DEA8992B, native_parameters!(vehicle, p1));

    value
}

pub fn stop_playback_recorded_vehicle(vehicle: i32) -> () {
    let value = native!((), 0x54833611C17ABDEA, native_parameters!(vehicle));

    value
}

pub fn pause_playback_recorded_vehicle(vehicle: i32) -> () {
    let value = native!((), 0x632A689BF42301B1, native_parameters!(vehicle));

    value
}

pub fn unpause_playback_recorded_vehicle(vehicle: i32) -> () {
    let value = native!((), 0x8879EE09268305D5, native_parameters!(vehicle));

    value
}

pub fn is_playback_going_on_for_vehicle(vehicle: i32) -> bool {
    let value = native!(bool, 0x1C8A4C2C19E68EEC, native_parameters!(vehicle));

    value
}

pub fn is_playback_using_ai_going_on_for_vehicle(vehicle: i32) -> bool {
    let value = native!(bool, 0xAEA8FD591FAD4106, native_parameters!(vehicle));

    value
}

pub fn get_current_playback_for_vehicle(vehicle: i32) -> i32 {
    let value = native!(i32, 0x42BC05C27A946054, native_parameters!(vehicle));

    value
}

pub fn skip_to_end_and_stop_playback_recorded_vehicle(vehicle: i32) -> () {
    let value = native!((), 0xAB8E2EDA0C0A5883, native_parameters!(vehicle));

    value
}

pub fn set_playback_speed(vehicle: i32, speed: f32) -> () {
    let value = native!((), 0x6683AB880E427778, native_parameters!(vehicle, speed));

    value
}

pub fn start_playback_recorded_vehicle_using_ai(
    vehicle: i32,
    recording: i32,
    script: &std::ffi::CString,
    speed: f32,
    drivingStyle: i32,
) -> () {
    let value = native!(
        (),
        0x29DE5FA52D00428C,
        native_parameters!(vehicle, recording, script.as_ptr(), speed, drivingStyle)
    );

    value
}

pub fn skip_time_in_playback_recorded_vehicle(vehicle: i32, time: f32) -> () {
    let value = native!((), 0x9438F7AD68771A20, native_parameters!(vehicle, time));

    value
}

pub fn set_playback_to_use_ai(vehicle: i32, drivingStyle: i32) -> () {
    let value = native!(
        (),
        0xA549C3B37EA28131,
        native_parameters!(vehicle, drivingStyle)
    );

    value
}

pub fn set_playback_to_use_ai_try_to_revert_back_later(
    vehicle: i32,
    time: i32,
    drivingStyle: i32,
    p3: bool,
) -> () {
    let value = native!(
        (),
        0x6E63860BBB190730,
        native_parameters!(vehicle, time, drivingStyle, p3)
    );

    value
}

pub fn _0x5845066d8a1ea7f7(vehicle: i32, x: f32, y: f32, z: f32, p4: u32) -> () {
    let value = native!(
        (),
        0x5845066D8A1EA7F7,
        native_parameters!(vehicle, x, y, z, p4)
    );

    value
}

pub fn _0x796a877e459b99ea(p0: u32, p1: f32, p2: f32, p3: f32) -> () {
    let value = native!((), 0x796A877E459B99EA, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xfaf2a78061fd9ef4(p0: u32, p1: f32, p2: f32, p3: f32) -> () {
    let value = native!((), 0xFAF2A78061FD9EF4, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x063ae2b2cc273588(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x063AE2B2CC273588, native_parameters!(vehicle, p1));

    value
}

pub fn explode_vehicle_in_cutscene(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x786A4EB67B01BF0B, native_parameters!(vehicle, p1));

    value
}

pub fn add_vehicle_stuck_check_with_warp(
    p0: u32,
    p1: f32,
    p2: u32,
    p3: bool,
    p4: bool,
    p5: bool,
    p6: u32,
) -> () {
    let value = native!(
        (),
        0x2FA9923062DD396C,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn set_vehicle_model_is_suppressed(model: u32, suppressed: bool) -> () {
    let value = native!(
        (),
        0x0FC2D89AC25A5814,
        native_parameters!(model, suppressed)
    );

    value
}

pub fn get_random_vehicle_in_sphere(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    modelHash: u32,
    flags: i32,
) -> i32 {
    let value = native!(
        i32,
        0x386F6CE5BAF6091C,
        native_parameters!(x, y, z, radius, modelHash, flags)
    );

    value
}

pub fn get_random_vehicle_front_bumper_in_sphere(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: i32,
    p5: i32,
    p6: i32,
) -> i32 {
    let value = native!(
        i32,
        0xC5574E0AEB86BA68,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn get_random_vehicle_back_bumper_in_sphere(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: i32,
    p5: i32,
    p6: i32,
) -> i32 {
    let value = native!(
        i32,
        0xB50807EABE20A8DC,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn get_closest_vehicle(x: f32, y: f32, z: f32, radius: f32, modelHash: u32, flags: i32) -> i32 {
    let value = native!(
        i32,
        0xF73EB622C4F1689B,
        native_parameters!(x, y, z, radius, modelHash, flags)
    );

    value
}

pub fn get_train_carriage(train: i32, trailerNumber: i32) -> i32 {
    let value = native!(
        i32,
        0x08AAFD0814722BC3,
        native_parameters!(train, trailerNumber)
    );

    value
}

pub fn delete_mission_train(train: *mut i32) -> () {
    let value = native!((), 0x5B76B14AE875C795, native_parameters!(train));

    value
}

pub fn set_mission_train_as_no_longer_needed(train: *mut i32, p1: bool) -> () {
    let value = native!((), 0xBBE7648349B49BE8, native_parameters!(train, p1));

    value
}

pub fn set_mission_train_coords(train: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x591CA673AA6AB736, native_parameters!(train, x, y, z));

    value
}

pub fn is_this_model_a_boat(model: u32) -> bool {
    let value = native!(bool, 0x45A9187928F4B9E3, native_parameters!(model));

    value
}

pub fn is_this_model_a_jetski(model: u32) -> bool {
    let value = native!(bool, 0x9537097412CF75FE, native_parameters!(model));

    value
}

pub fn is_this_model_a_plane(model: u32) -> bool {
    let value = native!(bool, 0xA0948AB42D7BA0DE, native_parameters!(model));

    value
}

pub fn is_this_model_a_heli(model: u32) -> bool {
    let value = native!(bool, 0xDCE4334788AF94EA, native_parameters!(model));

    value
}

pub fn is_this_model_a_car(model: u32) -> bool {
    let value = native!(bool, 0x7F6DB52EEFC96DF8, native_parameters!(model));

    value
}

pub fn is_this_model_a_train(model: u32) -> bool {
    let value = native!(bool, 0xAB935175B22E822B, native_parameters!(model));

    value
}

pub fn is_this_model_a_bike(model: u32) -> bool {
    let value = native!(bool, 0xB50C0B0CEDC6CE84, native_parameters!(model));

    value
}

pub fn is_this_model_a_bicycle(model: u32) -> bool {
    let value = native!(bool, 0xBF94DD42F63BDED2, native_parameters!(model));

    value
}

pub fn is_this_model_a_quadbike(model: u32) -> bool {
    let value = native!(bool, 0x39DAC362EE65FA28, native_parameters!(model));

    value
}

pub fn _is_this_model_an_amphibious_car(model: u32) -> bool {
    let value = native!(bool, 0x633F6F44A537EBB6, native_parameters!(model));

    value
}

pub fn _is_this_model_an_amphibious_quadbike(model: u32) -> bool {
    let value = native!(bool, 0xA1A9FC1C76A6730D, native_parameters!(model));

    value
}

pub fn set_heli_blades_full_speed(vehicle: i32) -> () {
    let value = native!((), 0xA178472EBB8AE60D, native_parameters!(vehicle));

    value
}

pub fn set_heli_blades_speed(vehicle: i32, speed: f32) -> () {
    let value = native!((), 0xFD280B4D7F3ABC4D, native_parameters!(vehicle, speed));

    value
}

pub fn _0x99cad8e7afdb60fa(vehicle: i32, p1: f32, p2: f32) -> () {
    let value = native!((), 0x99CAD8E7AFDB60FA, native_parameters!(vehicle, p1, p2));

    value
}

pub fn set_vehicle_can_be_targetted(vehicle: i32, state: bool) -> () {
    let value = native!((), 0x3750146A28097A82, native_parameters!(vehicle, state));

    value
}

pub fn _0xdbc631f109350b8c(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0xDBC631F109350B8C, native_parameters!(vehicle, p1));

    value
}

pub fn set_vehicle_can_be_visibly_damaged(vehicle: i32, state: bool) -> () {
    let value = native!((), 0x4C7028F78FFD3681, native_parameters!(vehicle, state));

    value
}

pub fn set_vehicle_has_unbreakable_lights(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x1AA8A837D2169D94, native_parameters!(vehicle, p1));

    value
}

pub fn _0x2311dd7159f00582(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x2311DD7159F00582, native_parameters!(vehicle, p1));

    value
}

pub fn _0x065d03a9d6b2c6b5(p0: u32, p1: u32) -> () {
    let value = native!((), 0x065D03A9D6B2C6B5, native_parameters!(p0, p1));

    value
}

pub fn get_vehicle_dirt_level(vehicle: i32) -> f32 {
    let value = native!(f32, 0x8F17BC8BA08DA62B, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_dirt_level(vehicle: i32, dirtLevel: f32) -> () {
    let value = native!(
        (),
        0x79D3B596FE44EE8B,
        native_parameters!(vehicle, dirtLevel)
    );

    value
}

pub fn _is_vehicle_damaged(vehicle: i32) -> bool {
    let value = native!(bool, 0xBCDC5017D3CE1E9E, native_parameters!(vehicle));

    value
}

pub fn is_vehicle_door_fully_open(vehicle: i32, doorIndex: i32) -> bool {
    let value = native!(
        bool,
        0x3E933CFF7B111C22,
        native_parameters!(vehicle, doorIndex)
    );

    value
}

pub fn set_vehicle_engine_on(
    vehicle: i32,
    value: bool,
    instantly: bool,
    disableAutoStart: bool,
) -> () {
    let value = native!(
        (),
        0x2497C4717C8B881E,
        native_parameters!(vehicle, value, instantly, disableAutoStart)
    );

    value
}

pub fn set_vehicle_undriveable(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x8ABA6AF54B942B95, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_provides_cover(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x5AFEEDD9BB2899D7, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_door_control(vehicle: i32, doorIndex: i32, speed: i32, angle: f32) -> () {
    let value = native!(
        (),
        0xF2BFA0430F0A0FCB,
        native_parameters!(vehicle, doorIndex, speed, angle)
    );

    value
}

pub fn set_vehicle_door_latched(vehicle: i32, doorIndex: i32, p2: bool, p3: bool, p4: bool) -> () {
    let value = native!(
        (),
        0xA5A9653A8D2CAF48,
        native_parameters!(vehicle, doorIndex, p2, p3, p4)
    );

    value
}

pub fn get_vehicle_door_angle_ratio(vehicle: i32, door: i32) -> f32 {
    let value = native!(f32, 0xFE3F9C29F7B32BD5, native_parameters!(vehicle, door));

    value
}

pub fn get_ped_using_vehicle_door(vehicle: i32, doorIndex: i32) -> i32 {
    let value = native!(
        i32,
        0x218297BF0CFD853B,
        native_parameters!(vehicle, doorIndex)
    );

    value
}

pub fn set_vehicle_door_shut(vehicle: i32, doorIndex: i32, closeInstantly: bool) -> () {
    let value = native!(
        (),
        0x93D9BD300D7789E5,
        native_parameters!(vehicle, doorIndex, closeInstantly)
    );

    value
}

pub fn set_vehicle_door_broken(vehicle: i32, doorIndex: i32, deleteDoor: bool) -> () {
    let value = native!(
        (),
        0xD4D4F6A4AB575A33,
        native_parameters!(vehicle, doorIndex, deleteDoor)
    );

    value
}

pub fn set_vehicle_can_break(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x59BF8C3D52C92F66, native_parameters!(vehicle, toggle));

    value
}

pub fn does_vehicle_have_roof(vehicle: i32) -> bool {
    let value = native!(bool, 0x8AC862B0B32C5B80, native_parameters!(vehicle));

    value
}

pub fn _0xc4b3347bd68bd609(p0: u32) -> () {
    let value = native!((), 0xC4B3347BD68BD609, native_parameters!(p0));

    value
}

pub fn _0xd3301660a57c9272(p0: u32) -> () {
    let value = native!((), 0xD3301660A57C9272, native_parameters!(p0));

    value
}

pub fn _0xb9562064627ff9db(p0: u32, p1: u32) -> () {
    let value = native!((), 0xB9562064627FF9DB, native_parameters!(p0, p1));

    value
}

pub fn is_big_vehicle(vehicle: i32) -> bool {
    let value = native!(bool, 0x9F243D3919F442FE, native_parameters!(vehicle));

    value
}

pub fn get_number_of_vehicle_colours(vehicle: i32) -> i32 {
    let value = native!(i32, 0x3B963160CD65D41E, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_colour_combination(vehicle: i32, colorCombination: i32) -> () {
    let value = native!(
        (),
        0x33E8CD3322E2FE31,
        native_parameters!(vehicle, colorCombination)
    );

    value
}

pub fn get_vehicle_colour_combination(vehicle: i32) -> i32 {
    let value = native!(i32, 0x6A842D197F845D56, native_parameters!(vehicle));

    value
}

pub fn _set_vehicle_xenon_lights_color(vehicle: i32, colorIndex: i32) -> () {
    let value = native!(
        (),
        0xE41033B25D003A07,
        native_parameters!(vehicle, colorIndex)
    );

    value
}

pub fn _get_vehicle_xenon_lights_color(vehicle: i32) -> i32 {
    let value = native!(i32, 0x3DFF319A831E0CDB, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_is_considered_by_player(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x31B927BBC44156CD, native_parameters!(vehicle, toggle));

    value
}

pub fn _0xbe5c1255a1830ff5(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xBE5C1255A1830FF5, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x9becd4b9fef3f8a6(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x9BECD4B9FEF3F8A6, native_parameters!(vehicle, p1));

    value
}

pub fn _0x88bc673ca9e0ae99(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x88BC673CA9E0AE99, native_parameters!(vehicle, p1));

    value
}

pub fn _0xe851e480b814d4ba(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0xE851E480B814D4BA, native_parameters!(vehicle, p1));

    value
}

pub fn get_random_vehicle_model_in_memory(
    p0: bool,
    modelHash: *mut u32,
    successIndicator: *mut i32,
) -> () {
    let value = native!(
        (),
        0x055BF0AC0C34F4FD,
        native_parameters!(p0, modelHash, successIndicator)
    );

    value
}

pub fn get_vehicle_door_lock_status(vehicle: i32) -> i32 {
    let value = native!(i32, 0x25BC98A59C2EA962, native_parameters!(vehicle));

    value
}

pub fn _get_vehicle_door_destroy_type(vehicle: i32, doorIndex: i32) -> i32 {
    let value = native!(
        i32,
        0xCA4AC3EAAE46EC7B,
        native_parameters!(vehicle, doorIndex)
    );

    value
}

pub fn is_vehicle_door_damaged(veh: i32, doorID: i32) -> bool {
    let value = native!(bool, 0xB8E181E559464527, native_parameters!(veh, doorID));

    value
}

pub fn _set_vehicle_door_can_break(vehicle: i32, doorIndex: i32, isBreakable: bool) -> () {
    let value = native!(
        (),
        0x2FA133A4A9D37ED8,
        native_parameters!(vehicle, doorIndex, isBreakable)
    );

    value
}

pub fn is_vehicle_bumper_bouncing(vehicle: i32, frontBumper: bool) -> bool {
    let value = native!(
        bool,
        0x27B926779DEB502D,
        native_parameters!(vehicle, frontBumper)
    );

    value
}

pub fn is_vehicle_bumper_broken_off(vehicle: i32, front: bool) -> bool {
    let value = native!(bool, 0x468056A6BB6F3846, native_parameters!(vehicle, front));

    value
}

pub fn is_cop_vehicle_in_area_3d(x1: f32, x2: f32, y1: f32, y2: f32, z1: f32, z2: f32) -> bool {
    let value = native!(
        bool,
        0x7EEF65D5F153E26A,
        native_parameters!(x1, x2, y1, y2, z1, z2)
    );

    value
}

pub fn is_vehicle_on_all_wheels(vehicle: i32) -> bool {
    let value = native!(bool, 0xB104CD1BABF302E2, native_parameters!(vehicle));

    value
}

pub fn _get_vehicle_model_monetary_value(vehicleModel: u32) -> i32 {
    let value = native!(i32, 0x5873C14A52D74236, native_parameters!(vehicleModel));

    value
}

pub fn get_vehicle_layout_hash(vehicle: i32) -> u32 {
    let value = native!(u32, 0x28D37D4F71AC5C58, native_parameters!(vehicle));

    value
}

pub fn _0xa01bc64dd4bfbbac(vehicle: i32, p1: i32) -> u32 {
    let value = native!(u32, 0xA01BC64DD4BFBBAC, native_parameters!(vehicle, p1));

    value
}

pub fn set_render_train_as_derailed(train: i32, toggle: bool) -> () {
    let value = native!((), 0x317B11A312DF5534, native_parameters!(train, toggle));

    value
}

pub fn set_vehicle_extra_colours(vehicle: i32, pearlescentColor: i32, wheelColor: i32) -> () {
    let value = native!(
        (),
        0x2036F561ADD12E33,
        native_parameters!(vehicle, pearlescentColor, wheelColor)
    );

    value
}

pub fn get_vehicle_extra_colours(
    vehicle: i32,
    pearlescentColor: *mut i32,
    wheelColor: *mut i32,
) -> () {
    let value = native!(
        (),
        0x3BC4245933A166F7,
        native_parameters!(vehicle, pearlescentColor, wheelColor)
    );

    value
}

pub fn _set_vehicle_interior_color(vehicle: i32, color: i32) -> () {
    let value = native!((), 0xF40DD601A65F7F19, native_parameters!(vehicle, color));

    value
}

pub fn _get_vehicle_interior_color(vehicle: i32, color: *mut i32) -> () {
    let value = native!((), 0x7D1464D472D32136, native_parameters!(vehicle, color));

    value
}

pub fn _set_vehicle_dashboard_color(vehicle: i32, color: i32) -> () {
    let value = native!((), 0x6089CDF6A57F326C, native_parameters!(vehicle, color));

    value
}

pub fn _get_vehicle_dashboard_color(vehicle: i32, color: *mut i32) -> () {
    let value = native!((), 0xB7635E80A5C31BFF, native_parameters!(vehicle, color));

    value
}

pub fn stop_all_garage_activity() -> () {
    let value = native!((), 0x0F87E938BDF29D66, native_parameters!());

    value
}

pub fn set_vehicle_fixed(vehicle: i32) -> () {
    let value = native!((), 0x115722B1B9C14C1C, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_deformation_fixed(vehicle: i32) -> () {
    let value = native!((), 0x953DA1E1B12C0491, native_parameters!(vehicle));

    value
}

pub fn _set_vehicle_can_engine_operate_on_fire(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x206BC5DC9D1AC70A, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_can_leak_oil(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x51BB2D88D31A914B, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_can_leak_petrol(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x192547247864DFDD, native_parameters!(vehicle, toggle));

    value
}

pub fn set_disable_vehicle_petrol_tank_fires(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x465BF26AB9684352, native_parameters!(vehicle, toggle));

    value
}

pub fn set_disable_vehicle_petrol_tank_damage(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x37C8252A7C92D017, native_parameters!(vehicle, toggle));

    value
}

pub fn set_disable_vehicle_engine_fires(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x91A0BD635321F145, native_parameters!(vehicle, toggle));

    value
}

pub fn _0xc50ce861b55eab8b(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0xC50CE861B55EAB8B, native_parameters!(vehicle, p1));

    value
}

pub fn _0x6ebfb22d646ffc18(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x6EBFB22D646FFC18, native_parameters!(vehicle, p1));

    value
}

pub fn set_disable_pretend_occupants(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x25367DE49D64CF16, native_parameters!(vehicle, toggle));

    value
}

pub fn remove_vehicles_from_generators_in_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    unk: u32,
) -> () {
    let value = native!(
        (),
        0x46A1E1A299EC4BBA,
        native_parameters!(x1, y1, z1, x2, y2, z2, unk)
    );

    value
}

pub fn set_vehicle_steer_bias(vehicle: i32, value: f32) -> () {
    let value = native!((), 0x42A8EC77D5150CBE, native_parameters!(vehicle, value));

    value
}

pub fn is_vehicle_extra_turned_on(vehicle: i32, extraId: i32) -> bool {
    let value = native!(
        bool,
        0xD2E6822DBFD6C8BD,
        native_parameters!(vehicle, extraId)
    );

    value
}

pub fn set_vehicle_extra(vehicle: i32, extraId: i32, disable: bool) -> () {
    let value = native!(
        (),
        0x7EE3A3C5E4A40CC9,
        native_parameters!(vehicle, extraId, disable)
    );

    value
}

pub fn does_extra_exist(vehicle: i32, extraId: i32) -> bool {
    let value = native!(
        bool,
        0x1262D55792428154,
        native_parameters!(vehicle, extraId)
    );

    value
}

pub fn _does_vehicle_tyre_exist(vehicle: i32, tyreIndex: i32) -> bool {
    let value = native!(
        bool,
        0x534E36D4DB9ECC5D,
        native_parameters!(vehicle, tyreIndex)
    );

    value
}

pub fn set_convertible_roof(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0xF39C4F538B5124C2, native_parameters!(vehicle, p1));

    value
}

pub fn lower_convertible_roof(vehicle: i32, instantlyLower: bool) -> () {
    let value = native!(
        (),
        0xDED51F703D0FA83D,
        native_parameters!(vehicle, instantlyLower)
    );

    value
}

pub fn raise_convertible_roof(vehicle: i32, instantlyRaise: bool) -> () {
    let value = native!(
        (),
        0x8F5FB35D7E88FC70,
        native_parameters!(vehicle, instantlyRaise)
    );

    value
}

pub fn get_convertible_roof_state(vehicle: i32) -> i32 {
    let value = native!(i32, 0xF8C397922FC03F41, native_parameters!(vehicle));

    value
}

pub fn is_vehicle_a_convertible(vehicle: i32, p1: bool) -> bool {
    let value = native!(bool, 0x52F357A30698BCCE, native_parameters!(vehicle, p1));

    value
}

pub fn _transform_vehicle_to_submarine(vehicle: i32, noAnimation: bool) -> () {
    let value = native!(
        (),
        0xBE4C854FFDB6EEBE,
        native_parameters!(vehicle, noAnimation)
    );

    value
}

pub fn _transform_submarine_to_vehicle(vehicle: i32, noAnimation: bool) -> () {
    let value = native!(
        (),
        0x2A69FFD1B42BFF9E,
        native_parameters!(vehicle, noAnimation)
    );

    value
}

pub fn _get_is_submarine_vehicle_transformed(vehicle: i32) -> bool {
    let value = native!(bool, 0xA77DC70BD689A1E5, native_parameters!(vehicle));

    value
}

pub fn is_vehicle_stopped_at_traffic_lights(vehicle: i32) -> bool {
    let value = native!(bool, 0x2959F696AE390A99, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_damage(
    vehicle: i32,
    xOffset: f32,
    yOffset: f32,
    zOffset: f32,
    damage: f32,
    radius: f32,
    focusOnModel: bool,
) -> () {
    let value = native!(
        (),
        0xA1DD317EA8FD4F29,
        native_parameters!(
            vehicle,
            xOffset,
            yOffset,
            zOffset,
            damage,
            radius,
            focusOnModel
        )
    );

    value
}

pub fn _0x35bb21de06784373(p0: u32, p1: u32) -> () {
    let value = native!((), 0x35BB21DE06784373, native_parameters!(p0, p1));

    value
}

pub fn get_vehicle_engine_health(vehicle: i32) -> f32 {
    let value = native!(f32, 0xC45D23BAF168AAB8, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_engine_health(vehicle: i32, health: f32) -> () {
    let value = native!((), 0x45F6D8EEF34ABEF1, native_parameters!(vehicle, health));

    value
}

pub fn _set_plane_engine_health(vehicle: i32, health: f32) -> () {
    let value = native!((), 0x2A86A0475B6A1434, native_parameters!(vehicle, health));

    value
}

pub fn get_vehicle_petrol_tank_health(vehicle: i32) -> f32 {
    let value = native!(f32, 0x7D5DABE888D2D074, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_petrol_tank_health(vehicle: i32, health: f32) -> () {
    let value = native!((), 0x70DB57649FA8D0D8, native_parameters!(vehicle, health));

    value
}

pub fn is_vehicle_stuck_timer_up(vehicle: i32, p1: i32, p2: i32) -> bool {
    let value = native!(
        bool,
        0x679BE1DAF71DA874,
        native_parameters!(vehicle, p1, p2)
    );

    value
}

pub fn reset_vehicle_stuck_timer(vehicle: i32, nullAttributes: i32) -> () {
    let value = native!(
        (),
        0xD7591B0065AFAA7A,
        native_parameters!(vehicle, nullAttributes)
    );

    value
}

pub fn is_vehicle_driveable(vehicle: i32, isOnFireCheck: bool) -> bool {
    let value = native!(
        bool,
        0x4C241E39B23DF959,
        native_parameters!(vehicle, isOnFireCheck)
    );

    value
}

pub fn set_vehicle_has_been_owned_by_player(vehicle: i32, owned: bool) -> () {
    let value = native!((), 0x2B5F9D2AF1F1722D, native_parameters!(vehicle, owned));

    value
}

pub fn set_vehicle_needs_to_be_hotwired(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xFBA550EA44404EE6, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x9f3f689b814f2599(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x9F3F689B814F2599, native_parameters!(vehicle, p1));

    value
}

pub fn _0x4e74e62e0a97e901(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x4E74E62E0A97E901, native_parameters!(vehicle, p1));

    value
}

pub fn start_vehicle_horn(vehicle: i32, duration: i32, mode: u32, forever: bool) -> () {
    let value = native!(
        (),
        0x9C8C6504B5B63D2C,
        native_parameters!(vehicle, duration, mode, forever)
    );

    value
}

pub fn _set_vehicle_silent(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x9D44FCCE98450843, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_has_strong_axles(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x92F0CF722BC4202F, native_parameters!(vehicle, toggle));

    value
}

pub fn get_display_name_from_vehicle_model(modelHash: u32) -> String {
    let value = native!(*const i8, 0xB215AAC32D25D019, native_parameters!(modelHash));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _get_make_name_from_vehicle_model(modelHash: u32) -> String {
    let value = native!(*const i8, 0xF7AF4F159FF99F97, native_parameters!(modelHash));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_vehicle_deformation_at_pos(
    vehicle: i32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x4EC6CFBC7B2E9536,
        native_parameters!(vehicle, offsetX, offsetY, offsetZ)
    );

    value
}

pub fn set_vehicle_livery(vehicle: i32, livery: i32) -> () {
    let value = native!((), 0x60BF608F1B8CD1B6, native_parameters!(vehicle, livery));

    value
}

pub fn get_vehicle_livery(vehicle: i32) -> i32 {
    let value = native!(i32, 0x2BB9230590DA5E8A, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_livery_count(vehicle: i32) -> i32 {
    let value = native!(i32, 0x87B63E25A529D526, native_parameters!(vehicle));

    value
}

pub fn _set_vehicle_roof_livery(vehicle: i32, livery: i32) -> () {
    let value = native!((), 0xA6D3A8750DC73270, native_parameters!(vehicle, livery));

    value
}

pub fn _get_vehicle_roof_livery(vehicle: i32) -> i32 {
    let value = native!(i32, 0x60190048C0764A26, native_parameters!(vehicle));

    value
}

pub fn _get_vehicle_roof_livery_count(vehicle: i32) -> i32 {
    let value = native!(i32, 0x5ECB40269053C0D4, native_parameters!(vehicle));

    value
}

pub fn is_vehicle_window_intact(vehicle: i32, windowIndex: i32) -> bool {
    let value = native!(
        bool,
        0x46E571A0E20D01F1,
        native_parameters!(vehicle, windowIndex)
    );

    value
}

pub fn are_all_vehicle_windows_intact(vehicle: i32) -> bool {
    let value = native!(bool, 0x11D862A3E977A9EF, native_parameters!(vehicle));

    value
}

pub fn are_any_vehicle_seats_free(vehicle: i32) -> bool {
    let value = native!(bool, 0x2D34FC3BC4ADB780, native_parameters!(vehicle));

    value
}

pub fn reset_vehicle_wheels(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x21D2E5662C1F6FED, native_parameters!(vehicle, toggle));

    value
}

pub fn is_heli_part_broken(vehicle: i32, p1: bool, p2: bool, p3: bool) -> bool {
    let value = native!(
        bool,
        0xBC74B4BE25EB6C8A,
        native_parameters!(vehicle, p1, p2, p3)
    );

    value
}

pub fn get_heli_main_rotor_health(vehicle: i32) -> f32 {
    let value = native!(f32, 0xE4CB7541F413D2C5, native_parameters!(vehicle));

    value
}

pub fn get_heli_tail_rotor_health(vehicle: i32) -> f32 {
    let value = native!(f32, 0xAE8CE82A4219AC8C, native_parameters!(vehicle));

    value
}

pub fn get_heli_tail_boom_health(vehicle: i32) -> f32 {
    let value = native!(f32, 0xAC51915D27E4A5F7, native_parameters!(vehicle));

    value
}

pub fn _0x4056ea1105f5abd7(p0: u32, p1: u32) -> () {
    let value = native!((), 0x4056EA1105F5ABD7, native_parameters!(p0, p1));

    value
}

pub fn _set_heli_tail_rotor_health(vehicle: i32, health: f32) -> () {
    let value = native!((), 0xFE205F38AAA58E5B, native_parameters!(vehicle, health));

    value
}

pub fn set_heli_tail_explode_throw_dashboard(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x3EC8BF18AA453FE9, native_parameters!(vehicle, p1));

    value
}

pub fn set_vehicle_name_debug(vehicle: i32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xBFDF984E2C22B94F,
        native_parameters!(vehicle, name.as_ptr())
    );

    value
}

pub fn set_vehicle_explodes_on_high_explosion_damage(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x71B0892EC081D60A, native_parameters!(vehicle, toggle));

    value
}

pub fn _0xd565f438137f0e10(p0: u32, p1: u32) -> () {
    let value = native!((), 0xD565F438137F0E10, native_parameters!(p0, p1));

    value
}

pub fn _0x3441cad2f2231923(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x3441CAD2F2231923, native_parameters!(vehicle, p1));

    value
}

pub fn set_vehicle_disable_towing(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x2B6747FAA9DB9D6B, native_parameters!(vehicle, toggle));

    value
}

pub fn _does_vehicle_have_landing_gear(vehicle: i32) -> bool {
    let value = native!(bool, 0xE43701C36CAFF1A4, native_parameters!(vehicle));

    value
}

pub fn control_landing_gear(vehicle: i32, state: i32) -> () {
    let value = native!((), 0xCFC8BE9A5E1FE575, native_parameters!(vehicle, state));

    value
}

pub fn get_landing_gear_state(vehicle: i32) -> i32 {
    let value = native!(i32, 0x9B0F3DCA3DB0F4CD, native_parameters!(vehicle));

    value
}

pub fn is_any_vehicle_near_point(x: f32, y: f32, z: f32, radius: f32) -> bool {
    let value = native!(
        bool,
        0x61E1DD6125A3EEE6,
        native_parameters!(x, y, z, radius)
    );

    value
}

pub fn request_vehicle_high_detail_model(vehicle: i32) -> () {
    let value = native!((), 0xA6E9FDCB2C76785E, native_parameters!(vehicle));

    value
}

pub fn remove_vehicle_high_detail_model(vehicle: i32) -> () {
    let value = native!((), 0x00689CDE5F7C6787, native_parameters!(vehicle));

    value
}

pub fn is_vehicle_high_detail(vehicle: i32) -> bool {
    let value = native!(bool, 0x1F25887F3C104278, native_parameters!(vehicle));

    value
}

pub fn request_vehicle_asset(vehicleHash: u32, vehicleAsset: i32) -> () {
    let value = native!(
        (),
        0x81A15811460FAB3A,
        native_parameters!(vehicleHash, vehicleAsset)
    );

    value
}

pub fn has_vehicle_asset_loaded(vehicleAsset: i32) -> bool {
    let value = native!(bool, 0x1BBE0523B8DB9A21, native_parameters!(vehicleAsset));

    value
}

pub fn remove_vehicle_asset(vehicleAsset: i32) -> () {
    let value = native!((), 0xACE699C71AB9DEB5, native_parameters!(vehicleAsset));

    value
}

pub fn set_vehicle_tow_truck_arm_position(vehicle: i32, position: f32) -> () {
    let value = native!(
        (),
        0xFE54B92A344583CA,
        native_parameters!(vehicle, position)
    );

    value
}

pub fn attach_vehicle_to_tow_truck(
    towTruck: i32,
    vehicle: i32,
    rear: bool,
    hookOffsetX: f32,
    hookOffsetY: f32,
    hookOffsetZ: f32,
) -> () {
    let value = native!(
        (),
        0x29A16F8D621C4508,
        native_parameters!(
            towTruck,
            vehicle,
            rear,
            hookOffsetX,
            hookOffsetY,
            hookOffsetZ
        )
    );

    value
}

pub fn detach_vehicle_from_tow_truck(towTruck: i32, vehicle: i32) -> () {
    let value = native!(
        (),
        0xC2DB6B6708350ED8,
        native_parameters!(towTruck, vehicle)
    );

    value
}

pub fn detach_vehicle_from_any_tow_truck(vehicle: i32) -> bool {
    let value = native!(bool, 0xD0E9CE05A1E68CD8, native_parameters!(vehicle));

    value
}

pub fn is_vehicle_attached_to_tow_truck(towTruck: i32, vehicle: i32) -> bool {
    let value = native!(
        bool,
        0x146DF9EC4C4B9FD4,
        native_parameters!(towTruck, vehicle)
    );

    value
}

pub fn get_entity_attached_to_tow_truck(towTruck: i32) -> i32 {
    let value = native!(i32, 0xEFEA18DCF10F8F75, native_parameters!(towTruck));

    value
}

pub fn set_vehicle_automatically_attaches(vehicle: i32, p1: bool, p2: u32) -> u32 {
    let value = native!(u32, 0x8BA6F76BC53A1493, native_parameters!(vehicle, p1, p2));

    value
}

pub fn set_vehicle_bulldozer_arm_position(vehicle: i32, position: f32, p2: bool) -> () {
    let value = native!(
        (),
        0xF8EBCCC96ADB9FB7,
        native_parameters!(vehicle, position, p2)
    );

    value
}

pub fn set_vehicle_tank_turret_position(vehicle: i32, position: f32, p2: bool) -> () {
    let value = native!(
        (),
        0x56B94C6D7127DFBA,
        native_parameters!(vehicle, position, p2)
    );

    value
}

pub fn _0x0581730ab9380412(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0x0581730AB9380412,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _0x737e398138550fff(p0: u32, p1: u32) -> () {
    let value = native!((), 0x737E398138550FFF, native_parameters!(p0, p1));

    value
}

pub fn set_vehicle_turret_speed_this_frame(vehicle: i32, speed: f32) -> () {
    let value = native!((), 0x1093408B4B9D1146, native_parameters!(vehicle, speed));

    value
}

pub fn _disable_vehicle_turret_movement_this_frame(vehicle: i32) -> () {
    let value = native!((), 0x32CAEDF24A583345, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_flight_nozzle_position(vehicle: i32, angleRatio: f32) -> () {
    let value = native!(
        (),
        0x30D779DE7C4F6DD3,
        native_parameters!(vehicle, angleRatio)
    );

    value
}

pub fn set_vehicle_flight_nozzle_position_immediate(vehicle: i32, angle: f32) -> () {
    let value = native!((), 0x9AA47FFF660CB932, native_parameters!(vehicle, angle));

    value
}

pub fn _get_vehicle_flight_nozzle_position(plane: i32) -> f32 {
    let value = native!(f32, 0xDA62027C8BDB326E, native_parameters!(plane));

    value
}

pub fn _set_disable_vehicle_flight_nozzle_position(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xCE2B43770B655F8F, native_parameters!(vehicle, toggle));

    value
}

pub fn _0xa4822f1cf23f4810(
    outVec: *mut NativeVector3,
    p1: u32,
    outVec1: *mut NativeVector3,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
    p8: u32,
) -> bool {
    let value = native!(
        bool,
        0xA4822F1CF23F4810,
        native_parameters!(outVec, p1, outVec1, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn set_vehicle_burnout(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xFB8794444A7D60FB, native_parameters!(vehicle, toggle));

    value
}

pub fn is_vehicle_in_burnout(vehicle: i32) -> bool {
    let value = native!(bool, 0x1297A88E081430EB, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_reduce_grip(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x222FF6A823D122E2, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_vehicle_reduce_traction(vehicle: i32, val: i32) -> () {
    let value = native!((), 0x6DEE944E1EE90CFB, native_parameters!(vehicle, val));

    value
}

pub fn set_vehicle_indicator_lights(vehicle: i32, turnSignal: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0xB5D45264751B7DF0,
        native_parameters!(vehicle, turnSignal, toggle)
    );

    value
}

pub fn set_vehicle_brake_lights(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x92B35082E0B42F66, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_handbrake(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x684785568EF26A22, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_brake(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xE4E2FD323574965C, native_parameters!(vehicle, toggle));

    value
}

pub fn instantly_fill_vehicle_population() -> () {
    let value = native!((), 0x48ADC8A773564670, native_parameters!());

    value
}

pub fn _has_filled_vehicle_population() -> bool {
    let value = native!(bool, 0x91D6DD290888CBAB, native_parameters!());

    value
}

pub fn _0x51db102f4a3ba5e0(toggle: bool) -> () {
    let value = native!((), 0x51DB102F4A3BA5E0, native_parameters!(toggle));

    value
}

pub fn _0xa4a9a4c40e615885(p0: i32) -> () {
    let value = native!((), 0xA4A9A4C40E615885, native_parameters!(p0));

    value
}

pub fn get_vehicle_trailer_vehicle(vehicle: i32, trailer: *mut i32) -> bool {
    let value = native!(
        bool,
        0x1CDD6BADC297830D,
        native_parameters!(vehicle, trailer)
    );

    value
}

pub fn set_vehicle_uses_large_rear_ramp(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xCAC66558B944DA67, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_rudder_broken(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x09606148B6C71DEF, native_parameters!(vehicle, toggle));

    value
}

pub fn set_convertible_roof_latch_state(vehicle: i32, state: bool) -> () {
    let value = native!((), 0x1A78AD3D8240536F, native_parameters!(vehicle, state));

    value
}

pub fn get_vehicle_estimated_max_speed(vehicle: i32) -> f32 {
    let value = native!(f32, 0x53AF99BAA671CA47, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_max_braking(vehicle: i32) -> f32 {
    let value = native!(f32, 0xAD7E85FC227197C4, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_max_traction(vehicle: i32) -> f32 {
    let value = native!(f32, 0xA132FB5370554DB0, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_acceleration(vehicle: i32) -> f32 {
    let value = native!(f32, 0x5DD35C8D074E57AE, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_model_estimated_max_speed(modelHash: u32) -> f32 {
    let value = native!(f32, 0xF417C2502FFFED43, native_parameters!(modelHash));

    value
}

pub fn get_vehicle_model_max_braking(modelHash: u32) -> f32 {
    let value = native!(f32, 0xDC53FD41B4ED944C, native_parameters!(modelHash));

    value
}

pub fn get_vehicle_model_max_braking_max_mods(modelHash: u32) -> f32 {
    let value = native!(f32, 0xBFBA3BA79CFF7EBF, native_parameters!(modelHash));

    value
}

pub fn get_vehicle_model_max_traction(modelHash: u32) -> f32 {
    let value = native!(f32, 0x539DE94D44FDFD0D, native_parameters!(modelHash));

    value
}

pub fn get_vehicle_model_acceleration(modelHash: u32) -> f32 {
    let value = native!(f32, 0x8C044C5C84505B6A, native_parameters!(modelHash));

    value
}

pub fn _get_vehicle_model_estimated_agility(modelHash: u32) -> f32 {
    let value = native!(f32, 0x53409B5163D5B846, native_parameters!(modelHash));

    value
}

pub fn _get_vehicle_model_max_knots(modelHash: u32) -> f32 {
    let value = native!(f32, 0xC6AD107DDC9054CC, native_parameters!(modelHash));

    value
}

pub fn _get_vehicle_model_move_resistance(modelHash: u32) -> f32 {
    let value = native!(f32, 0x5AA3F878A178C4FC, native_parameters!(modelHash));

    value
}

pub fn get_vehicle_class_estimated_max_speed(vehicleClass: i32) -> f32 {
    let value = native!(f32, 0x00C09F246ABEDD82, native_parameters!(vehicleClass));

    value
}

pub fn get_vehicle_class_max_traction(vehicleClass: i32) -> f32 {
    let value = native!(f32, 0xDBC86D85C5059461, native_parameters!(vehicleClass));

    value
}

pub fn get_vehicle_class_max_agility(vehicleClass: i32) -> f32 {
    let value = native!(f32, 0x4F930AD022D6DE3B, native_parameters!(vehicleClass));

    value
}

pub fn get_vehicle_class_max_acceleration(vehicleClass: i32) -> f32 {
    let value = native!(f32, 0x2F83E7E45D9EA7AE, native_parameters!(vehicleClass));

    value
}

pub fn get_vehicle_class_max_braking(vehicleClass: i32) -> f32 {
    let value = native!(f32, 0x4BF54C16EC8FEC03, native_parameters!(vehicleClass));

    value
}

pub fn add_road_node_speed_zone(x: f32, y: f32, z: f32, radius: f32, speed: f32, p5: bool) -> i32 {
    let value = native!(
        i32,
        0x2CE544C68FB812A0,
        native_parameters!(x, y, z, radius, speed, p5)
    );

    value
}

pub fn remove_road_node_speed_zone(speedzone: i32) -> bool {
    let value = native!(bool, 0x1033371FC8E842A7, native_parameters!(speedzone));

    value
}

pub fn open_bomb_bay_doors(vehicle: i32) -> () {
    let value = native!((), 0x87E7F24270732CB1, native_parameters!(vehicle));

    value
}

pub fn close_bomb_bay_doors(vehicle: i32) -> () {
    let value = native!((), 0x3556041742A0DC74, native_parameters!(vehicle));

    value
}

pub fn _are_bomb_bay_doors_open(aircraft: i32) -> bool {
    let value = native!(bool, 0xD0917A423314BBA8, native_parameters!(aircraft));

    value
}

pub fn is_vehicle_searchlight_on(vehicle: i32) -> bool {
    let value = native!(bool, 0xC0F97FCE55094987, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_searchlight(heli: i32, toggle: bool, canBeUsedByAI: bool) -> () {
    let value = native!(
        (),
        0x14E85C5EE7A4D542,
        native_parameters!(heli, toggle, canBeUsedByAI)
    );

    value
}

pub fn _does_vehicle_have_searchlight(vehicle: i32) -> bool {
    let value = native!(bool, 0x99015ED7DBEA5113, native_parameters!(vehicle));

    value
}

pub fn _is_vehicle_seat_accessible(
    ped: i32,
    vehicle: i32,
    seatIndex: i32,
    side: bool,
    onEnter: bool,
) -> bool {
    let value = native!(
        bool,
        0x639431E895B9AA57,
        native_parameters!(ped, vehicle, seatIndex, side, onEnter)
    );

    value
}

pub fn _get_entry_position_of_door(vehicle: i32, doorIndex: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xC0572928C0ABFDA3,
        native_parameters!(vehicle, doorIndex)
    );

    value
}

pub fn can_shuffle_seat(vehicle: i32, seatIndex: i32) -> bool {
    let value = native!(
        bool,
        0x30785D90C956BF35,
        native_parameters!(vehicle, seatIndex)
    );

    value
}

pub fn get_num_mod_kits(vehicle: i32) -> i32 {
    let value = native!(i32, 0x33F2E3FE70EAAE1D, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_mod_kit(vehicle: i32, modKit: i32) -> () {
    let value = native!((), 0x1F2AA07F00B3217A, native_parameters!(vehicle, modKit));

    value
}

pub fn get_vehicle_mod_kit(vehicle: i32) -> i32 {
    let value = native!(i32, 0x6325D1A044AE510D, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_mod_kit_type(vehicle: i32) -> i32 {
    let value = native!(i32, 0xFC058F5121E54C32, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_wheel_type(vehicle: i32) -> i32 {
    let value = native!(i32, 0xB3ED1BFB4BE636DC, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_wheel_type(vehicle: i32, WheelType: i32) -> () {
    let value = native!(
        (),
        0x487EB21CC7295BA1,
        native_parameters!(vehicle, WheelType)
    );

    value
}

pub fn get_num_mod_colors(paintType: i32, p1: bool) -> i32 {
    let value = native!(i32, 0xA551BE18C11A476D, native_parameters!(paintType, p1));

    value
}

pub fn set_vehicle_mod_color_1(
    vehicle: i32,
    paintType: i32,
    color: i32,
    pearlescentColor: i32,
) -> () {
    let value = native!(
        (),
        0x43FEB945EE7F85B8,
        native_parameters!(vehicle, paintType, color, pearlescentColor)
    );

    value
}

pub fn set_vehicle_mod_color_2(vehicle: i32, paintType: i32, color: i32) -> () {
    let value = native!(
        (),
        0x816562BADFDEC83E,
        native_parameters!(vehicle, paintType, color)
    );

    value
}

pub fn get_vehicle_mod_color_1(
    vehicle: i32,
    paintType: *mut i32,
    color: *mut i32,
    pearlescentColor: *mut i32,
) -> () {
    let value = native!(
        (),
        0xE8D65CA700C9A693,
        native_parameters!(vehicle, paintType, color, pearlescentColor)
    );

    value
}

pub fn get_vehicle_mod_color_2(vehicle: i32, paintType: *mut i32, color: *mut i32) -> () {
    let value = native!(
        (),
        0x81592BE4E3878728,
        native_parameters!(vehicle, paintType, color)
    );

    value
}

pub fn get_vehicle_mod_color_1_name(vehicle: i32, p1: bool) -> String {
    let value = native!(
        *const i8,
        0xB45085B721EFD38C,
        native_parameters!(vehicle, p1)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_vehicle_mod_color_2_name(vehicle: i32) -> String {
    let value = native!(*const i8, 0x4967A516ED23A5A1, native_parameters!(vehicle));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn have_vehicle_mods_streamed_in(vehicle: i32) -> bool {
    let value = native!(bool, 0x9A83F5F9963775EF, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_mod(vehicle: i32, modType: i32, modIndex: i32, customTires: bool) -> () {
    let value = native!(
        (),
        0x6AF0636DDEDCB6DD,
        native_parameters!(vehicle, modType, modIndex, customTires)
    );

    value
}

pub fn get_vehicle_mod(vehicle: i32, modType: i32) -> i32 {
    let value = native!(
        i32,
        0x772960298DA26FDB,
        native_parameters!(vehicle, modType)
    );

    value
}

pub fn get_vehicle_mod_variation(vehicle: i32, modType: i32) -> bool {
    let value = native!(
        bool,
        0xB3924ECD70E095DC,
        native_parameters!(vehicle, modType)
    );

    value
}

pub fn get_num_vehicle_mods(vehicle: i32, modType: i32) -> i32 {
    let value = native!(
        i32,
        0xE38E9162A2500646,
        native_parameters!(vehicle, modType)
    );

    value
}

pub fn remove_vehicle_mod(vehicle: i32, modType: i32) -> () {
    let value = native!((), 0x92D619E420858204, native_parameters!(vehicle, modType));

    value
}

pub fn toggle_vehicle_mod(vehicle: i32, modType: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0x2A1F4F37F95BAD08,
        native_parameters!(vehicle, modType, toggle)
    );

    value
}

pub fn is_toggle_mod_on(vehicle: i32, modType: i32) -> bool {
    let value = native!(
        bool,
        0x84B233A8C8FC8AE7,
        native_parameters!(vehicle, modType)
    );

    value
}

pub fn get_mod_text_label(vehicle: i32, modType: i32, modValue: i32) -> String {
    let value = native!(
        *const i8,
        0x8935624F8C5592CC,
        native_parameters!(vehicle, modType, modValue)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_mod_slot_name(vehicle: i32, modType: i32) -> String {
    let value = native!(
        *const i8,
        0x51F0FEB9F6AE98C0,
        native_parameters!(vehicle, modType)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_livery_name(vehicle: i32, liveryIndex: i32) -> String {
    let value = native!(
        *const i8,
        0xB4C7A93837C91A1F,
        native_parameters!(vehicle, liveryIndex)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_vehicle_mod_modifier_value(vehicle: i32, modType: i32, modIndex: i32) -> f32 {
    let value = native!(
        f32,
        0x90A38E9838E0A8C1,
        native_parameters!(vehicle, modType, modIndex)
    );

    value
}

pub fn get_vehicle_mod_identifier_hash(vehicle: i32, modType: i32, modIndex: i32) -> u32 {
    let value = native!(
        u32,
        0x4593CF82AA179706,
        native_parameters!(vehicle, modType, modIndex)
    );

    value
}

pub fn preload_vehicle_mod(p0: u32, modType: i32, p2: u32) -> () {
    let value = native!((), 0x758F49C24925568A, native_parameters!(p0, modType, p2));

    value
}

pub fn has_preload_mods_finished(p0: u32) -> bool {
    let value = native!(bool, 0x06F43E5175EB6D96, native_parameters!(p0));

    value
}

pub fn release_preload_mods(vehicle: i32) -> () {
    let value = native!((), 0x445D79F995508307, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_tyre_smoke_color(vehicle: i32, r: i32, g: i32, b: i32) -> () {
    let value = native!((), 0xB5BA80F839791C0F, native_parameters!(vehicle, r, g, b));

    value
}

pub fn get_vehicle_tyre_smoke_color(vehicle: i32, r: *mut i32, g: *mut i32, b: *mut i32) -> () {
    let value = native!((), 0xB635392A4938B3C3, native_parameters!(vehicle, r, g, b));

    value
}

pub fn set_vehicle_window_tint(vehicle: i32, tint: i32) -> () {
    let value = native!((), 0x57C51E6BAD752696, native_parameters!(vehicle, tint));

    value
}

pub fn get_vehicle_window_tint(vehicle: i32) -> i32 {
    let value = native!(i32, 0x0EE21293DAD47C95, native_parameters!(vehicle));

    value
}

pub fn get_num_vehicle_window_tints() -> i32 {
    let value = native!(i32, 0x9D1224004B3A6707, native_parameters!());

    value
}

pub fn get_vehicle_color(vehicle: i32, r: *mut i32, g: *mut i32, b: *mut i32) -> () {
    let value = native!((), 0xF3CC740D36221548, native_parameters!(vehicle, r, g, b));

    value
}

pub fn _0xeebfc7a7efdc35b4(vehicle: i32) -> i32 {
    let value = native!(i32, 0xEEBFC7A7EFDC35B4, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_cause_of_destruction(vehicle: i32) -> u32 {
    let value = native!(u32, 0xE495D1EF4C91FD20, native_parameters!(vehicle));

    value
}

pub fn _0x5ee5632f47ae9695(vehicle: i32, health: f32) -> () {
    let value = native!((), 0x5EE5632F47AE9695, native_parameters!(vehicle, health));

    value
}

pub fn get_is_left_vehicle_headlight_damaged(vehicle: i32) -> bool {
    let value = native!(bool, 0x5EF77C9ADD3B11A3, native_parameters!(vehicle));

    value
}

pub fn get_is_right_vehicle_headlight_damaged(vehicle: i32) -> bool {
    let value = native!(bool, 0xA7ECB73355EB2F20, native_parameters!(vehicle));

    value
}

pub fn _is_vehicle_engine_on_fire(vehicle: i32) -> bool {
    let value = native!(bool, 0xEC69ADF931AAE0C3, native_parameters!(vehicle));

    value
}

pub fn modify_vehicle_top_speed(vehicle: i32, value: f32) -> () {
    let value = native!((), 0x93A3996368C94158, native_parameters!(vehicle, value));

    value
}

pub fn _set_vehicle_max_speed(vehicle: i32, speed: f32) -> () {
    let value = native!((), 0xBAA045B4E42F3C06, native_parameters!(vehicle, speed));

    value
}

pub fn _0x1cf38d529d7441d9(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x1CF38D529D7441D9, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x1f9fb66f3a3842d2(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x1F9FB66F3A3842D2, native_parameters!(vehicle, p1));

    value
}

pub fn _0x59c3757b3b7408e8(vehicle: i32, toggle: bool, p2: f32) -> () {
    let value = native!(
        (),
        0x59C3757B3B7408E8,
        native_parameters!(vehicle, toggle, p2)
    );

    value
}

pub fn add_vehicle_combat_angled_avoidance_area(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
) -> u32 {
    let value = native!(
        u32,
        0x54B0F614960F4A5F,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn remove_vehicle_combat_avoidance_area(p0: u32) -> () {
    let value = native!((), 0xE30524E1871F481D, native_parameters!(p0));

    value
}

pub fn is_any_ped_rappelling_from_heli(vehicle: i32) -> bool {
    let value = native!(bool, 0x291E373D483E7EE7, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_cheat_power_increase(vehicle: i32, value: f32) -> () {
    let value = native!((), 0xB59E4BD37AE292DB, native_parameters!(vehicle, value));

    value
}

pub fn _0x0ad9e8f87ff7c16f(p0: u32, p1: bool) -> () {
    let value = native!((), 0x0AD9E8F87FF7C16F, native_parameters!(p0, p1));

    value
}

pub fn set_vehicle_is_wanted(vehicle: i32, state: bool) -> () {
    let value = native!((), 0xF7EC25A3EBEEC726, native_parameters!(vehicle, state));

    value
}

pub fn _set_boat_boom_position_ratio(vehicle: i32, ratio: f32) -> () {
    let value = native!((), 0xF488C566413B4232, native_parameters!(vehicle, ratio));

    value
}

pub fn _get_boat_boom_position_ratio_2(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0xC1F981A6F74F0C23, native_parameters!(vehicle, p1));

    value
}

pub fn _get_boat_boom_position_ratio_3(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x0F3B4D4E43177236, native_parameters!(vehicle, p1));

    value
}

pub fn get_boat_boom_position_ratio(vehicle: i32) -> f32 {
    let value = native!(f32, 0x6636C535F6CC2725, native_parameters!(vehicle));

    value
}

pub fn disable_plane_aileron(vehicle: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x23428FC53C60919C, native_parameters!(vehicle, p1, p2));

    value
}

pub fn get_is_vehicle_engine_running(vehicle: i32) -> bool {
    let value = native!(bool, 0xAE31E7DF9B5B132E, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_use_alternate_handling(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x1D97D1E3A70A649F, native_parameters!(vehicle, toggle));

    value
}

pub fn set_bike_on_stand(vehicle: i32, x: f32, y: f32) -> () {
    let value = native!((), 0x9CFA4896C3A53CBB, native_parameters!(vehicle, x, y));

    value
}

pub fn _0xab04325045427aae(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0xAB04325045427AAE, native_parameters!(vehicle, p1));

    value
}

pub fn _0xcfd778e7904c255e(vehicle: i32) -> () {
    let value = native!((), 0xCFD778E7904C255E, native_parameters!(vehicle));

    value
}

pub fn set_last_driven_vehicle(vehicle: i32) -> () {
    let value = native!((), 0xACFB2463CC22BED2, native_parameters!(vehicle));

    value
}

pub fn get_last_driven_vehicle() -> i32 {
    let value = native!(i32, 0xB2D06FAEDE65B577, native_parameters!());

    value
}

pub fn clear_last_driven_vehicle() -> () {
    let value = native!((), 0xE01903C47C7AC89E, native_parameters!());

    value
}

pub fn set_vehicle_has_been_driven_flag(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x02398B627547189C, native_parameters!(vehicle, toggle));

    value
}

pub fn set_task_vehicle_goto_plane_min_height_above_terrain(plane: i32, height: i32) -> () {
    let value = native!((), 0xB893215D8D4C015B, native_parameters!(plane, height));

    value
}

pub fn set_vehicle_lod_multiplier(vehicle: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0x93AE6A61BE015BF1,
        native_parameters!(vehicle, multiplier)
    );

    value
}

pub fn set_vehicle_can_save_in_garage(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x428BACCDF5E26EAD, native_parameters!(vehicle, toggle));

    value
}

pub fn _get_vehicle_number_of_broken_off_bones(vehicle: i32) -> i32 {
    let value = native!(i32, 0x42A4BEB35D372407, native_parameters!(vehicle));

    value
}

pub fn _get_vehicle_number_of_broken_bones(vehicle: i32) -> i32 {
    let value = native!(i32, 0x2C8CBFE1EA5FC631, native_parameters!(vehicle));

    value
}

pub fn _0x4d9d109f63fee1d4(p0: u32, p1: bool) -> () {
    let value = native!((), 0x4D9D109F63FEE1D4, native_parameters!(p0, p1));

    value
}

pub fn _0x279d50de5652d935(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x279D50DE5652D935, native_parameters!(vehicle, toggle));

    value
}

pub fn copy_vehicle_damages(sourceVehicle: i32, targetVehicle: i32) -> () {
    let value = native!(
        (),
        0xE44A982368A4AF23,
        native_parameters!(sourceVehicle, targetVehicle)
    );

    value
}

pub fn _0xf25e02cb9c5818f8() -> () {
    let value = native!((), 0xF25E02CB9C5818F8, native_parameters!());

    value
}

pub fn set_lights_cutoff_distance_tweak(distance: f32) -> () {
    let value = native!((), 0xBC3CCA5844452B06, native_parameters!(distance));

    value
}

pub fn set_vehicle_shoot_at_target(
    driver: i32,
    entity: i32,
    xTarget: f32,
    yTarget: f32,
    zTarget: f32,
) -> () {
    let value = native!(
        (),
        0x74CD9A9327A282EA,
        native_parameters!(driver, entity, xTarget, yTarget, zTarget)
    );

    value
}

pub fn get_vehicle_lock_on_target(vehicle: i32, entity: *mut i32) -> bool {
    let value = native!(
        bool,
        0x8F5EBAB1F260CFCE,
        native_parameters!(vehicle, entity)
    );

    value
}

pub fn set_force_hd_vehicle(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x97CE68CB032583F0, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x182f266c2d9e2beb(vehicle: i32, p1: f32) -> () {
    let value = native!((), 0x182F266C2D9E2BEB, native_parameters!(vehicle, p1));

    value
}

pub fn get_vehicle_plate_type(vehicle: i32) -> i32 {
    let value = native!(i32, 0x9CCC9525BF2408E0, native_parameters!(vehicle));

    value
}

pub fn track_vehicle_visibility(vehicle: i32) -> () {
    let value = native!((), 0x64473AEFDCF47DCA, native_parameters!(vehicle));

    value
}

pub fn is_vehicle_visible(vehicle: i32) -> bool {
    let value = native!(bool, 0xAA0A52D24FB98293, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_gravity(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x89F149B6131E57DA, native_parameters!(vehicle, toggle));

    value
}

pub fn set_enable_vehicle_slipstreaming(toggle: bool) -> () {
    let value = native!((), 0xE6C0C80B8C867537, native_parameters!(toggle));

    value
}

pub fn _0xf051d9bfb6ba39c0(p0: u32) -> () {
    let value = native!((), 0xF051D9BFB6BA39C0, native_parameters!(p0));

    value
}

pub fn _get_vehicle_current_slipstream_draft(vehicle: i32) -> f32 {
    let value = native!(f32, 0x36492C2F0D134C56, native_parameters!(vehicle));

    value
}

pub fn _is_vehicle_slipstream_leader(vehicle: i32) -> bool {
    let value = native!(bool, 0x48C633E94A8142A7, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_inactive_during_playback(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x06582AFF74894C75, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_active_during_playback(p0: u32, p1: bool) -> () {
    let value = native!((), 0xDFFCEF48E511DB48, native_parameters!(p0, p1));

    value
}

pub fn is_vehicle_sprayable(vehicle: i32) -> bool {
    let value = native!(bool, 0x8D474C8FAEFF6CDE, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_engine_can_degrade(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x983765856F2564F9, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_vehicle_shadow_effect(vehicle: i32, p1: i32, p2: i32) -> () {
    let value = native!((), 0xF0E4BA16D1DB546C, native_parameters!(vehicle, p1, p2));

    value
}

pub fn _remove_vehicle_shadow_effect(vehicle: i32) -> () {
    let value = native!((), 0xF87D9F2301F7D206, native_parameters!(vehicle));

    value
}

pub fn is_plane_landing_gear_intact(plane: i32) -> bool {
    let value = native!(bool, 0x4198AB0022B15F87, native_parameters!(plane));

    value
}

pub fn are_plane_propellers_intact(plane: i32) -> bool {
    let value = native!(bool, 0x755D6D5267CBBD7E, native_parameters!(plane));

    value
}

pub fn _0x4c815eb175086f84(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x4C815EB175086F84, native_parameters!(p0, p1));

    value
}

pub fn set_vehicle_can_deform_wheels(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x0CDDA42F9E360CA6, native_parameters!(vehicle, toggle));

    value
}

pub fn is_vehicle_stolen(vehicle: i32) -> bool {
    let value = native!(bool, 0x4AF9BD80EEBEB453, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_is_stolen(vehicle: i32, isStolen: bool) -> () {
    let value = native!(
        (),
        0x67B2C79AA7FF5738,
        native_parameters!(vehicle, isStolen)
    );

    value
}

pub fn set_plane_turbulence_multiplier(vehicle: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0xAD2D28A1AFDFF131,
        native_parameters!(vehicle, multiplier)
    );

    value
}

pub fn _are_plane_wings_intact(plane: i32) -> bool {
    let value = native!(bool, 0x5991A01434CE9677, native_parameters!(plane));

    value
}

pub fn _0xb264c4d2f2b0a78b(vehicle: i32) -> () {
    let value = native!((), 0xB264C4D2F2B0A78B, native_parameters!(vehicle));

    value
}

pub fn detach_vehicle_from_cargobob(vehicle: i32, cargobob: i32) -> () {
    let value = native!(
        (),
        0x0E21D3DF1051399D,
        native_parameters!(vehicle, cargobob)
    );

    value
}

pub fn detach_vehicle_from_any_cargobob(vehicle: i32) -> bool {
    let value = native!(bool, 0xADF7BE450512C12F, native_parameters!(vehicle));

    value
}

pub fn _detach_entity_from_cargobob(cargobob: i32, entity: i32) -> u32 {
    let value = native!(
        u32,
        0xAF03011701811146,
        native_parameters!(cargobob, entity)
    );

    value
}

pub fn is_vehicle_attached_to_cargobob(cargobob: i32, vehicleAttached: i32) -> bool {
    let value = native!(
        bool,
        0xD40148F22E81A1D9,
        native_parameters!(cargobob, vehicleAttached)
    );

    value
}

pub fn get_vehicle_attached_to_cargobob(cargobob: i32) -> i32 {
    let value = native!(i32, 0x873B82D42AC2B9E5, native_parameters!(cargobob));

    value
}

pub fn _get_entity_attached_to_cargobob(p0: u32) -> u32 {
    let value = native!(u32, 0x99093F60746708CA, native_parameters!(p0));

    value
}

pub fn attach_vehicle_to_cargobob(
    vehicle: i32,
    cargobob: i32,
    p2: i32,
    x: f32,
    y: f32,
    z: f32,
) -> () {
    let value = native!(
        (),
        0x4127F1D84E347769,
        native_parameters!(vehicle, cargobob, p2, x, y, z)
    );

    value
}

pub fn _attach_entity_to_cargobob(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0xA1DD82F3CCF9A01E,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _set_cargobob_hook_can_detach(cargobob: i32, toggle: bool) -> () {
    let value = native!((), 0x571FEB383F629926, native_parameters!(cargobob, toggle));

    value
}

pub fn _0x1f34b0626c594380(p0: u32, p1: u32) -> () {
    let value = native!((), 0x1F34B0626C594380, native_parameters!(p0, p1));

    value
}

pub fn _0x2c1d8b3b19e517cc(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x2C1D8B3B19E517CC, native_parameters!(p0, p1));

    value
}

pub fn _get_cargobob_hook_position(cargobob: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xCBDB9B923CACC92D,
        native_parameters!(cargobob)
    );

    value
}

pub fn does_cargobob_have_pick_up_rope(cargobob: i32) -> bool {
    let value = native!(bool, 0x1821D91AD4B56108, native_parameters!(cargobob));

    value
}

pub fn create_pick_up_rope_for_cargobob(cargobob: i32, state: i32) -> () {
    let value = native!((), 0x7BEB0C7A235F6F3B, native_parameters!(cargobob, state));

    value
}

pub fn remove_pick_up_rope_for_cargobob(cargobob: i32) -> () {
    let value = native!((), 0x9768CF648F54C804, native_parameters!(cargobob));

    value
}

pub fn set_pickup_rope_length_for_cargobob(
    cargobob: i32,
    length1: f32,
    length2: f32,
    p3: bool,
) -> () {
    let value = native!(
        (),
        0x877C1EAEAC531023,
        native_parameters!(cargobob, length1, length2, p3)
    );

    value
}

pub fn _0xc0ed6438e6d39ba8(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xC0ED6438E6D39BA8, native_parameters!(p0, p1, p2));

    value
}

pub fn set_cargobob_pickup_rope_damping_multiplier(p0: u32, p1: u32) -> () {
    let value = native!((), 0xCF1182F682F65307, native_parameters!(p0, p1));

    value
}

pub fn set_cargobob_pickup_rope_type(p0: u32, p1: u32) -> () {
    let value = native!((), 0x0D5F65A8F4EBDAB5, native_parameters!(p0, p1));

    value
}

pub fn does_cargobob_have_pickup_magnet(cargobob: i32) -> bool {
    let value = native!(bool, 0x6E08BF5B3722BAC9, native_parameters!(cargobob));

    value
}

pub fn set_cargobob_pickup_magnet_active(cargobob: i32, isActive: bool) -> () {
    let value = native!(
        (),
        0x9A665550F8DA349B,
        native_parameters!(cargobob, isActive)
    );

    value
}

pub fn set_cargobob_pickup_magnet_strength(cargobob: i32, strength: f32) -> () {
    let value = native!(
        (),
        0xBCBFCD9D1DAC19E2,
        native_parameters!(cargobob, strength)
    );

    value
}

pub fn set_cargobob_pickup_magnet_effect_radius(cargobob: i32, p1: f32) -> () {
    let value = native!((), 0xA17BAD153B51547E, native_parameters!(cargobob, p1));

    value
}

pub fn set_cargobob_pickup_magnet_reduced_falloff(cargobob: i32, p1: f32) -> () {
    let value = native!((), 0x66979ACF5102FD2F, native_parameters!(cargobob, p1));

    value
}

pub fn set_cargobob_pickup_magnet_pull_rope_length(cargobob: i32, p1: f32) -> () {
    let value = native!((), 0x6D8EAC07506291FB, native_parameters!(cargobob, p1));

    value
}

pub fn set_cargobob_pickup_magnet_pull_strength(cargobob: i32, p1: f32) -> () {
    let value = native!((), 0xED8286F71A819BAA, native_parameters!(cargobob, p1));

    value
}

pub fn set_cargobob_pickup_magnet_falloff(vehicle: i32, p1: f32) -> () {
    let value = native!((), 0x685D5561680D088B, native_parameters!(vehicle, p1));

    value
}

pub fn set_cargobob_pickup_magnet_reduced_strength(vehicle: i32, cargobob: i32) -> () {
    let value = native!(
        (),
        0xE301BD63E9E13CF0,
        native_parameters!(vehicle, cargobob)
    );

    value
}

pub fn _0x9bddc73cc6a115d4(vehicle: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x9BDDC73CC6A115D4, native_parameters!(vehicle, p1, p2));

    value
}

pub fn _0x56eb5e94318d3fb6(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x56EB5E94318D3FB6, native_parameters!(vehicle, p1));

    value
}

pub fn does_vehicle_have_weapons(vehicle: i32) -> bool {
    let value = native!(bool, 0x25ECB9F8017D98E0, native_parameters!(vehicle));

    value
}

pub fn _0x2c4a1590abf43e8b(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x2C4A1590ABF43E8B, native_parameters!(vehicle, p1));

    value
}

pub fn disable_vehicle_weapon(disabled: bool, weaponHash: u32, vehicle: i32, owner: i32) -> () {
    let value = native!(
        (),
        0xF4FC6A6F67D8D856,
        native_parameters!(disabled, weaponHash, vehicle, owner)
    );

    value
}

pub fn _is_vehicle_weapon_disabled(weaponHash: u32, vehicle: i32, owner: i32) -> bool {
    let value = native!(
        bool,
        0x563B65A643ED072E,
        native_parameters!(weaponHash, vehicle, owner)
    );

    value
}

pub fn _0xe05dd0e9707003a3(p0: u32, p1: bool) -> () {
    let value = native!((), 0xE05DD0E9707003A3, native_parameters!(p0, p1));

    value
}

pub fn set_vehicle_active_for_ped_navigation(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x21115BCD6E44656A, native_parameters!(vehicle, toggle));

    value
}

pub fn get_vehicle_class(vehicle: i32) -> i32 {
    let value = native!(i32, 0x29439776AAA00A62, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_class_from_name(modelHash: u32) -> i32 {
    let value = native!(i32, 0xDEDF1C8BD47C2200, native_parameters!(modelHash));

    value
}

pub fn set_players_last_vehicle(vehicle: i32) -> () {
    let value = native!((), 0xBCDF8BAF56C87B6A, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_can_be_used_by_fleeing_peds(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x300504B23BD3B711, native_parameters!(vehicle, toggle));

    value
}

pub fn _0xe5810ac70602f2f5(vehicle: i32, p1: f32) -> () {
    let value = native!((), 0xE5810AC70602F2F5, native_parameters!(vehicle, p1));

    value
}

pub fn set_vehicle_drops_money_when_blown_up(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x068F64F2470F9656, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_vehicle_jet_engine_on(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xB8FBC8B1330CA9B4, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x6a973569ba094650(vehicle: i32, p1: u32) -> () {
    let value = native!((), 0x6A973569BA094650, native_parameters!(vehicle, p1));

    value
}

pub fn _set_vehicle_handling_hash_for_ai(vehicle: i32, hash: u32) -> () {
    let value = native!((), 0x10655FAB9915623D, native_parameters!(vehicle, hash));

    value
}

pub fn set_vehicle_extended_removal_range(vehicle: i32, range: i32) -> () {
    let value = native!((), 0x79DF7E806202CE01, native_parameters!(vehicle, range));

    value
}

pub fn set_vehicle_steering_bias_scalar(p0: u32, p1: f32) -> () {
    let value = native!((), 0x9007A2F21DC108D4, native_parameters!(p0, p1));

    value
}

pub fn _set_helicopter_roll_pitch_yaw_mult(helicopter: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0x6E0859B530A365CC,
        native_parameters!(helicopter, multiplier)
    );

    value
}

pub fn set_vehicle_friction_override(vehicle: i32, friction: f32) -> () {
    let value = native!(
        (),
        0x1837AF7C627009BA,
        native_parameters!(vehicle, friction)
    );

    value
}

pub fn set_vehicle_wheels_can_break_off_when_blow_up(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xA37B9A517B133349, native_parameters!(vehicle, toggle));

    value
}

pub fn _0xf78f94d60248c737(vehicle: i32, p1: bool) -> bool {
    let value = native!(bool, 0xF78F94D60248C737, native_parameters!(vehicle, p1));

    value
}

pub fn set_vehicle_ceiling_height(vehicle: i32, height: f32) -> () {
    let value = native!((), 0xA46413066687A328, native_parameters!(vehicle, height));

    value
}

pub fn _0x5e569ec46ec21cae(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x5E569EC46EC21CAE, native_parameters!(vehicle, toggle));

    value
}

pub fn clear_vehicle_route_history(vehicle: i32) -> () {
    let value = native!((), 0x6D6AF961B72728AE, native_parameters!(vehicle));

    value
}

pub fn does_vehicle_exist_with_decorator(decorator: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x956B409B984D9BF7,
        native_parameters!(decorator.as_ptr())
    );

    value
}

pub fn _0x41062318f23ed854(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x41062318F23ED854, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_exclusive_driver(vehicle: i32, ped: i32, index: i32) -> () {
    let value = native!(
        (),
        0xB5C51B5502E85E83,
        native_parameters!(vehicle, ped, index)
    );

    value
}

pub fn _is_ped_exclusive_driver_of_vehicle(ped: i32, vehicle: i32, outIndex: *mut i32) -> bool {
    let value = native!(
        bool,
        0xB09D25E77C33EB3F,
        native_parameters!(ped, vehicle, outIndex)
    );

    value
}

pub fn _disable_plane_propeller(vehicle: i32, p1: u32) -> () {
    let value = native!((), 0x500873A45724C863, native_parameters!(vehicle, p1));

    value
}

pub fn set_vehicle_force_afterburner(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xB055A34527CB8FD7, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_disable_vehicle_window_collisions(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x1087BC8EC540DAEB, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x4ad280eb48b2d8e6(vehicle: i32, togle: bool) -> () {
    let value = native!((), 0x4AD280EB48B2D8E6, native_parameters!(vehicle, togle));

    value
}

pub fn _0xb68cfaf83a02768d(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xB68CFAF83A02768D, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x0205f5365292d2eb(vehicle: i32, p1: f32) -> () {
    let value = native!((), 0x0205F5365292D2EB, native_parameters!(vehicle, p1));

    value
}

pub fn _0xcf9159024555488c(p0: u32) -> () {
    let value = native!((), 0xCF9159024555488C, native_parameters!(p0));

    value
}

pub fn set_distant_cars_enabled(toggle: bool) -> () {
    let value = native!((), 0xF796359A959DF65D, native_parameters!(toggle));

    value
}

pub fn _set_vehicle_neon_lights_colour(vehicle: i32, r: i32, g: i32, b: i32) -> () {
    let value = native!((), 0x8E0A582209A62695, native_parameters!(vehicle, r, g, b));

    value
}

pub fn _0xb93b2867f7b479d1(vehicle: i32, index: i32) -> () {
    let value = native!((), 0xB93B2867F7B479D1, native_parameters!(vehicle, index));

    value
}

pub fn _get_vehicle_neon_lights_colour(vehicle: i32, r: *mut i32, g: *mut i32, b: *mut i32) -> () {
    let value = native!((), 0x7619EEE8C886757F, native_parameters!(vehicle, r, g, b));

    value
}

pub fn _set_vehicle_neon_light_enabled(vehicle: i32, index: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0x2AA720E4287BF269,
        native_parameters!(vehicle, index, toggle)
    );

    value
}

pub fn _is_vehicle_neon_light_enabled(vehicle: i32, index: i32) -> bool {
    let value = native!(bool, 0x8C4B92553E4766A5, native_parameters!(vehicle, index));

    value
}

pub fn _0x35e0654f4bad7971(p0: bool) -> () {
    let value = native!((), 0x35E0654F4BAD7971, native_parameters!(p0));

    value
}

pub fn _disable_vehicle_neon_lights(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x83F813570FF519DE, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_disable_superdummy_mode(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0xB088E9A47AE6EDD5, native_parameters!(vehicle, p1));

    value
}

pub fn _request_vehicle_dashboard_scaleform_movie(vehicle: i32) -> () {
    let value = native!((), 0xDBA3C090E3D74690, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_body_health(vehicle: i32) -> f32 {
    let value = native!(f32, 0xF271147EB7B40F12, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_body_health(vehicle: i32, value: f32) -> () {
    let value = native!((), 0xB77D05AC8C78AADB, native_parameters!(vehicle, value));

    value
}

pub fn _get_vehicle_suspension_bounds(
    vehicle: i32,
    out1: *mut NativeVector3,
    out2: *mut NativeVector3,
) -> () {
    let value = native!(
        (),
        0xDF7E3EEB29642C38,
        native_parameters!(vehicle, out1, out2)
    );

    value
}

pub fn _get_vehicle_suspension_height(vehicle: i32) -> f32 {
    let value = native!(f32, 0x53952FD2BAA19F17, native_parameters!(vehicle));

    value
}

pub fn _set_car_high_speed_bump_severity_multiplier(multiplier: f32) -> () {
    let value = native!((), 0x84FD40F56075E816, native_parameters!(multiplier));

    value
}

pub fn _get_number_of_vehicle_doors(vehicle: i32) -> i32 {
    let value = native!(i32, 0x92922A607497B14D, native_parameters!(vehicle));

    value
}

pub fn _set_hydraulic_raised(p0: u32, p1: u32) -> () {
    let value = native!((), 0x28B18377EB6E25F6, native_parameters!(p0, p1));

    value
}

pub fn _0xa7dcdf4ded40a8f4(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0xA7DCDF4DED40A8F4, native_parameters!(vehicle, p1));

    value
}

pub fn _get_vehicle_body_health_2(
    vehicle: i32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
) -> f32 {
    let value = native!(
        f32,
        0xB8EF61207C2393A9,
        native_parameters!(vehicle, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn _0xd4c4642cb7f50b5d(vehicle: i32) -> bool {
    let value = native!(bool, 0xD4C4642CB7F50B5D, native_parameters!(vehicle));

    value
}

pub fn _0xc361aa040d6637a8(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0xC361AA040D6637A8, native_parameters!(vehicle, p1));

    value
}

pub fn set_vehicle_kers_allowed(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x99C82F8A139F3E4E, native_parameters!(vehicle, toggle));

    value
}

pub fn get_vehicle_has_kers(vehicle: i32) -> bool {
    let value = native!(bool, 0x50634E348C8D44EF, native_parameters!(vehicle));

    value
}

pub fn _0xe16142b94664defd(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0xE16142B94664DEFD, native_parameters!(vehicle, p1));

    value
}

pub fn _0x26d99d5a82fd18e8(p0: u32) -> () {
    let value = native!((), 0x26D99D5A82FD18E8, native_parameters!(p0));

    value
}

pub fn _set_hydraulic_wheel_value(vehicle: i32, wheelId: i32, value: f32) -> () {
    let value = native!(
        (),
        0x84EA99C62CB3EF0C,
        native_parameters!(vehicle, wheelId, value)
    );

    value
}

pub fn _set_cambered_wheels_disabled(p0: u32, p1: u32) -> () {
    let value = native!((), 0x1201E8A3290A3B98, native_parameters!(p0, p1));

    value
}

pub fn _set_hydraulic_wheel_state(p0: u32, p1: u32) -> () {
    let value = native!((), 0x8EA86DF356801C7D, native_parameters!(p0, p1));

    value
}

pub fn _set_hydraulic_wheel_state_transition(
    vehicle: i32,
    wheelId: i32,
    state: i32,
    value: f32,
    p4: u32,
) -> () {
    let value = native!(
        (),
        0xC24075310A8B9CD1,
        native_parameters!(vehicle, wheelId, state, value, p4)
    );

    value
}

pub fn _0x5ba68a0840d546ac(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x5BA68A0840D546AC, native_parameters!(p0, p1));

    value
}

pub fn _0x4419966c9936071a(vehicle: i32) -> () {
    let value = native!((), 0x4419966C9936071A, native_parameters!(vehicle));

    value
}

pub fn _0x870b8b7a766615c8(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x870B8B7A766615C8, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x8533cafde1f0f336(p0: u32) -> u32 {
    let value = native!(u32, 0x8533CAFDE1F0F336, native_parameters!(p0));

    value
}

pub fn _set_vehicle_damage_modifier(vehicle: i32, p1: f32) -> u32 {
    let value = native!(u32, 0x4E20D2A627011E8E, native_parameters!(vehicle, p1));

    value
}

pub fn _set_vehicle_unk_damage_multiplier(vehicle: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0x45A561A9421AB6AD,
        native_parameters!(vehicle, multiplier)
    );

    value
}

pub fn _0xd4196117af7bb974(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0xD4196117AF7BB974, native_parameters!(p0, p1));

    value
}

pub fn _0xbb2333bb87ddd87f(p0: u32, p1: u32) -> () {
    let value = native!((), 0xBB2333BB87DDD87F, native_parameters!(p0, p1));

    value
}

pub fn _0x73561d4425a021a2(p0: u32, p1: u32) -> () {
    let value = native!((), 0x73561D4425A021A2, native_parameters!(p0, p1));

    value
}

pub fn _set_vehicle_controls_inverted(vehicle: i32, state: bool) -> () {
    let value = native!((), 0x5B91B229243351A8, native_parameters!(vehicle, state));

    value
}

pub fn _0x7bbe7ff626a591fe(p0: u32) -> () {
    let value = native!((), 0x7BBE7FF626A591FE, native_parameters!(p0));

    value
}

pub fn _0x65b080555ea48149(p0: u32) -> () {
    let value = native!((), 0x65B080555EA48149, native_parameters!(p0));

    value
}

pub fn _0x428ad3e26c8d9eb0(vehicle: i32, x: f32, y: f32, z: f32, p4: f32) -> () {
    let value = native!(
        (),
        0x428AD3E26C8D9EB0,
        native_parameters!(vehicle, x, y, z, p4)
    );

    value
}

pub fn _0xe2f53f172b45ede1() -> () {
    let value = native!((), 0xE2F53F172B45EDE1, native_parameters!());

    value
}

pub fn _0xba91d045575699ad(vehicle: i32) -> bool {
    let value = native!(bool, 0xBA91D045575699AD, native_parameters!(vehicle));

    value
}

pub fn _0x80e3357fdef45c21(p0: u32, p1: u32) -> () {
    let value = native!((), 0x80E3357FDEF45C21, native_parameters!(p0, p1));

    value
}

pub fn _set_vehicle_ramp_launch_modifier(p0: u32, p1: u32) -> () {
    let value = native!((), 0xEFC13B1CE30D755D, native_parameters!(p0, p1));

    value
}

pub fn _get_is_door_valid(vehicle: i32, doorIndex: i32) -> bool {
    let value = native!(
        bool,
        0x645F4B6E8499F632,
        native_parameters!(vehicle, doorIndex)
    );

    value
}

pub fn _set_vehicle_rocket_boost_refill_time(vehicle: i32, seconds: f32) -> () {
    let value = native!((), 0xE00F2AB100B76E89, native_parameters!(vehicle, seconds));

    value
}

pub fn _get_has_rocket_boost(vehicle: i32) -> bool {
    let value = native!(bool, 0x36D782F68B309BDA, native_parameters!(vehicle));

    value
}

pub fn _is_vehicle_rocket_boost_active(vehicle: i32) -> bool {
    let value = native!(bool, 0x3D34E80EED4AE3BE, native_parameters!(vehicle));

    value
}

pub fn _set_vehicle_rocket_boost_active(vehicle: i32, active: bool) -> () {
    let value = native!((), 0x81E1552E35DC3839, native_parameters!(vehicle, active));

    value
}

pub fn _get_has_retractable_wheels(vehicle: i32) -> bool {
    let value = native!(bool, 0xDCA174A42133F08C, native_parameters!(vehicle));

    value
}

pub fn _get_is_wheels_lowered_state_active(vehicle: i32) -> bool {
    let value = native!(bool, 0x1DA0DA9CB3F0C8BF, native_parameters!(vehicle));

    value
}

pub fn _raise_retractable_wheels(vehicle: i32) -> () {
    let value = native!((), 0xF660602546D27BA8, native_parameters!(vehicle));

    value
}

pub fn _lower_retractable_wheels(vehicle: i32) -> () {
    let value = native!((), 0x5335BE58C083E74E, native_parameters!(vehicle));

    value
}

pub fn _get_can_vehicle_jump(vehicle: i32) -> bool {
    let value = native!(bool, 0x9078C0C5EF8C19E9, native_parameters!(vehicle));

    value
}

pub fn _set_use_higher_vehicle_jump_force(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xF06A16CA55D138D8, native_parameters!(vehicle, toggle));

    value
}

pub fn _0xb2e0c0d6922d31f2(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xB2E0C0D6922D31F2, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_vehicle_weapon_capacity(vehicle: i32, weaponIndex: i32, capacity: i32) -> () {
    let value = native!(
        (),
        0x44CD1F493DB2A0A6,
        native_parameters!(vehicle, weaponIndex, capacity)
    );

    value
}

pub fn _get_vehicle_weapon_capacity(vehicle: i32, weaponIndex: i32) -> i32 {
    let value = native!(
        i32,
        0x8181CE2F25CB9BB7,
        native_parameters!(vehicle, weaponIndex)
    );

    value
}

pub fn _get_vehicle_has_parachute(vehicle: i32) -> bool {
    let value = native!(bool, 0xBC9CFF381338CB4F, native_parameters!(vehicle));

    value
}

pub fn _get_vehicle_can_activate_parachute(vehicle: i32) -> bool {
    let value = native!(bool, 0xA916396DF4154EE3, native_parameters!(vehicle));

    value
}

pub fn _set_vehicle_parachute_active(vehicle: i32, active: bool) -> () {
    let value = native!((), 0x0BFFB028B3DD0A97, native_parameters!(vehicle, active));

    value
}

pub fn _0x3de51e9c80b116cf(p0: u32) -> u32 {
    let value = native!(u32, 0x3DE51E9C80B116CF, native_parameters!(p0));

    value
}

pub fn _set_vehicle_receives_ramp_damage(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x28D034A93FE31BF5, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_vehicle_ramp_sideways_launch_motion(p0: u32, p1: u32) -> () {
    let value = native!((), 0x1BBAC99C0BC53656, native_parameters!(p0, p1));

    value
}

pub fn _set_vehicle_ramp_upwards_launch_motion(p0: u32, p1: u32) -> () {
    let value = native!((), 0x756AE6E962168A04, native_parameters!(p0, p1));

    value
}

pub fn _0x9d30687c57baa0bb(p0: u32) -> () {
    let value = native!((), 0x9D30687C57BAA0BB, native_parameters!(p0));

    value
}

pub fn _set_vehicle_weapons_disabled(p0: u32, p1: u32) -> () {
    let value = native!((), 0x86B4B6212CB8B627, native_parameters!(p0, p1));

    value
}

pub fn _0x41290b40fa63e6da(p0: u32) -> () {
    let value = native!((), 0x41290B40FA63E6DA, native_parameters!(p0));

    value
}

pub fn _set_vehicle_parachute_model(vehicle: i32, modelHash: u32) -> () {
    let value = native!(
        (),
        0x4D610C6B56031351,
        native_parameters!(vehicle, modelHash)
    );

    value
}

pub fn _set_vehicle_parachute_texture_variatiion(vehicle: i32, textureVariation: i32) -> () {
    let value = native!(
        (),
        0xA74AD2439468C883,
        native_parameters!(vehicle, textureVariation)
    );

    value
}

pub fn _0x0419b167ee128f33(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x0419B167EE128F33, native_parameters!(p0, p1));

    value
}

pub fn _0xf3b0e0aed097a3f5(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0xF3B0E0AED097A3F5, native_parameters!(p0, p1));

    value
}

pub fn _0xd3e51c0ab8c26eee(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0xD3E51C0AB8C26EEE, native_parameters!(p0, p1));

    value
}

pub fn _get_all_vehicles(vehsStruct: *mut i32) -> i32 {
    let value = native!(i32, 0x9B8E1BF04B51F2E8, native_parameters!(vehsStruct));

    value
}

pub fn _0x72beccf4b829522e(p0: u32, p1: u32) -> () {
    let value = native!((), 0x72BECCF4B829522E, native_parameters!(p0, p1));

    value
}

pub fn _0x66e3aaface2d1eb8(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x66E3AAFACE2D1EB8, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x1312ddd8385aee4e(p0: u32, p1: u32) -> () {
    let value = native!((), 0x1312DDD8385AEE4E, native_parameters!(p0, p1));

    value
}

pub fn _0xedbc8405b3895cc9(p0: u32, p1: u32) -> () {
    let value = native!((), 0xEDBC8405B3895CC9, native_parameters!(p0, p1));

    value
}

pub fn _0x26e13d440e7f6064(vehicle: i32, value: f32) -> () {
    let value = native!((), 0x26E13D440E7F6064, native_parameters!(vehicle, value));

    value
}

pub fn _0x2fa2494b47fdd009(p0: u32, p1: u32) -> () {
    let value = native!((), 0x2FA2494B47FDD009, native_parameters!(p0, p1));

    value
}

pub fn _set_vehicle_rocket_boost_percentage(vehicle: i32, percentage: f32) -> () {
    let value = native!(
        (),
        0xFEB2DDED3509562E,
        native_parameters!(vehicle, percentage)
    );

    value
}

pub fn _set_oppressor_transform_state(vehicle: i32, state: bool) -> () {
    let value = native!((), 0x544996C0081ABDEB, native_parameters!(vehicle, state));

    value
}

pub fn _0x78ceee41f49f421f(p0: u32, p1: u32) -> () {
    let value = native!((), 0x78CEEE41F49F421F, native_parameters!(p0, p1));

    value
}

pub fn _0xaf60e6a2936f982a(p0: u32, p1: u32) -> () {
    let value = native!((), 0xAF60E6A2936F982A, native_parameters!(p0, p1));

    value
}

pub fn _0x430a7631a84c9be7(p0: u32) -> () {
    let value = native!((), 0x430A7631A84C9BE7, native_parameters!(p0));

    value
}

pub fn _disable_vehicle_world_collision(vehicle: i32) -> () {
    let value = native!((), 0x75627043C6AA90AD, native_parameters!(vehicle));

    value
}

pub fn _0x8235f1bead557629(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x8235F1BEAD557629, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x9640e30a7f395e4b(vehicle: i32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x9640E30A7F395E4B,
        native_parameters!(vehicle, p1, p2, p3, p4)
    );

    value
}

pub fn _0x0bbb9a7a8ffe931b(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x0BBB9A7A8FFE931B, native_parameters!(p0, p1, p2));

    value
}

pub fn _set_cargobob_hook_can_attach(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x94A68DA412C4007D, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_vehicle_bomb_count(vehicle: i32, bombCount: i32) -> () {
    let value = native!(
        (),
        0xF4B2ED59DEB5D774,
        native_parameters!(vehicle, bombCount)
    );

    value
}

pub fn _get_vehicle_bomb_count(vehicle: i32) -> i32 {
    let value = native!(i32, 0xEA12BD130D7569A1, native_parameters!(vehicle));

    value
}

pub fn _set_vehicle_countermeasure_count(vehicle: i32, counterMeasureCount: i32) -> () {
    let value = native!(
        (),
        0x9BDA23BF666F0855,
        native_parameters!(vehicle, counterMeasureCount)
    );

    value
}

pub fn _get_vehicle_countermeasure_count(vehicle: i32) -> i32 {
    let value = native!(i32, 0xF846AA63DF56B804, native_parameters!(vehicle));

    value
}

pub fn _0x0a3f820a9a9a9ac5(vehicle: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x0A3F820A9A9A9AC5, native_parameters!(vehicle, x, y, z));

    value
}

pub fn _0x51f30db60626a20e(
    vehicle: i32,
    x: f32,
    y: f32,
    z: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    p7: i32,
    p8: u32,
) -> bool {
    let value = native!(
        bool,
        0x51F30DB60626A20E,
        native_parameters!(vehicle, x, y, z, rotX, rotY, rotZ, p7, p8)
    );

    value
}

pub fn _0x97841634ef7df1d6(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x97841634EF7DF1D6, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_vehicle_hover_transform_ratio(vehicle: i32, ratio: f32) -> () {
    let value = native!((), 0xD138FA15C9776837, native_parameters!(vehicle, ratio));

    value
}

pub fn _set_vehicle_hover_transform_percentage(vehicle: i32, percentage: f32) -> () {
    let value = native!(
        (),
        0x438B3D7CA026FE91,
        native_parameters!(vehicle, percentage)
    );

    value
}

pub fn _set_vehicle_hover_transform_enabled(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xF1211889DF15A763, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_vehicle_hover_transform_active(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x2D55FE374D5FDB91, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x3a9128352eac9e85(p0: u32) -> u32 {
    let value = native!(u32, 0x3A9128352EAC9E85, native_parameters!(p0));

    value
}

pub fn _find_random_point_in_space(ped: i32) -> NativeVector3 {
    let value = native!(NativeVector3, 0x8DC9675797123522, native_parameters!(ped));

    value
}

pub fn _set_deploy_heli_stub_wings(vehicle: i32, deploy: bool, p2: bool) -> () {
    let value = native!(
        (),
        0xB251E0B33E58B424,
        native_parameters!(vehicle, deploy, p2)
    );

    value
}

pub fn _are_heli_stub_wings_deployed(vehicle: i32) -> bool {
    let value = native!(bool, 0xAEF12960FA943792, native_parameters!(vehicle));

    value
}

pub fn _0xaa653ae61924b0a0(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xAA653AE61924B0A0, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_vehicle_turret_unk(vehicle: i32, index: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0xC60060EB0D8AC7B1,
        native_parameters!(vehicle, index, toggle)
    );

    value
}

pub fn _set_specialflight_wing_ratio(vehicle: i32, ratio: f32) -> () {
    let value = native!((), 0x70A252F60A3E036B, native_parameters!(vehicle, ratio));

    value
}

pub fn _set_disable_turret_movement_this_frame(vehicle: i32, turretId: i32) -> () {
    let value = native!(
        (),
        0xE615BB7A7752C76A,
        native_parameters!(vehicle, turretId)
    );

    value
}

pub fn _0x887fa38787de8c72(vehicle: i32) -> () {
    let value = native!((), 0x887FA38787DE8C72, native_parameters!(vehicle));

    value
}

pub fn _set_unk_float_0x104_for_submarine_vehicle_task(vehicle: i32, value: f32) -> () {
    let value = native!((), 0x498218259FB7C72D, native_parameters!(vehicle, value));

    value
}

pub fn _set_unk_bool_0x102_for_submarine_vehicle_task(vehicle: i32, value: bool) -> () {
    let value = native!((), 0x41B9FB92EDED32A6, native_parameters!(vehicle, value));

    value
}

pub fn _0x36de109527a2c0c4(toggle: bool) -> () {
    let value = native!((), 0x36DE109527A2C0C4, native_parameters!(toggle));

    value
}

pub fn _0x82e0ac411e41a5b4(toggle: bool) -> () {
    let value = native!((), 0x82E0AC411E41A5B4, native_parameters!(toggle));

    value
}

pub fn _0x99a05839c46ce316(toggle: bool) -> () {
    let value = native!((), 0x99A05839C46CE316, native_parameters!(toggle));

    value
}

pub fn _get_is_vehicle_shunt_boost_active(vehicle: i32) -> bool {
    let value = native!(bool, 0xA2459F72C14E2E8D, native_parameters!(vehicle));

    value
}

pub fn _0xe8718faf591fd224(vehicle: i32) -> bool {
    let value = native!(bool, 0xE8718FAF591FD224, native_parameters!(vehicle));

    value
}

pub fn _get_last_rammed_vehicle(vehicle: i32) -> i32 {
    let value = native!(i32, 0x04F2FA6E234162F7, native_parameters!(vehicle));

    value
}

pub fn _set_disable_vehicle_unk(toggle: bool) -> () {
    let value = native!((), 0x143921E45EC44D62, native_parameters!(toggle));

    value
}

pub fn _set_vehicle_nitro_enabled(
    vehicle: i32,
    toggle: bool,
    level: f32,
    power: f32,
    rechargeTime: f32,
    disableSound: bool,
) -> () {
    let value = native!(
        (),
        0xC8E9B6B71B8E660D,
        native_parameters!(vehicle, toggle, level, power, rechargeTime, disableSound)
    );

    value
}

pub fn _set_vehicle_wheels_deal_damage(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x2970EAA18FD5E42F, native_parameters!(vehicle, toggle));

    value
}

pub fn _set_disable_vehicle_unk_2(toggle: bool) -> () {
    let value = native!((), 0x211E95CE9903940C, native_parameters!(toggle));

    value
}

pub fn _0x5bbcf35bf6e456f7(toggle: bool) -> () {
    let value = native!((), 0x5BBCF35BF6E456F7, native_parameters!(toggle));

    value
}

pub fn _get_does_vehicle_have_tombstone(vehicle: i32) -> bool {
    let value = native!(bool, 0x71AFB258CCED3A27, native_parameters!(vehicle));

    value
}

pub fn _hide_vehicle_tombstone(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xAE71FB656C600587, native_parameters!(vehicle, toggle));

    value
}

pub fn _get_is_vehicle_emp_disabled(vehicle: i32) -> bool {
    let value = native!(bool, 0x0506ED94363AD905, native_parameters!(vehicle));

    value
}

pub fn _0x8f0d5ba1c2cc91d7(toggle: bool) -> () {
    let value = native!((), 0x8F0D5BA1C2CC91D7, native_parameters!(toggle));

    value
}

pub fn _get_tyre_health(vehicle: i32, wheelIndex: i32) -> f32 {
    let value = native!(
        f32,
        0x55EAB010FAEE9380,
        native_parameters!(vehicle, wheelIndex)
    );

    value
}

pub fn _set_tyre_health(vehicle: i32, wheelIndex: i32, health: f32) -> () {
    let value = native!(
        (),
        0x74C68EF97645E79D,
        native_parameters!(vehicle, wheelIndex, health)
    );

    value
}

pub fn _get_tyre_wear_multiplier(vehicle: i32, wheelIndex: i32) -> f32 {
    let value = native!(
        f32,
        0x6E387895952F4F71,
        native_parameters!(vehicle, wheelIndex)
    );

    value
}

pub fn _set_tyre_wear_multiplier(vehicle: i32, wheelIndex: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0x01894E2EDE923CA2,
        native_parameters!(vehicle, wheelIndex, multiplier)
    );

    value
}

pub fn _set_tyre_softness_multiplier(vehicle: i32, wheelIndex: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0x392183BB9EA57697,
        native_parameters!(vehicle, wheelIndex, multiplier)
    );

    value
}

pub fn _set_tyre_traction_loss_multiplier(vehicle: i32, wheelIndex: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0xC970D0E0FC31D768,
        native_parameters!(vehicle, wheelIndex, multiplier)
    );

    value
}

pub fn _0xf8b49f5ba7f850e7(vehicle: i32, p1: i32) -> () {
    let value = native!((), 0xF8B49F5BA7F850E7, native_parameters!(vehicle, p1));

    value
}
