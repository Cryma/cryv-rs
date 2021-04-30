use crate::types::NativeVector3;

pub fn create_ped(
    pedType: i32,
    modelHash: u32,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    isNetwork: bool,
    bScriptHostPed: bool,
) -> i32 {
    let value = native!(
        i32,
        0xD49F9B0955C367DE,
        native_parameters!(
            pedType,
            modelHash,
            x,
            y,
            z,
            heading,
            isNetwork,
            bScriptHostPed
        )
    );

    value
}

pub fn delete_ped(ped: *mut i32) -> () {
    let value = native!((), 0x9614299DCB53E54B, native_parameters!(ped));

    value
}

pub fn clone_ped(ped: i32, heading: f32, isNetwork: bool, bScriptHostPed: bool) -> i32 {
    let value = native!(
        i32,
        0xEF29A16337FACADB,
        native_parameters!(ped, heading, isNetwork, bScriptHostPed)
    );

    value
}

pub fn _clone_ped_ex(
    ped: i32,
    heading: f32,
    isNetwork: bool,
    bScriptHostPed: bool,
    p4: u32,
) -> i32 {
    let value = native!(
        i32,
        0x668FD40BCBA5DE48,
        native_parameters!(ped, heading, isNetwork, bScriptHostPed, p4)
    );

    value
}

pub fn clone_ped_to_target(ped: i32, targetPed: i32) -> () {
    let value = native!((), 0xE952D6431689AD9A, native_parameters!(ped, targetPed));

    value
}

pub fn _clone_ped_to_target_ex(ped: i32, targetPed: i32, p2: u32) -> () {
    let value = native!(
        (),
        0x148B08C2D2ACB884,
        native_parameters!(ped, targetPed, p2)
    );

    value
}

pub fn is_ped_in_vehicle(ped: i32, vehicle: i32, atGetIn: bool) -> bool {
    let value = native!(
        bool,
        0xA3EE4A07279BB9DB,
        native_parameters!(ped, vehicle, atGetIn)
    );

    value
}

pub fn is_ped_in_model(ped: i32, modelHash: u32) -> bool {
    let value = native!(bool, 0x796D90EFB19AA332, native_parameters!(ped, modelHash));

    value
}

pub fn is_ped_in_any_vehicle(ped: i32, atGetIn: bool) -> bool {
    let value = native!(bool, 0x997ABD671D25CA0B, native_parameters!(ped, atGetIn));

    value
}

pub fn is_cop_ped_in_area_3d(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
    let value = native!(
        bool,
        0x16EC4839969F9F5E,
        native_parameters!(x1, y1, z1, x2, y2, z2)
    );

    value
}

pub fn is_ped_injured(ped: i32) -> bool {
    let value = native!(bool, 0x84A2DD9AC37C35C1, native_parameters!(ped));

    value
}

pub fn is_ped_hurt(ped: i32) -> bool {
    let value = native!(bool, 0x5983BB449D7FDB12, native_parameters!(ped));

    value
}

pub fn is_ped_fatally_injured(ped: i32) -> bool {
    let value = native!(bool, 0xD839450756ED5A80, native_parameters!(ped));

    value
}

pub fn is_ped_dead_or_dying(ped: i32, p1: bool) -> bool {
    let value = native!(bool, 0x3317DEDB88C95038, native_parameters!(ped, p1));

    value
}

pub fn is_conversation_ped_dead(ped: i32) -> bool {
    let value = native!(bool, 0xE0A0AEC214B1FABA, native_parameters!(ped));

    value
}

pub fn is_ped_aiming_from_cover(ped: i32) -> bool {
    let value = native!(bool, 0x3998B1276A3300E5, native_parameters!(ped));

    value
}

pub fn is_ped_reloading(ped: i32) -> bool {
    let value = native!(bool, 0x24B100C68C645951, native_parameters!(ped));

    value
}

pub fn is_ped_a_player(ped: i32) -> bool {
    let value = native!(bool, 0x12534C348C6CB68B, native_parameters!(ped));

    value
}

pub fn create_ped_inside_vehicle(
    vehicle: i32,
    pedType: i32,
    modelHash: u32,
    seat: i32,
    isNetwork: bool,
    bScriptHostPed: bool,
) -> i32 {
    let value = native!(
        i32,
        0x7DD959874C1FD534,
        native_parameters!(vehicle, pedType, modelHash, seat, isNetwork, bScriptHostPed)
    );

    value
}

pub fn set_ped_desired_heading(ped: i32, heading: f32) -> () {
    let value = native!((), 0xAA5A7ECE2AA8FE70, native_parameters!(ped, heading));

    value
}

pub fn _freeze_ped_camera_rotation(ped: i32) -> () {
    let value = native!((), 0xFF287323B0E2C69A, native_parameters!(ped));

    value
}

pub fn is_ped_facing_ped(ped: i32, otherPed: i32, angle: f32) -> bool {
    let value = native!(
        bool,
        0xD71649DB0A545AA3,
        native_parameters!(ped, otherPed, angle)
    );

    value
}

pub fn is_ped_in_melee_combat(ped: i32) -> bool {
    let value = native!(bool, 0x4E209B2C1EAD5159, native_parameters!(ped));

    value
}

pub fn is_ped_stopped(ped: i32) -> bool {
    let value = native!(bool, 0x530944F6F4B8A214, native_parameters!(ped));

    value
}

pub fn is_ped_shooting_in_area(
    ped: i32,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    p7: bool,
    p8: bool,
) -> bool {
    let value = native!(
        bool,
        0x7E9DFE24AC1E58EF,
        native_parameters!(ped, x1, y1, z1, x2, y2, z2, p7, p8)
    );

    value
}

pub fn is_any_ped_shooting_in_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    p6: bool,
    p7: bool,
) -> bool {
    let value = native!(
        bool,
        0xA0D3D71EA1086C55,
        native_parameters!(x1, y1, z1, x2, y2, z2, p6, p7)
    );

    value
}

pub fn is_ped_shooting(ped: i32) -> bool {
    let value = native!(bool, 0x34616828CD07F1A1, native_parameters!(ped));

    value
}

pub fn set_ped_accuracy(ped: i32, accuracy: i32) -> () {
    let value = native!((), 0x7AEFB85C1D49DEB6, native_parameters!(ped, accuracy));

    value
}

pub fn get_ped_accuracy(ped: i32) -> i32 {
    let value = native!(i32, 0x37F4AD56ECBC0CD6, native_parameters!(ped));

    value
}

pub fn _0x87ddeb611b329a9c(multiplier: f32) -> () {
    let value = native!((), 0x87DDEB611B329A9C, native_parameters!(multiplier));

    value
}

pub fn is_ped_model(ped: i32, modelHash: u32) -> bool {
    let value = native!(bool, 0xC9D55B1A358A5BF7, native_parameters!(ped, modelHash));

    value
}

pub fn explode_ped_head(ped: i32, weaponHash: u32) -> () {
    let value = native!((), 0x2D05CED3A38D0F3A, native_parameters!(ped, weaponHash));

    value
}

pub fn remove_ped_elegantly(ped: *mut i32) -> () {
    let value = native!((), 0xAC6D445B994DF95E, native_parameters!(ped));

    value
}

pub fn add_armour_to_ped(ped: i32, amount: i32) -> () {
    let value = native!((), 0x5BA652A0CD14DF2F, native_parameters!(ped, amount));

    value
}

pub fn set_ped_armour(ped: i32, amount: i32) -> () {
    let value = native!((), 0xCEA04D83135264CC, native_parameters!(ped, amount));

    value
}

pub fn set_ped_into_vehicle(ped: i32, vehicle: i32, seatIndex: i32) -> () {
    let value = native!(
        (),
        0xF75B0D629E1C063D,
        native_parameters!(ped, vehicle, seatIndex)
    );

    value
}

pub fn set_ped_allow_vehicles_override(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x3C028C636A414ED9, native_parameters!(ped, toggle));

    value
}

pub fn can_create_random_ped(unk: bool) -> bool {
    let value = native!(bool, 0x3E8349C08E4B82E4, native_parameters!(unk));

    value
}

pub fn create_random_ped(posX: f32, posY: f32, posZ: f32) -> i32 {
    let value = native!(
        i32,
        0xB4AC7D0CF06BFE8F,
        native_parameters!(posX, posY, posZ)
    );

    value
}

pub fn create_random_ped_as_driver(vehicle: i32, returnHandle: bool) -> i32 {
    let value = native!(
        i32,
        0x9B62392B474F44A0,
        native_parameters!(vehicle, returnHandle)
    );

    value
}

pub fn can_create_random_driver() -> bool {
    let value = native!(bool, 0xB8EB95E5B4E56978, native_parameters!());

    value
}

pub fn can_create_random_bike_rider() -> bool {
    let value = native!(bool, 0xEACEEDA81751915C, native_parameters!());

    value
}

pub fn set_ped_move_anims_blend_out(ped: i32) -> () {
    let value = native!((), 0x9E8C908F41584ECD, native_parameters!(ped));

    value
}

pub fn set_ped_can_be_dragged_out(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xC1670E958EEE24E5, native_parameters!(ped, toggle));

    value
}

pub fn _0xf2bebcdfafdaa19e(toggle: bool) -> () {
    let value = native!((), 0xF2BEBCDFAFDAA19E, native_parameters!(toggle));

    value
}

pub fn is_ped_male(ped: i32) -> bool {
    let value = native!(bool, 0x6D9F5FAA7488BA46, native_parameters!(ped));

    value
}

pub fn is_ped_human(ped: i32) -> bool {
    let value = native!(bool, 0xB980061DA992779D, native_parameters!(ped));

    value
}

pub fn get_vehicle_ped_is_in(ped: i32, includeLastVehicle: bool) -> i32 {
    let value = native!(
        i32,
        0x9A9112A0FE9A4713,
        native_parameters!(ped, includeLastVehicle)
    );

    value
}

pub fn reset_ped_last_vehicle(ped: i32) -> () {
    let value = native!((), 0xBB8DE8CF6A8DD8BB, native_parameters!(ped));

    value
}

pub fn set_ped_density_multiplier_this_frame(multiplier: f32) -> () {
    let value = native!((), 0x95E3D6257B166CF2, native_parameters!(multiplier));

    value
}

pub fn set_scenario_ped_density_multiplier_this_frame(p0: f32, p1: f32) -> () {
    let value = native!((), 0x7A556143A1C03898, native_parameters!(p0, p1));

    value
}

pub fn _0x5a7f62fda59759bd() -> () {
    let value = native!((), 0x5A7F62FDA59759BD, native_parameters!());

    value
}

pub fn set_scripted_conversion_coord_this_frame(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x5086C7843552CF85, native_parameters!(x, y, z));

    value
}

pub fn set_ped_non_creation_area(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> () {
    let value = native!(
        (),
        0xEE01041D559983EA,
        native_parameters!(x1, y1, z1, x2, y2, z2)
    );

    value
}

pub fn clear_ped_non_creation_area() -> () {
    let value = native!((), 0x2E05208086BA0651, native_parameters!());

    value
}

pub fn instantly_fill_ped_population() -> () {
    let value = native!((), 0x4759CC730F947C81, native_parameters!());

    value
}

pub fn is_ped_on_mount(ped: i32) -> bool {
    let value = native!(bool, 0x460BC76A0E10655E, native_parameters!(ped));

    value
}

pub fn get_mount(ped: i32) -> i32 {
    let value = native!(i32, 0xE7E11B8DCBED1058, native_parameters!(ped));

    value
}

pub fn is_ped_on_vehicle(ped: i32) -> bool {
    let value = native!(bool, 0x67722AEB798E5FAB, native_parameters!(ped));

    value
}

pub fn is_ped_on_specific_vehicle(ped: i32, vehicle: i32) -> bool {
    let value = native!(bool, 0xEC5F66E459AF3BB2, native_parameters!(ped, vehicle));

    value
}

pub fn set_ped_money(ped: i32, amount: i32) -> () {
    let value = native!((), 0xA9C8960E8684C1B5, native_parameters!(ped, amount));

    value
}

pub fn get_ped_money(ped: i32) -> i32 {
    let value = native!(i32, 0x3F69145BBA87BAE7, native_parameters!(ped));

    value
}

pub fn _0xff4803bc019852d9(p0: f32, p1: u32) -> () {
    let value = native!((), 0xFF4803BC019852D9, native_parameters!(p0, p1));

    value
}

pub fn set_ambient_peds_drop_money(p0: bool) -> () {
    let value = native!((), 0x6B0E6172C9A4D902, native_parameters!(p0));

    value
}

pub fn _0x9911f4a24485f653(p0: bool) -> () {
    let value = native!((), 0x9911F4A24485F653, native_parameters!(p0));

    value
}

pub fn set_ped_suffers_critical_hits(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xEBD76F2359F190AC, native_parameters!(ped, toggle));

    value
}

pub fn _0xafc976fd0580c7b3(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xAFC976FD0580C7B3, native_parameters!(ped, toggle));

    value
}

pub fn is_ped_sitting_in_vehicle(ped: i32, vehicle: i32) -> bool {
    let value = native!(bool, 0xA808AA1D79230FC2, native_parameters!(ped, vehicle));

    value
}

pub fn is_ped_sitting_in_any_vehicle(ped: i32) -> bool {
    let value = native!(bool, 0x826AA586EDB9FEF8, native_parameters!(ped));

    value
}

pub fn is_ped_on_foot(ped: i32) -> bool {
    let value = native!(bool, 0x01FEE67DB37F59B2, native_parameters!(ped));

    value
}

pub fn is_ped_on_any_bike(ped: i32) -> bool {
    let value = native!(bool, 0x94495889E22C6479, native_parameters!(ped));

    value
}

pub fn is_ped_planting_bomb(ped: i32) -> bool {
    let value = native!(bool, 0xC70B5FAE151982D8, native_parameters!(ped));

    value
}

pub fn get_dead_ped_pickup_coords(ped: i32, p1: f32, p2: f32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xCD5003B097200F36,
        native_parameters!(ped, p1, p2)
    );

    value
}

pub fn is_ped_in_any_boat(ped: i32) -> bool {
    let value = native!(bool, 0x2E0E1C2B4F6CB339, native_parameters!(ped));

    value
}

pub fn is_ped_in_any_sub(ped: i32) -> bool {
    let value = native!(bool, 0xFBFC01CCFB35D99E, native_parameters!(ped));

    value
}

pub fn is_ped_in_any_heli(ped: i32) -> bool {
    let value = native!(bool, 0x298B91AE825E5705, native_parameters!(ped));

    value
}

