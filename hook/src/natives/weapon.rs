use crate::types::NativeVector3;

pub fn enable_laser_sight_rendering(toggle: bool) -> () {
    let value = native!((), 0xC8B46D7727D864AA, native_parameters!(toggle));

    value
}

pub fn get_weapon_component_type_model(componentHash: u32) -> u32 {
    let value = native!(u32, 0x0DB57B41EC1DB083, native_parameters!(componentHash));

    value
}

pub fn get_weapontype_model(weaponHash: u32) -> u32 {
    let value = native!(u32, 0xF46CDC33180FDA94, native_parameters!(weaponHash));

    value
}

pub fn get_weapontype_slot(weaponHash: u32) -> u32 {
    let value = native!(u32, 0x4215460B9B8B7FA0, native_parameters!(weaponHash));

    value
}

pub fn get_weapontype_group(weaponHash: u32) -> u32 {
    let value = native!(u32, 0xC3287EE3050FB74C, native_parameters!(weaponHash));

    value
}

pub fn _get_weapon_component_variant_extra_component_count(componentHash: u32) -> i32 {
    let value = native!(i32, 0x6558AC7C17BFEF58, native_parameters!(componentHash));

    value
}

pub fn _get_weapon_component_variant_extra_component_model(
    componentHash: u32,
    extraComponentIndex: i32,
) -> u32 {
    let value = native!(
        u32,
        0x4D1CB8DC40208A17,
        native_parameters!(componentHash, extraComponentIndex)
    );

    value
}

pub fn set_current_ped_weapon(ped: i32, weaponHash: u32, bForceInHand: bool) -> () {
    let value = native!(
        (),
        0xADF692B254977C0C,
        native_parameters!(ped, weaponHash, bForceInHand)
    );

    value
}

pub fn get_current_ped_weapon(ped: i32, weaponHash: *mut u32, p2: bool) -> bool {
    let value = native!(
        bool,
        0x3A87E44BB9A01D54,
        native_parameters!(ped, weaponHash, p2)
    );

    value
}

pub fn get_current_ped_weapon_entity_index(ped: i32, p1: u32) -> i32 {
    let value = native!(i32, 0x3B390A939AF0B5FC, native_parameters!(ped, p1));

    value
}

pub fn get_best_ped_weapon(ped: i32, p1: bool) -> u32 {
    let value = native!(u32, 0x8483E98E8B888AE2, native_parameters!(ped, p1));

    value
}

pub fn set_current_ped_vehicle_weapon(ped: i32, weaponHash: u32) -> bool {
    let value = native!(
        bool,
        0x75C55983C2C39DAA,
        native_parameters!(ped, weaponHash)
    );

    value
}

pub fn get_current_ped_vehicle_weapon(ped: i32, weaponHash: *mut u32) -> bool {
    let value = native!(
        bool,
        0x1017582BCD3832DC,
        native_parameters!(ped, weaponHash)
    );

    value
}

pub fn _0x50276ef8172f5f12(ped: i32) -> () {
    let value = native!((), 0x50276EF8172F5F12, native_parameters!(ped));

    value
}

pub fn is_ped_armed(ped: i32, typeFlags: i32) -> bool {
    let value = native!(bool, 0x475768A975D5AD17, native_parameters!(ped, typeFlags));

    value
}

pub fn is_weapon_valid(weaponHash: u32) -> bool {
    let value = native!(bool, 0x937C71165CF334B3, native_parameters!(weaponHash));

    value
}

pub fn has_ped_got_weapon(ped: i32, weaponHash: u32, p2: bool) -> bool {
    let value = native!(
        bool,
        0x8DECB02F88F428BC,
        native_parameters!(ped, weaponHash, p2)
    );

    value
}

pub fn is_ped_weapon_ready_to_shoot(ped: i32) -> bool {
    let value = native!(bool, 0xB80CA294F2F26749, native_parameters!(ped));

    value
}

pub fn get_ped_weapontype_in_slot(ped: i32, weaponSlot: u32) -> u32 {
    let value = native!(u32, 0xEFFED78E9011134D, native_parameters!(ped, weaponSlot));

    value
}

pub fn get_ammo_in_ped_weapon(ped: i32, weaponhash: u32) -> i32 {
    let value = native!(i32, 0x015A522136D7F951, native_parameters!(ped, weaponhash));

    value
}

