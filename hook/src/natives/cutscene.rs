use crate::types::NativeVector3;

pub fn request_cutscene(cutsceneName: &std::ffi::CString, flags: i32) -> () {
    let value = native!(
        (),
        0x7A86743F475D9E09,
        native_parameters!(cutsceneName.as_ptr(), flags)
    );

    value
}

pub fn request_cutscene_with_playback_list(
    cutsceneName: &std::ffi::CString,
    playbackFlags: i32,
    flags: i32,
) -> () {
    let value = native!(
        (),
        0xC23DE0E91C30B58C,
        native_parameters!(cutsceneName.as_ptr(), playbackFlags, flags)
    );

    value
}

pub fn remove_cutscene() -> () {
    let value = native!((), 0x440AF51A3462B86F, native_parameters!());

    value
}

pub fn has_cutscene_loaded() -> bool {
    let value = native!(bool, 0xC59F528E9AB9F339, native_parameters!());

    value
}

pub fn has_this_cutscene_loaded(cutsceneName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x228D3D94F8A11C3C,
        native_parameters!(cutsceneName.as_ptr())
    );

    value
}

pub fn _0x8d9df6eca8768583(threadId: i32) -> () {
    let value = native!((), 0x8D9DF6ECA8768583, native_parameters!(threadId));

    value
}

pub fn can_request_assets_for_cutscene_entity() -> bool {
    let value = native!(bool, 0xB56BBBCC2955D9CB, native_parameters!());

    value
}

pub fn is_cutscene_playback_flag_set(flag: i32) -> bool {
    let value = native!(bool, 0x71B74D2AE19338D0, native_parameters!(flag));

    value
}

pub fn set_cutscene_entity_streaming_flags(
    cutsceneEntName: &std::ffi::CString,
    p1: i32,
    p2: i32,
) -> () {
    let value = native!(
        (),
        0x4C61C75BEE8184C2,
        native_parameters!(cutsceneEntName.as_ptr(), p1, p2)
    );

    value
}

pub fn request_cut_file(cutsceneName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x06A3524161C502BA,
        native_parameters!(cutsceneName.as_ptr())
    );

    value
}

pub fn has_cut_file_loaded(cutsceneName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xA1C996C2A744262E,
        native_parameters!(cutsceneName.as_ptr())
    );

    value
}

pub fn remove_cut_file(cutsceneName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xD00D76A7DFC9D852,
        native_parameters!(cutsceneName.as_ptr())
    );

    value
}

pub fn _get_cut_file_num_sections(cutsceneName: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0x0ABC54DE641DC0FC,
        native_parameters!(cutsceneName.as_ptr())
    );

    value
}

pub fn start_cutscene(flags: i32) -> () {
    let value = native!((), 0x186D5CB5E7B0FF7B, native_parameters!(flags));

    value
}

pub fn start_cutscene_at_coords(x: f32, y: f32, z: f32, flags: i32) -> () {
    let value = native!((), 0x1C9ADDA3244A1FBF, native_parameters!(x, y, z, flags));

    value
}

pub fn stop_cutscene(p0: bool) -> () {
    let value = native!((), 0xC7272775B4DC786E, native_parameters!(p0));

    value
}

pub fn stop_cutscene_immediately() -> () {
    let value = native!((), 0xD220BDD222AC4A1E, native_parameters!());

    value
}

pub fn set_cutscene_origin(x: f32, y: f32, z: f32, p3: f32, p4: i32) -> () {
    let value = native!((), 0xB812B3FD1C01CF27, native_parameters!(x, y, z, p3, p4));

    value
}

pub fn _0x011883f41211432a(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, p6: i32) -> () {
    let value = native!(
        (),
        0x011883F41211432A,
        native_parameters!(x1, y1, z1, x2, y2, z2, p6)
    );

    value
}

pub fn get_cutscene_time() -> i32 {
    let value = native!(i32, 0xE625BEABBAFFDAB9, native_parameters!());

    value
}

pub fn get_cutscene_total_duration() -> i32 {
    let value = native!(i32, 0xEE53B14A19E480D4, native_parameters!());

    value
}

pub fn _0x971d7b15bcdbef99() -> i32 {
    let value = native!(i32, 0x971D7B15BCDBEF99, native_parameters!());

    value
}

pub fn was_cutscene_skipped() -> bool {
    let value = native!(bool, 0x40C8656EDAEDD569, native_parameters!());

    value
}

pub fn has_cutscene_finished() -> bool {
    let value = native!(bool, 0x7C0A893088881D57, native_parameters!());

    value
}

pub fn is_cutscene_active() -> bool {
    let value = native!(bool, 0x991251AFC3981F84, native_parameters!());

    value
}

pub fn is_cutscene_playing() -> bool {
    let value = native!(bool, 0xD3C2E180A40F031E, native_parameters!());

    value
}

pub fn get_cutscene_section_playing() -> i32 {
    let value = native!(i32, 0x49010A6A396553D8, native_parameters!());

    value
}

pub fn get_entity_index_of_cutscene_entity(
    cutsceneEntName: &std::ffi::CString,
    modelHash: u32,
) -> i32 {
    let value = native!(
        i32,
        0x0A2E9FDB9A8C62F6,
        native_parameters!(cutsceneEntName.as_ptr(), modelHash)
    );

    value
}

pub fn _0x583df8e3d4afbd98() -> i32 {
    let value = native!(i32, 0x583DF8E3D4AFBD98, native_parameters!());

    value
}