pub fn is_ped_in_any_plane(ped: i32) -> bool {
    let value = native!(bool, 0x5FFF4CFC74D8FB80, native_parameters!(ped));

    value
}

pub fn is_ped_in_flying_vehicle(ped: i32) -> bool {
    let value = native!(bool, 0x9134873537FA419C, native_parameters!(ped));

    value
}

pub fn set_ped_dies_in_water(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x56CEF0AC79073BDE, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_dies_in_sinking_vehicle(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xD718A22995E2B4BC, native_parameters!(ped, toggle));

    value
}

pub fn get_ped_armour(ped: i32) -> i32 {
    let value = native!(i32, 0x9483AF821605B1D8, native_parameters!(ped));

    value
}

pub fn set_ped_stay_in_vehicle_when_jacked(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xEDF4079F9D54C9A1, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_can_be_shot_in_vehicle(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xC7EF1BA83230BA07, native_parameters!(ped, toggle));

    value
}

pub fn get_ped_last_damage_bone(ped: i32, outBone: *mut i32) -> bool {
    let value = native!(bool, 0xD75960F6BD9EA49C, native_parameters!(ped, outBone));

    value
}

pub fn clear_ped_last_damage_bone(ped: i32) -> () {
    let value = native!((), 0x8EF6B7AC68E2F01B, native_parameters!(ped));

    value
}

pub fn set_ai_weapon_damage_modifier(value: f32) -> () {
    let value = native!((), 0x1B1E2A40A65B8521, native_parameters!(value));

    value
}

pub fn reset_ai_weapon_damage_modifier() -> () {
    let value = native!((), 0xEA16670E7BA4743C, native_parameters!());

    value
}

pub fn set_ai_melee_weapon_damage_modifier(modifier: f32) -> () {
    let value = native!((), 0x66460DEDDD417254, native_parameters!(modifier));

    value
}

pub fn reset_ai_melee_weapon_damage_modifier() -> () {
    let value = native!((), 0x46E56A7CD1D63C3F, native_parameters!());

    value
}

pub fn _0x2f3c3d9f50681de4(p0: u32, p1: bool) -> () {
    let value = native!((), 0x2F3C3D9F50681DE4, native_parameters!(p0, p1));

    value
}

pub fn set_ped_can_be_targetted(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x63F58F7C80513AAD, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_can_be_targetted_by_team(ped: i32, team: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0xBF1CA77833E58F2C,
        native_parameters!(ped, team, toggle)
    );

    value
}

pub fn set_ped_can_be_targetted_by_player(ped: i32, player: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0x66B57B72E0836A76,
        native_parameters!(ped, player, toggle)
    );

    value
}

pub fn _0x061cb768363d6424(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x061CB768363D6424, native_parameters!(ped, toggle));

    value
}

pub fn _0xfd325494792302d7(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xFD325494792302D7, native_parameters!(ped, toggle));

    value
}

pub fn is_ped_in_any_police_vehicle(ped: i32) -> bool {
    let value = native!(bool, 0x0BD04E29640C9C12, native_parameters!(ped));

    value
}

pub fn force_ped_to_open_parachute(ped: i32) -> () {
    let value = native!((), 0x16E42E800B472221, native_parameters!(ped));

    value
}

pub fn is_ped_in_parachute_free_fall(ped: i32) -> bool {
    let value = native!(bool, 0x7DCE8BDA0F1C1200, native_parameters!(ped));

    value
}

pub fn is_ped_falling(ped: i32) -> bool {
    let value = native!(bool, 0xFB92A102F1C4DFA3, native_parameters!(ped));

    value
}

pub fn is_ped_jumping(ped: i32) -> bool {
    let value = native!(bool, 0xCEDABC5900A0BF97, native_parameters!(ped));

    value
}

pub fn _0x412f1364fa066cfb(p0: u32) -> u32 {
    let value = native!(u32, 0x412F1364FA066CFB, native_parameters!(p0));

    value
}

pub fn _0x451d05012ccec234(p0: u32) -> u32 {
    let value = native!(u32, 0x451D05012CCEC234, native_parameters!(p0));

    value
}

pub fn is_ped_climbing(ped: i32) -> bool {
    let value = native!(bool, 0x53E8CB4F48BFE623, native_parameters!(ped));

    value
}

pub fn is_ped_vaulting(ped: i32) -> bool {
    let value = native!(bool, 0x117C70D1F5730B5E, native_parameters!(ped));

    value
}

pub fn is_ped_diving(ped: i32) -> bool {
    let value = native!(bool, 0x5527B8246FEF9B11, native_parameters!(ped));

    value
}

pub fn is_ped_jumping_out_of_vehicle(ped: i32) -> bool {
    let value = native!(bool, 0x433DDFFE2044B636, native_parameters!(ped));

    value
}

pub fn _is_ped_opening_a_door(ped: i32) -> bool {
    let value = native!(bool, 0x26AF0E8E30BD2A2C, native_parameters!(ped));

    value
}

pub fn get_ped_parachute_state(ped: i32) -> i32 {
    let value = native!(i32, 0x79CFD9827CC979B6, native_parameters!(ped));

    value
}

pub fn get_ped_parachute_landing_type(ped: i32) -> i32 {
    let value = native!(i32, 0x8B9F1FC6AE8166C0, native_parameters!(ped));

    value
}

pub fn set_ped_parachute_tint_index(ped: i32, tintIndex: i32) -> () {
    let value = native!((), 0x333FC8DB079B7186, native_parameters!(ped, tintIndex));

    value
}

pub fn get_ped_parachute_tint_index(ped: i32, outTintIndex: *mut i32) -> () {
    let value = native!(
        (),
        0xEAF5F7E5AE7C6C9D,
        native_parameters!(ped, outTintIndex)
    );

    value
}

pub fn set_ped_reserve_parachute_tint_index(ped: i32, p1: u32) -> () {
    let value = native!((), 0xE88DA0751C22A2AD, native_parameters!(ped, p1));

    value
}

pub fn create_parachute_bag_object(ped: i32, p1: bool, p2: bool) -> i32 {
    let value = native!(i32, 0x8C4F3BF23B6237DB, native_parameters!(ped, p1, p2));

    value
}

pub fn set_ped_ducking(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x030983CA930B692D, native_parameters!(ped, toggle));

    value
}

pub fn is_ped_ducking(ped: i32) -> bool {
    let value = native!(bool, 0xD125AE748725C6BC, native_parameters!(ped));

    value
}

pub fn is_ped_in_any_taxi(ped: i32) -> bool {
    let value = native!(bool, 0x6E575D6A898AB852, native_parameters!(ped));

    value
}

pub fn set_ped_id_range(ped: i32, value: f32) -> () {
    let value = native!((), 0xF107E836A70DCE05, native_parameters!(ped, value));

    value
}

pub fn set_ped_highly_perceptive(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x52D59AB61DDC05DD, native_parameters!(ped, toggle));

    value
}

pub fn _0x2f074c904d85129e(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32, p6: u32) -> () {
    let value = native!(
        (),
        0x2F074C904D85129E,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn _0xec4b4b3b9908052a(ped: i32, unk: f32) -> () {
    let value = native!((), 0xEC4B4B3B9908052A, native_parameters!(ped, unk));

    value
}

pub fn _0x733c87d4ce22bea2(p0: u32) -> () {
    let value = native!((), 0x733C87D4CE22BEA2, native_parameters!(p0));

    value
}

pub fn set_ped_seeing_range(ped: i32, value: f32) -> () {
    let value = native!((), 0xF29CF591C4BF6CEE, native_parameters!(ped, value));

    value
}

pub fn set_ped_hearing_range(ped: i32, value: f32) -> () {
    let value = native!((), 0x33A8F7F7D5F7F33C, native_parameters!(ped, value));

    value
}

pub fn set_ped_visual_field_min_angle(ped: i32, value: f32) -> () {
    let value = native!((), 0x2DB492222FB21E26, native_parameters!(ped, value));

    value
}

pub fn set_ped_visual_field_max_angle(ped: i32, value: f32) -> () {
    let value = native!((), 0x70793BDCA1E854D4, native_parameters!(ped, value));

    value
}

pub fn set_ped_visual_field_min_elevation_angle(ped: i32, angle: f32) -> () {
    let value = native!((), 0x7A276EB2C224D70F, native_parameters!(ped, angle));

    value
}

pub fn set_ped_visual_field_max_elevation_angle(ped: i32, angle: f32) -> () {
    let value = native!((), 0x78D0B67629D75856, native_parameters!(ped, angle));

    value
}

pub fn set_ped_visual_field_peripheral_range(ped: i32, range: f32) -> () {
    let value = native!((), 0x9C74B0BC831B753A, native_parameters!(ped, range));

    value
}

pub fn set_ped_visual_field_center_angle(ped: i32, angle: f32) -> () {
    let value = native!((), 0x3B6405E8AB34A907, native_parameters!(ped, angle));

    value
}

pub fn _get_ped_visual_field_center_angle(ped: i32) -> f32 {
    let value = native!(f32, 0xEF2C71A32CAD5FBD, native_parameters!(ped));

    value
}

pub fn set_ped_stealth_movement(ped: i32, p1: bool, action: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x88CBB5CEB96B7BD2,
        native_parameters!(ped, p1, action.as_ptr())
    );

    value
}

pub fn get_ped_stealth_movement(ped: i32) -> bool {
    let value = native!(bool, 0x7C2AC9CA66575FBF, native_parameters!(ped));

    value
}

pub fn create_group(unused: i32) -> i32 {
    let value = native!(i32, 0x90370EBE0FEE1A3D, native_parameters!(unused));

    value
}

pub fn set_ped_as_group_leader(ped: i32, groupId: i32) -> () {
    let value = native!((), 0x2A7819605465FBCE, native_parameters!(ped, groupId));

    value
}

pub fn set_ped_as_group_member(ped: i32, groupId: i32) -> () {
    let value = native!((), 0x9F3480FE65DB31B5, native_parameters!(ped, groupId));

    value
}

pub fn set_ped_can_teleport_to_group_leader(pedHandle: i32, groupHandle: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0x2E2F4240B3F24647,
        native_parameters!(pedHandle, groupHandle, toggle)
    );

    value
}

pub fn remove_group(groupId: i32) -> () {
    let value = native!((), 0x8EB2F69076AF7053, native_parameters!(groupId));

    value
}

pub fn remove_ped_from_group(ped: i32) -> () {
    let value = native!((), 0xED74007FFB146BC2, native_parameters!(ped));

    value
}

pub fn is_ped_group_member(ped: i32, groupId: i32) -> bool {
    let value = native!(bool, 0x9BB01E3834671191, native_parameters!(ped, groupId));

    value
}

pub fn is_ped_hanging_on_to_vehicle(ped: i32) -> bool {
    let value = native!(bool, 0x1C86D8AEF8254B78, native_parameters!(ped));

    value
}

pub fn set_group_separation_range(groupHandle: i32, separationRange: f32) -> () {
    let value = native!(
        (),
        0x4102C7858CFEE4E4,
        native_parameters!(groupHandle, separationRange)
    );

    value
}

pub fn set_ped_min_ground_time_for_stungun(ped: i32, ms: i32) -> () {
    let value = native!((), 0xFA0675AB151073FA, native_parameters!(ped, ms));

    value
}

pub fn is_ped_prone(ped: i32) -> bool {
    let value = native!(bool, 0xD6A86331A537A7B9, native_parameters!(ped));

    value
}

pub fn is_ped_in_combat(ped: i32, target: i32) -> bool {
    let value = native!(bool, 0x4859F1FC66A6278E, native_parameters!(ped, target));

    value
}

pub fn can_ped_in_combat_see_target(ped: i32, target: i32) -> bool {
    let value = native!(bool, 0xEAD42DE3610D0721, native_parameters!(ped, target));

    value
}

pub fn is_ped_doing_driveby(ped: i32) -> bool {
    let value = native!(bool, 0xB2C086CC1BF8F2BF, native_parameters!(ped));

    value
}

pub fn is_ped_jacking(ped: i32) -> bool {
    let value = native!(bool, 0x4AE4FF911DFB61DA, native_parameters!(ped));

    value
}

pub fn is_ped_being_jacked(ped: i32) -> bool {
    let value = native!(bool, 0x9A497FE2DF198913, native_parameters!(ped));

    value
}

pub fn is_ped_being_stunned(ped: i32, p1: i32) -> bool {
    let value = native!(bool, 0x4FBACCE3B4138EE8, native_parameters!(ped, p1));

    value
}

pub fn get_peds_jacker(ped: i32) -> i32 {
    let value = native!(i32, 0x9B128DC36C1E04CF, native_parameters!(ped));

    value
}

pub fn get_jack_target(ped: i32) -> i32 {
    let value = native!(i32, 0x5486A79D9FBD342D, native_parameters!(ped));

    value
}

pub fn is_ped_fleeing(ped: i32) -> bool {
    let value = native!(bool, 0xBBCCE00B381F8482, native_parameters!(ped));

    value
}

pub fn is_ped_in_cover(ped: i32, exceptUseWeapon: bool) -> bool {
    let value = native!(
        bool,
        0x60DFD0691A170B88,
        native_parameters!(ped, exceptUseWeapon)
    );

    value
}

pub fn is_ped_in_cover_facing_left(ped: i32) -> bool {
    let value = native!(bool, 0x845333B3150583AB, native_parameters!(ped));

    value
}

pub fn is_ped_in_high_cover(ped: i32) -> bool {
    let value = native!(bool, 0x6A03BF943D767C93, native_parameters!(ped));

    value
}

pub fn is_ped_going_into_cover(ped: i32) -> bool {
    let value = native!(bool, 0x9F65DBC537E59AD5, native_parameters!(ped));

    value
}

pub fn set_ped_pinned_down(ped: i32, pinned: bool, i: i32) -> u32 {
    let value = native!(u32, 0xAAD6D1ACF08F4612, native_parameters!(ped, pinned, i));

    value
}

pub fn get_seat_ped_is_trying_to_enter(ped: i32) -> i32 {
    let value = native!(i32, 0x6F4C85ACD641BCD2, native_parameters!(ped));

    value
}

pub fn get_vehicle_ped_is_trying_to_enter(ped: i32) -> i32 {
    let value = native!(i32, 0x814FA8BE5449445D, native_parameters!(ped));

    value
}

pub fn get_ped_source_of_death(ped: i32) -> i32 {
    let value = native!(i32, 0x93C8B64DEB84728C, native_parameters!(ped));

    value
}

pub fn get_ped_cause_of_death(ped: i32) -> u32 {
    let value = native!(u32, 0x16FFE42AB2D2DC59, native_parameters!(ped));

    value
}

