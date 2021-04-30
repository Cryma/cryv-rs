use crate::types::NativeVector3;

pub fn set_debug_lines_and_spheres_drawing_active(enabled: bool) -> () {
    let value = native!((), 0x175B6BFC15CDD0C5, native_parameters!(enabled));

    value
}

pub fn draw_debug_line(
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
        0x7FDFADE676AA3CB0,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn draw_debug_line_with_two_colours(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    r1: i32,
    g1: i32,
    b1: i32,
    r2: i32,
    g2: i32,
    b2: i32,
    alpha1: i32,
    alpha2: i32,
) -> () {
    let value = native!(
        (),
        0xD8B9A8AC5608FF94,
        native_parameters!(x1, y1, z1, x2, y2, z2, r1, g1, b1, r2, g2, b2, alpha1, alpha2)
    );

    value
}

pub fn draw_debug_sphere(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
) -> () {
    let value = native!(
        (),
        0xAAD68E1AB39DA632,
        native_parameters!(x, y, z, radius, red, green, blue, alpha)
    );

    value
}

pub fn draw_debug_box(
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
        0x083A2CA4F2E573BD,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn draw_debug_cross(
    x: f32,
    y: f32,
    z: f32,
    size: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
) -> () {
    let value = native!(
        (),
        0x73B1189623049839,
        native_parameters!(x, y, z, size, red, green, blue, alpha)
    );

    value
}

pub fn draw_debug_text(
    text: &std::ffi::CString,
    x: f32,
    y: f32,
    z: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
) -> () {
    let value = native!(
        (),
        0x3903E216620488E8,
        native_parameters!(text.as_ptr(), x, y, z, red, green, blue, alpha)
    );

    value
}

pub fn draw_debug_text_2d(
    text: &std::ffi::CString,
    x: f32,
    y: f32,
    z: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
) -> () {
    let value = native!(
        (),
        0xA3BB2E9555C05A8F,
        native_parameters!(text.as_ptr(), x, y, z, red, green, blue, alpha)
    );

    value
}

pub fn draw_line(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
) -> () {
    let value = native!(
        (),
        0x6B7256074AE34680,
        native_parameters!(x1, y1, z1, x2, y2, z2, red, green, blue, alpha)
    );

    value
}

pub fn draw_poly(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    x3: f32,
    y3: f32,
    z3: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
) -> () {
    let value = native!(
        (),
        0xAC26716048436851,
        native_parameters!(x1, y1, z1, x2, y2, z2, x3, y3, z3, red, green, blue, alpha)
    );

    value
}

pub fn _draw_sprite_poly(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    x3: f32,
    y3: f32,
    z3: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
    textureDict: &std::ffi::CString,
    textureName: &std::ffi::CString,
    u1: f32,
    v1: f32,
    w1: f32,
    u2: f32,
    v2: f32,
    w2: f32,
    u3: f32,
    v3: f32,
    w3: f32,
) -> () {
    let value = native!(
        (),
        0x29280002282F1928,
        native_parameters!(
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
            x3,
            y3,
            z3,
            red,
            green,
            blue,
            alpha,
            textureDict.as_ptr(),
            textureName.as_ptr(),
            u1,
            v1,
            w1,
            u2,
            v2,
            w2,
            u3,
            v3,
            w3
        )
    );

    value
}

pub fn _draw_sprite_poly_2(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    x3: f32,
    y3: f32,
    z3: f32,
    red1: f32,
    green1: f32,
    blue1: f32,
    alpha1: i32,
    red2: f32,
    green2: f32,
    blue2: f32,
    alpha2: i32,
    red3: f32,
    green3: f32,
    blue3: f32,
    alpha3: i32,
    textureDict: &std::ffi::CString,
    textureName: &std::ffi::CString,
    u1: f32,
    v1: f32,
    w1: f32,
    u2: f32,
    v2: f32,
    w2: f32,
    u3: f32,
    v3: f32,
    w3: f32,
) -> () {
    let value = native!(
        (),
        0x736D7AA1B750856B,
        native_parameters!(
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
            x3,
            y3,
            z3,
            red1,
            green1,
            blue1,
            alpha1,
            red2,
            green2,
            blue2,
            alpha2,
            red3,
            green3,
            blue3,
            alpha3,
            textureDict.as_ptr(),
            textureName.as_ptr(),
            u1,
            v1,
            w1,
            u2,
            v2,
            w2,
            u3,
            v3,
            w3
        )
    );

    value
}

pub fn draw_box(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
) -> () {
    let value = native!(
        (),
        0xD3A9971CADAC7252,
        native_parameters!(x1, y1, z1, x2, y2, z2, red, green, blue, alpha)
    );

    value
}

pub fn set_backfaceculling(toggle: bool) -> () {
    let value = native!((), 0x23BA6B0C2AD7B0D3, native_parameters!(toggle));

    value
}

pub fn _0xc5c8f970d4edff71(p0: u32) -> () {
    let value = native!((), 0xC5C8F970D4EDFF71, native_parameters!(p0));

    value
}

pub fn begin_take_mission_creator_photo() -> u32 {
    let value = native!(u32, 0x1DD2139A9A20DCE8, native_parameters!());

    value
}

pub fn get_status_of_take_mission_creator_photo() -> u32 {
    let value = native!(u32, 0x90A78ECAA4E78453, native_parameters!());

    value
}

pub fn free_memory_for_mission_creator_photo() -> () {
    let value = native!((), 0x0A46AF8A78DC5E0A, native_parameters!());

    value
}

pub fn load_mission_creator_photo(p0: *mut u32, p1: u32, p2: u32, p3: u32) -> bool {
    let value = native!(bool, 0x4862437A486F91B0, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn get_status_of_load_mission_creator_photo(p0: *mut u32) -> i32 {
    let value = native!(i32, 0x1670F8D05056F257, native_parameters!(p0));

    value
}

pub fn _0x7fa5d82b8f58ec06() -> u32 {
    let value = native!(u32, 0x7FA5D82B8F58EC06, native_parameters!());

    value
}

pub fn _0x5b0316762afd4a64() -> u32 {
    let value = native!(u32, 0x5B0316762AFD4A64, native_parameters!());

    value
}

pub fn _0x346ef3ecaaab149e() -> () {
    let value = native!((), 0x346EF3ECAAAB149E, native_parameters!());

    value
}

pub fn begin_take_high_quality_photo() -> bool {
    let value = native!(bool, 0xA67C35C56EB1BD9D, native_parameters!());

    value
}

pub fn get_status_of_take_high_quality_photo() -> i32 {
    let value = native!(i32, 0x0D6CA79EEEBD8CA3, native_parameters!());

    value
}

pub fn free_memory_for_high_quality_photo() -> () {
    let value = native!((), 0xD801CC02177FA3F1, native_parameters!());

    value
}

pub fn _0x1bbc135a4d25edde(p0: bool) -> () {
    let value = native!((), 0x1BBC135A4D25EDDE, native_parameters!(p0));

    value
}

pub fn _0xf3f776ada161e47d(p0: u32, p1: u32) -> () {
    let value = native!((), 0xF3F776ADA161E47D, native_parameters!(p0, p1));

    value
}

pub fn _0xadd6627c4d325458(p0: u32) -> () {
    let value = native!((), 0xADD6627C4D325458, native_parameters!(p0));

    value
}

pub fn save_high_quality_photo(unused: i32) -> bool {
    let value = native!(bool, 0x3DEC726C25A11BAC, native_parameters!(unused));

    value
}

pub fn get_status_of_save_high_quality_photo() -> i32 {
    let value = native!(i32, 0x0C0C4E81E1AC60A0, native_parameters!());

    value
}

pub fn _0x759650634f07b6b4(p0: u32) -> bool {
    let value = native!(bool, 0x759650634F07B6B4, native_parameters!(p0));

    value
}

pub fn _0xcb82a0bf0e3e3265(p0: u32) -> u32 {
    let value = native!(u32, 0xCB82A0BF0E3E3265, native_parameters!(p0));

    value
}

pub fn free_memory_for_low_quality_photo() -> () {
    let value = native!((), 0x6A12D88881435DCA, native_parameters!());

    value
}

pub fn draw_low_quality_photo_to_phone(p0: bool, p1: bool) -> () {
    let value = native!((), 0x1072F115DAB0717E, native_parameters!(p0, p1));

    value
}

pub fn get_maximum_number_of_photos() -> i32 {
    let value = native!(i32, 0x34D23450F028B0BF, native_parameters!());

    value
}

pub fn get_maximum_number_of_cloud_photos() -> i32 {
    let value = native!(i32, 0xDC54A7AF8B3A14EF, native_parameters!());

    value
}

pub fn get_current_number_of_cloud_photos() -> i32 {
    let value = native!(i32, 0x473151EBC762C6DA, native_parameters!());

    value
}

pub fn _0x2a893980e96b659a(p0: u32) -> u32 {
    let value = native!(u32, 0x2A893980E96B659A, native_parameters!(p0));

    value
}

pub fn get_status_of_sorted_list_operation(p0: u32) -> u32 {
    let value = native!(u32, 0xF5BED327CEA362B1, native_parameters!(p0));

    value
}

pub fn _0x4af92acd3141d96c() -> () {
    let value = native!((), 0x4AF92ACD3141D96C, native_parameters!());

    value
}

pub fn _0xe791df1f73ed2c8b(p0: u32) -> u32 {
    let value = native!(u32, 0xE791DF1F73ED2C8B, native_parameters!(p0));

    value
}

pub fn _0xec72c258667be5ea(p0: u32) -> u32 {
    let value = native!(u32, 0xEC72C258667BE5EA, native_parameters!(p0));

    value
}

pub fn _return_two(p0: i32) -> i32 {
    let value = native!(i32, 0x40AFB081F8ADD4EE, native_parameters!(p0));

    value
}

pub fn _draw_light_with_range_and_shadow(
    x: f32,
    y: f32,
    z: f32,
    r: i32,
    g: i32,
    b: i32,
    range: f32,
    intensity: f32,
    shadow: f32,
) -> () {
    let value = native!(
        (),
        0xF49E9A9716A04595,
        native_parameters!(x, y, z, r, g, b, range, intensity, shadow)
    );

    value
}

pub fn draw_light_with_range(
    posX: f32,
    posY: f32,
    posZ: f32,
    colorR: i32,
    colorG: i32,
    colorB: i32,
    range: f32,
    intensity: f32,
) -> () {
    let value = native!(
        (),
        0xF2A1B2771A01DBD4,
        native_parameters!(posX, posY, posZ, colorR, colorG, colorB, range, intensity)
    );

    value
}

pub fn draw_spot_light(
    posX: f32,
    posY: f32,
    posZ: f32,
    dirX: f32,
    dirY: f32,
    dirZ: f32,
    colorR: i32,
    colorG: i32,
    colorB: i32,
    distance: f32,
    brightness: f32,
    hardness: f32,
    radius: f32,
    falloff: f32,
) -> () {
    let value = native!(
        (),
        0xD0F64B265C8C8B33,
        native_parameters!(
            posX, posY, posZ, dirX, dirY, dirZ, colorR, colorG, colorB, distance, brightness,
            hardness, radius, falloff
        )
    );

    value
}

pub fn _draw_spot_light_with_shadow(
    posX: f32,
    posY: f32,
    posZ: f32,
    dirX: f32,
    dirY: f32,
    dirZ: f32,
    colorR: i32,
    colorG: i32,
    colorB: i32,
    distance: f32,
    brightness: f32,
    roundness: f32,
    radius: f32,
    falloff: f32,
    shadowId: i32,
) -> () {
    let value = native!(
        (),
        0x5BCA583A583194DB,
        native_parameters!(
            posX, posY, posZ, dirX, dirY, dirZ, colorR, colorG, colorB, distance, brightness,
            roundness, radius, falloff, shadowId
        )
    );

    value
}

pub fn fade_up_ped_light(p0: f32) -> () {
    let value = native!((), 0xC9B18B4619F48F7B, native_parameters!(p0));

    value
}

pub fn update_lights_on_entity(entity: i32) -> () {
    let value = native!((), 0xDEADC0DEDEADC0DE, native_parameters!(entity));

    value
}

pub fn _0x9641588dab93b4b5(p0: u32) -> () {
    let value = native!((), 0x9641588DAB93B4B5, native_parameters!(p0));

    value
}

pub fn _0x393bd2275ceb7793() -> u32 {
    let value = native!(u32, 0x393BD2275CEB7793, native_parameters!());

    value
}

pub fn draw_marker(
    type_esc: i32,
    posX: f32,
    posY: f32,
    posZ: f32,
    dirX: f32,
    dirY: f32,
    dirZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    scaleX: f32,
    scaleY: f32,
    scaleZ: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
    bobUpAndDown: bool,
    faceCamera: bool,
    p19: i32,
    rotate: bool,
    textureDict: &std::ffi::CString,
    textureName: &std::ffi::CString,
    drawOnEnts: bool,
) -> () {
    let value = native!(
        (),
        0x28477EC23D892089,
        native_parameters!(
            type_esc,
            posX,
            posY,
            posZ,
            dirX,
            dirY,
            dirZ,
            rotX,
            rotY,
            rotZ,
            scaleX,
            scaleY,
            scaleZ,
            red,
            green,
            blue,
            alpha,
            bobUpAndDown,
            faceCamera,
            p19,
            rotate,
            textureDict.as_ptr(),
            textureName.as_ptr(),
            drawOnEnts
        )
    );

    value
}

pub fn _draw_marker_2(
    type_esc: i32,
    posX: f32,
    posY: f32,
    posZ: f32,
    dirX: f32,
    dirY: f32,
    dirZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    scaleX: f32,
    scaleY: f32,
    scaleZ: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
    bobUpAndDown: bool,
    faceCamera: bool,
    p19: u32,
    rotate: bool,
    textureDict: &std::ffi::CString,
    textureName: &std::ffi::CString,
    drawOnEnts: bool,
    p24: bool,
    p25: bool,
) -> () {
    let value = native!(
        (),
        0xE82728F0DE75D13A,
        native_parameters!(
            type_esc,
            posX,
            posY,
            posZ,
            dirX,
            dirY,
            dirZ,
            rotX,
            rotY,
            rotZ,
            scaleX,
            scaleY,
            scaleZ,
            red,
            green,
            blue,
            alpha,
            bobUpAndDown,
            faceCamera,
            p19,
            rotate,
            textureDict.as_ptr(),
            textureName.as_ptr(),
            drawOnEnts,
            p24,
            p25
        )
    );

    value
}

pub fn _draw_sphere(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: f32,
) -> () {
    let value = native!(
        (),
        0x799017F9E3B10112,
        native_parameters!(x, y, z, radius, red, green, blue, alpha)
    );

    value
}

pub fn create_checkpoint(
    type_esc: i32,
    posX1: f32,
    posY1: f32,
    posZ1: f32,
    posX2: f32,
    posY2: f32,
    posZ2: f32,
    diameter: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
    reserved: i32,
) -> i32 {
    let value = native!(
        i32,
        0x0134F0835AB6BFCB,
        native_parameters!(
            type_esc, posX1, posY1, posZ1, posX2, posY2, posZ2, diameter, red, green, blue, alpha,
            reserved
        )
    );

    value
}

pub fn _set_checkpoint_scale(checkpoint: i32, p0: f32) -> () {
    let value = native!((), 0x4B5B4DA5D79F1943, native_parameters!(checkpoint, p0));

    value
}

pub fn _set_checkpoint_icon_scale(checkpoint: i32, scale: f32) -> () {
    let value = native!(
        (),
        0x44621483FF966526,
        native_parameters!(checkpoint, scale)
    );

    value
}

pub fn set_checkpoint_cylinder_height(
    checkpoint: i32,
    nearHeight: f32,
    farHeight: f32,
    radius: f32,
) -> () {
    let value = native!(
        (),
        0x2707AAE9D9297D89,
        native_parameters!(checkpoint, nearHeight, farHeight, radius)
    );

    value
}

pub fn set_checkpoint_rgba(checkpoint: i32, red: i32, green: i32, blue: i32, alpha: i32) -> () {
    let value = native!(
        (),
        0x7167371E8AD747F7,
        native_parameters!(checkpoint, red, green, blue, alpha)
    );

    value
}

pub fn set_checkpoint_rgba2(checkpoint: i32, red: i32, green: i32, blue: i32, alpha: i32) -> () {
    let value = native!(
        (),
        0xB9EA40907C680580,
        native_parameters!(checkpoint, red, green, blue, alpha)
    );

    value
}

pub fn _0xf51d36185993515d(
    checkpoint: i32,
    posX: f32,
    posY: f32,
    posZ: f32,
    unkX: f32,
    unkY: f32,
    unkZ: f32,
) -> () {
    let value = native!(
        (),
        0xF51D36185993515D,
        native_parameters!(checkpoint, posX, posY, posZ, unkX, unkY, unkZ)
    );

    value
}

pub fn _0xfcf6788fc4860cd4(checkpoint: i32) -> () {
    let value = native!((), 0xFCF6788FC4860CD4, native_parameters!(checkpoint));

    value
}

pub fn _0x615d3925e87a3b26(checkpoint: i32) -> () {
    let value = native!((), 0x615D3925E87A3B26, native_parameters!(checkpoint));

    value
}

pub fn _0xdb1ea9411c8911ec(p0: u32) -> () {
    let value = native!((), 0xDB1EA9411C8911EC, native_parameters!(p0));

    value
}

pub fn _0x3c788e7f6438754d(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x3C788E7F6438754D, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn delete_checkpoint(checkpoint: i32) -> () {
    let value = native!((), 0xF5ED37F54CD4D52E, native_parameters!(checkpoint));

    value
}

pub fn dont_render_in_game_ui(p0: bool) -> () {
    let value = native!((), 0x22A249A53034450A, native_parameters!(p0));

    value
}

pub fn force_render_in_game_ui(toggle: bool) -> () {
    let value = native!((), 0xDC459CFA0CCE245B, native_parameters!(toggle));

    value
}

pub fn request_streamed_texture_dict(textureDict: &std::ffi::CString, p1: bool) -> () {
    let value = native!(
        (),
        0xDFA2EF8E04127DD5,
        native_parameters!(textureDict.as_ptr(), p1)
    );

    value
}

pub fn has_streamed_texture_dict_loaded(textureDict: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x0145F696AAAAD2E4,
        native_parameters!(textureDict.as_ptr())
    );

    value
}

pub fn set_streamed_texture_dict_as_no_longer_needed(textureDict: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xBE2CACCF5A8AA805,
        native_parameters!(textureDict.as_ptr())
    );

    value
}

pub fn draw_rect(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    r: i32,
    g: i32,
    b: i32,
    a: i32,
    p8: bool,
) -> () {
    let value = native!(
        (),
        0x3A618A217E5154F0,
        native_parameters!(x, y, width, height, r, g, b, a, p8)
    );

    value
}

pub fn set_script_gfx_draw_behind_pausemenu(toggle: bool) -> () {
    let value = native!((), 0xC6372ECD45D73BCD, native_parameters!(toggle));

    value
}

pub fn set_script_gfx_draw_order(drawOrder: i32) -> () {
    let value = native!((), 0x61BB1D9B3A95D802, native_parameters!(drawOrder));

    value
}

pub fn set_script_gfx_align(horizontalAlign: i32, verticalAlign: i32) -> () {
    let value = native!(
        (),
        0xB8A850F20A067EB6,
        native_parameters!(horizontalAlign, verticalAlign)
    );

    value
}

pub fn reset_script_gfx_align() -> () {
    let value = native!((), 0xE3A3DB414A373DAB, native_parameters!());

    value
}

pub fn set_script_gfx_align_params(x: f32, y: f32, w: f32, h: f32) -> () {
    let value = native!((), 0xF5A2C681787E579D, native_parameters!(x, y, w, h));

    value
}

pub fn _get_script_gfx_position(
    x: f32,
    y: f32,
    calculatedX: *mut f32,
    calculatedY: *mut f32,
) -> () {
    let value = native!(
        (),
        0x6DD8F5AA635EB4B2,
        native_parameters!(x, y, calculatedX, calculatedY)
    );

    value
}

pub fn get_safe_zone_size() -> f32 {
    let value = native!(f32, 0xBAF107B6BB2C97F0, native_parameters!());

    value
}

pub fn draw_sprite(
    textureDict: &std::ffi::CString,
    textureName: &std::ffi::CString,
    screenX: f32,
    screenY: f32,
    width: f32,
    height: f32,
    heading: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
    p11: bool,
) -> () {
    let value = native!(
        (),
        0xE7FFAE5EBF23D890,
        native_parameters!(
            textureDict.as_ptr(),
            textureName.as_ptr(),
            screenX,
            screenY,
            width,
            height,
            heading,
            red,
            green,
            blue,
            alpha,
            p11
        )
    );

    value
}

pub fn _0x2d3b147afad49de0(
    textureDict: &std::ffi::CString,
    textureName: &std::ffi::CString,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    p6: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
    p11: u32,
) -> () {
    let value = native!(
        (),
        0x2D3B147AFAD49DE0,
        native_parameters!(
            textureDict.as_ptr(),
            textureName.as_ptr(),
            x,
            y,
            width,
            height,
            p6,
            red,
            green,
            blue,
            alpha,
            p11
        )
    );

    value
}

pub fn _draw_interactive_sprite(
    textureDict: &std::ffi::CString,
    textureName: &std::ffi::CString,
    screenX: f32,
    screenY: f32,
    width: f32,
    height: f32,
    heading: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
) -> () {
    let value = native!(
        (),
        0x2BC54A8188768488,
        native_parameters!(
            textureDict.as_ptr(),
            textureName.as_ptr(),
            screenX,
            screenY,
            width,
            height,
            heading,
            red,
            green,
            blue,
            alpha
        )
    );

    value
}

pub fn _draw_sprite_uv(
    textureDict: &std::ffi::CString,
    textureName: &std::ffi::CString,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    u1: f32,
    v1: f32,
    u2: f32,
    v2: f32,
    heading: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
) -> () {
    let value = native!(
        (),
        0x95812F9B26074726,
        native_parameters!(
            textureDict.as_ptr(),
            textureName.as_ptr(),
            x,
            y,
            width,
            height,
            u1,
            v1,
            u2,
            v2,
            heading,
            red,
            green,
            blue,
            alpha
        )
    );

    value
}

pub fn add_entity_icon(entity: i32, icon: &std::ffi::CString) -> u32 {
    let value = native!(
        u32,
        0x9CD43EEE12BF4DD0,
        native_parameters!(entity, icon.as_ptr())
    );

    value
}

pub fn set_entity_icon_visibility(entity: i32, toggle: bool) -> () {
    let value = native!((), 0xE0E8BEECCA96BA31, native_parameters!(entity, toggle));

    value
}

pub fn set_entity_icon_color(entity: i32, red: i32, green: i32, blue: i32, alpha: i32) -> () {
    let value = native!(
        (),
        0x1D5F595CCAE2E238,
        native_parameters!(entity, red, green, blue, alpha)
    );

    value
}

pub fn set_draw_origin(x: f32, y: f32, z: f32, p3: u32) -> () {
    let value = native!((), 0xAA0008F3BBB8F416, native_parameters!(x, y, z, p3));

    value
}

pub fn clear_draw_origin() -> () {
    let value = native!((), 0xFF0B610F6BE0D7AF, native_parameters!());

    value
}

pub fn _set_bink_movie(name: &std::ffi::CString) -> i32 {
    let value = native!(i32, 0x338D9F609FD632DB, native_parameters!(name.as_ptr()));

    value
}

pub fn _play_bink_movie(binkMovie: i32) -> () {
    let value = native!((), 0x70D2CC8A542A973C, native_parameters!(binkMovie));

    value
}

pub fn _stop_bink_movie(binkMovie: i32) -> () {
    let value = native!((), 0x63606A61DE68898A, native_parameters!(binkMovie));

    value
}

pub fn _release_bink_movie(binkMovie: i32) -> () {
    let value = native!((), 0x04D950EEFA4EED8C, native_parameters!(binkMovie));

    value
}

pub fn _draw_bink_movie(
    binkMovie: i32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    r: i32,
    g: i32,
    b: i32,
    a: i32,
) -> () {
    let value = native!(
        (),
        0x7118E83EEB9F7238,
        native_parameters!(binkMovie, p1, p2, p3, p4, p5, r, g, b, a)
    );

    value
}

pub fn _set_bink_movie_time(binkMovie: i32, progress: f32) -> () {
    let value = native!(
        (),
        0x0CB6B3446855B57A,
        native_parameters!(binkMovie, progress)
    );

    value
}

pub fn _get_bink_movie_time(binkMovie: i32) -> f32 {
    let value = native!(f32, 0x8E17DDD6B9D5BF29, native_parameters!(binkMovie));

    value
}

pub fn _set_bink_movie_volume(binkMovie: i32, value: f32) -> () {
    let value = native!((), 0xAFF33B1178172223, native_parameters!(binkMovie, value));

    value
}

pub fn attach_tv_audio_to_entity(entity: i32) -> () {
    let value = native!((), 0x845BAD77CC770633, native_parameters!(entity));

    value
}

pub fn _set_bink_movie_unk_2(binkMovie: i32, p1: bool) -> () {
    let value = native!((), 0xF816F2933752322D, native_parameters!(binkMovie, p1));

    value
}

pub fn set_tv_audio_frontend(toggle: bool) -> () {
    let value = native!((), 0x113D2C5DC57E1774, native_parameters!(toggle));

    value
}

pub fn _set_bink_should_skip(binkMovie: i32, bShouldSkip: bool) -> () {
    let value = native!(
        (),
        0x6805D58CAA427B72,
        native_parameters!(binkMovie, bShouldSkip)
    );

    value
}

pub fn load_movie_mesh_set(movieMeshSetName: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0xB66064452270E8F1,
        native_parameters!(movieMeshSetName.as_ptr())
    );

    value
}

pub fn release_movie_mesh_set(movieMeshSet: i32) -> () {
    let value = native!((), 0xEB119AA014E89183, native_parameters!(movieMeshSet));

    value
}

pub fn query_movie_mesh_set_state(p0: u32) -> u32 {
    let value = native!(u32, 0x9B6E70C5CEEF4EEB, native_parameters!(p0));

    value
}

pub fn get_screen_resolution(x: *mut i32, y: *mut i32) -> () {
    let value = native!((), 0x888D57E407E63624, native_parameters!(x, y));

    value
}

pub fn _get_active_screen_resolution(x: *mut i32, y: *mut i32) -> () {
    let value = native!((), 0x873C9F3104101DD3, native_parameters!(x, y));

    value
}

pub fn _get_aspect_ratio(b: bool) -> f32 {
    let value = native!(f32, 0xF1307EF624A80D87, native_parameters!(b));

    value
}

pub fn _0xb2ebe8cbc58b90e9() -> u32 {
    let value = native!(u32, 0xB2EBE8CBC58B90E9, native_parameters!());

    value
}

pub fn get_is_widescreen() -> bool {
    let value = native!(bool, 0x30CF4BDA4FCB1905, native_parameters!());

    value
}

pub fn get_is_hidef() -> bool {
    let value = native!(bool, 0x84ED31191CC5D2C9, native_parameters!());

    value
}

pub fn _0xefabc7722293da7c() -> () {
    let value = native!((), 0xEFABC7722293DA7C, native_parameters!());

    value
}

pub fn set_nightvision(toggle: bool) -> () {
    let value = native!((), 0x18F621F7A5B1F85D, native_parameters!(toggle));

    value
}

pub fn get_requestingnightvision() -> bool {
    let value = native!(bool, 0x35FB78DC42B7BD21, native_parameters!());

    value
}

pub fn get_usingnightvision() -> bool {
    let value = native!(bool, 0x2202A3F42C8E5F79, native_parameters!());

    value
}

pub fn _0xef398beee4ef45f9(p0: bool) -> () {
    let value = native!((), 0xEF398BEEE4EF45F9, native_parameters!(p0));

    value
}

pub fn _0x814af7dcaacc597b(p0: u32) -> () {
    let value = native!((), 0x814AF7DCAACC597B, native_parameters!(p0));

    value
}

pub fn _0x43fa7cbe20dab219(p0: u32) -> () {
    let value = native!((), 0x43FA7CBE20DAB219, native_parameters!(p0));

    value
}

pub fn set_noiseoveride(toggle: bool) -> () {
    let value = native!((), 0xE787BF1C5CF823C9, native_parameters!(toggle));

    value
}

pub fn set_noisinessoveride(value: f32) -> () {
    let value = native!((), 0xCB6A7C3BB17A0C67, native_parameters!(value));

    value
}

pub fn get_screen_coord_from_world_coord(
    worldX: f32,
    worldY: f32,
    worldZ: f32,
    screenX: *mut f32,
    screenY: *mut f32,
) -> bool {
    let value = native!(
        bool,
        0x34E82F05DF2974F5,
        native_parameters!(worldX, worldY, worldZ, screenX, screenY)
    );

    value
}

pub fn get_texture_resolution(
    textureDict: &std::ffi::CString,
    textureName: &std::ffi::CString,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x35736EE65BD00C11,
        native_parameters!(textureDict.as_ptr(), textureName.as_ptr())
    );

    value
}

pub fn _override_ped_badge_texture(
    ped: i32,
    txd: &std::ffi::CString,
    txn: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x95EB5E34F821BABE,
        native_parameters!(ped, txd.as_ptr(), txn.as_ptr())
    );

    value
}

