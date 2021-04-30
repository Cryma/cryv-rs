use crate::types::NativeVector3;

pub fn get_num_tattoo_shop_dlc_items(character: i32) -> i32 {
    let value = native!(i32, 0x278F76C3B0A8F109, native_parameters!(character));

    value
}

pub fn get_tattoo_shop_dlc_item_data(
    characterType: i32,
    decorationIndex: i32,
    outComponent: *mut u32,
) -> bool {
    let value = native!(
        bool,
        0xFF56381874F82086,
        native_parameters!(characterType, decorationIndex, outComponent)
    );

    value
}

pub fn _0x10144267dd22866c(overlayHash: u32, p1: u32, character: i32) -> i32 {
    let value = native!(
        i32,
        0x10144267DD22866C,
        native_parameters!(overlayHash, p1, character)
    );

    value
}

pub fn init_shop_ped_component(outComponent: *mut u32) -> () {
    let value = native!((), 0x1E8C308FD312C036, native_parameters!(outComponent));

    value
}

pub fn init_shop_ped_prop(outProp: *mut u32) -> () {
    let value = native!((), 0xEB0A2B758F7B850F, native_parameters!(outProp));

    value
}

pub fn setup_shop_ped_apparel_query(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
    let value = native!(i32, 0x50F457823CE6EB5F, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn setup_shop_ped_apparel_query_tu(
    character: i32,
    p1: i32,
    p2: i32,
    p3: bool,
    p4: i32,
    componentId: i32,
) -> i32 {
    let value = native!(
        i32,
        0x9BDF59818B1E38C1,
        native_parameters!(character, p1, p2, p3, p4, componentId)
    );

    value
}

pub fn get_shop_ped_query_component(componentId: i32, outComponent: *mut u32) -> () {
    let value = native!(
        (),
        0x249E310B2D920699,
        native_parameters!(componentId, outComponent)
    );

    value
}

pub fn _0x96e2929292a4db77(componentHash: u32) -> i32 {
    let value = native!(i32, 0x96E2929292A4DB77, native_parameters!(componentHash));

    value
}

pub fn get_shop_ped_component(componentHash: u32, outComponent: *mut u32) -> () {
    let value = native!(
        (),
        0x74C0E2A57EC66760,
        native_parameters!(componentHash, outComponent)
    );

    value
}

pub fn get_shop_ped_query_prop(componentId: i32, outProp: *mut u32) -> () {
    let value = native!(
        (),
        0xDE44A00999B2837D,
        native_parameters!(componentId, outProp)
    );

    value
}

pub fn _0x6cebe002e58dee97(componentHash: u32) -> i32 {
    let value = native!(i32, 0x6CEBE002E58DEE97, native_parameters!(componentHash));

    value
}

pub fn get_shop_ped_prop(componentHash: u32, outProp: *mut u32) -> () {
    let value = native!(
        (),
        0x5D5CAFF661DDF6FC,
        native_parameters!(componentHash, outProp)
    );

    value
}

pub fn get_hash_name_for_component(
    entity: i32,
    componentId: i32,
    drawableVariant: i32,
    textureVariant: i32,
) -> u32 {
    let value = native!(
        u32,
        0x0368B3A838070348,
        native_parameters!(entity, componentId, drawableVariant, textureVariant)
    );

    value
}

pub fn get_hash_name_for_prop(
    entity: i32,
    componentId: i32,
    propIndex: i32,
    propTextureIndex: i32,
) -> u32 {
    let value = native!(
        u32,
        0x5D6160275CAEC8DD,
        native_parameters!(entity, componentId, propIndex, propTextureIndex)
    );

    value
}

pub fn get_shop_ped_apparel_variant_component_count(componentHash: u32) -> i32 {
    let value = native!(i32, 0xC17AD0E5752BECDA, native_parameters!(componentHash));

    value
}

pub fn _get_shop_ped_apparel_variant_prop_count(propHash: u32) -> i32 {
    let value = native!(i32, 0xD40AAC51E8E4C663, native_parameters!(propHash));

    value
}

pub fn get_variant_component(
    componentHash: u32,
    variantComponentIndex: i32,
    nameHash: *mut u32,
    enumValue: *mut i32,
    componentType: *mut i32,
) -> () {
    let value = native!(
        (),
        0x6E11F282F11863B6,
        native_parameters!(
            componentHash,
            variantComponentIndex,
            nameHash,
            enumValue,
            componentType
        )
    );

    value
}

pub fn _get_variant_prop(
    componentHash: u32,
    variantPropIndex: i32,
    nameHash: *mut u32,
    enumValue: *mut i32,
    anchorPoint: *mut i32,
) -> () {
    let value = native!(
        (),
        0xD81B7F27BC773E66,
        native_parameters!(
            componentHash,
            variantPropIndex,
            nameHash,
            enumValue,
            anchorPoint
        )
    );

    value
}

pub fn get_shop_ped_apparel_forced_component_count(componentHash: u32) -> i32 {
    let value = native!(i32, 0xC6B9DB42C04DD8C3, native_parameters!(componentHash));

    value
}

pub fn get_shop_ped_apparel_forced_prop_count(componentHash: u32) -> i32 {
    let value = native!(i32, 0x017568A8182D98A6, native_parameters!(componentHash));

    value
}

pub fn get_forced_component(
    componentHash: u32,
    forcedComponentIndex: i32,
    nameHash: *mut u32,
    enumValue: *mut i32,
    componentType: *mut i32,
) -> () {
    let value = native!(
        (),
        0x6C93ED8C2F74859B,
        native_parameters!(
            componentHash,
            forcedComponentIndex,
            nameHash,
            enumValue,
            componentType
        )
    );

    value
}

pub fn get_forced_prop(
    componentHash: u32,
    forcedPropIndex: i32,
    nameHash: *mut u32,
    enumValue: *mut i32,
    anchorPoint: *mut i32,
) -> () {
    let value = native!(
        (),
        0xE1CA84EBF72E691D,
        native_parameters!(
            componentHash,
            forcedPropIndex,
            nameHash,
            enumValue,
            anchorPoint
        )
    );

    value
}

pub fn does_shop_ped_apparel_have_restriction_tag(
    componentHash: u32,
    restrictionTagHash: u32,
    componentId: i32,
) -> bool {
    let value = native!(
        bool,
        0x341DE7ED1D2A1BFD,
        native_parameters!(componentHash, restrictionTagHash, componentId)
    );

    value
}

pub fn setup_shop_ped_outfit_query(character: i32, p1: bool) -> i32 {
    let value = native!(i32, 0xF3FBE2D50A6A8C28, native_parameters!(character, p1));

    value
}

pub fn get_shop_ped_query_outfit(outfitIndex: i32, outfit: *mut u32) -> () {
    let value = native!(
        (),
        0x6D793F03A631FE56,
        native_parameters!(outfitIndex, outfit)
    );

    value
}

pub fn get_shop_ped_outfit(p0: u32, p1: *mut u32) -> () {
    let value = native!((), 0xB7952076E444979D, native_parameters!(p0, p1));

    value
}

pub fn get_shop_ped_outfit_locate(p0: u32) -> i32 {
    let value = native!(i32, 0x073CA26B079F956E, native_parameters!(p0));

    value
}

pub fn get_shop_ped_outfit_prop_variant(
    outfitHash: u32,
    variantIndex: i32,
    outPropVariant: *mut u32,
) -> bool {
    let value = native!(
        bool,
        0xA9F9C2E0FDE11CBB,
        native_parameters!(outfitHash, variantIndex, outPropVariant)
    );

    value
}

pub fn get_shop_ped_outfit_component_variant(
    outfitHash: u32,
    variantIndex: i32,
    outComponentVariant: *mut u32,
) -> bool {
    let value = native!(
        bool,
        0x19F2A026EDF0013F,
        native_parameters!(outfitHash, variantIndex, outComponentVariant)
    );

    value
}

pub fn get_num_dlc_vehicles() -> i32 {
    let value = native!(i32, 0xA7A866D21CD2329B, native_parameters!());

    value
}

pub fn get_dlc_vehicle_model(dlcVehicleIndex: i32) -> u32 {
    let value = native!(u32, 0xECC01B7C5763333C, native_parameters!(dlcVehicleIndex));

    value
}

pub fn get_dlc_vehicle_data(dlcVehicleIndex: i32, outData: *mut i32) -> bool {
    let value = native!(
        bool,
        0x33468EDC08E371F6,
        native_parameters!(dlcVehicleIndex, outData)
    );

    value
}

pub fn get_dlc_vehicle_flags(dlcVehicleIndex: i32) -> i32 {
    let value = native!(i32, 0x5549EE11FA22FCF2, native_parameters!(dlcVehicleIndex));

    value
}

pub fn get_num_dlc_weapons() -> i32 {
    let value = native!(i32, 0xEE47635F352DA367, native_parameters!());

    value
}

pub fn _get_num_dlc_weapons_sp() -> i32 {
    let value = native!(i32, 0x4160B65AE085B5A9, native_parameters!());

    value
}

pub fn get_dlc_weapon_data(dlcWeaponIndex: i32, outData: *mut i32) -> bool {
    let value = native!(
        bool,
        0x79923CD21BECE14E,
        native_parameters!(dlcWeaponIndex, outData)
    );

    value
}

pub fn _get_dlc_weapon_data_sp(dlcWeaponIndex: i32, outData: *mut i32) -> bool {
    let value = native!(
        bool,
        0x310836EE7129BA33,
        native_parameters!(dlcWeaponIndex, outData)
    );

    value
}

pub fn get_num_dlc_weapon_components(dlcWeaponIndex: i32) -> i32 {
    let value = native!(i32, 0x405425358A7D61FE, native_parameters!(dlcWeaponIndex));

    value
}

pub fn _get_num_dlc_weapon_components_sp(dlcWeaponIndex: i32) -> i32 {
    let value = native!(i32, 0xAD2A7A6DFF55841B, native_parameters!(dlcWeaponIndex));

    value
}

pub fn get_dlc_weapon_component_data(
    dlcWeaponIndex: i32,
    dlcWeapCompIndex: i32,
    ComponentDataPtr: *mut i32,
) -> bool {
    let value = native!(
        bool,
        0x6CF598A2957C2BF8,
        native_parameters!(dlcWeaponIndex, dlcWeapCompIndex, ComponentDataPtr)
    );

    value
}

pub fn _get_dlc_weapon_component_data_sp(
    dlcWeaponIndex: i32,
    dlcWeapCompIndex: i32,
    ComponentDataPtr: *mut i32,
) -> bool {
    let value = native!(
        bool,
        0x31D5E073B6F93CDC,
        native_parameters!(dlcWeaponIndex, dlcWeapCompIndex, ComponentDataPtr)
    );

    value
}

pub fn is_content_item_locked(itemHash: u32) -> bool {
    let value = native!(bool, 0xD4D7B033C3AA243C, native_parameters!(itemHash));

    value
}

pub fn is_dlc_vehicle_mod(hash: u32) -> bool {
    let value = native!(bool, 0x0564B9FF9631B82C, native_parameters!(hash));

    value
}

pub fn get_dlc_vehicle_mod_lock_hash(hash: u32) -> u32 {
    let value = native!(u32, 0xC098810437312FFF, native_parameters!(hash));

    value
}

pub fn _load_content_change_set_group(hash: u32) -> () {
    let value = native!((), 0x6BEDF5769AC2DC07, native_parameters!(hash));

    value
}

pub fn _unload_content_change_set_group(hash: u32) -> () {
    let value = native!((), 0x3C1978285B036B25, native_parameters!(hash));

    value
}