pub fn get_ped_time_of_death(ped: i32) -> i32 {
    let value = native!(i32, 0x1E98817B311AE98A, native_parameters!(ped));

    value
}

pub fn _0x5407b7288d0478b7(p0: u32) -> i32 {
    let value = native!(i32, 0x5407B7288D0478B7, native_parameters!(p0));

    value
}

pub fn _0x336b3d200ab007cb(p0: u32, p1: f32, p2: f32, p3: f32, p4: f32) -> u32 {
    let value = native!(
        u32,
        0x336B3D200AB007CB,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn set_ped_relationship_group_default_hash(ped: i32, hash: u32) -> () {
    let value = native!((), 0xADB3F206518799E8, native_parameters!(ped, hash));

    value
}

pub fn set_ped_relationship_group_hash(ped: i32, hash: u32) -> () {
    let value = native!((), 0xC80A74AC829DDD92, native_parameters!(ped, hash));

    value
}

pub fn set_relationship_between_groups(relationship: i32, group1: u32, group2: u32) -> () {
    let value = native!(
        (),
        0xBF25EB89375A37AD,
        native_parameters!(relationship, group1, group2)
    );

    value
}

pub fn clear_relationship_between_groups(relationship: i32, group1: u32, group2: u32) -> () {
    let value = native!(
        (),
        0x5E29243FB56FC6D4,
        native_parameters!(relationship, group1, group2)
    );

    value
}

pub fn add_relationship_group(name: &std::ffi::CString, groupHash: *mut u32) -> u32 {
    let value = native!(
        u32,
        0xF372BC22FCB88606,
        native_parameters!(name.as_ptr(), groupHash)
    );

    value
}

pub fn remove_relationship_group(groupHash: u32) -> () {
    let value = native!((), 0xB6BA2444AB393DA2, native_parameters!(groupHash));

    value
}

pub fn _does_relationship_group_exist(groupHash: u32) -> bool {
    let value = native!(bool, 0xCC6E3B6BB69501F1, native_parameters!(groupHash));

    value
}

pub fn get_relationship_between_peds(ped1: i32, ped2: i32) -> i32 {
    let value = native!(i32, 0xEBA5AD3A0EAF7121, native_parameters!(ped1, ped2));

    value
}

pub fn get_ped_relationship_group_default_hash(ped: i32) -> u32 {
    let value = native!(u32, 0x42FDD0F017B1E38E, native_parameters!(ped));

    value
}

pub fn get_ped_relationship_group_hash(ped: i32) -> u32 {
    let value = native!(u32, 0x7DBDD04862D95F04, native_parameters!(ped));

    value
}

pub fn get_relationship_between_groups(group1: u32, group2: u32) -> i32 {
    let value = native!(i32, 0x9E6B70061662AE5C, native_parameters!(group1, group2));

    value
}

pub fn _set_relationship_group_dont_affect_wanted_level(group: u32, p1: bool) -> () {
    let value = native!((), 0x5615E0C5EB2BC6E2, native_parameters!(group, p1));

    value
}

pub fn _0xad27d957598e49e9(ped: i32, p1: u32, p2: f32, hash: u32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0xAD27D957598E49E9,
        native_parameters!(ped, p1, p2, hash, p4, p5)
    );

    value
}

pub fn set_ped_can_be_targeted_without_los(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x4328652AE5769C71, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_to_inform_respected_friends(ped: i32, radius: f32, maxFriends: i32) -> () {
    let value = native!(
        (),
        0x112942C6E708F70B,
        native_parameters!(ped, radius, maxFriends)
    );

    value
}

pub fn is_ped_responding_to_event(ped: i32, event: u32) -> bool {
    let value = native!(bool, 0x625B774D75C87068, native_parameters!(ped, event));

    value
}

pub fn _get_ped_event_data(ped: i32, eventType: i32, outData: *mut u32) -> bool {
    let value = native!(
        bool,
        0xBA656A3BB01BDEA3,
        native_parameters!(ped, eventType, outData)
    );

    value
}

pub fn set_ped_firing_pattern(ped: i32, patternHash: u32) -> () {
    let value = native!((), 0x9AC577F5A12AD8A9, native_parameters!(ped, patternHash));

    value
}

pub fn set_ped_shoot_rate(ped: i32, shootRate: i32) -> () {
    let value = native!((), 0x614DA022990752DC, native_parameters!(ped, shootRate));

    value
}

pub fn set_combat_float(ped: i32, combatType: i32, p2: f32) -> () {
    let value = native!(
        (),
        0xFF41B4B141ED981C,
        native_parameters!(ped, combatType, p2)
    );

    value
}

pub fn get_combat_float(ped: i32, p1: i32) -> f32 {
    let value = native!(f32, 0x52DFF8A10508090A, native_parameters!(ped, p1));

    value
}

pub fn get_group_size(groupID: i32, unknown: *mut u32, sizeInMembers: *mut i32) -> () {
    let value = native!(
        (),
        0x8DE69FE35CA09A45,
        native_parameters!(groupID, unknown, sizeInMembers)
    );

    value
}

pub fn does_group_exist(groupId: i32) -> bool {
    let value = native!(bool, 0x7C6B0C22F9F40BBE, native_parameters!(groupId));

    value
}

pub fn get_ped_group_index(ped: i32) -> i32 {
    let value = native!(i32, 0xF162E133B4E7A675, native_parameters!(ped));

    value
}

pub fn is_ped_in_group(ped: i32) -> bool {
    let value = native!(bool, 0x5891CAC5D4ACFF74, native_parameters!(ped));

    value
}

pub fn get_player_ped_is_following(ped: i32) -> i32 {
    let value = native!(i32, 0x6A3975DEA89F9A17, native_parameters!(ped));

    value
}

pub fn set_group_formation(groupId: i32, formationType: i32) -> () {
    let value = native!(
        (),
        0xCE2F5FC3AF7E8C1E,
        native_parameters!(groupId, formationType)
    );

    value
}

pub fn set_group_formation_spacing(groupId: i32, p1: f32, p2: f32, p3: f32) -> () {
    let value = native!(
        (),
        0x1D9D45004C28C916,
        native_parameters!(groupId, p1, p2, p3)
    );

    value
}

pub fn reset_group_formation_default_spacing(groupHandle: i32) -> () {
    let value = native!((), 0x63DAB4CCB3273205, native_parameters!(groupHandle));

    value
}

pub fn get_vehicle_ped_is_using(ped: i32) -> i32 {
    let value = native!(i32, 0x6094AD011A2EA87D, native_parameters!(ped));

    value
}

pub fn get_vehicle_ped_is_entering(ped: i32) -> i32 {
    let value = native!(i32, 0xF92691AED837A5FC, native_parameters!(ped));

    value
}

pub fn set_ped_gravity(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x9FF447B6B6AD960A, native_parameters!(ped, toggle));

    value
}

pub fn apply_damage_to_ped(ped: i32, damageAmount: i32, p2: bool, p3: u32) -> () {
    let value = native!(
        (),
        0x697157CED63F18D4,
        native_parameters!(ped, damageAmount, p2, p3)
    );

    value
}

pub fn _get_time_of_last_ped_weapon_damage(ped: i32, weaponHash: u32) -> i32 {
    let value = native!(i32, 0x36B77BB84687C318, native_parameters!(ped, weaponHash));

    value
}

pub fn set_ped_allowed_to_duck(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xDA1F1B7BE1A8766F, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_never_leaves_group(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x3DBFC55D5C9BB447, native_parameters!(ped, toggle));

    value
}

pub fn get_ped_type(ped: i32) -> i32 {
    let value = native!(i32, 0xFF059E1E4C01E63C, native_parameters!(ped));

    value
}

pub fn set_ped_as_cop(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xBB03C38DD3FB7FFD, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_max_health(ped: i32, value: i32) -> () {
    let value = native!((), 0xF5F6378C4F3419D3, native_parameters!(ped, value));

    value
}

pub fn get_ped_max_health(ped: i32) -> i32 {
    let value = native!(i32, 0x4700A416E8324EF3, native_parameters!(ped));

    value
}

pub fn set_ped_max_time_in_water(ped: i32, value: f32) -> () {
    let value = native!((), 0x43C851690662113D, native_parameters!(ped, value));

    value
}

pub fn set_ped_max_time_underwater(ped: i32, value: f32) -> () {
    let value = native!((), 0x6BA428C528D9E522, native_parameters!(ped, value));

    value
}

pub fn _0x2735233a786b1bef(ped: i32, p1: f32) -> () {
    let value = native!((), 0x2735233A786B1BEF, native_parameters!(ped, p1));

    value
}

pub fn set_ped_vehicle_forced_seat_usage(
    ped: i32,
    vehicle: i32,
    seatIndex: i32,
    flags: i32,
    p4: u32,
) -> () {
    let value = native!(
        (),
        0x952F06BEECD775CC,
        native_parameters!(ped, vehicle, seatIndex, flags, p4)
    );

    value
}

pub fn clear_all_ped_vehicle_forced_seat_usage(ped: i32) -> () {
    let value = native!((), 0xE6CA85E7259CE16B, native_parameters!(ped));

    value
}

pub fn _0xb282749d5e028163(p0: u32, p1: u32) -> () {
    let value = native!((), 0xB282749D5E028163, native_parameters!(p0, p1));

    value
}

pub fn set_ped_can_be_knocked_off_vehicle(ped: i32, state: i32) -> () {
    let value = native!((), 0x7A6535691B477C48, native_parameters!(ped, state));

    value
}

pub fn can_knock_ped_off_vehicle(ped: i32) -> bool {
    let value = native!(bool, 0x51AC07A44D4F5B8A, native_parameters!(ped));

    value
}

pub fn knock_ped_off_vehicle(ped: i32) -> () {
    let value = native!((), 0x45BBCBA77C29A841, native_parameters!(ped));

    value
}

pub fn set_ped_coords_no_gang(ped: i32, posX: f32, posY: f32, posZ: f32) -> () {
    let value = native!(
        (),
        0x87052FE446E07247,
        native_parameters!(ped, posX, posY, posZ)
    );

    value
}

pub fn get_ped_as_group_member(groupID: i32, memberNumber: i32) -> i32 {
    let value = native!(
        i32,
        0x51455483CF23ED97,
        native_parameters!(groupID, memberNumber)
    );

    value
}

pub fn get_ped_as_group_leader(groupID: i32) -> i32 {
    let value = native!(i32, 0x5CCE68DBD5FE93EC, native_parameters!(groupID));

    value
}

pub fn set_ped_keep_task(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x971D38760FBC02EF, native_parameters!(ped, toggle));

    value
}

pub fn _0x49e50bdb8ba4dab2(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x49E50BDB8BA4DAB2, native_parameters!(ped, toggle));

    value
}

pub fn is_ped_swimming(ped: i32) -> bool {
    let value = native!(bool, 0x9DE327631295B4C2, native_parameters!(ped));

    value
}

pub fn is_ped_swimming_under_water(ped: i32) -> bool {
    let value = native!(bool, 0xC024869A53992F34, native_parameters!(ped));

    value
}

pub fn set_ped_coords_keep_vehicle(ped: i32, posX: f32, posY: f32, posZ: f32) -> () {
    let value = native!(
        (),
        0x9AFEFF481A85AB2E,
        native_parameters!(ped, posX, posY, posZ)
    );

    value
}

pub fn set_ped_dies_in_vehicle(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x2A30922C90C9B42C, native_parameters!(ped, toggle));

    value
}

pub fn set_create_random_cops(toggle: bool) -> () {
    let value = native!((), 0x102E68B2024D536D, native_parameters!(toggle));

    value
}

pub fn set_create_random_cops_not_on_scenarios(toggle: bool) -> () {
    let value = native!((), 0x8A4986851C4EF6E7, native_parameters!(toggle));

    value
}

pub fn set_create_random_cops_on_scenarios(toggle: bool) -> () {
    let value = native!((), 0x444CB7D7DBE6973D, native_parameters!(toggle));

    value
}

pub fn can_create_random_cops() -> bool {
    let value = native!(bool, 0x5EE2CAFF7F17770D, native_parameters!());

    value
}

pub fn set_ped_as_enemy(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x02A0C9720B854BFA, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_can_smash_glass(ped: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x1CCE141467FF42A2, native_parameters!(ped, p1, p2));

    value
}

pub fn is_ped_in_any_train(ped: i32) -> bool {
    let value = native!(bool, 0x6F972C1AB75A1ED0, native_parameters!(ped));

    value
}

pub fn is_ped_getting_into_a_vehicle(ped: i32) -> bool {
    let value = native!(bool, 0xBB062B2B5722478E, native_parameters!(ped));

    value
}

pub fn is_ped_trying_to_enter_a_locked_vehicle(ped: i32) -> bool {
    let value = native!(bool, 0x44D28D5DDFE5F68C, native_parameters!(ped));

    value
}

pub fn set_enable_handcuffs(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xDF1AF8B5D56542FA, native_parameters!(ped, toggle));

    value
}

pub fn set_enable_bound_ankles(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xC52E0F855C58FC2E, native_parameters!(ped, toggle));

    value
}

pub fn set_enable_scuba(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xF99F62004024D506, native_parameters!(ped, toggle));

    value
}

pub fn set_can_attack_friendly(ped: i32, toggle: bool, p2: bool) -> () {
    let value = native!((), 0xB3B1CB349FF9C75D, native_parameters!(ped, toggle, p2));

    value
}

pub fn get_ped_alertness(ped: i32) -> i32 {
    let value = native!(i32, 0xF6AA118530443FD2, native_parameters!(ped));

    value
}

pub fn set_ped_alertness(ped: i32, value: i32) -> () {
    let value = native!((), 0xDBA71115ED9941A6, native_parameters!(ped, value));

    value
}

pub fn set_ped_get_out_upside_down_vehicle(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xBC0ED94165A48BC2, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_movement_clipset(ped: i32, clipSet: &std::ffi::CString, p2: f32) -> () {
    let value = native!(
        (),
        0xAF8A94EDE7712BEF,
        native_parameters!(ped, clipSet.as_ptr(), p2)
    );

    value
}

pub fn reset_ped_movement_clipset(ped: i32, p1: f32) -> () {
    let value = native!((), 0xAA74EC0CB0AAEA2C, native_parameters!(ped, p1));

    value
}

pub fn set_ped_strafe_clipset(ped: i32, clipSet: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x29A28F3F8CF6D854,
        native_parameters!(ped, clipSet.as_ptr())
    );

    value
}