pub fn add_ammo_to_ped(ped: i32, weaponHash: u32, ammo: i32) -> () {
    let value = native!(
        (),
        0x78F0424C34306220,
        native_parameters!(ped, weaponHash, ammo)
    );

    value
}

pub fn set_ped_ammo(ped: i32, weaponHash: u32, ammo: i32, p3: bool) -> () {
    let value = native!(
        (),
        0x14E56BC5B5DB6A19,
        native_parameters!(ped, weaponHash, ammo, p3)
    );

    value
}

pub fn set_ped_infinite_ammo(ped: i32, toggle: bool, weaponHash: u32) -> () {
    let value = native!(
        (),
        0x3EDCB0505123623B,
        native_parameters!(ped, toggle, weaponHash)
    );

    value
}

pub fn set_ped_infinite_ammo_clip(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x183DADC6AA953186, native_parameters!(ped, toggle));

    value
}

pub fn _0x24c024ba8379a70a(p0: u32, p1: u32) -> () {
    let value = native!((), 0x24C024BA8379A70A, native_parameters!(p0, p1));

    value
}

pub fn give_weapon_to_ped(
    ped: i32,
    weaponHash: u32,
    ammoCount: i32,
    isHidden: bool,
    bForceInHand: bool,
) -> () {
    let value = native!(
        (),
        0xBF0FD6E56C964FCB,
        native_parameters!(ped, weaponHash, ammoCount, isHidden, bForceInHand)
    );

    value
}

pub fn give_delayed_weapon_to_ped(
    ped: i32,
    weaponHash: u32,
    ammoCount: i32,
    bForceInHand: bool,
) -> () {
    let value = native!(
        (),
        0xB282DC6EBD803C75,
        native_parameters!(ped, weaponHash, ammoCount, bForceInHand)
    );

    value
}

pub fn remove_all_ped_weapons(ped: i32, p1: bool) -> () {
    let value = native!((), 0xF25DF915FA38C5F3, native_parameters!(ped, p1));

    value
}

pub fn remove_weapon_from_ped(ped: i32, weaponHash: u32) -> () {
    let value = native!((), 0x4899CB088EDF59B8, native_parameters!(ped, weaponHash));

    value
}

pub fn hide_ped_weapon_for_scripted_cutscene(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x6F6981D2253C208F, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_current_weapon_visible(
    ped: i32,
    visible: bool,
    deselectWeapon: bool,
    p3: bool,
    p4: bool,
) -> () {
    let value = native!(
        (),
        0x0725A4CCFDED9A70,
        native_parameters!(ped, visible, deselectWeapon, p3, p4)
    );

    value
}

pub fn set_ped_drops_weapons_when_dead(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x476AE72C1D19D1A8, native_parameters!(ped, toggle));

    value
}

pub fn has_ped_been_damaged_by_weapon(ped: i32, weaponHash: u32, weaponType: i32) -> bool {
    let value = native!(
        bool,
        0x2D343D2219CD027A,
        native_parameters!(ped, weaponHash, weaponType)
    );

    value
}

pub fn clear_ped_last_weapon_damage(ped: i32) -> () {
    let value = native!((), 0x0E98F88A24C5F4B8, native_parameters!(ped));

    value
}

pub fn has_entity_been_damaged_by_weapon(entity: i32, weaponHash: u32, weaponType: i32) -> bool {
    let value = native!(
        bool,
        0x131D401334815E94,
        native_parameters!(entity, weaponHash, weaponType)
    );

    value
}

pub fn clear_entity_last_weapon_damage(entity: i32) -> () {
    let value = native!((), 0xAC678E40BE7C74D2, native_parameters!(entity));

    value
}

pub fn set_ped_drops_weapon(ped: i32) -> () {
    let value = native!((), 0x6B7513D9966FBEC0, native_parameters!(ped));

    value
}

pub fn set_ped_drops_inventory_weapon(
    ped: i32,
    weaponHash: u32,
    xOffset: f32,
    yOffset: f32,
    zOffset: f32,
    ammoCount: i32,
) -> () {
    let value = native!(
        (),
        0x208A1888007FC0E6,
        native_parameters!(ped, weaponHash, xOffset, yOffset, zOffset, ammoCount)
    );

    value
}

