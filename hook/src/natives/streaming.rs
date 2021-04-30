use crate::types::NativeVector3;

pub fn load_all_objects_now() -> () {
    let value = native!((), 0xBD6E84632DD4CB3F, native_parameters!());

    value
}

pub fn load_scene(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x4448EB75B4904BDB, native_parameters!(x, y, z));

    value
}

pub fn network_update_load_scene() -> bool {
    let value = native!(bool, 0xC4582015556D1C46, native_parameters!());

    value
}

pub fn is_network_loading_scene() -> bool {
    let value = native!(bool, 0x41CA5A33160EA4AB, native_parameters!());

    value
}

pub fn set_interior_active(interiorID: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0xE37B76C387BE28ED,
        native_parameters!(interiorID, toggle)
    );

    value
}

pub fn request_model(model: u32) -> () {
    let value = native!((), 0x963D27A58DF860AC, native_parameters!(model));

    value
}

pub fn request_menu_ped_model(model: u32) -> () {
    let value = native!((), 0xA0261AEF7ACFC51E, native_parameters!(model));

    value
}

pub fn has_model_loaded(model: u32) -> bool {
    let value = native!(bool, 0x98A4EB5D89A0C952, native_parameters!(model));

    value
}

pub fn request_models_in_room(interior: i32, roomName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x8A7A40100EDFEC58,
        native_parameters!(interior, roomName.as_ptr())
    );

    value
}

pub fn set_model_as_no_longer_needed(model: u32) -> () {
    let value = native!((), 0xE532F5D78798DAAB, native_parameters!(model));

    value
}

pub fn is_model_in_cdimage(model: u32) -> bool {
    let value = native!(bool, 0x35B9E0803292B641, native_parameters!(model));

    value
}

pub fn is_model_valid(model: u32) -> bool {
    let value = native!(bool, 0xC0296A2EDF545E92, native_parameters!(model));

    value
}

pub fn is_model_a_ped(model: u32) -> bool {
    let value = native!(bool, 0x75816577FEA6DAD5, native_parameters!(model));

    value
}

pub fn is_model_a_vehicle(model: u32) -> bool {
    let value = native!(bool, 0x19AAC8F07BFEC53E, native_parameters!(model));

    value
}

pub fn request_collision_at_coord(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x07503F7948F491A7, native_parameters!(x, y, z));

    value
}

pub fn request_collision_for_model(model: u32) -> () {
    let value = native!((), 0x923CB32A3B874FCB, native_parameters!(model));

    value
}

pub fn has_collision_for_model_loaded(model: u32) -> bool {
    let value = native!(bool, 0x22CCA434E368F03A, native_parameters!(model));

    value
}

pub fn request_additional_collision_at_coord(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xC9156DC11411A9EA, native_parameters!(x, y, z));

    value
}

pub fn does_anim_dict_exist(animDict: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x2DA49C3B79856961,
        native_parameters!(animDict.as_ptr())
    );

    value
}

pub fn request_anim_dict(animDict: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xD3BD40951412FEF6,
        native_parameters!(animDict.as_ptr())
    );

    value
}

pub fn has_anim_dict_loaded(animDict: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xD031A9162D01088C,
        native_parameters!(animDict.as_ptr())
    );

    value
}

pub fn remove_anim_dict(animDict: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xF66A602F829E2A06,
        native_parameters!(animDict.as_ptr())
    );

    value
}

pub fn request_anim_set(animSet: &std::ffi::CString) -> () {
    let value = native!((), 0x6EA47DAE7FAD0EED, native_parameters!(animSet.as_ptr()));

    value
}

pub fn has_anim_set_loaded(animSet: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xC4EA073D86FB29B0,
        native_parameters!(animSet.as_ptr())
    );

    value
}

pub fn remove_anim_set(animSet: &std::ffi::CString) -> () {
    let value = native!((), 0x16350528F93024B3, native_parameters!(animSet.as_ptr()));

    value
}

pub fn request_clip_set(clipSet: &std::ffi::CString) -> () {
    let value = native!((), 0xD2A71E1A77418A49, native_parameters!(clipSet.as_ptr()));

    value
}