pub fn reset_ped_strafe_clipset(ped: i32) -> () {
    let value = native!((), 0x20510814175EA477, native_parameters!(ped));

    value
}

pub fn set_ped_weapon_movement_clipset(ped: i32, clipSet: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x2622E35B77D3ACA2,
        native_parameters!(ped, clipSet.as_ptr())
    );

    value
}

pub fn reset_ped_weapon_movement_clipset(ped: i32) -> () {
    let value = native!((), 0x97B0DB5B4AA74E77, native_parameters!(ped));

    value
}

pub fn set_ped_drive_by_clipset_override(ped: i32, clipset: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xED34AB6C5CB36520,
        native_parameters!(ped, clipset.as_ptr())
    );

    value
}

pub fn clear_ped_drive_by_clipset_override(ped: i32) -> () {
    let value = native!((), 0x4AFE3690D7E0B5AC, native_parameters!(ped));

    value
}

pub fn _set_ped_cover_clipset_override(ped: i32, p1: &std::ffi::CString) -> () {
    let value = native!((), 0x9DBA107B4937F809, native_parameters!(ped, p1.as_ptr()));

    value
}

pub fn _clear_ped_cover_clipset_override(ped: i32) -> () {
    let value = native!((), 0xC79196DCB36F6121, native_parameters!(ped));

    value
}

pub fn _0x80054d7fcc70eec6(ped: i32) -> () {
    let value = native!((), 0x80054D7FCC70EEC6, native_parameters!(ped));

    value
}

pub fn set_ped_in_vehicle_context(ped: i32, context: u32) -> () {
    let value = native!((), 0x530071295899A8C6, native_parameters!(ped, context));

    value
}

pub fn reset_ped_in_vehicle_context(ped: i32) -> () {
    let value = native!((), 0x22EF8FF8778030EB, native_parameters!(ped));

    value
}

pub fn is_scripted_scenario_ped_using_conditional_anim(
    ped: i32,
    animDict: &std::ffi::CString,
    anim: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x6EC47A344923E1ED,
        native_parameters!(ped, animDict.as_ptr(), anim.as_ptr())
    );

    value
}

pub fn set_ped_alternate_walk_anim(
    ped: i32,
    animDict: &std::ffi::CString,
    animName: &std::ffi::CString,
    p3: f32,
    p4: bool,
) -> () {
    let value = native!(
        (),
        0x6C60394CB4F75E9A,
        native_parameters!(ped, animDict.as_ptr(), animName.as_ptr(), p3, p4)
    );

    value
}

pub fn clear_ped_alternate_walk_anim(ped: i32, p1: f32) -> () {
    let value = native!((), 0x8844BBFCE30AA9E9, native_parameters!(ped, p1));

    value
}

pub fn set_ped_alternate_movement_anim(
    ped: i32,
    stance: i32,
    animDictionary: &std::ffi::CString,
    animationName: &std::ffi::CString,
    p4: f32,
    p5: bool,
) -> () {
    let value = native!(
        (),
        0x90A43CC281FFAB46,
        native_parameters!(
            ped,
            stance,
            animDictionary.as_ptr(),
            animationName.as_ptr(),
            p4,
            p5
        )
    );

    value
}

pub fn clear_ped_alternate_movement_anim(ped: i32, stance: i32, p2: f32) -> () {
    let value = native!((), 0xD8D19675ED5FBDCE, native_parameters!(ped, stance, p2));

    value
}

pub fn set_ped_gesture_group(ped: i32, animGroupGesture: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xDDF803377F94AAA8,
        native_parameters!(ped, animGroupGesture.as_ptr())
    );

    value
}

pub fn get_anim_initial_offset_position(
    animDict: &std::ffi::CString,
    animName: &std::ffi::CString,
    x: f32,
    y: f32,
    z: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    p8: f32,
    p9: i32,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xBE22B26DD764C040,
        native_parameters!(
            animDict.as_ptr(),
            animName.as_ptr(),
            x,
            y,
            z,
            xRot,
            yRot,
            zRot,
            p8,
            p9
        )
    );

    value
}

pub fn get_anim_initial_offset_rotation(
    animDict: &std::ffi::CString,
    animName: &std::ffi::CString,
    x: f32,
    y: f32,
    z: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    p8: f32,
    p9: i32,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x4B805E6046EE9E47,
        native_parameters!(
            animDict.as_ptr(),
            animName.as_ptr(),
            x,
            y,
            z,
            xRot,
            yRot,
            zRot,
            p8,
            p9
        )
    );

    value
}

pub fn get_ped_drawable_variation(ped: i32, componentId: i32) -> i32 {
    let value = native!(
        i32,
        0x67F3780DD425D4FC,
        native_parameters!(ped, componentId)
    );

    value
}

pub fn get_number_of_ped_drawable_variations(ped: i32, componentId: i32) -> i32 {
    let value = native!(
        i32,
        0x27561561732A7842,
        native_parameters!(ped, componentId)
    );

    value
}

pub fn get_ped_texture_variation(ped: i32, componentId: i32) -> i32 {
    let value = native!(
        i32,
        0x04A355E041E004E6,
        native_parameters!(ped, componentId)
    );

    value
}

pub fn get_number_of_ped_texture_variations(ped: i32, componentId: i32, drawableId: i32) -> i32 {
    let value = native!(
        i32,
        0x8F7156A3142A6BAD,
        native_parameters!(ped, componentId, drawableId)
    );

    value
}

pub fn get_number_of_ped_prop_drawable_variations(ped: i32, propId: i32) -> i32 {
    let value = native!(i32, 0x5FAF9754E789FB47, native_parameters!(ped, propId));

    value
}

pub fn get_number_of_ped_prop_texture_variations(ped: i32, propId: i32, drawableId: i32) -> i32 {
    let value = native!(
        i32,
        0xA6E7F1CEB523E171,
        native_parameters!(ped, propId, drawableId)
    );

    value
}

pub fn get_ped_palette_variation(ped: i32, componentId: i32) -> i32 {
    let value = native!(
        i32,
        0xE3DD5F2A84B42281,
        native_parameters!(ped, componentId)
    );

    value
}

pub fn _0x9e30e91fb03a2caf(p0: *mut u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0x9E30E91FB03A2CAF, native_parameters!(p0, p1));

    value
}

pub fn _0x1e77fa7a62ee6c4c(p0: u32) -> u32 {
    let value = native!(u32, 0x1E77FA7A62EE6C4C, native_parameters!(p0));

    value
}

pub fn _0xf033419d1b81fae8(p0: u32) -> u32 {
    let value = native!(u32, 0xF033419D1B81FAE8, native_parameters!(p0));

    value
}

pub fn is_ped_component_variation_valid(
    ped: i32,
    componentId: i32,
    drawableId: i32,
    textureId: i32,
) -> bool {
    let value = native!(
        bool,
        0xE825F6B6CEA7671D,
        native_parameters!(ped, componentId, drawableId, textureId)
    );

    value
}

pub fn set_ped_component_variation(
    ped: i32,
    componentId: i32,
    drawableId: i32,
    textureId: i32,
    paletteId: i32,
) -> () {
    let value = native!(
        (),
        0x262B14F48D29DE80,
        native_parameters!(ped, componentId, drawableId, textureId, paletteId)
    );

    value
}

pub fn set_ped_random_component_variation(ped: i32, p1: i32) -> () {
    let value = native!((), 0xC8A9481A01E63C28, native_parameters!(ped, p1));

    value
}

pub fn set_ped_random_props(ped: i32) -> () {
    let value = native!((), 0xC44AA05345C992C6, native_parameters!(ped));

    value
}

pub fn set_ped_default_component_variation(ped: i32) -> () {
    let value = native!((), 0x45EEE61580806D63, native_parameters!(ped));

    value
}

pub fn set_ped_blend_from_parents(ped: i32, p1: u32, p2: u32, p3: f32, p4: f32) -> () {
    let value = native!(
        (),
        0x137BBD05230DB22D,
        native_parameters!(ped, p1, p2, p3, p4)
    );

    value
}

pub fn set_ped_head_blend_data(
    ped: i32,
    shapeFirstID: i32,
    shapeSecondID: i32,
    shapeThirdID: i32,
    skinFirstID: i32,
    skinSecondID: i32,
    skinThirdID: i32,
    shapeMix: f32,
    skinMix: f32,
    thirdMix: f32,
    isParent: bool,
) -> () {
    let value = native!(
        (),
        0x9414E18B9434C2FE,
        native_parameters!(
            ped,
            shapeFirstID,
            shapeSecondID,
            shapeThirdID,
            skinFirstID,
            skinSecondID,
            skinThirdID,
            shapeMix,
            skinMix,
            thirdMix,
            isParent
        )
    );

    value
}

pub fn get_ped_head_blend_data(ped: i32, headBlendData: *mut u32) -> bool {
    let value = native!(
        bool,
        0x2746BD9D88C5C5D0,
        native_parameters!(ped, headBlendData)
    );

    value
}

pub fn update_ped_head_blend_data(ped: i32, shapeMix: f32, skinMix: f32, thirdMix: f32) -> () {
    let value = native!(
        (),
        0x723538F61C647C5A,
        native_parameters!(ped, shapeMix, skinMix, thirdMix)
    );

    value
}

pub fn _set_ped_eye_color(ped: i32, index: i32) -> () {
    let value = native!((), 0x50B56988B170AFDF, native_parameters!(ped, index));

    value
}

pub fn _get_ped_eye_color(ped: i32) -> i32 {
    let value = native!(i32, 0x76BBA2CEE66D47E9, native_parameters!(ped));

    value
}

pub fn set_ped_head_overlay(ped: i32, overlayID: i32, index: i32, opacity: f32) -> () {
    let value = native!(
        (),
        0x48F44967FA05CC1E,
        native_parameters!(ped, overlayID, index, opacity)
    );

    value
}

pub fn _get_ped_head_overlay_value(ped: i32, overlayID: i32) -> i32 {
    let value = native!(i32, 0xA60EF3B6461A4D43, native_parameters!(ped, overlayID));

    value
}

pub fn get_ped_head_overlay_num(overlayID: i32) -> i32 {
    let value = native!(i32, 0xCF1CE768BB43480E, native_parameters!(overlayID));

    value
}

pub fn _set_ped_head_overlay_color(
    ped: i32,
    overlayID: i32,
    colorType: i32,
    colorID: i32,
    secondColorID: i32,
) -> () {
    let value = native!(
        (),
        0x497BF74A7B9CB952,
        native_parameters!(ped, overlayID, colorType, colorID, secondColorID)
    );

    value
}

pub fn _set_ped_hair_color(ped: i32, colorID: i32, highlightColorID: i32) -> () {
    let value = native!(
        (),
        0x4CFFC65454C93A49,
        native_parameters!(ped, colorID, highlightColorID)
    );

    value
}

pub fn _get_num_hair_colors() -> i32 {
    let value = native!(i32, 0xE5C0CF872C2AD150, native_parameters!());

    value
}

pub fn _get_num_makeup_colors() -> i32 {
    let value = native!(i32, 0xD1F7CA1535D22818, native_parameters!());

    value
}

pub fn _get_ped_hair_rgb_color(
    hairColorIndex: i32,
    outR: *mut i32,
    outG: *mut i32,
    outB: *mut i32,
) -> () {
    let value = native!(
        (),
        0x4852FC386E2E1BB5,
        native_parameters!(hairColorIndex, outR, outG, outB)
    );

    value
}

pub fn _get_ped_makeup_rgb_color(
    makeupColorIndex: i32,
    outR: *mut i32,
    outG: *mut i32,
    outB: *mut i32,
) -> () {
    let value = native!(
        (),
        0x013E5CFC38CD5387,
        native_parameters!(makeupColorIndex, outR, outG, outB)
    );

    value
}

pub fn _is_ped_hair_color_valid_2(colorId: i32) -> bool {
    let value = native!(bool, 0xED6D8E27A43B8CDE, native_parameters!(colorId));

    value
}

pub fn _0xea9960d07dadcf10(p0: u32) -> i32 {
    let value = native!(i32, 0xEA9960D07DADCF10, native_parameters!(p0));

    value
}

pub fn _is_ped_lipstick_color_valid_2(colorId: i32) -> bool {
    let value = native!(bool, 0x3E802F11FBE27674, native_parameters!(colorId));

    value
}

pub fn _is_ped_blush_color_valid_2(colorId: i32) -> bool {
    let value = native!(bool, 0xF41B5D290C99A3D6, native_parameters!(colorId));

    value
}

pub fn _is_ped_hair_color_valid(colorID: i32) -> bool {
    let value = native!(bool, 0xE0D36E5D9E99CC21, native_parameters!(colorID));

    value
}

pub fn _0xaaa6a3698a69e048(p0: u32) -> u32 {
    let value = native!(u32, 0xAAA6A3698A69E048, native_parameters!(p0));

    value
}

pub fn _is_ped_lipstick_color_valid(colorID: i32) -> bool {
    let value = native!(bool, 0x0525A2C2562F3CD4, native_parameters!(colorID));

    value
}

pub fn _is_ped_blush_color_valid(colorID: i32) -> bool {
    let value = native!(bool, 0x604E810189EE3A59, native_parameters!(colorID));

    value
}

pub fn _is_ped_body_blemish_valid(colorId: i32) -> bool {
    let value = native!(bool, 0x09E7ECA981D9B210, native_parameters!(colorId));

    value
}

pub fn _0xc56fbf2f228e1dac(modelHash: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(
        u32,
        0xC56FBF2F228E1DAC,
        native_parameters!(modelHash, p1, p2)
    );

    value
}

pub fn _set_ped_face_feature(ped: i32, index: i32, scale: f32) -> () {
    let value = native!(
        (),
        0x71A5C1DBA060049E,
        native_parameters!(ped, index, scale)
    );

    value
}

pub fn has_ped_head_blend_finished(ped: i32) -> bool {
    let value = native!(bool, 0x654CD0A825161131, native_parameters!(ped));

    value
}

pub fn finalize_head_blend(ped: i32) -> () {
    let value = native!((), 0x4668D80430D6C299, native_parameters!(ped));

    value
}

pub fn set_head_blend_palette_color(ped: i32, r: i32, g: i32, b: i32, id: i32) -> () {
    let value = native!((), 0xCC9682B8951C5229, native_parameters!(ped, r, g, b, id));

    value
}

pub fn disable_head_blend_palette_color(ped: i32) -> () {
    let value = native!((), 0xA21C118553BBDF02, native_parameters!(ped));

    value
}

pub fn get_ped_head_blend_first_index(type_esc: i32) -> i32 {
    let value = native!(i32, 0x68D353AB88B97E0C, native_parameters!(type_esc));

    value
}