pub fn get_max_ammo_in_clip(ped: i32, weaponHash: u32, p2: bool) -> i32 {
    let value = native!(
        i32,
        0xA38DCFFCEA8962FA,
        native_parameters!(ped, weaponHash, p2)
    );

    value
}

pub fn get_ammo_in_clip(ped: i32, weaponHash: u32, ammo: *mut i32) -> bool {
    let value = native!(
        bool,
        0x2E1202248937775C,
        native_parameters!(ped, weaponHash, ammo)
    );

    value
}

pub fn set_ammo_in_clip(ped: i32, weaponHash: u32, ammo: i32) -> bool {
    let value = native!(
        bool,
        0xDCD2A934D65CB497,
        native_parameters!(ped, weaponHash, ammo)
    );

    value
}

pub fn get_max_ammo(ped: i32, weaponHash: u32, ammo: *mut i32) -> bool {
    let value = native!(
        bool,
        0xDC16122C7A20C933,
        native_parameters!(ped, weaponHash, ammo)
    );

    value
}

pub fn _get_max_ammo_by_type(ped: i32, ammoTypeHash: u32, ammo: *mut i32) -> bool {
    let value = native!(
        bool,
        0x585847C5E4E11709,
        native_parameters!(ped, ammoTypeHash, ammo)
    );

    value
}

pub fn _add_ammo_to_ped_by_type(ped: i32, ammoTypeHash: u32, ammo: i32) -> () {
    let value = native!(
        (),
        0x2472622CE1F2D45F,
        native_parameters!(ped, ammoTypeHash, ammo)
    );

    value
}

pub fn set_ped_ammo_by_type(ped: i32, ammoTypeHash: u32, ammo: i32) -> () {
    let value = native!(
        (),
        0x5FD1E1F011E76D7E,
        native_parameters!(ped, ammoTypeHash, ammo)
    );

    value
}

pub fn get_ped_ammo_by_type(ped: i32, ammoTypeHash: u32) -> i32 {
    let value = native!(
        i32,
        0x39D22031557946C1,
        native_parameters!(ped, ammoTypeHash)
    );

    value
}

pub fn set_ped_ammo_to_drop(ped: i32, p1: i32) -> () {
    let value = native!((), 0xA4EFEF9440A5B0EF, native_parameters!(ped, p1));

    value
}

pub fn set_pickup_ammo_amount_scaler(p0: f32) -> () {
    let value = native!((), 0xE620FD3512A04F18, native_parameters!(p0));

    value
}

pub fn get_ped_ammo_type_from_weapon(ped: i32, weaponHash: u32) -> u32 {
    let value = native!(u32, 0x7FEAD38B326B9F74, native_parameters!(ped, weaponHash));

    value
}

pub fn _get_ped_ammo_type_from_weapon_2(ped: i32, weaponHash: u32) -> u32 {
    let value = native!(u32, 0xF489B44DD5AF4BD9, native_parameters!(ped, weaponHash));

    value
}

pub fn get_ped_last_weapon_impact_coord(ped: i32, coords: *mut NativeVector3) -> bool {
    let value = native!(bool, 0x6C4D0409BA1A2BC2, native_parameters!(ped, coords));

    value
}

pub fn set_ped_gadget(ped: i32, gadgetHash: u32, p2: bool) -> () {
    let value = native!(
        (),
        0xD0D7B1E680ED4A1A,
        native_parameters!(ped, gadgetHash, p2)
    );

    value
}

pub fn get_is_ped_gadget_equipped(ped: i32, gadgetHash: u32) -> bool {
    let value = native!(
        bool,
        0xF731332072F5156C,
        native_parameters!(ped, gadgetHash)
    );

    value
}

pub fn get_selected_ped_weapon(ped: i32) -> u32 {
    let value = native!(u32, 0x0A6DB4965674D243, native_parameters!(ped));

    value
}

pub fn explode_projectiles(ped: i32, weaponHash: u32, p2: bool) -> () {
    let value = native!(
        (),
        0xFC4BD125DE7611E4,
        native_parameters!(ped, weaponHash, p2)
    );

    value
}

pub fn remove_all_projectiles_of_type(weaponHash: u32, explode: bool) -> () {
    let value = native!(
        (),
        0xFC52E0F37E446528,
        native_parameters!(weaponHash, explode)
    );

    value
}

pub fn get_lockon_distance_of_current_ped_weapon(ped: i32) -> f32 {
    let value = native!(f32, 0x840F03E9041E2C9C, native_parameters!(ped));

    value
}