pub fn _0x4cebc1ed31e8925e(cutsceneName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x4CEBC1ED31E8925E,
        native_parameters!(cutsceneName.as_ptr())
    );

    value
}

pub fn _0x4fcd976da686580c(p0: u32) -> u32 {
    let value = native!(u32, 0x4FCD976DA686580C, native_parameters!(p0));

    value
}

pub fn register_entity_for_cutscene(
    cutscenePed: i32,
    cutsceneEntName: &std::ffi::CString,
    p2: i32,
    modelHash: u32,
    p4: i32,
) -> () {
    let value = native!(
        (),
        0xE40C1C56DF95C2E8,
        native_parameters!(cutscenePed, cutsceneEntName.as_ptr(), p2, modelHash, p4)
    );

    value
}

pub fn get_entity_index_of_registered_entity(
    cutsceneEntName: &std::ffi::CString,
    modelHash: u32,
) -> i32 {
    let value = native!(
        i32,
        0xC0741A26499654CD,
        native_parameters!(cutsceneEntName.as_ptr(), modelHash)
    );

    value
}

pub fn _0x7f96f23fa9b73327(modelHash: u32) -> () {
    let value = native!((), 0x7F96F23FA9B73327, native_parameters!(modelHash));

    value
}

pub fn set_cutscene_trigger_area(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32) -> () {
    let value = native!(
        (),
        0x9896CE4721BE84BA,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn can_set_enter_state_for_registered_entity(
    cutsceneEntName: &std::ffi::CString,
    modelHash: u32,
) -> bool {
    let value = native!(
        bool,
        0x645D0B458D8E17B5,
        native_parameters!(cutsceneEntName.as_ptr(), modelHash)
    );

    value
}

pub fn can_set_exit_state_for_registered_entity(
    cutsceneEntName: &std::ffi::CString,
    modelHash: u32,
) -> bool {
    let value = native!(
        bool,
        0x4C6A6451C79E4662,
        native_parameters!(cutsceneEntName.as_ptr(), modelHash)
    );

    value
}

pub fn can_set_exit_state_for_camera(p0: bool) -> bool {
    let value = native!(bool, 0xB2CBCD0930DFB420, native_parameters!(p0));

    value
}

pub fn _0xc61b86c9f61eb404(toggle: bool) -> () {
    let value = native!((), 0xC61B86C9F61EB404, native_parameters!(toggle));

    value
}

pub fn set_cutscene_fade_values(p0: bool, p1: bool, p2: bool, p3: bool) -> () {
    let value = native!((), 0x8093F23ABACCC7D4, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x20746f7b1032a3c7(p0: bool, p1: bool, p2: bool, p3: bool) -> () {
    let value = native!((), 0x20746F7B1032A3C7, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x06ee9048fd080382(p0: bool) -> () {
    let value = native!((), 0x06EE9048FD080382, native_parameters!(p0));

    value
}

pub fn _0xa0fe76168a189ddb() -> i32 {
    let value = native!(i32, 0xA0FE76168A189DDB, native_parameters!());

    value
}

pub fn _0x2f137b508de238f2(p0: bool) -> () {
    let value = native!((), 0x2F137B508DE238F2, native_parameters!(p0));

    value
}

pub fn _0xe36a98d8ab3d3c66(p0: bool) -> () {
    let value = native!((), 0xE36A98D8AB3D3C66, native_parameters!(p0));

    value
}

pub fn _0x5edef0cf8c1dab3c() -> u32 {
    let value = native!(u32, 0x5EDEF0CF8C1DAB3C, native_parameters!());

    value
}

pub fn set_cutscene_can_be_skipped(p0: bool) -> () {
    let value = native!((), 0x41FAA8FB2ECE8720, native_parameters!(p0));

    value
}

pub fn register_synchronised_script_speech() -> () {
    let value = native!((), 0x2131046957F31B04, native_parameters!());

    value
}

pub fn set_cutscene_ped_component_variation(
    cutsceneEntName: &std::ffi::CString,
    p1: i32,
    p2: i32,
    p3: i32,
    modelHash: u32,
) -> () {
    let value = native!(
        (),
        0xBA01E7B6DEEFBBC9,
        native_parameters!(cutsceneEntName.as_ptr(), p1, p2, p3, modelHash)
    );

    value
}

pub fn set_cutscene_ped_component_variation_from_ped(
    cutsceneEntName: &std::ffi::CString,
    ped: i32,
    modelHash: u32,
) -> () {
    let value = native!(
        (),
        0x2A56C06EBEF2B0D9,
        native_parameters!(cutsceneEntName.as_ptr(), ped, modelHash)
    );

    value
}

pub fn does_cutscene_entity_exist(cutsceneEntName: &std::ffi::CString, modelHash: u32) -> bool {
    let value = native!(
        bool,
        0x499EF20C5DB25C59,
        native_parameters!(cutsceneEntName.as_ptr(), modelHash)
    );

    value
}

pub fn set_cutscene_ped_prop_variation(
    cutsceneEntName: &std::ffi::CString,
    p1: i32,
    p2: i32,
    p3: i32,
    modelHash: u32,
) -> () {
    let value = native!(
        (),
        0x0546524ADE2E9723,
        native_parameters!(cutsceneEntName.as_ptr(), p1, p2, p3, modelHash)
    );

    value
}

pub fn _has_cutscene_cut_this_frame() -> bool {
    let value = native!(bool, 0x708BDD8CD795B043, native_parameters!());

    value
}