pub fn _get_num_parent_peds_of_type(type_esc: i32) -> i32 {
    let value = native!(i32, 0x5EF37013A6539C9D, native_parameters!(type_esc));

    value
}

pub fn set_ped_preload_variation_data(ped: i32, slot: i32, drawableId: i32, textureId: i32) -> u32 {
    let value = native!(
        u32,
        0x39D55A620FCB6A3A,
        native_parameters!(ped, slot, drawableId, textureId)
    );

    value
}

pub fn has_ped_preload_variation_data_finished(ped: i32) -> bool {
    let value = native!(bool, 0x66680A92700F43DF, native_parameters!(ped));

    value
}

pub fn release_ped_preload_variation_data(ped: i32) -> () {
    let value = native!((), 0x5AAB586FFEC0FD96, native_parameters!(ped));

    value
}

pub fn set_ped_preload_prop_data(
    ped: i32,
    componentId: i32,
    drawableId: i32,
    TextureId: i32,
) -> bool {
    let value = native!(
        bool,
        0x2B16A3BFF1FBCE49,
        native_parameters!(ped, componentId, drawableId, TextureId)
    );

    value
}

pub fn has_ped_preload_prop_data_finished(ped: i32) -> bool {
    let value = native!(bool, 0x784002A632822099, native_parameters!(ped));

    value
}

pub fn release_ped_preload_prop_data(ped: i32) -> () {
    let value = native!((), 0xF79F9DEF0AADE61A, native_parameters!(ped));

    value
}

pub fn get_ped_prop_index(ped: i32, componentId: i32) -> i32 {
    let value = native!(
        i32,
        0x898CC20EA75BACD8,
        native_parameters!(ped, componentId)
    );

    value
}

pub fn set_ped_prop_index(
    ped: i32,
    componentId: i32,
    drawableId: i32,
    TextureId: i32,
    attach: bool,
) -> () {
    let value = native!(
        (),
        0x93376B65A266EB5F,
        native_parameters!(ped, componentId, drawableId, TextureId, attach)
    );

    value
}

pub fn knock_off_ped_prop(ped: i32, p1: bool, p2: bool, p3: bool, p4: bool) -> () {
    let value = native!(
        (),
        0x6FD7816A36615F48,
        native_parameters!(ped, p1, p2, p3, p4)
    );

    value
}

pub fn clear_ped_prop(ped: i32, propId: i32) -> () {
    let value = native!((), 0x0943E5B8E078E76E, native_parameters!(ped, propId));

    value
}

pub fn clear_all_ped_props(ped: i32) -> () {
    let value = native!((), 0xCD8A7537A9B52F06, native_parameters!(ped));

    value
}

pub fn drop_ambient_prop(ped: i32) -> () {
    let value = native!((), 0xAFF4710E2A0A6C12, native_parameters!(ped));

    value
}

pub fn get_ped_prop_texture_index(ped: i32, componentId: i32) -> i32 {
    let value = native!(
        i32,
        0xE131A28626F81AB2,
        native_parameters!(ped, componentId)
    );

    value
}

pub fn clear_ped_parachute_pack_variation(ped: i32) -> () {
    let value = native!((), 0x1280804F7CFD2D6C, native_parameters!(ped));

    value
}

pub fn _set_ped_scuba_gear_variation(ped: i32) -> () {
    let value = native!((), 0x36C6984C3ED0C911, native_parameters!(ped));

    value
}

pub fn clear_ped_scuba_gear_variation(ped: i32) -> () {
    let value = native!((), 0xB50EB4CCB29704AC, native_parameters!(ped));

    value
}

pub fn _0xfec9a3b1820f3331(p0: u32) -> bool {
    let value = native!(bool, 0xFEC9A3B1820F3331, native_parameters!(p0));

    value
}

pub fn set_blocking_of_non_temporary_events(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x9F8AA94D6D97DBF4, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_bounds_orientation(ped: i32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32) -> () {
    let value = native!(
        (),
        0x4F5F651ACCC9C4CF,
        native_parameters!(ped, p1, p2, p3, p4, p5)
    );

    value
}

pub fn register_target(ped: i32, target: i32) -> () {
    let value = native!((), 0x2F25D9AEFA34FBA2, native_parameters!(ped, target));

    value
}

pub fn register_hated_targets_around_ped(ped: i32, radius: f32) -> () {
    let value = native!((), 0x9222F300BF8354FE, native_parameters!(ped, radius));

    value
}

pub fn get_random_ped_at_coord(
    x: f32,
    y: f32,
    z: f32,
    xRadius: f32,
    yRadius: f32,
    zRadius: f32,
    pedType: i32,
) -> i32 {
    let value = native!(
        i32,
        0x876046A8E3A4B71C,
        native_parameters!(x, y, z, xRadius, yRadius, zRadius, pedType)
    );

    value
}

pub fn get_closest_ped(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    p4: bool,
    p5: bool,
    outPed: *mut i32,
    p7: bool,
    p8: bool,
    pedType: i32,
) -> bool {
    let value = native!(
        bool,
        0xC33AB876A77F8164,
        native_parameters!(x, y, z, radius, p4, p5, outPed, p7, p8, pedType)
    );

    value
}

pub fn set_scenario_peds_to_be_returned_by_next_command(value: bool) -> () {
    let value = native!((), 0x14F19A8782C8071E, native_parameters!(value));

    value
}

pub fn _0x03ea03af85a85cb7(
    ped: i32,
    p1: bool,
    p2: bool,
    p3: bool,
    p4: bool,
    p5: bool,
    p6: bool,
    p7: bool,
    p8: u32,
) -> bool {
    let value = native!(
        bool,
        0x03EA03AF85A85CB7,
        native_parameters!(ped, p1, p2, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn set_driver_racing_modifier(driver: i32, modifier: f32) -> () {
    let value = native!((), 0xDED5AF5A0EA4B297, native_parameters!(driver, modifier));

    value
}

pub fn set_driver_ability(driver: i32, ability: f32) -> () {
    let value = native!((), 0xB195FFA8042FC5C3, native_parameters!(driver, ability));

    value
}

pub fn set_driver_aggressiveness(driver: i32, aggressiveness: f32) -> () {
    let value = native!(
        (),
        0xA731F608CA104E3C,
        native_parameters!(driver, aggressiveness)
    );

    value
}

pub fn can_ped_ragdoll(ped: i32) -> bool {
    let value = native!(bool, 0x128F79EDCECE4FD5, native_parameters!(ped));

    value
}

pub fn set_ped_to_ragdoll(
    ped: i32,
    time1: i32,
    time2: i32,
    ragdollType: i32,
    p4: bool,
    p5: bool,
    p6: bool,
) -> bool {
    let value = native!(
        bool,
        0xAE99FB955581844A,
        native_parameters!(ped, time1, time2, ragdollType, p4, p5, p6)
    );

    value
}

pub fn set_ped_to_ragdoll_with_fall(
    ped: i32,
    time: i32,
    p2: i32,
    ragdollType: i32,
    x: f32,
    y: f32,
    z: f32,
    p7: f32,
    p8: f32,
    p9: f32,
    p10: f32,
    p11: f32,
    p12: f32,
    p13: f32,
) -> bool {
    let value = native!(
        bool,
        0xD76632D99E4966C8,
        native_parameters!(
            ped,
            time,
            p2,
            ragdollType,
            x,
            y,
            z,
            p7,
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

pub fn set_ped_ragdoll_on_collision(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xF0A4F1BBF4FA7497, native_parameters!(ped, toggle));

    value
}

pub fn is_ped_ragdoll(ped: i32) -> bool {
    let value = native!(bool, 0x47E4E977581C5B55, native_parameters!(ped));

    value
}

pub fn is_ped_running_ragdoll_task(ped: i32) -> bool {
    let value = native!(bool, 0xE3B6097CC25AA69E, native_parameters!(ped));

    value
}

pub fn set_ped_ragdoll_force_fall(ped: i32) -> () {
    let value = native!((), 0x01F6594B923B9251, native_parameters!(ped));

    value
}

pub fn reset_ped_ragdoll_timer(ped: i32) -> () {
    let value = native!((), 0x9FA4664CF62E47E8, native_parameters!(ped));

    value
}

pub fn set_ped_can_ragdoll(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xB128377056A54E2A, native_parameters!(ped, toggle));

    value
}

pub fn is_ped_running_melee_task(ped: i32) -> bool {
    let value = native!(bool, 0xD1871251F3B5ACD7, native_parameters!(ped));

    value
}

pub fn is_ped_running_mobile_phone_task(ped: i32) -> bool {
    let value = native!(bool, 0x2AFE52F782F25775, native_parameters!(ped));

    value
}

pub fn _0xa3f3564a5b3646c0(ped: i32) -> bool {
    let value = native!(bool, 0xA3F3564A5B3646C0, native_parameters!(ped));

    value
}

pub fn set_ragdoll_blocking_flags(ped: i32, flags: i32) -> () {
    let value = native!((), 0x26695EC767728D84, native_parameters!(ped, flags));

    value
}

pub fn clear_ragdoll_blocking_flags(ped: i32, flags: i32) -> () {
    let value = native!((), 0xD86D101FCFD00A4B, native_parameters!(ped, flags));

    value
}

pub fn set_ped_angled_defensive_area(
    ped: i32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
    p8: bool,
    p9: bool,
) -> () {
    let value = native!(
        (),
        0xC7F76DF27A5045A1,
        native_parameters!(ped, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn set_ped_sphere_defensive_area(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    p5: bool,
    p6: bool,
) -> () {
    let value = native!(
        (),
        0x9D3151A373974804,
        native_parameters!(ped, x, y, z, radius, p5, p6)
    );

    value
}

pub fn set_ped_defensive_sphere_attached_to_ped(
    ped: i32,
    target: i32,
    xOffset: f32,
    yOffset: f32,
    zOffset: f32,
    radius: f32,
    p6: bool,
) -> () {
    let value = native!(
        (),
        0xF9B8F91AAD3B953E,
        native_parameters!(ped, target, xOffset, yOffset, zOffset, radius, p6)
    );

    value
}

pub fn set_ped_defensive_sphere_attached_to_vehicle(
    ped: i32,
    target: i32,
    xOffset: f32,
    yOffset: f32,
    zOffset: f32,
    radius: f32,
    p6: bool,
) -> () {
    let value = native!(
        (),
        0xE4723DB6E736CCFF,
        native_parameters!(ped, target, xOffset, yOffset, zOffset, radius, p6)
    );

    value
}

pub fn set_ped_defensive_area_attached_to_ped(
    ped: i32,
    attachPed: i32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
    p8: f32,
    p9: bool,
    p10: bool,
) -> () {
    let value = native!(
        (),
        0x4EF47FE21698A8B6,
        native_parameters!(ped, attachPed, p2, p3, p4, p5, p6, p7, p8, p9, p10)
    );

    value
}

pub fn set_ped_defensive_area_direction(ped: i32, p1: f32, p2: f32, p3: f32, p4: bool) -> () {
    let value = native!(
        (),
        0x413C6C763A4AFFAD,
        native_parameters!(ped, p1, p2, p3, p4)
    );

    value
}

pub fn remove_ped_defensive_area(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x74D4E028107450A9, native_parameters!(ped, toggle));

    value
}

pub fn get_ped_defensive_area_position(ped: i32, p1: bool) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x3C06B8786DD94CD1,
        native_parameters!(ped, p1)
    );

    value
}

pub fn is_ped_defensive_area_active(ped: i32, p1: bool) -> bool {
    let value = native!(bool, 0xBA63D9FE45412247, native_parameters!(ped, p1));

    value
}

pub fn set_ped_preferred_cover_set(ped: i32, itemSet: u32) -> () {
    let value = native!((), 0x8421EB4DA7E391B9, native_parameters!(ped, itemSet));

    value
}

pub fn remove_ped_preferred_cover_set(ped: i32) -> () {
    let value = native!((), 0xFDDB234CF74073D9, native_parameters!(ped));

    value
}

pub fn revive_injured_ped(ped: i32) -> () {
    let value = native!((), 0x8D8ACD8388CD99CE, native_parameters!(ped));

    value
}

pub fn resurrect_ped(ped: i32) -> () {
    let value = native!((), 0x71BC8E838B9C6035, native_parameters!(ped));

    value
}

pub fn set_ped_name_debug(ped: i32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x98EFA132A4117BE1,
        native_parameters!(ped, name.as_ptr())
    );

    value
}

pub fn get_ped_extracted_displacement(ped: i32, worldSpace: bool) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xE0AF41401ADF87E3,
        native_parameters!(ped, worldSpace)
    );

    value
}

pub fn set_ped_dies_when_injured(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x5BA7919BED300023, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_enable_weapon_blocking(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x97A790315D3831FD, native_parameters!(ped, toggle));

    value
}

pub fn _0xf9acf4a08098ea25(ped: i32, p1: bool) -> () {
    let value = native!((), 0xF9ACF4A08098EA25, native_parameters!(ped, p1));

    value
}

pub fn reset_ped_visible_damage(ped: i32) -> () {
    let value = native!((), 0x3AC1F7B898F30C05, native_parameters!(ped));

    value
}

pub fn apply_ped_blood_damage_by_zone(ped: i32, p1: u32, p2: f32, p3: f32, p4: u32) -> () {
    let value = native!(
        (),
        0x816F6981C60BF53B,
        native_parameters!(ped, p1, p2, p3, p4)
    );

    value
}

pub fn apply_ped_blood(
    ped: i32,
    boneIndex: i32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    woundType: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0x83F7E01C7B769A26,
        native_parameters!(ped, boneIndex, xRot, yRot, zRot, woundType.as_ptr())
    );

    value
}

pub fn apply_ped_blood_by_zone(ped: i32, p1: u32, p2: f32, p3: f32, p4: *mut u32) -> () {
    let value = native!(
        (),
        0x3311E47B91EDCBBC,
        native_parameters!(ped, p1, p2, p3, p4)
    );

    value
}

pub fn apply_ped_blood_specific(
    ped: i32,
    p1: u32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: u32,
    p7: f32,
    p8: *mut u32,
) -> () {
    let value = native!(
        (),
        0xEF0D582CBF2D9B0F,
        native_parameters!(ped, p1, p2, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn apply_ped_damage_decal(
    ped: i32,
    damageZone: i32,
    xOffset: f32,
    yOffset: f32,
    heading: f32,
    scale: f32,
    alpha: f32,
    variation: i32,
    fadeIn: bool,
    decalName: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0x397C38AA7B4A5F83,
        native_parameters!(
            ped,
            damageZone,
            xOffset,
            yOffset,
            heading,
            scale,
            alpha,
            variation,
            fadeIn,
            decalName.as_ptr()
        )
    );

    value
}

pub fn apply_ped_damage_pack(
    ped: i32,
    damagePack: &std::ffi::CString,
    damage: f32,
    mult: f32,
) -> () {
    let value = native!(
        (),
        0x46DF918788CB093F,
        native_parameters!(ped, damagePack.as_ptr(), damage, mult)
    );

    value
}

pub fn clear_ped_blood_damage(ped: i32) -> () {
    let value = native!((), 0x8FE22675A5A45817, native_parameters!(ped));

    value
}

pub fn clear_ped_blood_damage_by_zone(ped: i32, p1: i32) -> () {
    let value = native!((), 0x56E3B78C5408D9F4, native_parameters!(ped, p1));

    value
}

pub fn hide_ped_blood_damage_by_zone(ped: i32, p1: u32, p2: bool) -> () {
    let value = native!((), 0x62AB793144DE75DC, native_parameters!(ped, p1, p2));

    value
}

pub fn clear_ped_damage_decal_by_zone(ped: i32, p1: i32, p2: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x523C79AEEFCC4A2A,
        native_parameters!(ped, p1, p2.as_ptr())
    );

    value
}

pub fn get_ped_decorations_state(ped: i32) -> i32 {
    let value = native!(i32, 0x71EAB450D86954A1, native_parameters!(ped));

    value
}

pub fn _0x2b694afcf64e6994(ped: i32, p1: bool) -> () {
    let value = native!((), 0x2B694AFCF64E6994, native_parameters!(ped, p1));

    value
}

pub fn clear_ped_wetness(ped: i32) -> () {
    let value = native!((), 0x9C720776DAA43E7E, native_parameters!(ped));

    value
}

pub fn set_ped_wetness_height(ped: i32, height: f32) -> () {
    let value = native!((), 0x44CB6447D2571AA0, native_parameters!(ped, height));

    value
}

pub fn set_ped_wetness_enabled_this_frame(ped: i32) -> () {
    let value = native!((), 0xB5485E4907B53019, native_parameters!(ped));

    value
}

pub fn clear_ped_env_dirt(ped: i32) -> () {
    let value = native!((), 0x6585D955A68452A5, native_parameters!(ped));

    value
}

pub fn set_ped_sweat(ped: i32, sweat: f32) -> () {
    let value = native!((), 0x27B0405F59637D1F, native_parameters!(ped, sweat));

    value
}

pub fn add_ped_decoration_from_hashes(ped: i32, collection: u32, overlay: u32) -> () {
    let value = native!(
        (),
        0x5F5D1665E352A839,
        native_parameters!(ped, collection, overlay)
    );

    value
}

pub fn add_ped_decoration_from_hashes_in_corona(ped: i32, collection: u32, overlay: u32) -> () {
    let value = native!(
        (),
        0x5619BFA07CFD7833,
        native_parameters!(ped, collection, overlay)
    );

    value
}

pub fn get_ped_decoration_zone_from_hashes(collection: u32, overlay: u32) -> i32 {
    let value = native!(
        i32,
        0x9FD452BFBE7A7A8B,
        native_parameters!(collection, overlay)
    );

    value
}

pub fn clear_ped_decorations(ped: i32) -> () {
    let value = native!((), 0x0E5173C163976E38, native_parameters!(ped));

    value
}

pub fn clear_ped_decorations_leave_scars(ped: i32) -> () {
    let value = native!((), 0xE3B27E70CEAB9F0C, native_parameters!(ped));

    value
}

pub fn was_ped_skeleton_updated(ped: i32) -> bool {
    let value = native!(bool, 0x11B499C1E0FF8559, native_parameters!(ped));

    value
}

pub fn get_ped_bone_coords(
    ped: i32,
    boneId: i32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x17C07FC640E86B4E,
        native_parameters!(ped, boneId, offsetX, offsetY, offsetZ)
    );

    value
}

pub fn create_nm_message(startImmediately: bool, messageId: i32) -> () {
    let value = native!(
        (),
        0x418EF2A1BCE56685,
        native_parameters!(startImmediately, messageId)
    );

    value
}

pub fn give_ped_nm_message(ped: i32) -> () {
    let value = native!((), 0xB158DFCCC56E5C5B, native_parameters!(ped));

    value
}

pub fn add_scenario_blocking_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    p6: bool,
    p7: bool,
    p8: bool,
    p9: bool,
) -> i32 {
    let value = native!(
        i32,
        0x1B5C85C612E5256E,
        native_parameters!(x1, y1, z1, x2, y2, z2, p6, p7, p8, p9)
    );

    value
}