pub fn _0xe2892e7e55d7073a(p0: f32) -> () {
    let value = native!((), 0xE2892E7E55D7073A, native_parameters!(p0));

    value
}

pub fn set_flash(p0: f32, p1: f32, fadeIn: f32, duration: f32, fadeOut: f32) -> () {
    let value = native!(
        (),
        0x0AB84296FED9CFC6,
        native_parameters!(p0, p1, fadeIn, duration, fadeOut)
    );

    value
}

pub fn disable_occlusion_this_frame() -> () {
    let value = native!((), 0x3669F1B198DCAA4F, native_parameters!());

    value
}

pub fn set_artificial_lights_state(state: bool) -> () {
    let value = native!((), 0x1268615ACE24D504, native_parameters!(state));

    value
}

pub fn _set_artificial_lights_state_affects_vehicles(toggle: bool) -> () {
    let value = native!((), 0xE2B187C0939B3D32, native_parameters!(toggle));

    value
}

pub fn _0xc35a6d07c93802b2() -> () {
    let value = native!((), 0xC35A6D07C93802B2, native_parameters!());

    value
}

pub fn create_tracked_point() -> i32 {
    let value = native!(i32, 0xE2C9439ED45DEA60, native_parameters!());

    value
}

pub fn set_tracked_point_info(point: i32, x: f32, y: f32, z: f32, radius: f32) -> () {
    let value = native!(
        (),
        0x164ECBB3CF750CB0,
        native_parameters!(point, x, y, z, radius)
    );

    value
}