pub fn has_clip_set_loaded(clipSet: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x318234F4F3738AF3,
        native_parameters!(clipSet.as_ptr())
    );

    value
}

pub fn remove_clip_set(clipSet: &std::ffi::CString) -> () {
    let value = native!((), 0x01F73A131C18CD94, native_parameters!(clipSet.as_ptr()));

    value
}

pub fn request_ipl(iplName: &std::ffi::CString) -> () {
    let value = native!((), 0x41B4893843BBDB74, native_parameters!(iplName.as_ptr()));

    value
}

pub fn remove_ipl(iplName: &std::ffi::CString) -> () {
    let value = native!((), 0xEE6C5AD3ECE0A82D, native_parameters!(iplName.as_ptr()));

    value
}

pub fn is_ipl_active(iplName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x88A741E44A2B3495,
        native_parameters!(iplName.as_ptr())
    );

    value
}

pub fn set_streaming(toggle: bool) -> () {
    let value = native!((), 0x6E0C692677008888, native_parameters!(toggle));

    value
}

pub fn _load_global_water_type(waterType: i32) -> () {
    let value = native!((), 0x7E3F55ED251B76D3, native_parameters!(waterType));

    value
}

pub fn _get_global_water_type() -> i32 {
    let value = native!(i32, 0xF741BD853611592D, native_parameters!());

    value
}

pub fn set_game_pauses_for_streaming(toggle: bool) -> () {
    let value = native!((), 0x717CD6E6FAEBBEDC, native_parameters!(toggle));

    value
}

pub fn set_reduce_ped_model_budget(toggle: bool) -> () {
    let value = native!((), 0x77B5F9A36BF96710, native_parameters!(toggle));

    value
}

pub fn set_reduce_vehicle_model_budget(toggle: bool) -> () {
    let value = native!((), 0x80C527893080CCF3, native_parameters!(toggle));

    value
}

pub fn set_ditch_police_models(toggle: bool) -> () {
    let value = native!((), 0x42CBE54462D92634, native_parameters!(toggle));

    value
}

pub fn get_number_of_streaming_requests() -> i32 {
    let value = native!(i32, 0x4060057271CEBC89, native_parameters!());

    value
}

pub fn request_ptfx_asset() -> () {
    let value = native!((), 0x944955FB2A3935C8, native_parameters!());

    value
}

pub fn has_ptfx_asset_loaded() -> bool {
    let value = native!(bool, 0xCA7D9B86ECA7481B, native_parameters!());

    value
}

pub fn remove_ptfx_asset() -> () {
    let value = native!((), 0x88C6814073DD4A73, native_parameters!());

    value
}

pub fn request_named_ptfx_asset(fxName: &std::ffi::CString) -> () {
    let value = native!((), 0xB80D8756B4668AB6, native_parameters!(fxName.as_ptr()));

    value
}

pub fn has_named_ptfx_asset_loaded(fxName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x8702416E512EC454,
        native_parameters!(fxName.as_ptr())
    );

    value
}

pub fn remove_named_ptfx_asset(fxName: &std::ffi::CString) -> () {
    let value = native!((), 0x5F61EBBE1A00F96D, native_parameters!(fxName.as_ptr()));

    value
}

pub fn set_vehicle_population_budget(p0: i32) -> () {
    let value = native!((), 0xCB9E1EB3BE2AF4E9, native_parameters!(p0));

    value
}

pub fn set_ped_population_budget(p0: i32) -> () {
    let value = native!((), 0x8C95333CFC3340F3, native_parameters!(p0));

    value
}

pub fn clear_focus() -> () {
    let value = native!((), 0x31B73D1EA9F01DA2, native_parameters!());

    value
}

pub fn set_focus_pos_and_vel(
    x: f32,
    y: f32,
    z: f32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
) -> () {
    let value = native!(
        (),
        0xBB7454BAFF08FE25,
        native_parameters!(x, y, z, offsetX, offsetY, offsetZ)
    );

    value
}

pub fn set_focus_entity(entity: i32) -> () {
    let value = native!((), 0x198F77705FA0931D, native_parameters!(entity));

    value
}