pub fn remove_scenario_blocking_areas() -> () {
    let value = native!((), 0xD37401D78A929A49, native_parameters!());

    value
}

pub fn remove_scenario_blocking_area(p0: u32, p1: bool) -> () {
    let value = native!((), 0x31D16B74C6E29D66, native_parameters!(p0, p1));

    value
}

pub fn set_scenario_peds_spawn_in_sphere_area(x: f32, y: f32, z: f32, range: f32, p4: i32) -> () {
    let value = native!(
        (),
        0x28157D43CF600981,
        native_parameters!(x, y, z, range, p4)
    );

    value
}

pub fn _does_scenario_blocking_area_exist(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
) -> bool {
    let value = native!(
        bool,
        0x8A24B067D175A7BD,
        native_parameters!(x1, y1, z1, x2, y2, z2)
    );

    value
}

pub fn is_ped_using_scenario(ped: i32, scenario: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x1BF094736DD62C2E,
        native_parameters!(ped, scenario.as_ptr())
    );

    value
}

pub fn is_ped_using_any_scenario(ped: i32) -> bool {
    let value = native!(bool, 0x57AB4A3080F85143, native_parameters!(ped));

    value
}

pub fn set_ped_panic_exit_scenario(p0: u32, p1: u32, p2: u32, p3: u32) -> u32 {
    let value = native!(u32, 0xFE07FF6495D52E2A, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x9a77dfd295e29b09(p0: u32, p1: bool) -> () {
    let value = native!((), 0x9A77DFD295E29B09, native_parameters!(p0, p1));

    value
}

pub fn _0x25361a96e0f7e419(p0: u32, p1: u32, p2: u32, p3: u32) -> u32 {
    let value = native!(u32, 0x25361A96E0F7E419, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xec6935ebe0847b90(p0: u32, p1: u32, p2: u32, p3: u32) -> u32 {
    let value = native!(u32, 0xEC6935EBE0847B90, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn set_ped_should_play_normal_scenario_exit(ped: i32) -> () {
    let value = native!((), 0xA3A9299C4F2ADB98, native_parameters!(ped));

    value
}

pub fn set_ped_should_play_immediate_scenario_exit(ped: i32) -> () {
    let value = native!((), 0xF1C03A5352243A30, native_parameters!(ped));

    value
}

pub fn set_ped_should_play_flee_scenario_exit(ped: i32, p1: u32, p2: u32, p3: u32) -> u32 {
    let value = native!(u32, 0xEEED8FAFEC331A70, native_parameters!(ped, p1, p2, p3));

    value
}

pub fn _0x425aecf167663f48(ped: i32, p1: bool) -> () {
    let value = native!((), 0x425AECF167663F48, native_parameters!(ped, p1));

    value
}

pub fn _0x5b6010b3cbc29095(p0: u32, p1: bool) -> () {
    let value = native!((), 0x5B6010B3CBC29095, native_parameters!(p0, p1));

    value
}

pub fn _0xceda60a74219d064(p0: u32, p1: bool) -> () {
    let value = native!((), 0xCEDA60A74219D064, native_parameters!(p0, p1));

    value
}

pub fn _0xc30bdaee47256c13(p0: u32) -> u32 {
    let value = native!(u32, 0xC30BDAEE47256C13, native_parameters!(p0));

    value
}

pub fn play_facial_anim(
    ped: i32,
    animName: &std::ffi::CString,
    animDict: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0xE1E65CA8AC9C00ED,
        native_parameters!(ped, animName.as_ptr(), animDict.as_ptr())
    );

    value
}

pub fn _set_facial_clipset_override(ped: i32, animDict: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x5687C7F05B39E401,
        native_parameters!(ped, animDict.as_ptr())
    );

    value
}

pub fn set_facial_idle_anim_override(
    ped: i32,
    animName: &std::ffi::CString,
    animDict: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0xFFC24B988B938B38,
        native_parameters!(ped, animName.as_ptr(), animDict.as_ptr())
    );

    value
}

pub fn clear_facial_idle_anim_override(ped: i32) -> () {
    let value = native!((), 0x726256CC1EEB182F, native_parameters!(ped));

    value
}

pub fn set_ped_can_play_gesture_anims(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xBAF20C5432058024, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_can_play_viseme_anims(ped: i32, toggle: bool, p2: bool) -> () {
    let value = native!((), 0xF833DDBA3B104D43, native_parameters!(ped, toggle, p2));

    value
}

pub fn _set_ped_can_play_injured_anims(ped: i32, p1: bool) -> () {
    let value = native!((), 0x33A60D8BDD6E508C, native_parameters!(ped, p1));

    value
}

pub fn set_ped_can_play_ambient_anims(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x6373D1349925A70E, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_can_play_ambient_base_anims(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x0EB0585D15254740, native_parameters!(ped, toggle));

    value
}

pub fn _0xc2ee020f5fb4db53(ped: i32) -> () {
    let value = native!((), 0xC2EE020F5FB4DB53, native_parameters!(ped));

    value
}

pub fn set_ped_can_arm_ik(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x6C3B4D6D13B4C841, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_can_head_ik(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xC11C18092C5530DC, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_can_leg_ik(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x73518ECE2485412B, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_can_torso_ik(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xF2B7106D37947CE0, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_can_torso_react_ik(ped: i32, p1: bool) -> () {
    let value = native!((), 0xF5846EDB26A98A24, native_parameters!(ped, p1));

    value
}

pub fn _0x6647c5f6f5792496(ped: i32, p1: bool) -> () {
    let value = native!((), 0x6647C5F6F5792496, native_parameters!(ped, p1));

    value
}

pub fn set_ped_can_use_auto_conversation_lookat(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xEC4686EC06434678, native_parameters!(ped, toggle));

    value
}

pub fn is_ped_headtracking_ped(ped1: i32, ped2: i32) -> bool {
    let value = native!(bool, 0x5CD3CB88A7F8850D, native_parameters!(ped1, ped2));

    value
}

pub fn is_ped_headtracking_entity(ped: i32, entity: i32) -> bool {
    let value = native!(bool, 0x813A0A7C9D2E831F, native_parameters!(ped, entity));

    value
}

pub fn set_ped_primary_lookat(ped: i32, lookAt: i32) -> () {
    let value = native!((), 0xCD17B554996A8D9E, native_parameters!(ped, lookAt));

    value
}

pub fn set_ped_cloth_package_index(p0: u32, p1: u32) -> () {
    let value = native!((), 0x78C4E9961DB3EB5B, native_parameters!(p0, p1));

    value
}

pub fn set_ped_cloth_prone(p0: u32, p1: u32) -> () {
    let value = native!((), 0x82A3D6D9CC2CB8E3, native_parameters!(p0, p1));

    value
}

pub fn _0xa660faf550eb37e5(p0: u32, p1: bool) -> () {
    let value = native!((), 0xA660FAF550EB37E5, native_parameters!(p0, p1));

    value
}

pub fn set_ped_config_flag(ped: i32, flagId: i32, value: bool) -> () {
    let value = native!(
        (),
        0x1913FE4CBF41C463,
        native_parameters!(ped, flagId, value)
    );

    value
}

pub fn set_ped_reset_flag(ped: i32, flagId: i32, doReset: bool) -> () {
    let value = native!(
        (),
        0xC1E8A365BF3B29F2,
        native_parameters!(ped, flagId, doReset)
    );

    value
}

pub fn get_ped_config_flag(ped: i32, flagId: i32, p2: bool) -> bool {
    let value = native!(
        bool,
        0x7EE53118C892B513,
        native_parameters!(ped, flagId, p2)
    );

    value
}

pub fn get_ped_reset_flag(ped: i32, flagId: i32) -> bool {
    let value = native!(bool, 0xAF9E59B1B1FBF2A0, native_parameters!(ped, flagId));

    value
}

pub fn set_ped_group_member_passenger_index(ped: i32, index: i32) -> () {
    let value = native!((), 0x0BDDB8D9EC6BCF3C, native_parameters!(ped, index));

    value
}

pub fn set_ped_can_evasive_dive(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x6B7A646C242A7059, native_parameters!(ped, toggle));

    value
}

pub fn is_ped_evasive_diving(ped: i32, evadingEntity: *mut i32) -> bool {
    let value = native!(
        bool,
        0x414641C26E105898,
        native_parameters!(ped, evadingEntity)
    );

    value
}

pub fn set_ped_shoots_at_coord(ped: i32, x: f32, y: f32, z: f32, toggle: bool) -> () {
    let value = native!(
        (),
        0x96A05E4FB321B1BA,
        native_parameters!(ped, x, y, z, toggle)
    );

    value
}

pub fn set_ped_model_is_suppressed(modelHash: u32, toggle: bool) -> () {
    let value = native!(
        (),
        0xE163A4BCE4DE6F11,
        native_parameters!(modelHash, toggle)
    );

    value
}

pub fn stop_any_ped_model_being_suppressed() -> () {
    let value = native!((), 0xB47BD05FA66B40CF, native_parameters!());

    value
}

pub fn set_ped_can_be_targeted_when_injured(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x638C03B0F9878F57, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_generates_dead_body_events(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x7FB17BA2E7DECA5B, native_parameters!(ped, toggle));

    value
}

pub fn _block_ped_dead_body_shocking_events(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xE43A13C9E4CCCBCF, native_parameters!(ped, toggle));

    value
}

pub fn _0x3e9679c1dfcf422c(p0: u32, p1: u32) -> () {
    let value = native!((), 0x3E9679C1DFCF422C, native_parameters!(p0, p1));

    value
}

pub fn set_ped_can_ragdoll_from_player_impact(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xDF993EE5E90ABA25, native_parameters!(ped, toggle));

    value
}

pub fn give_ped_helmet(ped: i32, cannotRemove: bool, helmetFlag: i32, textureIndex: i32) -> () {
    let value = native!(
        (),
        0x54C7C4A94367717E,
        native_parameters!(ped, cannotRemove, helmetFlag, textureIndex)
    );

    value
}

pub fn remove_ped_helmet(ped: i32, instantly: bool) -> () {
    let value = native!((), 0xA7B2458D0AD6DED8, native_parameters!(ped, instantly));

    value
}

pub fn is_ped_taking_off_helmet(ped: i32) -> bool {
    let value = native!(bool, 0x14590DDBEDB1EC85, native_parameters!(ped));

    value
}

pub fn set_ped_helmet(ped: i32, canWearHelmet: bool) -> () {
    let value = native!(
        (),
        0x560A43136EB58105,
        native_parameters!(ped, canWearHelmet)
    );

    value
}

pub fn set_ped_helmet_flag(ped: i32, helmetFlag: i32) -> () {
    let value = native!((), 0xC0E78D5C2CE3EB25, native_parameters!(ped, helmetFlag));

    value
}

pub fn set_ped_helmet_prop_index(ped: i32, propIndex: i32, p2: bool) -> () {
    let value = native!(
        (),
        0x26D83693ED99291C,
        native_parameters!(ped, propIndex, p2)
    );

    value
}

pub fn _set_ped_helmet_unk(ped: i32, p1: bool, p2: i32, p3: i32) -> () {
    let value = native!((), 0x3F7325574E41B44D, native_parameters!(ped, p1, p2, p3));

    value
}

pub fn _is_ped_helmet_unk(ped: i32) -> bool {
    let value = native!(bool, 0xB9496CE47546DB2C, native_parameters!(ped));

    value
}

pub fn set_ped_helmet_texture_index(ped: i32, textureIndex: i32) -> () {
    let value = native!(
        (),
        0xF1550C4BD22582E2,
        native_parameters!(ped, textureIndex)
    );

    value
}

pub fn is_ped_wearing_helmet(ped: i32) -> bool {
    let value = native!(bool, 0xF33BDFE19B309B19, native_parameters!(ped));

    value
}

pub fn clear_ped_stored_hat_prop(ped: i32) -> () {
    let value = native!((), 0x687C0B594907D2E8, native_parameters!(ped));

    value
}

pub fn get_ped_helmet_stored_hat_prop_index(ped: i32) -> u32 {
    let value = native!(u32, 0x451294E859ECC018, native_parameters!(ped));

    value
}

pub fn get_ped_helmet_stored_hat_tex_index(ped: i32) -> u32 {
    let value = native!(u32, 0x9D728C1E12BF5518, native_parameters!(ped));

    value
}

pub fn _0xf2385935bffd4d92(p0: u32) -> bool {
    let value = native!(bool, 0xF2385935BFFD4D92, native_parameters!(p0));

    value
}

pub fn set_ped_to_load_cover(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x332B562EEDA62399, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_can_cower_in_cover(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xCB7553CDCEF4A735, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_can_peek_in_cover(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xC514825C507E3736, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_plays_head_on_horn_anim_when_dies_in_vehicle(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x94D94BF1A75AED3D, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_leg_ik_mode(ped: i32, mode: i32) -> () {
    let value = native!((), 0xC396F5B86FF9FEBD, native_parameters!(ped, mode));

    value
}

pub fn set_ped_motion_blur(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x0A986918B102B448, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_can_switch_weapon(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xED7F7EFE9FABF340, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_dies_instantly_in_water(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xEEB64139BA29A7CF, native_parameters!(ped, toggle));

    value
}

pub fn _0x1a330d297aac6bc1(ped: i32, p1: i32) -> () {
    let value = native!((), 0x1A330D297AAC6BC1, native_parameters!(ped, p1));

    value
}

pub fn stop_ped_weapon_firing_when_dropped(ped: i32) -> () {
    let value = native!((), 0xC158D28142A34608, native_parameters!(ped));

    value
}

pub fn set_scripted_anim_seat_offset(ped: i32, p1: f32) -> () {
    let value = native!((), 0x5917BBA32D06C230, native_parameters!(ped, p1));

    value
}

pub fn set_ped_combat_movement(ped: i32, combatMovement: i32) -> () {
    let value = native!(
        (),
        0x4D9CA1009AFBD057,
        native_parameters!(ped, combatMovement)
    );

    value
}

pub fn get_ped_combat_movement(ped: i32) -> i32 {
    let value = native!(i32, 0xDEA92412FCAEB3F5, native_parameters!(ped));

    value
}

pub fn set_ped_combat_ability(ped: i32, p1: i32) -> () {
    let value = native!((), 0xC7622C0D36B2FDA8, native_parameters!(ped, p1));

    value
}

pub fn set_ped_combat_range(ped: i32, p1: i32) -> () {
    let value = native!((), 0x3C606747B23E497B, native_parameters!(ped, p1));

    value
}

pub fn get_ped_combat_range(ped: i32) -> i32 {
    let value = native!(i32, 0xF9D9F7F2DB8E2FA0, native_parameters!(ped));

    value
}

pub fn set_ped_combat_attributes(ped: i32, attributeIndex: i32, enabled: bool) -> () {
    let value = native!(
        (),
        0x9F7794730795E019,
        native_parameters!(ped, attributeIndex, enabled)
    );

    value
}

pub fn set_ped_target_loss_response(ped: i32, responseType: i32) -> () {
    let value = native!(
        (),
        0x0703B9079823DA4A,
        native_parameters!(ped, responseType)
    );

    value
}

pub fn is_ped_performing_melee_action(ped: i32) -> bool {
    let value = native!(bool, 0xDCCA191DF9980FD7, native_parameters!(ped));

    value
}

pub fn is_ped_performing_stealth_kill(ped: i32) -> bool {
    let value = native!(bool, 0xFD4CCDBCC59941B7, native_parameters!(ped));

    value
}

pub fn is_ped_performing_dependent_combo_limit(ped: i32) -> bool {
    let value = native!(bool, 0xEBD0EDBA5BE957CF, native_parameters!(ped));

    value
}

pub fn is_ped_being_stealth_killed(ped: i32) -> bool {
    let value = native!(bool, 0x863B23EFDE9C5DF2, native_parameters!(ped));

    value
}

pub fn get_melee_target_for_ped(ped: i32) -> i32 {
    let value = native!(i32, 0x18A3E9EE1297FD39, native_parameters!(ped));

    value
}

pub fn was_ped_killed_by_stealth(ped: i32) -> bool {
    let value = native!(bool, 0xF9800AA1A771B000, native_parameters!(ped));

    value
}

pub fn was_ped_killed_by_takedown(ped: i32) -> bool {
    let value = native!(bool, 0x7F08E26039C7347C, native_parameters!(ped));

    value
}

pub fn was_ped_knocked_out(ped: i32) -> bool {
    let value = native!(bool, 0x61767F73EACEED21, native_parameters!(ped));

    value
}

pub fn set_ped_flee_attributes(ped: i32, attributeFlags: i32, enable: bool) -> () {
    let value = native!(
        (),
        0x70A2D1137C8ED7C9,
        native_parameters!(ped, attributeFlags, enable)
    );

    value
}

pub fn set_ped_cower_hash(ped: i32, p1: &std::ffi::CString) -> () {
    let value = native!((), 0xA549131166868ED3, native_parameters!(ped, p1.as_ptr()));

    value
}

pub fn _0x2016c603d6b8987c(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x2016C603D6B8987C, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_steers_around_peds(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x46F2193B3AD1D891, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_steers_around_objects(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x1509C089ADC208BF, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_steers_around_vehicles(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xEB6FB9D48DDE23EC, native_parameters!(ped, toggle));

    value
}

pub fn _0xa9b61a329bfdcbea(p0: u32, p1: bool) -> () {
    let value = native!((), 0xA9B61A329BFDCBEA, native_parameters!(p0, p1));

    value
}

pub fn set_ped_increased_avoidance_radius(ped: i32) -> () {
    let value = native!((), 0x570389D1C3DE3C6B, native_parameters!(ped));

    value
}

pub fn set_ped_blocks_pathing_when_dead(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x576594E8D64375E2, native_parameters!(ped, toggle));

    value
}

pub fn _0xa52d5247a4227e14(p0: u32) -> () {
    let value = native!((), 0xA52D5247A4227E14, native_parameters!(p0));

    value
}

pub fn is_any_ped_near_point(x: f32, y: f32, z: f32, radius: f32) -> bool {
    let value = native!(
        bool,
        0x083961498679DC9F,
        native_parameters!(x, y, z, radius)
    );

    value
}

pub fn force_ped_ai_and_animation_update(ped: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x2208438012482A1A, native_parameters!(ped, p1, p2));

    value
}

pub fn is_ped_heading_towards_position(ped: i32, x: f32, y: f32, z: f32, p4: f32) -> bool {
    let value = native!(
        bool,
        0xFCF37A457CB96DC0,
        native_parameters!(ped, x, y, z, p4)
    );

    value
}

pub fn request_ped_visibility_tracking(ped: i32) -> () {
    let value = native!((), 0x7D7A2E43E74E2EB8, native_parameters!(ped));

    value
}

pub fn request_ped_vehicle_visibility_tracking(ped: i32, p1: bool) -> () {
    let value = native!((), 0x2BC338A7B21F4608, native_parameters!(ped, p1));

    value
}

pub fn _0xcd018c591f94cb43(ped: i32, p1: bool) -> () {
    let value = native!((), 0xCD018C591F94CB43, native_parameters!(ped, p1));

    value
}

pub fn _0x75ba1cb3b7d40caf(ped: i32, p1: bool) -> () {
    let value = native!((), 0x75BA1CB3B7D40CAF, native_parameters!(ped, p1));

    value
}

pub fn is_tracked_ped_visible(ped: i32) -> bool {
    let value = native!(bool, 0x91C8E617F64188AC, native_parameters!(ped));

    value
}

pub fn _0x511f1a683387c7e2(ped: i32) -> i32 {
    let value = native!(i32, 0x511F1A683387C7E2, native_parameters!(ped));

    value
}

pub fn is_ped_tracked(ped: i32) -> bool {
    let value = native!(bool, 0x4C5E1F087CD10BB7, native_parameters!(ped));

    value
}

pub fn has_ped_received_event(ped: i32, eventId: i32) -> bool {
    let value = native!(bool, 0x8507BCB710FA6DC0, native_parameters!(ped, eventId));

    value
}

pub fn can_ped_see_hated_ped(ped1: i32, ped2: i32) -> bool {
    let value = native!(bool, 0x6CD5A433374D4CFB, native_parameters!(ped1, ped2));

    value
}

pub fn _0x9c6a6c19b6c0c496(ped: i32, p1: *mut i32) -> bool {
    let value = native!(bool, 0x9C6A6C19B6C0C496, native_parameters!(ped, p1));

    value
}

pub fn _0x2dfc81c9b9608549(ped: i32, p1: *mut i32) -> bool {
    let value = native!(bool, 0x2DFC81C9B9608549, native_parameters!(ped, p1));

    value
}

pub fn get_ped_bone_index(ped: i32, boneId: i32) -> i32 {
    let value = native!(i32, 0x3F428D08BE5AAE31, native_parameters!(ped, boneId));

    value
}

pub fn get_ped_ragdoll_bone_index(ped: i32, bone: i32) -> i32 {
    let value = native!(i32, 0x2057EF813397A772, native_parameters!(ped, bone));

    value
}

pub fn set_ped_enveff_scale(ped: i32, value: f32) -> () {
    let value = native!((), 0xBF29516833893561, native_parameters!(ped, value));

    value
}

pub fn get_ped_enveff_scale(ped: i32) -> f32 {
    let value = native!(f32, 0x9C14D30395A51A3C, native_parameters!(ped));

    value
}

pub fn set_enable_ped_enveff_scale(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xD2C5AA0C0E8D0F1E, native_parameters!(ped, toggle));

    value
}

pub fn _0x110f526ab784111f(ped: i32, p1: f32) -> () {
    let value = native!((), 0x110F526AB784111F, native_parameters!(ped, p1));

    value
}

pub fn set_ped_enveff_color_modulator(ped: i32, p1: i32, p2: i32, p3: i32) -> () {
    let value = native!((), 0xD69411AA0CEBF9E9, native_parameters!(ped, p1, p2, p3));

    value
}

pub fn _set_ped_emissive_intensity(ped: i32, intensity: f32) -> () {
    let value = native!((), 0x4E90D746056E273D, native_parameters!(ped, intensity));

    value
}

pub fn _get_ped_emissive_intensity(ped: i32) -> f32 {
    let value = native!(f32, 0x1461B28A06717D68, native_parameters!(ped));

    value
}

pub fn _is_ped_shader_effect_valid(ped: i32) -> bool {
    let value = native!(bool, 0x81AA517FBBA05D39, native_parameters!(ped));

    value
}

pub fn _0xe906ec930f5fe7c8(p0: u32, p1: u32) -> () {
    let value = native!((), 0xE906EC930F5FE7C8, native_parameters!(p0, p1));

    value
}

pub fn _0x1216e0bfa72cc703(p0: u32, p1: u32) -> () {
    let value = native!((), 0x1216E0BFA72CC703, native_parameters!(p0, p1));

    value
}

pub fn set_ped_ao_blob_rendering(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x2B5AA717A181FB4C, native_parameters!(ped, toggle));

    value
}

pub fn _0xb8b52e498014f5b0(ped: i32) -> bool {
    let value = native!(bool, 0xB8B52E498014F5B0, native_parameters!(ped));

    value
}

pub fn create_synchronized_scene(
    x: f32,
    y: f32,
    z: f32,
    roll: f32,
    pitch: f32,
    yaw: f32,
    p6: i32,
) -> i32 {
    let value = native!(
        i32,
        0x8C18E0F9080ADD73,
        native_parameters!(x, y, z, roll, pitch, yaw, p6)
    );

    value
}

pub fn _create_synchronized_scene_2(x: f32, y: f32, z: f32, radius: f32, object: u32) -> i32 {
    let value = native!(
        i32,
        0x62EC273D00187DCA,
        native_parameters!(x, y, z, radius, object)
    );

    value
}

pub fn is_synchronized_scene_running(sceneId: i32) -> bool {
    let value = native!(bool, 0x25D39B935A038A26, native_parameters!(sceneId));

    value
}

pub fn set_synchronized_scene_origin(
    sceneID: i32,
    x: f32,
    y: f32,
    z: f32,
    roll: f32,
    pitch: f32,
    yaw: f32,
    p7: bool,
) -> () {
    let value = native!(
        (),
        0x6ACF6B7225801CD7,
        native_parameters!(sceneID, x, y, z, roll, pitch, yaw, p7)
    );

    value
}

pub fn set_synchronized_scene_phase(sceneID: i32, phase: f32) -> () {
    let value = native!((), 0x734292F4F0ABF6D0, native_parameters!(sceneID, phase));

    value
}

pub fn get_synchronized_scene_phase(sceneID: i32) -> f32 {
    let value = native!(f32, 0xE4A310B1D7FA73CC, native_parameters!(sceneID));

    value
}

pub fn set_synchronized_scene_rate(sceneID: i32, rate: f32) -> () {
    let value = native!((), 0xB6C49F8A5E295A5D, native_parameters!(sceneID, rate));

    value
}

pub fn get_synchronized_scene_rate(sceneID: i32) -> f32 {
    let value = native!(f32, 0xD80932D577274D40, native_parameters!(sceneID));

    value
}

pub fn set_synchronized_scene_looped(sceneID: i32, toggle: bool) -> () {
    let value = native!((), 0xD9A897A4C6C2974F, native_parameters!(sceneID, toggle));

    value
}

pub fn is_synchronized_scene_looped(sceneID: i32) -> bool {
    let value = native!(bool, 0x62522002E0C391BA, native_parameters!(sceneID));

    value
}

pub fn set_synchronized_scene_hold_last_frame(sceneID: i32, toggle: bool) -> () {
    let value = native!((), 0x394B9CD12435C981, native_parameters!(sceneID, toggle));

    value
}

pub fn is_synchronized_scene_hold_last_frame(sceneID: i32) -> bool {
    let value = native!(bool, 0x7F2F4F13AC5257EF, native_parameters!(sceneID));

    value
}

pub fn attach_synchronized_scene_to_entity(sceneID: i32, entity: i32, boneIndex: i32) -> () {
    let value = native!(
        (),
        0x272E4723B56A3B96,
        native_parameters!(sceneID, entity, boneIndex)
    );

    value
}

pub fn detach_synchronized_scene(sceneID: i32) -> () {
    let value = native!((), 0x6D38F1F04CBB37EA, native_parameters!(sceneID));

    value
}

pub fn _dispose_synchronized_scene(scene: i32) -> () {
    let value = native!((), 0xCD9CC7E200A52A6F, native_parameters!(scene));

    value
}

pub fn force_ped_motion_state(ped: i32, motionStateHash: u32, p2: bool, p3: i32, p4: bool) -> bool {
    let value = native!(
        bool,
        0xF28965D04F570DCA,
        native_parameters!(ped, motionStateHash, p2, p3, p4)
    );

    value
}

pub fn _get_ped_current_movement_speed(ped: i32, speedX: *mut f32, speedY: *mut f32) -> bool {
    let value = native!(
        bool,
        0xF60165E1D2C5370B,
        native_parameters!(ped, speedX, speedY)
    );

    value
}

pub fn set_ped_max_move_blend_ratio(ped: i32, value: f32) -> () {
    let value = native!((), 0x433083750C5E064A, native_parameters!(ped, value));

    value
}

pub fn set_ped_min_move_blend_ratio(ped: i32, value: f32) -> () {
    let value = native!((), 0x01A898D26E2333DD, native_parameters!(ped, value));

    value
}

pub fn set_ped_move_rate_override(ped: i32, value: f32) -> () {
    let value = native!((), 0x085BF80FA50A39D1, native_parameters!(ped, value));

    value
}

pub fn _0x0b3e35ac043707d9(p0: u32, p1: u32) -> () {
    let value = native!((), 0x0B3E35AC043707D9, native_parameters!(p0, p1));

    value
}

pub fn _0x46b05bcae43856b0(ped: i32, flag: i32) -> bool {
    let value = native!(bool, 0x46B05BCAE43856B0, native_parameters!(ped, flag));

    value
}

pub fn get_ped_nearby_vehicles(ped: i32, sizeAndVehs: *mut i32) -> i32 {
    let value = native!(
        i32,
        0xCFF869CBFA210D82,
        native_parameters!(ped, sizeAndVehs)
    );

    value
}

pub fn get_ped_nearby_peds(ped: i32, sizeAndPeds: *mut i32, ignore: i32) -> i32 {
    let value = native!(
        i32,
        0x23F8F5FC7E8C4A6B,
        native_parameters!(ped, sizeAndPeds, ignore)
    );

    value
}

pub fn have_all_streaming_requests_completed(ped: i32) -> bool {
    let value = native!(bool, 0x7350823473013C02, native_parameters!(ped));

    value
}

pub fn is_ped_using_action_mode(ped: i32) -> bool {
    let value = native!(bool, 0x00E73468D085F745, native_parameters!(ped));

    value
}

pub fn set_ped_using_action_mode(ped: i32, p1: bool, p2: i32, action: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xD75ACCF5E0FB5367,
        native_parameters!(ped, p1, p2, action.as_ptr())
    );

    value
}

pub fn set_movement_mode_override(ped: i32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x781DE8FA214E87D2,
        native_parameters!(ped, name.as_ptr())
    );

    value
}

pub fn set_ped_capsule(ped: i32, value: f32) -> () {
    let value = native!((), 0x364DF566EC833DE2, native_parameters!(ped, value));

    value
}

pub fn register_pedheadshot(ped: i32) -> i32 {
    let value = native!(i32, 0x4462658788425076, native_parameters!(ped));

    value
}

pub fn _register_pedheadshot_3(ped: i32) -> i32 {
    let value = native!(i32, 0xBA8805A1108A2515, native_parameters!(ped));

    value
}

pub fn register_pedheadshot_transparent(ped: i32) -> i32 {
    let value = native!(i32, 0x953563CE563143AF, native_parameters!(ped));

    value
}

pub fn unregister_pedheadshot(id: i32) -> () {
    let value = native!((), 0x96B1361D9B24C2FF, native_parameters!(id));

    value
}

pub fn is_pedheadshot_valid(id: i32) -> bool {
    let value = native!(bool, 0xA0A9668F158129A2, native_parameters!(id));

    value
}

pub fn is_pedheadshot_ready(id: i32) -> bool {
    let value = native!(bool, 0x7085228842B13A67, native_parameters!(id));

    value
}

pub fn get_pedheadshot_txd_string(id: i32) -> String {
    let value = native!(*const i8, 0xDB4EACD4AD0A5D6B, native_parameters!(id));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn request_pedheadshot_img_upload(id: i32) -> bool {
    let value = native!(bool, 0xF0DAEF2F545BEE25, native_parameters!(id));

    value
}

pub fn release_pedheadshot_img_upload(id: i32) -> () {
    let value = native!((), 0x5D517B27CF6ECD04, native_parameters!(id));

    value
}

pub fn is_pedheadshot_img_upload_available() -> bool {
    let value = native!(bool, 0xEBB376779A760AA8, native_parameters!());

    value
}

pub fn has_pedheadshot_img_upload_failed() -> bool {
    let value = native!(bool, 0x876928DDDFCCC9CD, native_parameters!());

    value
}

pub fn has_pedheadshot_img_upload_succeeded() -> bool {
    let value = native!(bool, 0xE8A169E666CBC541, native_parameters!());

    value
}

pub fn set_ped_heatscale_override(ped: i32, heatScale: f32) -> () {
    let value = native!((), 0xC1F6EBF9A3D55538, native_parameters!(ped, heatScale));

    value
}

pub fn disable_ped_heatscale_override(ped: i32) -> () {
    let value = native!((), 0x600048C60D5C2C51, native_parameters!(ped));

    value
}

pub fn spawnpoints_start_search(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    interiorFlags: i32,
    scale: f32,
    duration: i32,
) -> () {
    let value = native!(
        (),
        0x2DF9038C90AD5264,
        native_parameters!(p0, p1, p2, p3, p4, interiorFlags, scale, duration)
    );

    value
}

pub fn spawnpoints_start_search_in_angled_area(
    x: f32,
    y: f32,
    z: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    interiorFlags: i32,
    scale: f32,
    duration: i32,
) -> () {
    let value = native!(
        (),
        0xB2AFF10216DEFA2F,
        native_parameters!(x, y, z, p3, p4, p5, p6, interiorFlags, scale, duration)
    );

    value
}

pub fn spawnpoints_cancel_search() -> () {
    let value = native!((), 0xFEE4A5459472A9F8, native_parameters!());

    value
}

pub fn spawnpoints_is_search_active() -> bool {
    let value = native!(bool, 0x3C67506996001F5E, native_parameters!());

    value
}

pub fn spawnpoints_is_search_complete() -> bool {
    let value = native!(bool, 0xA586FBEB32A53DBB, native_parameters!());

    value
}

pub fn spawnpoints_is_search_failed() -> bool {
    let value = native!(bool, 0xF445DE8DA80A1792, native_parameters!());

    value
}

pub fn spawnpoints_get_num_search_results() -> i32 {
    let value = native!(i32, 0xA635C11B8C44AFC2, native_parameters!());

    value
}

pub fn spawnpoints_get_search_result(randomInt: i32, x: *mut f32, y: *mut f32, z: *mut f32) -> () {
    let value = native!(
        (),
        0x280C7E3AC7F56E90,
        native_parameters!(randomInt, x, y, z)
    );

    value
}

pub fn spawnpoints_get_search_result_flags(p0: u32, p1: *mut u32) -> () {
    let value = native!((), 0xB782F8238512BAD5, native_parameters!(p0, p1));

    value
}

pub fn set_ik_target(
    ped: i32,
    ikIndex: i32,
    entityLookAt: i32,
    boneLookAt: i32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    p7: u32,
    blendInDuration: i32,
    blendOutDuration: i32,
) -> () {
    let value = native!(
        (),
        0xC32779C16FCEECD9,
        native_parameters!(
            ped,
            ikIndex,
            entityLookAt,
            boneLookAt,
            offsetX,
            offsetY,
            offsetZ,
            p7,
            blendInDuration,
            blendOutDuration
        )
    );

    value
}

pub fn _0xed3c76adfa6d07c4(ped: i32) -> () {
    let value = native!((), 0xED3C76ADFA6D07C4, native_parameters!(ped));

    value
}

pub fn request_action_mode_asset(asset: &std::ffi::CString) -> () {
    let value = native!((), 0x290E2780BB7AA598, native_parameters!(asset.as_ptr()));

    value
}

pub fn has_action_mode_asset_loaded(asset: &std::ffi::CString) -> bool {
    let value = native!(bool, 0xE4B5F4BF2CB24E65, native_parameters!(asset.as_ptr()));

    value
}

pub fn remove_action_mode_asset(asset: &std::ffi::CString) -> () {
    let value = native!((), 0x13E940F88470FA51, native_parameters!(asset.as_ptr()));

    value
}

pub fn request_stealth_mode_asset(asset: &std::ffi::CString) -> () {
    let value = native!((), 0x2A0A62FCDEE16D4F, native_parameters!(asset.as_ptr()));

    value
}

pub fn has_stealth_mode_asset_loaded(asset: &std::ffi::CString) -> bool {
    let value = native!(bool, 0xE977FC5B08AF3441, native_parameters!(asset.as_ptr()));

    value
}

pub fn remove_stealth_mode_asset(asset: &std::ffi::CString) -> () {
    let value = native!((), 0x9219857D21F0E842, native_parameters!(asset.as_ptr()));

    value
}

pub fn set_ped_lod_multiplier(ped: i32, multiplier: f32) -> () {
    let value = native!((), 0xDC2C5C242AAC342B, native_parameters!(ped, multiplier));

    value
}

pub fn _0xe861d0b05c7662b8(ped: i32, p1: bool, p2: i32) -> () {
    let value = native!((), 0xE861D0B05C7662B8, native_parameters!(ped, p1, p2));

    value
}

pub fn set_force_footstep_update(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x129466ED55140F8D, native_parameters!(ped, toggle));

    value
}

pub fn set_force_step_type(ped: i32, p1: bool, type_esc: i32, p3: i32) -> () {
    let value = native!(
        (),
        0xCB968B53FC7F916D,
        native_parameters!(ped, p1, type_esc, p3)
    );

    value
}

pub fn is_any_hostile_ped_near_point(ped: i32, x: f32, y: f32, z: f32, radius: f32) -> bool {
    let value = native!(
        bool,
        0x68772DB2B2526F9F,
        native_parameters!(ped, x, y, z, radius)
    );

    value
}

pub fn _0x820e9892a77e97cd(p0: u32, p1: u32) -> () {
    let value = native!((), 0x820E9892A77E97CD, native_parameters!(p0, p1));

    value
}

pub fn _0x06087579e7aa85a9(p0: u32, p1: u32, p2: f32, p3: f32, p4: f32, p5: f32) -> bool {
    let value = native!(
        bool,
        0x06087579E7AA85A9,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn set_pop_control_sphere_this_frame(x: f32, y: f32, z: f32, min: f32, max: f32) -> () {
    let value = native!(
        (),
        0xD8C3BE3EE94CAF2D,
        native_parameters!(x, y, z, min, max)
    );

    value
}

pub fn _0xd33daa36272177c4(ped: i32) -> () {
    let value = native!((), 0xD33DAA36272177C4, native_parameters!(ped));

    value
}

pub fn _0x711794453cfd692b(p0: u32, p1: u32) -> () {
    let value = native!((), 0x711794453CFD692B, native_parameters!(p0, p1));

    value
}

pub fn _0x83a169eabcdb10a2(p0: u32, p1: u32) -> () {
    let value = native!((), 0x83A169EABCDB10A2, native_parameters!(p0, p1));

    value
}

pub fn _0x288df530c92dad6f(p0: u32, p1: f32) -> () {
    let value = native!((), 0x288DF530C92DAD6F, native_parameters!(p0, p1));

    value
}

pub fn _is_ped_swapping_weapon(Ped: i32) -> bool {
    let value = native!(bool, 0x3795688A307E1EB6, native_parameters!(Ped));

    value
}

pub fn _0x0f62619393661d6e(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x0F62619393661D6E, native_parameters!(p0, p1, p2));

    value
}

pub fn _0xdfe68c4b787e1bfb(ped: i32) -> () {
    let value = native!((), 0xDFE68C4B787E1BFB, native_parameters!(ped));

    value
}

pub fn _set_enable_scuba_gear_light(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xEE2476B9EE4A094F, native_parameters!(ped, toggle));

    value
}

pub fn _is_scuba_gear_light_enabled(ped: i32) -> bool {
    let value = native!(bool, 0x88274C11CF0D866D, native_parameters!(ped));

    value
}

pub fn _clear_facial_clipset_override(ped: i32) -> () {
    let value = native!((), 0x637822DC2AFEEBF8, native_parameters!(ped));

    value
}

pub fn _0xfab944d4d481accb(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xFAB944D4D481ACCB, native_parameters!(ped, toggle));

    value
}