pub fn get_max_range_of_current_ped_weapon(ped: i32) -> f32 {
    let value = native!(f32, 0x814C9D19DFD69679, native_parameters!(ped));

    value
}

pub fn has_vehicle_got_projectile_attached(
    driver: i32,
    vehicle: i32,
    weaponHash: u32,
    p3: u32,
) -> bool {
    let value = native!(
        bool,
        0x717C8481234E3B88,
        native_parameters!(driver, vehicle, weaponHash, p3)
    );

    value
}

pub fn give_weapon_component_to_ped(ped: i32, weaponHash: u32, componentHash: u32) -> () {
    let value = native!(
        (),
        0xD966D51AA5B28BB9,
        native_parameters!(ped, weaponHash, componentHash)
    );

    value
}

pub fn remove_weapon_component_from_ped(ped: i32, weaponHash: u32, componentHash: u32) -> () {
    let value = native!(
        (),
        0x1E8BE90C74FB4C09,
        native_parameters!(ped, weaponHash, componentHash)
    );

    value
}

pub fn has_ped_got_weapon_component(ped: i32, weaponHash: u32, componentHash: u32) -> bool {
    let value = native!(
        bool,
        0xC593212475FAE340,
        native_parameters!(ped, weaponHash, componentHash)
    );

    value
}

pub fn is_ped_weapon_component_active(ped: i32, weaponHash: u32, componentHash: u32) -> bool {
    let value = native!(
        bool,
        0x0D78DE0572D3969E,
        native_parameters!(ped, weaponHash, componentHash)
    );

    value
}

pub fn refill_ammo_instantly(ped: i32) -> bool {
    let value = native!(bool, 0x8C0D57EA686FAD87, native_parameters!(ped));

    value
}

pub fn make_ped_reload(ped: i32) -> bool {
    let value = native!(bool, 0x20AE33F3AC9C0033, native_parameters!(ped));

    value
}

pub fn request_weapon_asset(weaponHash: u32, p1: i32, p2: i32) -> () {
    let value = native!(
        (),
        0x5443438F033E29C3,
        native_parameters!(weaponHash, p1, p2)
    );

    value
}

pub fn has_weapon_asset_loaded(weaponHash: u32) -> bool {
    let value = native!(bool, 0x36E353271F0E90EE, native_parameters!(weaponHash));

    value
}

pub fn remove_weapon_asset(weaponHash: u32) -> () {
    let value = native!((), 0xAA08EF13F341C8FC, native_parameters!(weaponHash));

    value
}

pub fn create_weapon_object(
    weaponHash: u32,
    ammoCount: i32,
    x: f32,
    y: f32,
    z: f32,
    showWorldModel: bool,
    scale: f32,
    p7: u32,
    p8: u32,
    p9: u32,
) -> i32 {
    let value = native!(
        i32,
        0x9541D3CF0D398F36,
        native_parameters!(
            weaponHash,
            ammoCount,
            x,
            y,
            z,
            showWorldModel,
            scale,
            p7,
            p8,
            p9
        )
    );

    value
}

pub fn give_weapon_component_to_weapon_object(weaponObject: i32, addonHash: u32) -> () {
    let value = native!(
        (),
        0x33E179436C0B31DB,
        native_parameters!(weaponObject, addonHash)
    );

    value
}

pub fn remove_weapon_component_from_weapon_object(p0: u32, p1: u32) -> () {
    let value = native!((), 0xF7D82B0D66777611, native_parameters!(p0, p1));

    value
}

pub fn has_weapon_got_weapon_component(weapon: i32, addonHash: u32) -> bool {
    let value = native!(
        bool,
        0x76A18844E743BF91,
        native_parameters!(weapon, addonHash)
    );

    value
}

pub fn give_weapon_object_to_ped(weaponObject: i32, ped: i32) -> () {
    let value = native!(
        (),
        0xB1FA61371AF7C4B7,
        native_parameters!(weaponObject, ped)
    );

    value
}

pub fn does_weapon_take_weapon_component(weaponHash: u32, componentHash: u32) -> bool {
    let value = native!(
        bool,
        0x5CEE3DF569CECAB0,
        native_parameters!(weaponHash, componentHash)
    );

    value
}