pub fn is_tracked_point_visible(point: i32) -> bool {
    let value = native!(bool, 0xC45CCDAAC9221CA8, native_parameters!(point));

    value
}

pub fn destroy_tracked_point(point: i32) -> () {
    let value = native!((), 0xB25DC90BAD56CA42, native_parameters!(point));

    value
}

pub fn _0xbe197eaa669238f4(p0: u32, p1: u32, p2: u32, p3: u32) -> u32 {
    let value = native!(u32, 0xBE197EAA669238F4, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x61f95e5bb3e0a8c6(p0: u32) -> () {
    let value = native!((), 0x61F95E5BB3E0A8C6, native_parameters!(p0));

    value
}

pub fn _0xae51bc858f32ba66(p0: u32, p1: f32, p2: f32, p3: f32, p4: f32) -> () {
    let value = native!(
        (),
        0xAE51BC858F32BA66,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _0x649c97d52332341a(p0: u32) -> () {
    let value = native!((), 0x649C97D52332341A, native_parameters!(p0));

    value
}

pub fn _0x2c42340f916c5930(p0: u32) -> u32 {
    let value = native!(u32, 0x2C42340F916C5930, native_parameters!(p0));

    value
}

pub fn _0x14fc5833464340a8() -> () {
    let value = native!((), 0x14FC5833464340A8, native_parameters!());

    value
}

pub fn _0x0218ba067d249dea() -> () {
    let value = native!((), 0x0218BA067D249DEA, native_parameters!());

    value
}

pub fn _0x1612c45f9e3e0d44() -> () {
    let value = native!((), 0x1612C45F9E3E0D44, native_parameters!());

    value
}

pub fn _0x5debd9c4dc995692() -> () {
    let value = native!((), 0x5DEBD9C4DC995692, native_parameters!());

    value
}

pub fn _0xaae9be70ec7c69ab(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
) -> () {
    let value = native!(
        (),
        0xAAE9BE70EC7C69AB,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn _grass_lod_shrink_script_areas(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    p4: f32,
    p5: f32,
    p6: f32,
) -> () {
    let value = native!(
        (),
        0x6D955F6A9E0295B1,
        native_parameters!(x, y, z, radius, p4, p5, p6)
    );

    value
}

pub fn _grass_lod_reset_script_areas() -> () {
    let value = native!((), 0x302C91AB2D477F7E, native_parameters!());

    value
}

pub fn cascade_shadows_init_session() -> () {
    let value = native!((), 0x03FC694AE06C5A20, native_parameters!());

    value
}

pub fn cascade_shadows_set_cascade_bounds(
    p0: u32,
    p1: bool,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: bool,
    p7: f32,
) -> () {
    let value = native!(
        (),
        0xD2936CAB8B58FCBD,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn cascade_shadows_set_cascade_bounds_scale(p0: f32) -> () {
    let value = native!((), 0x5F0F3F56635809EF, native_parameters!(p0));

    value
}

pub fn cascade_shadows_set_entity_tracker_scale(p0: f32) -> () {
    let value = native!((), 0x5E9DAF5A20F15908, native_parameters!(p0));

    value
}

pub fn _0x36f6626459d91457(p0: f32) -> () {
    let value = native!((), 0x36F6626459D91457, native_parameters!(p0));

    value
}

pub fn _0x259ba6d4e6f808f1(p0: u32) -> () {
    let value = native!((), 0x259BA6D4E6F808F1, native_parameters!(p0));

    value
}

pub fn cascade_shadows_enable_entity_tracker(toggle: bool) -> () {
    let value = native!((), 0x80ECBC0C856D3B0B, native_parameters!(toggle));

    value
}

pub fn _0x25fc3e33a31ad0c9(p0: bool) -> () {
    let value = native!((), 0x25FC3E33A31AD0C9, native_parameters!(p0));

    value
}

pub fn cascade_shadows_set_shadow_sample_type(type_esc: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xB11D94BC55F41932,
        native_parameters!(type_esc.as_ptr())
    );

    value
}

pub fn _cascade_shadows_clear_shadow_sample_type() -> () {
    let value = native!((), 0x27CB772218215325, native_parameters!());

    value
}

pub fn cascade_shadows_set_aircraft_mode(p0: bool) -> () {
    let value = native!((), 0x6DDBF9DFFC4AC080, native_parameters!(p0));

    value
}

pub fn cascade_shadows_set_dynamic_depth_mode(p0: bool) -> () {
    let value = native!((), 0xD39D13C9FEBF0511, native_parameters!(p0));

    value
}

pub fn cascade_shadows_set_dynamic_depth_value(p0: f32) -> () {
    let value = native!((), 0x02AC28F3A01FA04A, native_parameters!(p0));

    value
}

pub fn _0x0ae73d8df3a762b2(p0: bool) -> () {
    let value = native!((), 0x0AE73D8DF3A762B2, native_parameters!(p0));

    value
}

pub fn _0xca465d9cc0d231ba(p0: u32) -> () {
    let value = native!((), 0xCA465D9CC0D231BA, native_parameters!(p0));

    value
}

pub fn golf_trail_set_enabled(toggle: bool) -> () {
    let value = native!((), 0xA51C4B86B71652AE, native_parameters!(toggle));

    value
}

pub fn golf_trail_set_path(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
    p8: bool,
) -> () {
    let value = native!(
        (),
        0x312342E1A4874F3F,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn golf_trail_set_radius(p0: f32, p1: f32, p2: f32) -> () {
    let value = native!((), 0x2485D34E50A22E84, native_parameters!(p0, p1, p2));

    value
}

pub fn golf_trail_set_colour(
    p0: i32,
    p1: i32,
    p2: i32,
    p3: i32,
    p4: i32,
    p5: i32,
    p6: i32,
    p7: i32,
    p8: i32,
    p9: i32,
    p10: i32,
    p11: i32,
) -> () {
    let value = native!(
        (),
        0x12995F2E53FFA601,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11)
    );

    value
}

pub fn golf_trail_set_tessellation(p0: i32, p1: i32) -> () {
    let value = native!((), 0xDBAA5EC848BA2D46, native_parameters!(p0, p1));

    value
}

pub fn _0xc0416b061f2b7e5e(p0: bool) -> () {
    let value = native!((), 0xC0416B061F2B7E5E, native_parameters!(p0));

    value
}

pub fn golf_trail_set_fixed_control_point(
    type_esc: i32,
    xPos: f32,
    yPos: f32,
    zPos: f32,
    p4: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
) -> () {
    let value = native!(
        (),
        0xB1BB03742917A5D6,
        native_parameters!(type_esc, xPos, yPos, zPos, p4, red, green, blue, alpha)
    );

    value
}

pub fn golf_trail_set_shader_params(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32) -> () {
    let value = native!(
        (),
        0x9CFDD90B2B844BF7,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn golf_trail_set_facing(p0: bool) -> () {
    let value = native!((), 0x06F761EA47C1D3ED, native_parameters!(p0));

    value
}

pub fn golf_trail_get_max_height() -> f32 {
    let value = native!(f32, 0xA4819F5E23E2FFAD, native_parameters!());

    value
}

pub fn golf_trail_get_visual_control_point(p0: i32) -> NativeVector3 {
    let value = native!(NativeVector3, 0xA4664972A9B8F8BA, native_parameters!(p0));

    value
}

pub fn set_seethrough(toggle: bool) -> () {
    let value = native!((), 0x7E08924259E08CE0, native_parameters!(toggle));

    value
}

pub fn get_usingseethrough() -> bool {
    let value = native!(bool, 0x44B80ABAB9D80BD3, native_parameters!());

    value
}

pub fn seethrough_reset() -> () {
    let value = native!((), 0x70A64C0234EF522C, native_parameters!());

    value
}

pub fn _seethrough_set_fade_start_distance(distance: f32) -> () {
    let value = native!((), 0xA78DE25577300BA1, native_parameters!(distance));

    value
}

pub fn _seethrough_set_fade_end_distance(distance: f32) -> () {
    let value = native!((), 0x9D75795B9DC6EBBF, native_parameters!(distance));

    value
}

pub fn _seethrough_get_max_thickness() -> f32 {
    let value = native!(f32, 0x43DBAE39626CE83F, native_parameters!());

    value
}

pub fn _seethrough_set_max_thickness(thickness: f32) -> () {
    let value = native!((), 0x0C8FAC83902A62DF, native_parameters!(thickness));

    value
}

pub fn _seethrough_set_noise_amount_min(amount: f32) -> () {
    let value = native!((), 0xFF5992E1C9E65D05, native_parameters!(amount));

    value
}

pub fn _seethrough_set_noise_amount_max(amount: f32) -> () {
    let value = native!((), 0xFEBFBFDFB66039DE, native_parameters!(amount));

    value
}

pub fn _seethrough_set_hi_light_intensity(intensity: f32) -> () {
    let value = native!((), 0x19E50EB6E33E1D28, native_parameters!(intensity));

    value
}

pub fn _seethrough_set_hi_light_noise(noise: f32) -> () {
    let value = native!((), 0x1636D7FC127B10D2, native_parameters!(noise));

    value
}

pub fn seethrough_set_heatscale(index: i32, heatScale: f32) -> () {
    let value = native!((), 0xD7D0B00177485411, native_parameters!(index, heatScale));

    value
}

pub fn _seethrough_set_color_near(red: i32, green: i32, blue: i32) -> () {
    let value = native!((), 0x1086127B3A63505E, native_parameters!(red, green, blue));

    value
}

pub fn _0xb3c641f3630bf6da(p0: f32) -> () {
    let value = native!((), 0xB3C641F3630BF6DA, native_parameters!(p0));

    value
}

pub fn _0xe59343e9e96529e7() -> f32 {
    let value = native!(f32, 0xE59343E9E96529E7, native_parameters!());

    value
}

pub fn _0x6a51f78772175a51(toggle: bool) -> () {
    let value = native!((), 0x6A51F78772175A51, native_parameters!(toggle));

    value
}

pub fn _0xe63d7c6eececb66b(toggle: bool) -> () {
    let value = native!((), 0xE63D7C6EECECB66B, native_parameters!(toggle));

    value
}

pub fn _0xe3e2c1b4c59dbc77(unk: i32) -> () {
    let value = native!((), 0xE3E2C1B4C59DBC77, native_parameters!(unk));

    value
}

pub fn trigger_screenblur_fade_in(transitionTime: f32) -> bool {
    let value = native!(bool, 0xA328A24AAA6B7FDC, native_parameters!(transitionTime));

    value
}

pub fn trigger_screenblur_fade_out(transitionTime: f32) -> bool {
    let value = native!(bool, 0xEFACC8AEF94430D5, native_parameters!(transitionTime));

    value
}

pub fn disable_screenblur_fade() -> () {
    let value = native!((), 0xDE81239437E8C5A8, native_parameters!());

    value
}

pub fn get_screenblur_fade_current_time() -> f32 {
    let value = native!(f32, 0x5CCABFFCA31DDE33, native_parameters!());

    value
}

pub fn is_screenblur_fade_running() -> bool {
    let value = native!(bool, 0x7B226C785A52A0A9, native_parameters!());

    value
}

pub fn toggle_paused_renderphases(toggle: bool) -> () {
    let value = native!((), 0xDFC252D8A3E15AB7, native_parameters!(toggle));

    value
}

pub fn get_toggle_paused_renderphases_status() -> bool {
    let value = native!(bool, 0xEB3DAC2C86001E5E, native_parameters!());

    value
}

pub fn reset_paused_renderphases() -> () {
    let value = native!((), 0xE1C8709406F2C41C, native_parameters!());

    value
}

pub fn _0x851cd923176eba7c() -> () {
    let value = native!((), 0x851CD923176EBA7C, native_parameters!());

    value
}

pub fn _set_hidof_env_blur_params(
    p0: bool,
    p1: bool,
    nearplaneOut: f32,
    nearplaneIn: f32,
    farplaneOut: f32,
    farplaneIn: f32,
) -> () {
    let value = native!(
        (),
        0xBA3D65906822BED5,
        native_parameters!(p0, p1, nearplaneOut, nearplaneIn, farplaneOut, farplaneIn)
    );

    value
}

pub fn _0xb569f41f3e7e83a4(p0: u32) -> () {
    let value = native!((), 0xB569F41F3E7E83A4, native_parameters!(p0));

    value
}

pub fn _0x7ac24eab6d74118d(p0: bool) -> bool {
    let value = native!(bool, 0x7AC24EAB6D74118D, native_parameters!(p0));

    value
}

pub fn _0xbcedb009461da156() -> u32 {
    let value = native!(u32, 0xBCEDB009461DA156, native_parameters!());

    value
}

pub fn _0x27feb5254759cde3(textureDict: &std::ffi::CString, p1: bool) -> bool {
    let value = native!(
        bool,
        0x27FEB5254759CDE3,
        native_parameters!(textureDict.as_ptr(), p1)
    );

    value
}

pub fn start_particle_fx_non_looped_at_coord(
    effectName: &std::ffi::CString,
    xPos: f32,
    yPos: f32,
    zPos: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    scale: f32,
    xAxis: bool,
    yAxis: bool,
    zAxis: bool,
) -> i32 {
    let value = native!(
        i32,
        0x25129531F77B9ED3,
        native_parameters!(
            effectName.as_ptr(),
            xPos,
            yPos,
            zPos,
            xRot,
            yRot,
            zRot,
            scale,
            xAxis,
            yAxis,
            zAxis
        )
    );

    value
}

pub fn start_networked_particle_fx_non_looped_at_coord(
    effectName: &std::ffi::CString,
    xPos: f32,
    yPos: f32,
    zPos: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    scale: f32,
    xAxis: bool,
    yAxis: bool,
    zAxis: bool,
    p11: bool,
) -> bool {
    let value = native!(
        bool,
        0xF56B8137DF10135D,
        native_parameters!(
            effectName.as_ptr(),
            xPos,
            yPos,
            zPos,
            xRot,
            yRot,
            zRot,
            scale,
            xAxis,
            yAxis,
            zAxis,
            p11
        )
    );

    value
}

pub fn start_particle_fx_non_looped_on_ped_bone(
    effectName: &std::ffi::CString,
    ped: i32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    boneIndex: i32,
    scale: f32,
    axisX: bool,
    axisY: bool,
    axisZ: bool,
) -> bool {
    let value = native!(
        bool,
        0x0E7E72961BA18619,
        native_parameters!(
            effectName.as_ptr(),
            ped,
            offsetX,
            offsetY,
            offsetZ,
            rotX,
            rotY,
            rotZ,
            boneIndex,
            scale,
            axisX,
            axisY,
            axisZ
        )
    );

    value
}

pub fn start_networked_particle_fx_non_looped_on_ped_bone(
    effectName: &std::ffi::CString,
    ped: i32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    boneIndex: i32,
    scale: f32,
    axisX: bool,
    axisY: bool,
    axisZ: bool,
) -> bool {
    let value = native!(
        bool,
        0xA41B6A43642AC2CF,
        native_parameters!(
            effectName.as_ptr(),
            ped,
            offsetX,
            offsetY,
            offsetZ,
            rotX,
            rotY,
            rotZ,
            boneIndex,
            scale,
            axisX,
            axisY,
            axisZ
        )
    );

    value
}

pub fn start_particle_fx_non_looped_on_entity(
    effectName: &std::ffi::CString,
    entity: i32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    scale: f32,
    axisX: bool,
    axisY: bool,
    axisZ: bool,
) -> bool {
    let value = native!(
        bool,
        0x0D53A3B8DA0809D2,
        native_parameters!(
            effectName.as_ptr(),
            entity,
            offsetX,
            offsetY,
            offsetZ,
            rotX,
            rotY,
            rotZ,
            scale,
            axisX,
            axisY,
            axisZ
        )
    );

    value
}

pub fn start_networked_particle_fx_non_looped_on_entity(
    effectName: &std::ffi::CString,
    entity: i32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    scale: f32,
    axisX: bool,
    axisY: bool,
    axisZ: bool,
) -> bool {
    let value = native!(
        bool,
        0xC95EB1DB6E92113D,
        native_parameters!(
            effectName.as_ptr(),
            entity,
            offsetX,
            offsetY,
            offsetZ,
            rotX,
            rotY,
            rotZ,
            scale,
            axisX,
            axisY,
            axisZ
        )
    );

    value
}

pub fn _start_networked_particle_fx_non_looped_on_entity_bone(
    effectName: &std::ffi::CString,
    entity: i32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    boneIndex: i32,
    scale: f32,
    axisX: bool,
    axisY: bool,
    axisZ: bool,
) -> bool {
    let value = native!(
        bool,
        0x02B1F2A72E0F5325,
        native_parameters!(
            effectName.as_ptr(),
            entity,
            offsetX,
            offsetY,
            offsetZ,
            rotX,
            rotY,
            rotZ,
            boneIndex,
            scale,
            axisX,
            axisY,
            axisZ
        )
    );

    value
}

pub fn set_particle_fx_non_looped_colour(r: f32, g: f32, b: f32) -> () {
    let value = native!((), 0x26143A59EF48B262, native_parameters!(r, g, b));

    value
}

pub fn set_particle_fx_non_looped_alpha(alpha: f32) -> () {
    let value = native!((), 0x77168D722C58B2FC, native_parameters!(alpha));

    value
}

pub fn _0x8cde909a0370bb3a(toggle: bool) -> () {
    let value = native!((), 0x8CDE909A0370BB3A, native_parameters!(toggle));

    value
}

pub fn start_particle_fx_looped_at_coord(
    effectName: &std::ffi::CString,
    x: f32,
    y: f32,
    z: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    scale: f32,
    xAxis: bool,
    yAxis: bool,
    zAxis: bool,
    p11: bool,
) -> i32 {
    let value = native!(
        i32,
        0xE184F4F0DC5910E7,
        native_parameters!(
            effectName.as_ptr(),
            x,
            y,
            z,
            xRot,
            yRot,
            zRot,
            scale,
            xAxis,
            yAxis,
            zAxis,
            p11
        )
    );

    value
}

pub fn start_particle_fx_looped_on_ped_bone(
    effectName: &std::ffi::CString,
    ped: i32,
    xOffset: f32,
    yOffset: f32,
    zOffset: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    boneIndex: i32,
    scale: f32,
    xAxis: bool,
    yAxis: bool,
    zAxis: bool,
) -> i32 {
    let value = native!(
        i32,
        0xF28DA9F38CD1787C,
        native_parameters!(
            effectName.as_ptr(),
            ped,
            xOffset,
            yOffset,
            zOffset,
            xRot,
            yRot,
            zRot,
            boneIndex,
            scale,
            xAxis,
            yAxis,
            zAxis
        )
    );

    value
}

pub fn start_particle_fx_looped_on_entity(
    effectName: &std::ffi::CString,
    entity: i32,
    xOffset: f32,
    yOffset: f32,
    zOffset: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    scale: f32,
    xAxis: bool,
    yAxis: bool,
    zAxis: bool,
) -> i32 {
    let value = native!(
        i32,
        0x1AE42C1660FD6517,
        native_parameters!(
            effectName.as_ptr(),
            entity,
            xOffset,
            yOffset,
            zOffset,
            xRot,
            yRot,
            zRot,
            scale,
            xAxis,
            yAxis,
            zAxis
        )
    );

    value
}

pub fn start_particle_fx_looped_on_entity_bone(
    effectName: &std::ffi::CString,
    entity: i32,
    xOffset: f32,
    yOffset: f32,
    zOffset: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    boneIndex: i32,
    scale: f32,
    xAxis: bool,
    yAxis: bool,
    zAxis: bool,
) -> i32 {
    let value = native!(
        i32,
        0xC6EB449E33977F0B,
        native_parameters!(
            effectName.as_ptr(),
            entity,
            xOffset,
            yOffset,
            zOffset,
            xRot,
            yRot,
            zRot,
            boneIndex,
            scale,
            xAxis,
            yAxis,
            zAxis
        )
    );

    value
}

pub fn start_networked_particle_fx_looped_on_entity(
    effectName: &std::ffi::CString,
    entity: i32,
    xOffset: f32,
    yOffset: f32,
    zOffset: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    scale: f32,
    xAxis: bool,
    yAxis: bool,
    zAxis: bool,
    p12: u32,
    p13: u32,
    p14: u32,
    p15: u32,
) -> i32 {
    let value = native!(
        i32,
        0x6F60E89A7B64EE1D,
        native_parameters!(
            effectName.as_ptr(),
            entity,
            xOffset,
            yOffset,
            zOffset,
            xRot,
            yRot,
            zRot,
            scale,
            xAxis,
            yAxis,
            zAxis,
            p12,
            p13,
            p14,
            p15
        )
    );

    value
}

pub fn start_networked_particle_fx_looped_on_entity_bone(
    effectName: &std::ffi::CString,
    entity: i32,
    xOffset: f32,
    yOffset: f32,
    zOffset: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    boneIndex: i32,
    scale: f32,
    xAxis: bool,
    yAxis: bool,
    zAxis: bool,
    p13: u32,
    p14: u32,
    p15: u32,
    p16: u32,
) -> i32 {
    let value = native!(
        i32,
        0xDDE23F30CC5A0F03,
        native_parameters!(
            effectName.as_ptr(),
            entity,
            xOffset,
            yOffset,
            zOffset,
            xRot,
            yRot,
            zRot,
            boneIndex,
            scale,
            xAxis,
            yAxis,
            zAxis,
            p13,
            p14,
            p15,
            p16
        )
    );

    value
}

pub fn stop_particle_fx_looped(ptfxHandle: i32, p1: bool) -> () {
    let value = native!((), 0x8F75998877616996, native_parameters!(ptfxHandle, p1));

    value
}

pub fn remove_particle_fx(ptfxHandle: i32, p1: bool) -> () {
    let value = native!((), 0xC401503DFE8D53CF, native_parameters!(ptfxHandle, p1));

    value
}

pub fn remove_particle_fx_from_entity(entity: i32) -> () {
    let value = native!((), 0xB8FEAEEBCC127425, native_parameters!(entity));

    value
}

pub fn remove_particle_fx_in_range(X: f32, Y: f32, Z: f32, radius: f32) -> () {
    let value = native!((), 0xDD19FA1C6D657305, native_parameters!(X, Y, Z, radius));

    value
}

pub fn _0xba0127da25fd54c9(p0: u32, p1: u32) -> () {
    let value = native!((), 0xBA0127DA25FD54C9, native_parameters!(p0, p1));

    value
}

pub fn does_particle_fx_looped_exist(ptfxHandle: i32) -> bool {
    let value = native!(bool, 0x74AFEF0D2E1E409B, native_parameters!(ptfxHandle));

    value
}

pub fn set_particle_fx_looped_offsets(
    ptfxHandle: i32,
    x: f32,
    y: f32,
    z: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
) -> () {
    let value = native!(
        (),
        0xF7DDEBEC43483C43,
        native_parameters!(ptfxHandle, x, y, z, rotX, rotY, rotZ)
    );

    value
}

pub fn set_particle_fx_looped_evolution(
    ptfxHandle: i32,
    propertyName: &std::ffi::CString,
    amount: f32,
    noNetwork: bool,
) -> () {
    let value = native!(
        (),
        0x5F0C4B5B1C393BE2,
        native_parameters!(ptfxHandle, propertyName.as_ptr(), amount, noNetwork)
    );

    value
}

pub fn set_particle_fx_looped_colour(ptfxHandle: i32, r: f32, g: f32, b: f32, p4: bool) -> () {
    let value = native!(
        (),
        0x7F8F65877F88783B,
        native_parameters!(ptfxHandle, r, g, b, p4)
    );

    value
}

pub fn set_particle_fx_looped_alpha(ptfxHandle: i32, alpha: f32) -> () {
    let value = native!(
        (),
        0x726845132380142E,
        native_parameters!(ptfxHandle, alpha)
    );

    value
}

pub fn set_particle_fx_looped_scale(ptfxHandle: i32, scale: f32) -> () {
    let value = native!(
        (),
        0xB44250AAA456492D,
        native_parameters!(ptfxHandle, scale)
    );

    value
}

pub fn set_particle_fx_looped_far_clip_dist(ptfxHandle: i32, range: f32) -> () {
    let value = native!(
        (),
        0xDCB194B85EF7B541,
        native_parameters!(ptfxHandle, range)
    );

    value
}

pub fn set_particle_fx_cam_inside_vehicle(p0: bool) -> () {
    let value = native!((), 0xEEC4047028426510, native_parameters!(p0));

    value
}

pub fn set_particle_fx_cam_inside_nonplayer_vehicle(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0xACEE6F360FC1F6B6, native_parameters!(vehicle, p1));

    value
}

pub fn set_particle_fx_shootout_boat(p0: u32) -> () {
    let value = native!((), 0x96EF97DAEB89BEF5, native_parameters!(p0));

    value
}

pub fn _0x2a251aa48b2b46db() -> () {
    let value = native!((), 0x2A251AA48B2B46DB, native_parameters!());

    value
}

pub fn _0x908311265d42a820(p0: u32) -> () {
    let value = native!((), 0x908311265D42A820, native_parameters!(p0));

    value
}

pub fn _0xcfd16f0db5a3535c(toggle: bool) -> () {
    let value = native!((), 0xCFD16F0DB5A3535C, native_parameters!(toggle));

    value
}

pub fn _0x5f6df3d92271e8a1(toggle: bool) -> () {
    let value = native!((), 0x5F6DF3D92271E8A1, native_parameters!(toggle));

    value
}

pub fn _0x2b40a97646381508(p0: u32) -> () {
    let value = native!((), 0x2B40A97646381508, native_parameters!(p0));

    value
}

pub fn enable_clown_blood_vfx(toggle: bool) -> () {
    let value = native!((), 0xD821490579791273, native_parameters!(toggle));

    value
}

pub fn enable_alien_blood_vfx(toggle: bool) -> () {
    let value = native!((), 0x9DCE1F0F78260875, native_parameters!(toggle));

    value
}

pub fn set_particle_fx_bullet_impact_scale(scale: f32) -> () {
    let value = native!((), 0x27E32866E9A5C416, native_parameters!(scale));

    value
}

pub fn _0xbb90e12cac1dab25(p0: f32) -> () {
    let value = native!((), 0xBB90E12CAC1DAB25, native_parameters!(p0));

    value
}

pub fn _0xca4ae345a153d573(p0: bool) -> () {
    let value = native!((), 0xCA4AE345A153D573, native_parameters!(p0));

    value
}

pub fn _0x54e22ea2c1956a8d(p0: f32) -> () {
    let value = native!((), 0x54E22EA2C1956A8D, native_parameters!(p0));

    value
}

pub fn _0x949f397a288b28b3(p0: f32) -> () {
    let value = native!((), 0x949F397A288B28B3, native_parameters!(p0));

    value
}

pub fn _0xba3d194057c79a7b(p0: &std::ffi::CString) -> () {
    let value = native!((), 0xBA3D194057C79A7B, native_parameters!(p0.as_ptr()));

    value
}

pub fn _0x5dbf05db5926d089(p0: u32) -> () {
    let value = native!((), 0x5DBF05DB5926D089, native_parameters!(p0));

    value
}

pub fn _0x9b079e5221d984d3(p0: bool) -> () {
    let value = native!((), 0x9B079E5221D984D3, native_parameters!(p0));

    value
}

pub fn use_particle_fx_asset(name: &std::ffi::CString) -> () {
    let value = native!((), 0x6C38AF3693A69A91, native_parameters!(name.as_ptr()));

    value
}

pub fn set_particle_fx_override(oldAsset: &std::ffi::CString, newAsset: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xEA1E2D93F6F75ED9,
        native_parameters!(oldAsset.as_ptr(), newAsset.as_ptr())
    );

    value
}

pub fn reset_particle_fx_override(name: &std::ffi::CString) -> () {
    let value = native!((), 0x89C8553DD3274AAE, native_parameters!(name.as_ptr()));

    value
}

pub fn _0xa46b73faa3460ae1(p0: bool) -> () {
    let value = native!((), 0xA46B73FAA3460AE1, native_parameters!(p0));

    value
}

pub fn _0xf78b803082d4386f(p0: f32) -> () {
    let value = native!((), 0xF78B803082D4386F, native_parameters!(p0));

    value
}

pub fn wash_decals_in_range(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x9C30613D50A6ADEF,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn wash_decals_from_vehicle(vehicle: i32, p1: f32) -> () {
    let value = native!((), 0x5B712761429DBC14, native_parameters!(vehicle, p1));

    value
}

pub fn fade_decals_in_range(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0xD77EDADB0420E6E0,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn remove_decals_in_range(x: f32, y: f32, z: f32, range: f32) -> () {
    let value = native!((), 0x5D6B2D4830A67C62, native_parameters!(x, y, z, range));

    value
}

pub fn remove_decals_from_object(obj: i32) -> () {
    let value = native!((), 0xCCF71CBDDF5B6CB9, native_parameters!(obj));

    value
}

pub fn remove_decals_from_object_facing(obj: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xA6F6F70FDC6D144C, native_parameters!(obj, x, y, z));

    value
}

pub fn remove_decals_from_vehicle(vehicle: i32) -> () {
    let value = native!((), 0xE91F1B65F2B48D57, native_parameters!(vehicle));

    value
}

pub fn add_decal(
    decalType: i32,
    posX: f32,
    posY: f32,
    posZ: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
    p8: f32,
    p9: f32,
    width: f32,
    height: f32,
    rCoef: f32,
    gCoef: f32,
    bCoef: f32,
    opacity: f32,
    timeout: f32,
    p17: bool,
    p18: bool,
    p19: bool,
) -> i32 {
    let value = native!(
        i32,
        0xB302244A1839BDAD,
        native_parameters!(
            decalType, posX, posY, posZ, p4, p5, p6, p7, p8, p9, width, height, rCoef, gCoef,
            bCoef, opacity, timeout, p17, p18, p19
        )
    );

    value
}

pub fn add_petrol_decal(
    x: f32,
    y: f32,
    z: f32,
    groundLvl: f32,
    width: f32,
    transparency: f32,
) -> i32 {
    let value = native!(
        i32,
        0x4F5212C7AD880DF8,
        native_parameters!(x, y, z, groundLvl, width, transparency)
    );

    value
}

pub fn start_petrol_trail_decals(p0: f32) -> () {
    let value = native!((), 0x99AC7F0D8B9C893D, native_parameters!(p0));

    value
}

pub fn add_petrol_trail_decal_info(x: f32, y: f32, z: f32, p3: f32) -> () {
    let value = native!((), 0x967278682CB6967A, native_parameters!(x, y, z, p3));

    value
}

pub fn end_petrol_trail_decals() -> () {
    let value = native!((), 0x0A123435A26C36CD, native_parameters!());

    value
}

pub fn remove_decal(decal: i32) -> () {
    let value = native!((), 0xED3F346429CCD659, native_parameters!(decal));

    value
}

pub fn is_decal_alive(decal: i32) -> bool {
    let value = native!(bool, 0xC694D74949CAFD0C, native_parameters!(decal));

    value
}

pub fn get_decal_wash_level(decal: i32) -> f32 {
    let value = native!(f32, 0x323F647679A09103, native_parameters!(decal));

    value
}

pub fn _0xd9454b5752c857dc() -> () {
    let value = native!((), 0xD9454B5752C857DC, native_parameters!());

    value
}

pub fn _0x27cfb1b1e078cb2d() -> () {
    let value = native!((), 0x27CFB1B1E078CB2D, native_parameters!());

    value
}

pub fn set_disable_decal_rendering_this_frame() -> () {
    let value = native!((), 0x4B5CFC83122DF602, native_parameters!());

    value
}

pub fn get_is_petrol_decal_in_range(xCoord: f32, yCoord: f32, zCoord: f32, radius: f32) -> bool {
    let value = native!(
        bool,
        0x2F09F7976C512404,
        native_parameters!(xCoord, yCoord, zCoord, radius)
    );

    value
}

pub fn patch_decal_diffuse_map(
    decalType: i32,
    textureDict: &std::ffi::CString,
    textureName: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0x8A35C742130C6080,
        native_parameters!(decalType, textureDict.as_ptr(), textureName.as_ptr())
    );

    value
}

pub fn unpatch_decal_diffuse_map(decalType: i32) -> () {
    let value = native!((), 0xB7ED70C49521A61D, native_parameters!(decalType));

    value
}

pub fn move_vehicle_decals(p0: u32, p1: u32) -> () {
    let value = native!((), 0x84C8D7C2D30D3280, native_parameters!(p0, p1));

    value
}

pub fn add_vehicle_crew_emblem(
    vehicle: i32,
    ped: i32,
    boneIndex: i32,
    x1: f32,
    x2: f32,
    x3: f32,
    y1: f32,
    y2: f32,
    y3: f32,
    z1: f32,
    z2: f32,
    z3: f32,
    scale: f32,
    p13: u32,
    alpha: i32,
) -> bool {
    let value = native!(
        bool,
        0x428BDCB9DA58DA53,
        native_parameters!(
            vehicle, ped, boneIndex, x1, x2, x3, y1, y2, y3, z1, z2, z3, scale, p13, alpha
        )
    );

    value
}

pub fn _0x82acc484ffa3b05f(p0: u32) -> u32 {
    let value = native!(u32, 0x82ACC484FFA3B05F, native_parameters!(p0));

    value
}

pub fn remove_vehicle_crew_emblem(vehicle: i32, p1: i32) -> () {
    let value = native!((), 0xD2300034310557E4, native_parameters!(vehicle, p1));

    value
}

pub fn get_vehicle_crew_emblem_request_state(vehicle: i32, p1: i32) -> i32 {
    let value = native!(i32, 0xFE26117A5841B2FF, native_parameters!(vehicle, p1));

    value
}

pub fn does_vehicle_have_crew_emblem(vehicle: i32, p1: i32) -> bool {
    let value = native!(bool, 0x060D935D3981A275, native_parameters!(vehicle, p1));

    value
}

pub fn _0x0e4299c549f0d1f1(toggle: bool) -> () {
    let value = native!((), 0x0E4299C549F0D1F1, native_parameters!(toggle));

    value
}

pub fn _0x02369d5c8a51fdcf(toggle: bool) -> () {
    let value = native!((), 0x02369D5C8A51FDCF, native_parameters!(toggle));

    value
}

pub fn _0x46d1a61a21f566fc(p0: f32) -> () {
    let value = native!((), 0x46D1A61A21F566FC, native_parameters!(p0));

    value
}

pub fn override_interior_smoke_name(name: &std::ffi::CString) -> () {
    let value = native!((), 0x2A2A52824DB96700, native_parameters!(name.as_ptr()));

    value
}

pub fn override_interior_smoke_level(level: f32) -> () {
    let value = native!((), 0x1600FD8CF72EBC12, native_parameters!(level));

    value
}

pub fn override_interior_smoke_end() -> () {
    let value = native!((), 0xEFB55E7C25D3B3BE, native_parameters!());

    value
}

pub fn _register_noir_screen_effect_this_frame() -> () {
    let value = native!((), 0xA44FF770DFBC5DAE, native_parameters!());

    value
}

pub fn disable_vehicle_distantlights(toggle: bool) -> () {
    let value = native!((), 0xC9F98AC1884E73A2, native_parameters!(toggle));

    value
}

pub fn _0x03300b57fcac6ddb(p0: bool) -> () {
    let value = native!((), 0x03300B57FCAC6DDB, native_parameters!(p0));

    value
}

pub fn _0x98edf76a7271e4f2() -> () {
    let value = native!((), 0x98EDF76A7271E4F2, native_parameters!());

    value
}

pub fn _set_force_ped_footsteps_tracks(toggle: bool) -> () {
    let value = native!((), 0xAEEDAD1420C65CC0, native_parameters!(toggle));

    value
}

pub fn _set_force_vehicle_trails(toggle: bool) -> () {
    let value = native!((), 0x4CC7F0FEA5283FE0, native_parameters!(toggle));

    value
}

pub fn _disable_script_ambient_effects(p0: u32) -> () {
    let value = native!((), 0xEFD97FF47B745B8D, native_parameters!(p0));

    value
}

pub fn preset_interior_ambient_cache(timecycleModifierName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xD7021272EB0A451E,
        native_parameters!(timecycleModifierName.as_ptr())
    );

    value
}

pub fn set_timecycle_modifier(modifierName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x2C933ABF17A1DF41,
        native_parameters!(modifierName.as_ptr())
    );

    value
}

pub fn set_timecycle_modifier_strength(strength: f32) -> () {
    let value = native!((), 0x82E7FFCD5B2326B3, native_parameters!(strength));

    value
}

pub fn set_transition_timecycle_modifier(modifierName: &std::ffi::CString, transition: f32) -> () {
    let value = native!(
        (),
        0x3BCF567485E1971C,
        native_parameters!(modifierName.as_ptr(), transition)
    );

    value
}

pub fn _0x1cba05ae7bd7ee05(p0: f32) -> () {
    let value = native!((), 0x1CBA05AE7BD7EE05, native_parameters!(p0));

    value
}

pub fn clear_timecycle_modifier() -> () {
    let value = native!((), 0x0F07E7745A236711, native_parameters!());

    value
}

pub fn get_timecycle_modifier_index() -> i32 {
    let value = native!(i32, 0xFDF3D97C674AFB66, native_parameters!());

    value
}

pub fn get_timecycle_transition_modifier_index() -> i32 {
    let value = native!(i32, 0x459FD2C8D0AB78BC, native_parameters!());

    value
}

pub fn _0x98d18905bf723b99() -> u32 {
    let value = native!(u32, 0x98D18905BF723B99, native_parameters!());

    value
}

pub fn push_timecycle_modifier() -> () {
    let value = native!((), 0x58F735290861E6B4, native_parameters!());

    value
}

pub fn pop_timecycle_modifier() -> () {
    let value = native!((), 0x3C8938D7D872211E, native_parameters!());

    value
}

pub fn set_current_player_tcmodifier(modifierName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xBBF327DED94E4DEB,
        native_parameters!(modifierName.as_ptr())
    );

    value
}

pub fn set_player_tcmodifier_transition(value: f32) -> () {
    let value = native!((), 0xBDEB86F4D5809204, native_parameters!(value));

    value
}

pub fn set_next_player_tcmodifier(modifierName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xBF59707B3E5ED531,
        native_parameters!(modifierName.as_ptr())
    );

    value
}

pub fn add_tcmodifier_override(
    modifierName1: &std::ffi::CString,
    modifierName2: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0x1A8E2C8B9CF4549C,
        native_parameters!(modifierName1.as_ptr(), modifierName2.as_ptr())
    );

    value
}

pub fn remove_tcmodifier_override(p0: &std::ffi::CString) -> () {
    let value = native!((), 0x15E33297C3E8DC60, native_parameters!(p0.as_ptr()));

    value
}

pub fn _set_extra_timecycle_modifier(modifierName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x5096FD9CCB49056D,
        native_parameters!(modifierName.as_ptr())
    );

    value
}

pub fn _clear_extra_timecycle_modifier() -> () {
    let value = native!((), 0x92CCC17A7A2285DA, native_parameters!());

    value
}

pub fn _get_extra_timecycle_modifier_index() -> i32 {
    let value = native!(i32, 0xBB0527EC6341496D, native_parameters!());

    value
}

pub fn _set_extra_timecycle_modifier_strength(strength: f32) -> () {
    let value = native!((), 0x2C328AF17210F009, native_parameters!(strength));

    value
}

pub fn _reset_extra_timecycle_modifier_strength() -> () {
    let value = native!((), 0x2BF72AD5B41AA739, native_parameters!());

    value
}

pub fn request_scaleform_movie(scaleformName: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0x11FE353CF9733E6F,
        native_parameters!(scaleformName.as_ptr())
    );

    value
}

pub fn _request_scaleform_movie_2(scaleformName: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0x65E7E78842E74CDB,
        native_parameters!(scaleformName.as_ptr())
    );

    value
}