pub fn is_entity_focus(entity: i32) -> bool {
    let value = native!(bool, 0x2DDFF3FB9075D747, native_parameters!(entity));

    value
}

pub fn _0x0811381ef5062fec(p0: i32) -> () {
    let value = native!((), 0x0811381EF5062FEC, native_parameters!(p0));

    value
}

pub fn set_mapdatacullbox_enabled(name: &std::ffi::CString, toggle: bool) -> () {
    let value = native!(
        (),
        0xAF12610C644A35C9,
        native_parameters!(name.as_ptr(), toggle)
    );

    value
}

pub fn _0x4e52e752c76e7e7a(p0: u32) -> () {
    let value = native!((), 0x4E52E752C76E7E7A, native_parameters!(p0));

    value
}

pub fn format_focus_heading(x: f32, y: f32, z: f32, rad: f32, p4: u32, p5: u32) -> u32 {
    let value = native!(
        u32,
        0x219C7B8D53E429FD,
        native_parameters!(x, y, z, rad, p4, p5)
    );

    value
}

pub fn _0x1f3f018bc3afa77c(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: u32,
    p8: u32,
) -> u32 {
    let value = native!(
        u32,
        0x1F3F018BC3AFA77C,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn _0x0ad9710cee2f590f(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: u32) -> u32 {
    let value = native!(
        u32,
        0x0AD9710CEE2F590F,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn _0x1ee7d8df4425f053(p0: u32) -> () {
    let value = native!((), 0x1EE7D8DF4425F053, native_parameters!(p0));

    value
}

pub fn _0x7d41e9d2d17c5b2d(p0: u32) -> u32 {
    let value = native!(u32, 0x7D41E9D2D17C5B2D, native_parameters!(p0));

    value
}

pub fn _0x07c313f94746702c(p0: u32) -> u32 {
    let value = native!(u32, 0x07C313F94746702C, native_parameters!(p0));

    value
}

pub fn _0xbc9823ab80a3dcac() -> u32 {
    let value = native!(u32, 0xBC9823AB80A3DCAC, native_parameters!());

    value
}

pub fn new_load_scene_start(
    posX: f32,
    posY: f32,
    posZ: f32,
    offsetX: f32,
    offsetY: f32,
    offsetZ: f32,
    radius: f32,
    p7: i32,
) -> bool {
    let value = native!(
        bool,
        0x212A8D0D2BABFAC2,
        native_parameters!(posX, posY, posZ, offsetX, offsetY, offsetZ, radius, p7)
    );

    value
}

pub fn new_load_scene_start_sphere(x: f32, y: f32, z: f32, radius: f32, p4: u32) -> bool {
    let value = native!(
        bool,
        0xACCFB4ACF53551B0,
        native_parameters!(x, y, z, radius, p4)
    );

    value
}

pub fn new_load_scene_stop() -> () {
    let value = native!((), 0xC197616D221FF4A4, native_parameters!());

    value
}

pub fn is_new_load_scene_active() -> bool {
    let value = native!(bool, 0xA41A05B6CB741B85, native_parameters!());

    value
}

pub fn is_new_load_scene_loaded() -> bool {
    let value = native!(bool, 0x01B8247A7A8B9AD1, native_parameters!());

    value
}

pub fn _0x71e7b2e657449aad() -> u32 {
    let value = native!(u32, 0x71E7B2E657449AAD, native_parameters!());

    value
}

pub fn start_player_switch(from: i32, to: i32, flags: i32, switchType: i32) -> () {
    let value = native!(
        (),
        0xFAA23F2CBA159D67,
        native_parameters!(from, to, flags, switchType)
    );

    value
}

pub fn stop_player_switch() -> () {
    let value = native!((), 0x95C0A5BBDC189AA1, native_parameters!());

    value
}

pub fn is_player_switch_in_progress() -> bool {
    let value = native!(bool, 0xD9D2CFFF49FAB35F, native_parameters!());

    value
}

pub fn get_player_switch_type() -> i32 {
    let value = native!(i32, 0xB3C94A90D9FC9E62, native_parameters!());

    value
}

pub fn get_ideal_player_switch_type(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> i32 {
    let value = native!(
        i32,
        0xB5D7B26B45720E05,
        native_parameters!(x1, y1, z1, x2, y2, z2)
    );

    value
}

pub fn get_player_switch_state() -> i32 {
    let value = native!(i32, 0x470555300D10B2A5, native_parameters!());

    value
}

pub fn get_player_short_switch_state() -> i32 {
    let value = native!(i32, 0x20F898A5D9782800, native_parameters!());

    value
}

pub fn _0x5f2013f8bc24ee69(p0: i32) -> () {
    let value = native!((), 0x5F2013F8BC24EE69, native_parameters!(p0));

    value
}

pub fn get_player_switch_jump_cut_index() -> i32 {
    let value = native!(i32, 0x78C0D93253149435, native_parameters!());

    value
}

pub fn set_player_switch_outro(
    cameraCoordX: f32,
    cameraCoordY: f32,
    cameraCoordZ: f32,
    camRotationX: f32,
    camRotationY: f32,
    camRotationZ: f32,
    camFov: f32,
    camFarClip: f32,
    rotationOrder: i32,
) -> () {
    let value = native!(
        (),
        0xC208B673CE446B61,
        native_parameters!(
            cameraCoordX,
            cameraCoordY,
            cameraCoordZ,
            camRotationX,
            camRotationY,
            camRotationZ,
            camFov,
            camFarClip,
            rotationOrder
        )
    );

    value
}

pub fn set_player_switch_establishing_shot(name: &std::ffi::CString) -> () {
    let value = native!((), 0x0FDE9DBFC0A6BC65, native_parameters!(name.as_ptr()));

    value
}

pub fn allow_player_switch_pan() -> () {
    let value = native!((), 0x43D1680C6D19A8E9, native_parameters!());

    value
}

pub fn allow_player_switch_outro() -> () {
    let value = native!((), 0x74DE2E8739086740, native_parameters!());

    value
}

pub fn allow_player_switch_ascent() -> () {
    let value = native!((), 0x8E2A065ABDAE6994, native_parameters!());

    value
}

pub fn allow_player_switch_descent() -> () {
    let value = native!((), 0xAD5FDF34B81BFE79, native_parameters!());

    value
}

pub fn is_switch_ready_for_descent() -> bool {
    let value = native!(bool, 0xDFA80CB25D0A19B3, native_parameters!());

    value
}

pub fn enable_switch_pause_before_descent() -> () {
    let value = native!((), 0xD4793DFF3AF2ABCD, native_parameters!());

    value
}

pub fn disable_switch_outro_fx() -> () {
    let value = native!((), 0xBD605B8E0E18B3BB, native_parameters!());

    value
}

pub fn _switch_out_player(ped: i32, flags: i32, switchType: i32) -> () {
    let value = native!(
        (),
        0xAAB3200ED59016BC,
        native_parameters!(ped, flags, switchType)
    );

    value
}

pub fn _switch_in_player(ped: i32) -> () {
    let value = native!((), 0xD8295AF639FD9CB8, native_parameters!(ped));

    value
}

pub fn _0x933bbeeb8c61b5f4() -> bool {
    let value = native!(bool, 0x933BBEEB8C61B5F4, native_parameters!());

    value
}

pub fn get_player_switch_interp_out_duration() -> i32 {
    let value = native!(i32, 0x08C2D6C52A3104BB, native_parameters!());

    value
}

pub fn _0x5b48a06dd0e792a5() -> u32 {
    let value = native!(u32, 0x5B48A06DD0E792A5, native_parameters!());

    value
}

pub fn is_switch_skipping_descent() -> bool {
    let value = native!(bool, 0x5B74EA8CFD5E3E7E, native_parameters!());

    value
}

pub fn _0x1e9057a74fd73e23() -> () {
    let value = native!((), 0x1E9057A74FD73E23, native_parameters!());

    value
}

pub fn get_lodscale() -> f32 {
    let value = native!(f32, 0x0C15B0E443B2349D, native_parameters!());

    value
}

pub fn override_lodscale_this_frame(scaling: f32) -> () {
    let value = native!((), 0xA76359FC80B2438E, native_parameters!(scaling));

    value
}

pub fn _0xbed8ca5ff5e04113(p0: f32, p1: f32, p2: f32, p3: f32) -> () {
    let value = native!((), 0xBED8CA5FF5E04113, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x472397322e92a856() -> () {
    let value = native!((), 0x472397322E92A856, native_parameters!());

    value
}

pub fn _0x40aefd1a244741f2(p0: bool) -> () {
    let value = native!((), 0x40AEFD1A244741F2, native_parameters!(p0));

    value
}

pub fn _0x03f1a106bda7dd3e() -> () {
    let value = native!((), 0x03F1A106BDA7DD3E, native_parameters!());

    value
}

pub fn _0x95a7dabddbb78ae7(iplName1: &std::ffi::CString, iplName2: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x95A7DABDDBB78AE7,
        native_parameters!(iplName1.as_ptr(), iplName2.as_ptr())
    );

    value
}

pub fn _0x63eb2b972a218cac() -> () {
    let value = native!((), 0x63EB2B972A218CAC, native_parameters!());

    value
}

pub fn _0xfb199266061f820a() -> bool {
    let value = native!(bool, 0xFB199266061F820A, native_parameters!());

    value
}

pub fn _0xf4a0dadb70f57fa6() -> () {
    let value = native!((), 0xF4A0DADB70F57FA6, native_parameters!());

    value
}

pub fn _0x5068f488ddb54dd8() -> u32 {
    let value = native!(u32, 0x5068F488DDB54DD8, native_parameters!());

    value
}

pub fn prefetch_srl(srl: &std::ffi::CString) -> () {
    let value = native!((), 0x3D245789CE12982C, native_parameters!(srl.as_ptr()));

    value
}

pub fn is_srl_loaded() -> bool {
    let value = native!(bool, 0xD0263801A4C5B0BB, native_parameters!());

    value
}

pub fn begin_srl() -> () {
    let value = native!((), 0x9BADDC94EF83B823, native_parameters!());

    value
}

pub fn end_srl() -> () {
    let value = native!((), 0x0A41540E63C9EE17, native_parameters!());

    value
}

pub fn set_srl_time(p0: f32) -> () {
    let value = native!((), 0xA74A541C6884E7B8, native_parameters!(p0));

    value
}

pub fn _0xef39ee20c537e98c(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0xEF39EE20C537E98C,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _0xbeb2d9a1d9a8f55a(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xBEB2D9A1D9A8F55A, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x20c6c7e4eb082a7f(p0: bool) -> () {
    let value = native!((), 0x20C6C7E4EB082A7F, native_parameters!(p0));

    value
}

pub fn _0xf8155a7f03ddfc8e(p0: u32) -> () {
    let value = native!((), 0xF8155A7F03DDFC8E, native_parameters!(p0));

    value
}

pub fn set_hd_area(x: f32, y: f32, z: f32, radius: f32) -> () {
    let value = native!((), 0xB85F26619073E775, native_parameters!(x, y, z, radius));

    value
}

pub fn clear_hd_area() -> () {
    let value = native!((), 0xCE58B1CFB9290813, native_parameters!());

    value
}

pub fn init_creator_budget() -> () {
    let value = native!((), 0xB5A4DB34FE89B88A, native_parameters!());

    value
}

pub fn shutdown_creator_budget() -> () {
    let value = native!((), 0xCCE26000E9A6FAD7, native_parameters!());

    value
}

pub fn add_model_to_creator_budget(modelHash: u32) -> bool {
    let value = native!(bool, 0x0BC3144DEB678666, native_parameters!(modelHash));

    value
}

pub fn remove_model_from_creator_budget(modelHash: u32) -> () {
    let value = native!((), 0xF086AD9354FAC3A3, native_parameters!(modelHash));

    value
}

pub fn _get_used_creator_model_memory_percentage() -> f32 {
    let value = native!(f32, 0x3D3D8B3BE5A83D35, native_parameters!());

    value
}

pub fn _set_island_hopper_enabled(name: &std::ffi::CString, toggle: bool) -> () {
    let value = native!(
        (),
        0x9A9D1BA639675CF1,
        native_parameters!(name.as_ptr(), toggle)
    );

    value
}