pub fn get_weapon_object_from_ped(ped: i32, p1: bool) -> i32 {
    let value = native!(i32, 0xCAE1DC9A0E22A16D, native_parameters!(ped, p1));

    value
}

pub fn _give_loadout_to_ped(ped: i32, loadoutHash: u32) -> () {
    let value = native!((), 0x68F8BE6AF5CDF8A6, native_parameters!(ped, loadoutHash));

    value
}

pub fn set_ped_weapon_tint_index(ped: i32, weaponHash: u32, tintIndex: i32) -> () {
    let value = native!(
        (),
        0x50969B9B89ED5738,
        native_parameters!(ped, weaponHash, tintIndex)
    );

    value
}

pub fn get_ped_weapon_tint_index(ped: i32, weaponHash: u32) -> i32 {
    let value = native!(i32, 0x2B9EEDC07BD06B9F, native_parameters!(ped, weaponHash));

    value
}

pub fn set_weapon_object_tint_index(weapon: i32, tintIndex: i32) -> () {
    let value = native!(
        (),
        0xF827589017D4E4A9,
        native_parameters!(weapon, tintIndex)
    );

    value
}

pub fn get_weapon_object_tint_index(weapon: i32) -> i32 {
    let value = native!(i32, 0xCD183314F7CD2E57, native_parameters!(weapon));

    value
}

pub fn get_weapon_tint_count(weaponHash: u32) -> i32 {
    let value = native!(i32, 0x5DCF6C5CAB2E9BF7, native_parameters!(weaponHash));

    value
}

pub fn _set_ped_weapon_livery_color(
    ped: i32,
    weaponHash: u32,
    camoComponentHash: u32,
    colorIndex: i32,
) -> () {
    let value = native!(
        (),
        0x9FE5633880ECD8ED,
        native_parameters!(ped, weaponHash, camoComponentHash, colorIndex)
    );

    value
}

pub fn _get_ped_weapon_livery_color(ped: i32, weaponHash: u32, camoComponentHash: u32) -> i32 {
    let value = native!(
        i32,
        0xF0A60040BE558F2D,
        native_parameters!(ped, weaponHash, camoComponentHash)
    );

    value
}

pub fn _set_weapon_object_livery_color(
    weaponObject: i32,
    camoComponentHash: u32,
    colorIndex: i32,
) -> () {
    let value = native!(
        (),
        0x5DA825A85D0EA6E6,
        native_parameters!(weaponObject, camoComponentHash, colorIndex)
    );

    value
}

pub fn _get_weapon_object_livery_color(weaponObject: i32, camoComponentHash: u32) -> i32 {
    let value = native!(
        i32,
        0xB3EA4FEABF41464B,
        native_parameters!(weaponObject, camoComponentHash)
    );

    value
}

pub fn _0xa2c9ac24b4061285(ped: i32, weaponHash: u32) -> i32 {
    let value = native!(i32, 0xA2C9AC24B4061285, native_parameters!(ped, weaponHash));

    value
}

pub fn _0x977ca98939e82e4b(weaponObject: i32, p1: i32) -> () {
    let value = native!((), 0x977CA98939E82E4B, native_parameters!(weaponObject, p1));

    value
}

pub fn get_weapon_hud_stats(weaponHash: u32, outData: *mut u32) -> bool {
    let value = native!(
        bool,
        0xD92C739EE34C9EBA,
        native_parameters!(weaponHash, outData)
    );

    value
}

pub fn get_weapon_component_hud_stats(componentHash: u32, outData: *mut i32) -> bool {
    let value = native!(
        bool,
        0xB3CAF387AE12E9F8,
        native_parameters!(componentHash, outData)
    );

    value
}

pub fn get_weapon_damage(weaponHash: u32, componentHash: u32) -> f32 {
    let value = native!(
        f32,
        0x3133B907D8B32053,
        native_parameters!(weaponHash, componentHash)
    );

    value
}

pub fn get_weapon_clip_size(weaponHash: u32) -> i32 {
    let value = native!(i32, 0x583BE370B1EC6EB4, native_parameters!(weaponHash));

    value
}

pub fn _get_weapon_time_between_shots(weaponHash: u32) -> f32 {
    let value = native!(f32, 0x065D2AACAD8CF7A4, native_parameters!(weaponHash));

    value
}