pub fn request_scaleform_movie_instance(scaleformName: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0xC514489CFB8AF806,
        native_parameters!(scaleformName.as_ptr())
    );

    value
}

pub fn _request_scaleform_movie_interactive(scaleformName: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0xBD06C611BB9048C2,
        native_parameters!(scaleformName.as_ptr())
    );

    value
}

pub fn has_scaleform_movie_loaded(scaleformHandle: i32) -> bool {
    let value = native!(
        bool,
        0x85F01B8D5B90570E,
        native_parameters!(scaleformHandle)
    );

    value
}

pub fn _0x2fcb133ca50a49eb(val: i32) -> bool {
    let value = native!(bool, 0x2FCB133CA50A49EB, native_parameters!(val));

    value
}

pub fn _0x86255b1fc929e33e(val: i32) -> bool {
    let value = native!(bool, 0x86255B1FC929E33E, native_parameters!(val));

    value
}

pub fn has_scaleform_movie_filename_loaded(scaleformName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x0C1C5D756FB5F337,
        native_parameters!(scaleformName.as_ptr())
    );

    value
}

pub fn has_scaleform_container_movie_loaded_into_parent(scaleformHandle: i32) -> bool {
    let value = native!(
        bool,
        0x8217150E1217EBFD,
        native_parameters!(scaleformHandle)
    );

    value
}

