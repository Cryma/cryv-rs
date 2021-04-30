use crate::types::NativeVector3;

pub fn start_script_fire(X: f32, Y: f32, Z: f32, maxChildren: i32, isGasFire: bool) -> i32 {
    let value = native!(
        i32,
        0x6B83617E04503888,
        native_parameters!(X, Y, Z, maxChildren, isGasFire)
    );

    value
}

pub fn remove_script_fire(fireHandle: i32) -> () {
    let value = native!((), 0x7FF548385680673F, native_parameters!(fireHandle));

    value
}

pub fn start_entity_fire(entity: i32) -> i32 {
    let value = native!(i32, 0xF6A9D9708F6F23DF, native_parameters!(entity));

    value
}

pub fn stop_entity_fire(entity: i32) -> () {
    let value = native!((), 0x7F0DD2EBBB651AFF, native_parameters!(entity));

    value
}

pub fn is_entity_on_fire(entity: i32) -> bool {
    let value = native!(bool, 0x28D3FED7190D3A0B, native_parameters!(entity));

    value
}

pub fn get_number_of_fires_in_range(x: f32, y: f32, z: f32, radius: f32) -> i32 {
    let value = native!(i32, 0x50CAD495A460B305, native_parameters!(x, y, z, radius));

    value
}

pub fn _set_fire_spread_rate(p0: f32) -> () {
    let value = native!((), 0x8F390AC4155099BA, native_parameters!(p0));

    value
}

pub fn stop_fire_in_range(x: f32, y: f32, z: f32, radius: f32) -> () {
    let value = native!((), 0x056A8A219B8E829F, native_parameters!(x, y, z, radius));

    value
}

pub fn get_closest_fire_pos(outPosition: *mut NativeVector3, x: f32, y: f32, z: f32) -> bool {
    let value = native!(
        bool,
        0x352A9F6BCF90081F,
        native_parameters!(outPosition, x, y, z)
    );

    value
}

pub fn add_explosion(
    x: f32,
    y: f32,
    z: f32,
    explosionType: i32,
    damageScale: f32,
    isAudible: bool,
    isInvisible: bool,
    cameraShake: f32,
    noDamage: bool,
) -> () {
    let value = native!(
        (),
        0xE3AD2BDBAEE269AC,
        native_parameters!(
            x,
            y,
            z,
            explosionType,
            damageScale,
            isAudible,
            isInvisible,
            cameraShake,
            noDamage
        )
    );

    value
}

pub fn add_owned_explosion(
    ped: i32,
    x: f32,
    y: f32,
    z: f32,
    explosionType: i32,
    damageScale: f32,
    isAudible: bool,
    isInvisible: bool,
    cameraShake: f32,
) -> () {
    let value = native!(
        (),
        0x172AA1B624FA1013,
        native_parameters!(
            ped,
            x,
            y,
            z,
            explosionType,
            damageScale,
            isAudible,
            isInvisible,
            cameraShake
        )
    );

    value
}

pub fn add_explosion_with_user_vfx(
    x: f32,
    y: f32,
    z: f32,
    explosionType: i32,
    explosionFx: u32,
    damageScale: f32,
    isAudible: bool,
    isInvisible: bool,
    cameraShake: f32,
) -> () {
    let value = native!(
        (),
        0x36DD3FE58B5E5212,
        native_parameters!(
            x,
            y,
            z,
            explosionType,
            explosionFx,
            damageScale,
            isAudible,
            isInvisible,
            cameraShake
        )
    );

    value
}

pub fn is_explosion_in_area(
    explosionType: i32,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
) -> bool {
    let value = native!(
        bool,
        0x2E2EBA0EE7CED0E0,
        native_parameters!(explosionType, x1, y1, z1, x2, y2, z2)
    );

    value
}

pub fn is_explosion_active_in_area(
    explosionType: i32,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
) -> bool {
    let value = native!(
        bool,
        0x6070104B699B2EF4,
        native_parameters!(explosionType, x1, y1, z1, x2, y2, z2)
    );

    value
}

pub fn is_explosion_in_sphere(explosionType: i32, x: f32, y: f32, z: f32, radius: f32) -> bool {
    let value = native!(
        bool,
        0xAB0F816885B0E483,
        native_parameters!(explosionType, x, y, z, radius)
    );

    value
}

pub fn _get_entity_inside_explosion_sphere(
    explosionType: i32,
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
) -> i32 {
    let value = native!(
        i32,
        0xB3CD51E3DB86F176,
        native_parameters!(explosionType, x, y, z, radius)
    );

    value
}

pub fn is_explosion_in_angled_area(
    explosionType: i32,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    width: f32,
) -> bool {
    let value = native!(
        bool,
        0xA079A6C51525DC4B,
        native_parameters!(explosionType, x1, y1, z1, x2, y2, z2, width)
    );

    value
}

pub fn _get_entity_inside_explosion_area(
    explosionType: i32,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    radius: f32,
) -> i32 {
    let value = native!(
        i32,
        0x14BA4BA137AF6CEC,
        native_parameters!(explosionType, x1, y1, z1, x2, y2, z2, radius)
    );

    value
}