pub fn set_ped_chance_of_firing_blanks(ped: i32, xBias: f32, yBias: f32) -> () {
    let value = native!(
        (),
        0x8378627201D5497D,
        native_parameters!(ped, xBias, yBias)
    );

    value
}

pub fn set_ped_shoot_ordnance_weapon(ped: i32, p1: f32) -> i32 {
    let value = native!(i32, 0xB4C8D77C80C0421E, native_parameters!(ped, p1));

    value
}

pub fn request_weapon_high_detail_model(weaponObject: i32) -> () {
    let value = native!((), 0x48164DBB970AC3F0, native_parameters!(weaponObject));

    value
}

pub fn _set_weapon_damage_modifier_this_frame(weaponHash: u32, damageMultiplier: f32) -> () {
    let value = native!(
        (),
        0x4757F00BC6323CFE,
        native_parameters!(weaponHash, damageMultiplier)
    );

    value
}

pub fn is_ped_current_weapon_silenced(ped: i32) -> bool {
    let value = native!(bool, 0x65F0C5AE05943EC7, native_parameters!(ped));

    value
}

pub fn is_flash_light_on(ped: i32) -> bool {
    let value = native!(bool, 0x4B7620C47217126C, native_parameters!(ped));

    value
}

pub fn set_flash_light_fade_distance(distance: f32) -> u32 {
    let value = native!(u32, 0xCEA66DAD478CD39B, native_parameters!(distance));

    value
}

pub fn _set_flash_light_enabled(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x988DB6FE9B3AC000, native_parameters!(ped, toggle));

    value
}

pub fn set_weapon_animation_override(ped: i32, animStyle: u32) -> () {
    let value = native!((), 0x1055AC3A667F09D9, native_parameters!(ped, animStyle));

    value
}

pub fn get_weapon_damage_type(weaponHash: u32) -> i32 {
    let value = native!(i32, 0x3BE0BB12D25FB305, native_parameters!(weaponHash));

    value
}

pub fn _0xe4dcec7fd5b739a5(ped: i32) -> () {
    let value = native!((), 0xE4DCEC7FD5B739A5, native_parameters!(ped));

    value
}

pub fn can_use_weapon_on_parachute(weaponHash: u32) -> bool {
    let value = native!(bool, 0xBC7BE5ABC0879F74, native_parameters!(weaponHash));

    value
}

pub fn _create_air_defense_sphere(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    weaponHash: u32,
) -> i32 {
    let value = native!(
        i32,
        0x91EF34584710BE99,
        native_parameters!(x, y, z, radius, p4, p5, p6, weaponHash)
    );

    value
}

pub fn _create_air_defense_area(
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
    weaponHash: u32,
) -> i32 {
    let value = native!(
        i32,
        0x9DA58CDBF6BDBC08,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, weaponHash)
    );

    value
}

pub fn _remove_air_defense_zone(zoneId: i32) -> bool {
    let value = native!(bool, 0x0ABF535877897560, native_parameters!(zoneId));

    value
}

pub fn _remove_all_air_defense_zones() -> () {
    let value = native!((), 0x1E45B34ADEBEE48E, native_parameters!());

    value
}

pub fn _set_player_air_defense_zone_flag(player: i32, zoneId: i32, enable: bool) -> () {
    let value = native!(
        (),
        0xECDC202B25E5CF48,
        native_parameters!(player, zoneId, enable)
    );

    value
}

pub fn _is_any_air_defense_zone_inside_sphere(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    outZoneId: *mut i32,
) -> bool {
    let value = native!(
        bool,
        0xDAB963831DBFD3F4,
        native_parameters!(x, y, z, radius, outZoneId)
    );

    value
}

pub fn _fire_air_defense_weapon(zoneId: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x44F1012B69313374, native_parameters!(zoneId, x, y, z));

    value
}

pub fn _does_air_defense_zone_exist(zoneId: i32) -> bool {
    let value = native!(bool, 0xCD79A550999D7D4F, native_parameters!(zoneId));

    value
}

pub fn _set_can_ped_equip_weapon(ped: i32, weaponHash: u32, toggle: bool) -> () {
    let value = native!(
        (),
        0xB4771B9AAF4E68E4,
        native_parameters!(ped, weaponHash, toggle)
    );

    value
}

pub fn _set_can_ped_equip_all_weapons(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xEFF296097FF1E509, native_parameters!(ped, toggle));

    value
}