pub fn set_scaleform_movie_as_no_longer_needed(scaleformHandle: *mut i32) -> () {
    let value = native!((), 0x1D132D614DD86811, native_parameters!(scaleformHandle));

    value
}

pub fn set_scaleform_movie_to_use_system_time(scaleform: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0x6D8EB211944DCE08,
        native_parameters!(scaleform, toggle)
    );

    value
}

pub fn _0x32f34ff7f617643b(p0: u32, p1: u32) -> () {
    let value = native!((), 0x32F34FF7F617643B, native_parameters!(p0, p1));

    value
}

pub fn _set_scaleform_fit_rendertarget(scaleformHandle: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0xE6A9F00D4240B519,
        native_parameters!(scaleformHandle, toggle)
    );

    value
}

pub fn draw_scaleform_movie(
    scaleformHandle: i32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
    unk: i32,
) -> () {
    let value = native!(
        (),
        0x54972ADAF0294A93,
        native_parameters!(
            scaleformHandle,
            x,
            y,
            width,
            height,
            red,
            green,
            blue,
            alpha,
            unk
        )
    );

    value
}

pub fn draw_scaleform_movie_fullscreen(
    scaleform: i32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
    unk: i32,
) -> () {
    let value = native!(
        (),
        0x0DF606929C105BE1,
        native_parameters!(scaleform, red, green, blue, alpha, unk)
    );

    value
}

pub fn draw_scaleform_movie_fullscreen_masked(
    scaleform1: i32,
    scaleform2: i32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
) -> () {
    let value = native!(
        (),
        0xCF537FDE4FBD4CE5,
        native_parameters!(scaleform1, scaleform2, red, green, blue, alpha)
    );

    value
}

pub fn draw_scaleform_movie_3d(
    scaleform: i32,
    posX: f32,
    posY: f32,
    posZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    p7: f32,
    p8: f32,
    p9: f32,
    scaleX: f32,
    scaleY: f32,
    scaleZ: f32,
    p13: u32,
) -> () {
    let value = native!(
        (),
        0x87D51D72255D4E78,
        native_parameters!(
            scaleform, posX, posY, posZ, rotX, rotY, rotZ, p7, p8, p9, scaleX, scaleY, scaleZ, p13
        )
    );

    value
}

pub fn draw_scaleform_movie_3d_solid(
    scaleform: i32,
    posX: f32,
    posY: f32,
    posZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    p7: f32,
    p8: f32,
    p9: f32,
    scaleX: f32,
    scaleY: f32,
    scaleZ: f32,
    p13: u32,
) -> () {
    let value = native!(
        (),
        0x1CE592FDC749D6F5,
        native_parameters!(
            scaleform, posX, posY, posZ, rotX, rotY, rotZ, p7, p8, p9, scaleX, scaleY, scaleZ, p13
        )
    );

    value
}

pub fn call_scaleform_movie_method(scaleform: i32, method: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xFBD96D87AC96D533,
        native_parameters!(scaleform, method.as_ptr())
    );

    value
}

pub fn call_scaleform_movie_method_with_number(
    scaleform: i32,
    methodName: &std::ffi::CString,
    param1: f32,
    param2: f32,
    param3: f32,
    param4: f32,
    param5: f32,
) -> () {
    let value = native!(
        (),
        0xD0837058AE2E4BEE,
        native_parameters!(
            scaleform,
            methodName.as_ptr(),
            param1,
            param2,
            param3,
            param4,
            param5
        )
    );

    value
}

pub fn call_scaleform_movie_method_with_string(
    scaleform: i32,
    methodName: &std::ffi::CString,
    param1: &std::ffi::CString,
    param2: &std::ffi::CString,
    param3: &std::ffi::CString,
    param4: &std::ffi::CString,
    param5: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0x51BC1ED3CC44E8F7,
        native_parameters!(
            scaleform,
            methodName.as_ptr(),
            param1.as_ptr(),
            param2.as_ptr(),
            param3.as_ptr(),
            param4.as_ptr(),
            param5.as_ptr()
        )
    );

    value
}

pub fn call_scaleform_movie_method_with_number_and_string(
    scaleform: i32,
    methodName: &std::ffi::CString,
    floatParam1: f32,
    floatParam2: f32,
    floatParam3: f32,
    floatParam4: f32,
    floatParam5: f32,
    stringParam1: &std::ffi::CString,
    stringParam2: &std::ffi::CString,
    stringParam3: &std::ffi::CString,
    stringParam4: &std::ffi::CString,
    stringParam5: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0xEF662D8D57E290B1,
        native_parameters!(
            scaleform,
            methodName.as_ptr(),
            floatParam1,
            floatParam2,
            floatParam3,
            floatParam4,
            floatParam5,
            stringParam1.as_ptr(),
            stringParam2.as_ptr(),
            stringParam3.as_ptr(),
            stringParam4.as_ptr(),
            stringParam5.as_ptr()
        )
    );

    value
}

pub fn begin_scaleform_script_hud_movie_method(
    hudComponent: i32,
    methodName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x98C494FD5BDFBFD5,
        native_parameters!(hudComponent, methodName.as_ptr())
    );

    value
}

pub fn begin_scaleform_movie_method(scaleform: i32, methodName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xF6E48914C7A8694E,
        native_parameters!(scaleform, methodName.as_ptr())
    );

    value
}

pub fn begin_scaleform_movie_method_on_frontend(methodName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xAB58C27C2E6123C6,
        native_parameters!(methodName.as_ptr())
    );

    value
}

pub fn begin_scaleform_movie_method_on_frontend_header(methodName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xB9449845F73F5E9C,
        native_parameters!(methodName.as_ptr())
    );

    value
}

pub fn end_scaleform_movie_method() -> () {
    let value = native!((), 0xC6796A8FFA375E53, native_parameters!());

    value
}

pub fn end_scaleform_movie_method_return_value() -> u32 {
    let value = native!(u32, 0xC50AA39A577AF886, native_parameters!());

    value
}

pub fn is_scaleform_movie_method_return_value_ready(methodReturn: i32) -> bool {
    let value = native!(bool, 0x768FF8961BA904D6, native_parameters!(methodReturn));

    value
}

pub fn get_scaleform_movie_method_return_value_int(methodReturn: i32) -> i32 {
    let value = native!(i32, 0x2DE7EFA66B906036, native_parameters!(methodReturn));

    value
}

pub fn _get_scaleform_movie_method_return_value_bool(methodReturn: i32) -> bool {
    let value = native!(bool, 0xD80A80346A45D761, native_parameters!(methodReturn));

    value
}

pub fn get_scaleform_movie_method_return_value_string(methodReturn: i32) -> String {
    let value = native!(
        *const i8,
        0xE1E258829A885245,
        native_parameters!(methodReturn)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn scaleform_movie_method_add_param_int(value: i32) -> () {
    let value = native!((), 0xC3D0841A0CC546A6, native_parameters!(value));

    value
}

pub fn scaleform_movie_method_add_param_float(value: f32) -> () {
    let value = native!((), 0xD69736AAE04DB51A, native_parameters!(value));

    value
}

pub fn scaleform_movie_method_add_param_bool(value: bool) -> () {
    let value = native!((), 0xC58424BA936EB458, native_parameters!(value));

    value
}

pub fn begin_text_command_scaleform_string(componentType: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x80338406F3475E55,
        native_parameters!(componentType.as_ptr())
    );

    value
}

pub fn end_text_command_scaleform_string() -> () {
    let value = native!((), 0x362E2D3FE93A9959, native_parameters!());

    value
}

pub fn _end_text_command_scaleform_string_2() -> () {
    let value = native!((), 0xAE4E8157D9ECF087, native_parameters!());

    value
}

pub fn _scaleform_movie_method_add_param_texture_name_string_2(string: &std::ffi::CString) -> () {
    let value = native!((), 0x77FE3402004CD1B0, native_parameters!(string.as_ptr()));

    value
}

pub fn scaleform_movie_method_add_param_texture_name_string(string: &std::ffi::CString) -> () {
    let value = native!((), 0xBA7148484BD90365, native_parameters!(string.as_ptr()));

    value
}

pub fn scaleform_movie_method_add_param_player_name_string(string: &std::ffi::CString) -> () {
    let value = native!((), 0xE83A3E3557A56640, native_parameters!(string.as_ptr()));

    value
}

pub fn _0x5e657ef1099edd65(p0: u32) -> bool {
    let value = native!(bool, 0x5E657EF1099EDD65, native_parameters!(p0));

    value
}

pub fn scaleform_movie_method_add_param_latest_brief_string(value: i32) -> () {
    let value = native!((), 0xEC52C631A1831C03, native_parameters!(value));

    value
}

pub fn request_scaleform_script_hud_movie(hudComponent: i32) -> () {
    let value = native!((), 0x9304881D6F6537EA, native_parameters!(hudComponent));

    value
}

pub fn has_scaleform_script_hud_movie_loaded(hudComponent: i32) -> bool {
    let value = native!(bool, 0xDF6E5987D2B4D140, native_parameters!(hudComponent));

    value
}

pub fn remove_scaleform_script_hud_movie(hudComponent: i32) -> () {
    let value = native!((), 0xF44A5456AC3F4F97, native_parameters!(hudComponent));

    value
}

pub fn _0xd1c7cb175e012964(scaleformHandle: i32) -> bool {
    let value = native!(
        bool,
        0xD1C7CB175E012964,
        native_parameters!(scaleformHandle)
    );

    value
}

pub fn set_tv_channel(channel: i32) -> () {
    let value = native!((), 0xBAABBB23EB6E484E, native_parameters!(channel));

    value
}

pub fn get_tv_channel() -> i32 {
    let value = native!(i32, 0xFC1E275A90D39995, native_parameters!());

    value
}

pub fn set_tv_volume(volume: f32) -> () {
    let value = native!((), 0x2982BF73F66E9DDC, native_parameters!(volume));

    value
}

pub fn get_tv_volume() -> f32 {
    let value = native!(f32, 0x2170813D3DD8661B, native_parameters!());

    value
}

pub fn draw_tv_channel(
    xPos: f32,
    yPos: f32,
    xScale: f32,
    yScale: f32,
    rotation: f32,
    red: i32,
    green: i32,
    blue: i32,
    alpha: i32,
) -> () {
    let value = native!(
        (),
        0xFDDC2B4ED3C69DF0,
        native_parameters!(xPos, yPos, xScale, yScale, rotation, red, green, blue, alpha)
    );

    value
}

pub fn set_tv_channel_playlist(
    tvChannel: i32,
    playlistName: &std::ffi::CString,
    restart: bool,
) -> () {
    let value = native!(
        (),
        0xF7B38B8305F1FE8B,
        native_parameters!(tvChannel, playlistName.as_ptr(), restart)
    );

    value
}

pub fn set_tv_channel_playlist_at_hour(
    tvChannel: i32,
    playlistName: &std::ffi::CString,
    hour: i32,
) -> () {
    let value = native!(
        (),
        0x2201C576FACAEBE8,
        native_parameters!(tvChannel, playlistName.as_ptr(), hour)
    );

    value
}

pub fn clear_tv_channel_playlist(tvChannel: i32) -> () {
    let value = native!((), 0xBEB3D46BB7F043C0, native_parameters!(tvChannel));

    value
}

pub fn _is_playlist_unk(tvChannel: i32, p1: u32) -> bool {
    let value = native!(bool, 0x1F710BFF7DAE6261, native_parameters!(tvChannel, p1));

    value
}

pub fn _is_tv_playlist_item_playing(videoCliphash: u32) -> bool {
    let value = native!(bool, 0x0AD973CA1E077B60, native_parameters!(videoCliphash));

    value
}

pub fn enable_movie_keyframe_wait(toggle: bool) -> () {
    let value = native!((), 0x74C180030FDE4B69, native_parameters!(toggle));

    value
}

pub fn _0xd1c55b110e4df534(p0: u32) -> () {
    let value = native!((), 0xD1C55B110E4DF534, native_parameters!(p0));

    value
}

pub fn _0x30432a0118736e00() -> u32 {
    let value = native!(u32, 0x30432A0118736E00, native_parameters!());

    value
}

pub fn enable_movie_subtitles(toggle: bool) -> () {
    let value = native!((), 0x873FA65C778AD970, native_parameters!(toggle));

    value
}

pub fn ui3dscene_is_available() -> bool {
    let value = native!(bool, 0xD3A10FC7FD8D98CD, native_parameters!());

    value
}

pub fn ui3dscene_push_preset(presetName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xF1CEA8A4198D8E9A,
        native_parameters!(presetName.as_ptr())
    );

    value
}

pub fn _0x98c4fe6ec34154ca(
    presetName: &std::ffi::CString,
    ped: i32,
    p2: i32,
    posX: f32,
    posY: f32,
    posZ: f32,
) -> bool {
    let value = native!(
        bool,
        0x98C4FE6EC34154CA,
        native_parameters!(presetName.as_ptr(), ped, p2, posX, posY, posZ)
    );

    value
}

pub fn _0x7a42b2e236e71415() -> () {
    let value = native!((), 0x7A42B2E236E71415, native_parameters!());

    value
}

pub fn _0x108be26959a9d9bb(toggle: bool) -> () {
    let value = native!((), 0x108BE26959A9D9BB, native_parameters!(toggle));

    value
}

pub fn terraingrid_activate(toggle: bool) -> () {
    let value = native!((), 0xA356990E161C9E65, native_parameters!(toggle));

    value
}

pub fn terraingrid_set_params(
    x: f32,
    y: f32,
    z: f32,
    p3: f32,
    rotation: f32,
    p5: f32,
    width: f32,
    height: f32,
    p8: f32,
    scale: f32,
    glowIntensity: f32,
    normalHeight: f32,
    heightDiff: f32,
) -> () {
    let value = native!(
        (),
        0x1C4FC5752BCD8E48,
        native_parameters!(
            x,
            y,
            z,
            p3,
            rotation,
            p5,
            width,
            height,
            p8,
            scale,
            glowIntensity,
            normalHeight,
            heightDiff
        )
    );

    value
}

pub fn terraingrid_set_colours(
    lowR: i32,
    lowG: i32,
    lowB: i32,
    lowAlpha: i32,
    r: i32,
    g: i32,
    b: i32,
    alpha: i32,
    highR: i32,
    highG: i32,
    highB: i32,
    highAlpha: i32,
) -> () {
    let value = native!(
        (),
        0x5CE62918F8D703C7,
        native_parameters!(
            lowR, lowG, lowB, lowAlpha, r, g, b, alpha, highR, highG, highB, highAlpha
        )
    );

    value
}

pub fn animpostfx_play(effectName: &std::ffi::CString, duration: i32, looped: bool) -> () {
    let value = native!(
        (),
        0x2206BF9A37B7F724,
        native_parameters!(effectName.as_ptr(), duration, looped)
    );

    value
}

pub fn animpostfx_stop(effectName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x068E835A1D0DC0E3,
        native_parameters!(effectName.as_ptr())
    );

    value
}

pub fn _animpostfx_get_unk(effectName: &std::ffi::CString) -> f32 {
    let value = native!(
        f32,
        0xE35B38A27E8E7179,
        native_parameters!(effectName.as_ptr())
    );

    value
}

pub fn animpostfx_is_running(effectName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x36AD3E690DA5ACEB,
        native_parameters!(effectName.as_ptr())
    );

    value
}

pub fn animpostfx_stop_all() -> () {
    let value = native!((), 0xB4EDDC19532BFB85, native_parameters!());

    value
}

pub fn _animpostfx_stop_and_do_unk(effectName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xD2209BE128B5418C,
        native_parameters!(effectName.as_ptr())
    );

    value
}
