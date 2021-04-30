use crate::types::NativeVector3;

pub fn get_player_ped(player: i32) -> i32 {
    let value = native!(i32, 0x43A66C31C68491C0, native_parameters!(player));

    value
}

pub fn get_player_ped_script_index(player: i32) -> i32 {
    let value = native!(i32, 0x50FAC3A3E030A6E1, native_parameters!(player));

    value
}

pub fn set_player_model(player: i32, model: u32) -> () {
    let value = native!((), 0x00A1CADD00108836, native_parameters!(player, model));

    value
}

pub fn change_player_ped(player: i32, ped: i32, p2: bool, resetDamage: bool) -> () {
    let value = native!(
        (),
        0x048189FAC643DEEE,
        native_parameters!(player, ped, p2, resetDamage)
    );

    value
}

pub fn get_player_rgb_colour(player: i32, r: *mut i32, g: *mut i32, b: *mut i32) -> () {
    let value = native!((), 0xE902EF951DCE178F, native_parameters!(player, r, g, b));

    value
}

pub fn get_number_of_players() -> i32 {
    let value = native!(i32, 0x407C7F91DDB46C16, native_parameters!());

    value
}

pub fn get_player_team(player: i32) -> i32 {
    let value = native!(i32, 0x37039302F4E0A008, native_parameters!(player));

    value
}

pub fn set_player_team(player: i32, team: i32) -> () {
    let value = native!((), 0x0299FA38396A4940, native_parameters!(player, team));

    value
}

pub fn _get_number_of_players_in_team(team: i32) -> i32 {
    let value = native!(i32, 0x1FC200409F10E6F1, native_parameters!(team));

    value
}

pub fn get_player_name(player: i32) -> String {
    let value = native!(*const i8, 0x6D0DE6A7B5DA71F8, native_parameters!(player));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_wanted_level_radius(player: i32) -> f32 {
    let value = native!(f32, 0x085DEB493BE80812, native_parameters!(player));

    value
}

pub fn get_player_wanted_centre_position(player: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x0C92BA89F1AF26F8,
        native_parameters!(player)
    );

    value
}

pub fn set_player_wanted_centre_position(
    player: i32,
    position: *mut NativeVector3,
    p2: bool,
    p3: bool,
) -> () {
    let value = native!(
        (),
        0x520E541A97A13354,
        native_parameters!(player, position, p2, p3)
    );

    value
}

pub fn get_wanted_level_threshold(wantedLevel: i32) -> i32 {
    let value = native!(i32, 0xFDD179EAF45B556C, native_parameters!(wantedLevel));

    value
}

pub fn set_player_wanted_level(player: i32, wantedLevel: i32, disableNoMission: bool) -> () {
    let value = native!(
        (),
        0x39FF19C64EF7DA5B,
        native_parameters!(player, wantedLevel, disableNoMission)
    );

    value
}

pub fn set_player_wanted_level_no_drop(player: i32, wantedLevel: i32, p2: bool) -> () {
    let value = native!(
        (),
        0x340E61DE7F471565,
        native_parameters!(player, wantedLevel, p2)
    );

    value
}

pub fn set_player_wanted_level_now(player: i32, p1: bool) -> () {
    let value = native!((), 0xE0A7D1E497FFCD6F, native_parameters!(player, p1));

    value
}

pub fn are_player_flashing_stars_about_to_drop(player: i32) -> bool {
    let value = native!(bool, 0xAFAF86043E5874E9, native_parameters!(player));

    value
}

pub fn are_player_stars_greyed_out(player: i32) -> bool {
    let value = native!(bool, 0x0A6EB355EE14A2DB, native_parameters!(player));

    value
}

pub fn _0x7e07c78925d5fd96(p0: u32) -> u32 {
    let value = native!(u32, 0x7E07C78925D5FD96, native_parameters!(p0));

    value
}

pub fn set_dispatch_cops_for_player(player: i32, toggle: bool) -> () {
    let value = native!((), 0xDB172424876553F4, native_parameters!(player, toggle));

    value
}

pub fn is_player_wanted_level_greater(player: i32, wantedLevel: i32) -> bool {
    let value = native!(
        bool,
        0x238DB2A2C23EE9EF,
        native_parameters!(player, wantedLevel)
    );

    value
}

pub fn clear_player_wanted_level(player: i32) -> () {
    let value = native!((), 0xB302540597885499, native_parameters!(player));

    value
}

pub fn is_player_dead(player: i32) -> bool {
    let value = native!(bool, 0x424D4687FA1E5652, native_parameters!(player));

    value
}

pub fn is_player_pressing_horn(player: i32) -> bool {
    let value = native!(bool, 0xFA1E2BF8B10598F9, native_parameters!(player));

    value
}

pub fn set_player_control(player: i32, bHasControl: bool, flags: i32) -> () {
    let value = native!(
        (),
        0x8D32347D6D4C40A2,
        native_parameters!(player, bHasControl, flags)
    );

    value
}

pub fn get_player_wanted_level(player: i32) -> i32 {
    let value = native!(i32, 0xE28E54788CE8F12D, native_parameters!(player));

    value
}

pub fn set_max_wanted_level(maxWantedLevel: i32) -> () {
    let value = native!((), 0xAA5F02DB48D704B9, native_parameters!(maxWantedLevel));

    value
}

pub fn set_police_radar_blips(toggle: bool) -> () {
    let value = native!((), 0x43286D561B72B8BF, native_parameters!(toggle));

    value
}

pub fn set_police_ignore_player(player: i32, toggle: bool) -> () {
    let value = native!((), 0x32C62AA929C2DA6A, native_parameters!(player, toggle));

    value
}

pub fn is_player_playing(player: i32) -> bool {
    let value = native!(bool, 0x5E9564D8246B909A, native_parameters!(player));

    value
}

pub fn set_everyone_ignore_player(player: i32, toggle: bool) -> () {
    let value = native!((), 0x8EEDA153AD141BA4, native_parameters!(player, toggle));

    value
}

pub fn set_all_random_peds_flee(player: i32, toggle: bool) -> () {
    let value = native!((), 0x056E0FE8534C2949, native_parameters!(player, toggle));

    value
}

pub fn set_all_random_peds_flee_this_frame(player: i32) -> () {
    let value = native!((), 0x471D2FF42A94B4F2, native_parameters!(player));

    value
}

pub fn _0xde45d1a1ef45ee61(player: i32, toggle: bool) -> () {
    let value = native!((), 0xDE45D1A1EF45EE61, native_parameters!(player, toggle));

    value
}

pub fn _0xc3376f42b1faccc6(player: i32) -> () {
    let value = native!((), 0xC3376F42B1FACCC6, native_parameters!(player));

    value
}

pub fn _0xfac75988a7d078d3(player: i32) -> () {
    let value = native!((), 0xFAC75988A7D078D3, native_parameters!(player));

    value
}

pub fn set_ignore_low_priority_shocking_events(player: i32, toggle: bool) -> () {
    let value = native!((), 0x596976B02B6B5700, native_parameters!(player, toggle));

    value
}

pub fn set_wanted_level_multiplier(multiplier: f32) -> () {
    let value = native!((), 0x020E5F00CDA207BA, native_parameters!(multiplier));

    value
}

pub fn set_wanted_level_difficulty(player: i32, difficulty: f32) -> () {
    let value = native!(
        (),
        0x9B0BB33B04405E7A,
        native_parameters!(player, difficulty)
    );

    value
}

pub fn reset_wanted_level_difficulty(player: i32) -> () {
    let value = native!((), 0xB9D0DD990DC141DD, native_parameters!(player));

    value
}

pub fn _0x49b856b1360c47c7(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x49B856B1360C47C7, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x823ec8e82ba45986(p0: u32) -> () {
    let value = native!((), 0x823EC8E82BA45986, native_parameters!(p0));

    value
}

pub fn start_firing_amnesty(duration: i32) -> () {
    let value = native!((), 0xBF9BD71691857E48, native_parameters!(duration));

    value
}

pub fn report_crime(player: i32, crimeType: i32, wantedLvlThresh: i32) -> () {
    let value = native!(
        (),
        0xE9B09589827545E7,
        native_parameters!(player, crimeType, wantedLvlThresh)
    );

    value
}

pub fn _switch_crime_type(player: i32, p1: i32) -> () {
    let value = native!((), 0x9A987297ED8BD838, native_parameters!(player, p1));

    value
}

pub fn _0xbc9490ca15aea8fb(player: i32) -> () {
    let value = native!((), 0xBC9490CA15AEA8FB, native_parameters!(player));

    value
}

pub fn _0x4669b3ed80f24b4e(player: i32) -> () {
    let value = native!((), 0x4669B3ED80F24B4E, native_parameters!(player));

    value
}

pub fn _0x2f41a3bae005e5fa(p0: u32, p1: u32) -> () {
    let value = native!((), 0x2F41A3BAE005E5FA, native_parameters!(p0, p1));

    value
}

pub fn _0xad73ce5a09e42d12(player: i32) -> () {
    let value = native!((), 0xAD73CE5A09E42D12, native_parameters!(player));

    value
}

pub fn _0x36f1b38855f2a8df(player: i32) -> () {
    let value = native!((), 0x36F1B38855F2A8DF, native_parameters!(player));

    value
}

pub fn report_police_spotted_player(player: i32) -> () {
    let value = native!((), 0xDC64D2C53493ED12, native_parameters!(player));

    value
}

pub fn _0xb45eff719d8427a6(p0: f32) -> () {
    let value = native!((), 0xB45EFF719D8427A6, native_parameters!(p0));

    value
}

pub fn _0x0032a6dba562c518() -> () {
    let value = native!((), 0x0032A6DBA562C518, native_parameters!());

    value
}

pub fn can_player_start_mission(player: i32) -> bool {
    let value = native!(bool, 0xDE7465A27D403C06, native_parameters!(player));

    value
}

pub fn is_player_ready_for_cutscene(player: i32) -> bool {
    let value = native!(bool, 0x908CBECC2CAA3690, native_parameters!(player));

    value
}

pub fn is_player_targetting_entity(player: i32, entity: i32) -> bool {
    let value = native!(bool, 0x7912F7FC4F6264B6, native_parameters!(player, entity));

    value
}

pub fn get_player_target_entity(player: i32, entity: *mut i32) -> bool {
    let value = native!(bool, 0x13EDE1A5DBF797C9, native_parameters!(player, entity));

    value
}

pub fn is_player_free_aiming(player: i32) -> bool {
    let value = native!(bool, 0x2E397FD2ECD37C87, native_parameters!(player));

    value
}

pub fn is_player_free_aiming_at_entity(player: i32, entity: i32) -> bool {
    let value = native!(bool, 0x3C06B5C839B38F7B, native_parameters!(player, entity));

    value
}

pub fn get_entity_player_is_free_aiming_at(player: i32, entity: *mut i32) -> bool {
    let value = native!(bool, 0x2975C866E6713290, native_parameters!(player, entity));

    value
}

pub fn set_player_lockon_range_override(player: i32, range: f32) -> () {
    let value = native!((), 0x29961D490E5814FD, native_parameters!(player, range));

    value
}

pub fn set_player_can_do_drive_by(player: i32, toggle: bool) -> () {
    let value = native!((), 0x6E8834B52EC20C77, native_parameters!(player, toggle));

    value
}

pub fn set_player_can_be_hassled_by_gangs(player: i32, toggle: bool) -> () {
    let value = native!((), 0xD5E460AD7020A246, native_parameters!(player, toggle));

    value
}

pub fn set_player_can_use_cover(player: i32, toggle: bool) -> () {
    let value = native!((), 0xD465A8599DFF6814, native_parameters!(player, toggle));

    value
}

pub fn get_max_wanted_level() -> i32 {
    let value = native!(i32, 0x462E0DB9B137DC5F, native_parameters!());

    value
}

pub fn is_player_targetting_anything(player: i32) -> bool {
    let value = native!(bool, 0x78CFE51896B6B8A4, native_parameters!(player));

    value
}

pub fn set_player_sprint(player: i32, toggle: bool) -> () {
    let value = native!((), 0xA01B8075D8B92DF4, native_parameters!(player, toggle));

    value
}

pub fn reset_player_stamina(player: i32) -> () {
    let value = native!((), 0xA6F312FCCE9C1DFE, native_parameters!(player));

    value
}

pub fn restore_player_stamina(player: i32, p1: f32) -> () {
    let value = native!((), 0xA352C1B864CAFD33, native_parameters!(player, p1));

    value
}

pub fn get_player_sprint_stamina_remaining(player: i32) -> f32 {
    let value = native!(f32, 0x3F9F16F8E65A7ED7, native_parameters!(player));

    value
}

pub fn get_player_sprint_time_remaining(player: i32) -> f32 {
    let value = native!(f32, 0x1885BC9B108B4C99, native_parameters!(player));

    value
}

pub fn get_player_underwater_time_remaining(player: i32) -> f32 {
    let value = native!(f32, 0xA1FCF8E6AF40B731, native_parameters!(player));

    value
}

pub fn _set_player_underwater_time_remaining(player: i32, time: f32) -> u32 {
    let value = native!(u32, 0xA0D3E4F7AAFB7E78, native_parameters!(player, time));

    value
}

pub fn get_player_group(player: i32) -> i32 {
    let value = native!(i32, 0x0D127585F77030AF, native_parameters!(player));

    value
}

pub fn get_player_max_armour(player: i32) -> i32 {
    let value = native!(i32, 0x92659B4CE1863CB3, native_parameters!(player));

    value
}

pub fn is_player_control_on(player: i32) -> bool {
    let value = native!(bool, 0x49C32D60007AFA47, native_parameters!(player));

    value
}

pub fn _is_player_cam_control_disabled() -> bool {
    let value = native!(bool, 0x7C814D2FB49F40C0, native_parameters!());

    value
}

pub fn is_player_script_control_on(player: i32) -> bool {
    let value = native!(bool, 0x8A876A65283DD7D7, native_parameters!(player));

    value
}

pub fn is_player_climbing(player: i32) -> bool {
    let value = native!(bool, 0x95E8F73DC65EFB9C, native_parameters!(player));

    value
}

pub fn is_player_being_arrested(player: i32, atArresting: bool) -> bool {
    let value = native!(
        bool,
        0x388A47C51ABDAC8E,
        native_parameters!(player, atArresting)
    );

    value
}

pub fn reset_player_arrest_state(player: i32) -> () {
    let value = native!((), 0x2D03E13C460760D6, native_parameters!(player));

    value
}

pub fn get_players_last_vehicle() -> i32 {
    let value = native!(i32, 0xB6997A7EB3F5C8C0, native_parameters!());

    value
}

pub fn get_player_index() -> i32 {
    let value = native!(i32, 0xA5EDC40EF369B48D, native_parameters!());

    value
}

pub fn int_to_playerindex(value: i32) -> i32 {
    let value = native!(i32, 0x41BD2A6B006AF756, native_parameters!(value));

    value
}

pub fn int_to_participantindex(value: i32) -> i32 {
    let value = native!(i32, 0x9EC6603812C24710, native_parameters!(value));

    value
}

pub fn get_time_since_player_hit_vehicle(player: i32) -> i32 {
    let value = native!(i32, 0x5D35ECF3A81A0EE0, native_parameters!(player));

    value
}

pub fn get_time_since_player_hit_ped(player: i32) -> i32 {
    let value = native!(i32, 0xE36A25322DC35F42, native_parameters!(player));

    value
}

pub fn get_time_since_player_drove_on_pavement(player: i32) -> i32 {
    let value = native!(i32, 0xD559D2BE9E37853B, native_parameters!(player));

    value
}

pub fn get_time_since_player_drove_against_traffic(player: i32) -> i32 {
    let value = native!(i32, 0xDB89591E290D9182, native_parameters!(player));

    value
}

pub fn is_player_free_for_ambient_task(player: i32) -> bool {
    let value = native!(bool, 0xDCCFD3F106C36AB4, native_parameters!(player));

    value
}

pub fn player_id() -> i32 {
    let value = native!(i32, 0x4F8644AF03D0E0D6, native_parameters!());

    value
}

pub fn player_ped_id() -> i32 {
    let value = native!(i32, 0xD80958FC74E988A6, native_parameters!());

    value
}

pub fn network_player_id_to_int() -> i32 {
    let value = native!(i32, 0xEE68096F9F37341E, native_parameters!());

    value
}

pub fn has_force_cleanup_occurred(cleanupFlags: i32) -> bool {
    let value = native!(bool, 0xC968670BFACE42D9, native_parameters!(cleanupFlags));

    value
}

pub fn force_cleanup(cleanupFlags: i32) -> () {
    let value = native!((), 0xBC8983F38F78ED51, native_parameters!(cleanupFlags));

    value
}

pub fn force_cleanup_for_all_threads_with_this_name(
    name: &std::ffi::CString,
    cleanupFlags: i32,
) -> () {
    let value = native!(
        (),
        0x4C68DDDDF0097317,
        native_parameters!(name.as_ptr(), cleanupFlags)
    );

    value
}

pub fn force_cleanup_for_thread_with_this_id(id: i32, cleanupFlags: i32) -> () {
    let value = native!((), 0xF745B37630DF176B, native_parameters!(id, cleanupFlags));

    value
}

pub fn get_cause_of_most_recent_force_cleanup() -> i32 {
    let value = native!(i32, 0x9A41CF4674A12272, native_parameters!());

    value
}

pub fn set_player_may_only_enter_this_vehicle(player: i32, vehicle: i32) -> () {
    let value = native!((), 0x8026FF78F208978A, native_parameters!(player, vehicle));

    value
}

pub fn set_player_may_not_enter_any_vehicle(player: i32) -> () {
    let value = native!((), 0x1DE37BBF9E9CC14A, native_parameters!(player));

    value
}

pub fn give_achievement_to_player(achievement: i32) -> bool {
    let value = native!(bool, 0xBEC7076D64130195, native_parameters!(achievement));

    value
}

pub fn _set_achievement_progress(achievement: i32, progress: i32) -> bool {
    let value = native!(
        bool,
        0xC2AFFFDABBDC2C5C,
        native_parameters!(achievement, progress)
    );

    value
}

pub fn _get_achievement_progress(achievement: i32) -> i32 {
    let value = native!(i32, 0x1C186837D0619335, native_parameters!(achievement));

    value
}

pub fn has_achievement_been_passed(achievement: i32) -> bool {
    let value = native!(bool, 0x867365E111A3B6EB, native_parameters!(achievement));

    value
}

pub fn is_player_online() -> bool {
    let value = native!(bool, 0xF25D331DC2627BBC, native_parameters!());

    value
}

pub fn is_player_logging_in_np() -> bool {
    let value = native!(bool, 0x74556E1420867ECA, native_parameters!());

    value
}

pub fn display_system_signin_ui(unk: bool) -> () {
    let value = native!((), 0x94DD7888C10A979E, native_parameters!(unk));

    value
}

pub fn is_system_ui_being_displayed() -> bool {
    let value = native!(bool, 0x5D511E3867C87139, native_parameters!());

    value
}

pub fn set_player_invincible(player: i32, toggle: bool) -> () {
    let value = native!((), 0x239528EACDC3E7DE, native_parameters!(player, toggle));

    value
}

pub fn get_player_invincible(player: i32) -> bool {
    let value = native!(bool, 0xB721981B2B939E07, native_parameters!(player));

    value
}

pub fn _0xdcc07526b8ec45af(player: i32) -> bool {
    let value = native!(bool, 0xDCC07526B8EC45AF, native_parameters!(player));

    value
}

pub fn _set_player_invincible_keep_ragdoll_enabled(player: i32, toggle: bool) -> () {
    let value = native!((), 0x6BC97F4F4BB3C04B, native_parameters!(player, toggle));

    value
}

pub fn _0xcac57395b151135f(player: i32, p1: bool) -> () {
    let value = native!((), 0xCAC57395B151135F, native_parameters!(player, p1));

    value
}

pub fn remove_player_helmet(player: i32, p2: bool) -> () {
    let value = native!((), 0xF3AC26D3CC576528, native_parameters!(player, p2));

    value
}

pub fn give_player_ragdoll_control(player: i32, toggle: bool) -> () {
    let value = native!((), 0x3C49C870E66F0A28, native_parameters!(player, toggle));

    value
}

pub fn set_player_lockon(player: i32, toggle: bool) -> () {
    let value = native!((), 0x5C8B2F450EE4328E, native_parameters!(player, toggle));

    value
}

pub fn set_player_targeting_mode(targetMode: i32) -> () {
    let value = native!((), 0xB1906895227793F3, native_parameters!(targetMode));

    value
}

pub fn set_player_target_level(targetLevel: i32) -> () {
    let value = native!((), 0x5702B917B99DB1CD, native_parameters!(targetLevel));

    value
}

pub fn _0xb9cf1f793a9f1bf1() -> bool {
    let value = native!(bool, 0xB9CF1F793A9F1BF1, native_parameters!());

    value
}

pub fn _0xcb645e85e97ea48b() -> bool {
    let value = native!(bool, 0xCB645E85E97EA48B, native_parameters!());

    value
}

pub fn clear_player_has_damaged_at_least_one_ped(player: i32) -> () {
    let value = native!((), 0xF0B67A4DE6AB5F98, native_parameters!(player));

    value
}

pub fn has_player_damaged_at_least_one_ped(player: i32) -> bool {
    let value = native!(bool, 0x20CE80B0C2BF4ACC, native_parameters!(player));

    value
}

pub fn clear_player_has_damaged_at_least_one_non_animal_ped(player: i32) -> () {
    let value = native!((), 0x4AACB96203D11A31, native_parameters!(player));

    value
}

pub fn has_player_damaged_at_least_one_non_animal_ped(player: i32) -> bool {
    let value = native!(bool, 0xE4B90F367BD81752, native_parameters!(player));

    value
}

pub fn set_air_drag_multiplier_for_players_vehicle(player: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0xCA7DC8329F0A1E9E,
        native_parameters!(player, multiplier)
    );

    value
}

pub fn set_swim_multiplier_for_player(player: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0xA91C6F0FF7D16A13,
        native_parameters!(player, multiplier)
    );

    value
}

pub fn set_run_sprint_multiplier_for_player(player: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0x6DB47AA77FD94E09,
        native_parameters!(player, multiplier)
    );

    value
}

pub fn get_time_since_last_arrest() -> i32 {
    let value = native!(i32, 0x5063F92F07C2A316, native_parameters!());

    value
}

pub fn get_time_since_last_death() -> i32 {
    let value = native!(i32, 0xC7034807558DDFCA, native_parameters!());

    value
}

pub fn assisted_movement_close_route() -> () {
    let value = native!((), 0xAEBF081FFC0A0E5E, native_parameters!());

    value
}

pub fn assisted_movement_flush_route() -> () {
    let value = native!((), 0x8621390F0CDCFE1F, native_parameters!());

    value
}

pub fn set_player_forced_aim(player: i32, toggle: bool) -> () {
    let value = native!((), 0x0FEE4F80AC44A726, native_parameters!(player, toggle));

    value
}

pub fn set_player_forced_zoom(player: i32, toggle: bool) -> () {
    let value = native!((), 0x75E7D505F2B15902, native_parameters!(player, toggle));

    value
}

pub fn set_player_force_skip_aim_intro(player: i32, toggle: bool) -> () {
    let value = native!((), 0x7651BC64AE59E128, native_parameters!(player, toggle));

    value
}

pub fn disable_player_firing(player: i32, toggle: bool) -> () {
    let value = native!((), 0x5E6CC07646BBEAB8, native_parameters!(player, toggle));

    value
}

pub fn _0xb885852c39cc265d() -> () {
    let value = native!((), 0xB885852C39CC265D, native_parameters!());

    value
}

pub fn set_disable_ambient_melee_move(player: i32, toggle: bool) -> () {
    let value = native!((), 0x2E8AABFA40A84F8C, native_parameters!(player, toggle));

    value
}

pub fn set_player_max_armour(player: i32, value: i32) -> () {
    let value = native!((), 0x77DFCCF5948B8C71, native_parameters!(player, value));

    value
}

pub fn _special_ability_activate(p0: u32, p1: u32) -> () {
    let value = native!((), 0x821FDC827D6F4090, native_parameters!(p0, p1));

    value
}

pub fn _set_special_ability(player: i32, p1: i32, p2: u32) -> () {
    let value = native!((), 0xB214D570EAD7F81A, native_parameters!(player, p1, p2));

    value
}

pub fn _special_ability_deplete(p0: u32, p1: u32) -> () {
    let value = native!((), 0x17F7471EACA78290, native_parameters!(p0, p1));

    value
}

pub fn special_ability_deactivate(player: i32, p1: u32) -> () {
    let value = native!((), 0xD6A953C6D1492057, native_parameters!(player, p1));

    value
}

pub fn special_ability_deactivate_fast(player: i32, p1: u32) -> () {
    let value = native!((), 0x9CB5CE07A3968D5A, native_parameters!(player, p1));

    value
}

pub fn special_ability_reset(player: i32, p1: u32) -> () {
    let value = native!((), 0x375F0E738F861A94, native_parameters!(player, p1));

    value
}

pub fn special_ability_charge_on_mission_failed(player: i32, p1: u32) -> () {
    let value = native!((), 0xC9A763D8FE87436A, native_parameters!(player, p1));

    value
}

pub fn special_ability_charge_small(player: i32, p1: bool, p2: bool, p3: u32) -> () {
    let value = native!(
        (),
        0x2E7B9B683481687D,
        native_parameters!(player, p1, p2, p3)
    );

    value
}

pub fn special_ability_charge_medium(player: i32, p1: bool, p2: bool, p3: u32) -> () {
    let value = native!(
        (),
        0xF113E3AA9BC54613,
        native_parameters!(player, p1, p2, p3)
    );

    value
}

pub fn special_ability_charge_large(player: i32, p1: bool, p2: bool, p3: u32) -> () {
    let value = native!(
        (),
        0xF733F45FA4497D93,
        native_parameters!(player, p1, p2, p3)
    );

    value
}

pub fn special_ability_charge_continuous(player: i32, p1: i32, p2: u32) -> () {
    let value = native!((), 0xED481732DFF7E997, native_parameters!(player, p1, p2));

    value
}

pub fn special_ability_charge_absolute(player: i32, p1: i32, p2: bool, p3: u32) -> () {
    let value = native!(
        (),
        0xB7B0870EB531D08D,
        native_parameters!(player, p1, p2, p3)
    );

    value
}

pub fn special_ability_charge_normalized(
    player: i32,
    normalizedValue: f32,
    p2: bool,
    p3: u32,
) -> () {
    let value = native!(
        (),
        0xA0696A65F009EE18,
        native_parameters!(player, normalizedValue, p2, p3)
    );

    value
}

pub fn special_ability_fill_meter(player: i32, p1: bool, p2: u32) -> () {
    let value = native!((), 0x3DACA8DDC6FD4980, native_parameters!(player, p1, p2));

    value
}

pub fn special_ability_deplete_meter(player: i32, p1: bool, p2: u32) -> () {
    let value = native!((), 0x1D506DBBBC51E64B, native_parameters!(player, p1, p2));

    value
}

pub fn special_ability_lock(playerModel: u32, p1: u32) -> () {
    let value = native!((), 0x6A09D0D590A47D13, native_parameters!(playerModel, p1));

    value
}

pub fn special_ability_unlock(playerModel: u32, p1: u32) -> () {
    let value = native!((), 0xF145F3BE2EFA9A3B, native_parameters!(playerModel, p1));

    value
}

pub fn is_special_ability_unlocked(playerModel: u32) -> bool {
    let value = native!(bool, 0xC6017F6A6CDFA694, native_parameters!(playerModel));

    value
}

pub fn is_special_ability_active(player: i32, p1: u32) -> bool {
    let value = native!(bool, 0x3E5F7FC85D854E15, native_parameters!(player, p1));

    value
}

pub fn is_special_ability_meter_full(player: i32, p1: u32) -> bool {
    let value = native!(bool, 0x05A1FE504B7F2587, native_parameters!(player, p1));

    value
}

pub fn enable_special_ability(player: i32, toggle: bool, p2: u32) -> () {
    let value = native!(
        (),
        0x181EC197DAEFE121,
        native_parameters!(player, toggle, p2)
    );

    value
}

pub fn is_special_ability_enabled(player: i32, p1: u32) -> bool {
    let value = native!(bool, 0xB1D200FE26AEF3CB, native_parameters!(player, p1));

    value
}

pub fn set_special_ability_multiplier(multiplier: f32) -> () {
    let value = native!((), 0xA49C426ED0CA4AB7, native_parameters!(multiplier));

    value
}

pub fn _0xffee8fa29ab9a18e(player: i32, p1: u32) -> () {
    let value = native!((), 0xFFEE8FA29AB9A18E, native_parameters!(player, p1));

    value
}

pub fn _0x5fc472c501ccadb3(player: i32) -> bool {
    let value = native!(bool, 0x5FC472C501CCADB3, native_parameters!(player));

    value
}

pub fn _0xf10b44fd479d69f3(player: i32, p1: i32) -> bool {
    let value = native!(bool, 0xF10B44FD479D69F3, native_parameters!(player, p1));

    value
}

pub fn _0xdd2620b7b9d16ff1(player: i32, p1: f32) -> bool {
    let value = native!(bool, 0xDD2620B7B9D16FF1, native_parameters!(player, p1));

    value
}

pub fn start_player_teleport(
    player: i32,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    p5: bool,
    findCollisionLand: bool,
    p7: bool,
) -> () {
    let value = native!(
        (),
        0xAD15F075A4DA0FDE,
        native_parameters!(player, x, y, z, heading, p5, findCollisionLand, p7)
    );

    value
}

pub fn update_player_teleport(player: i32) -> bool {
    let value = native!(bool, 0xE23D5873C2394C61, native_parameters!(player));

    value
}

pub fn stop_player_teleport() -> () {
    let value = native!((), 0xC449EDED9D73009C, native_parameters!());

    value
}

pub fn is_player_teleport_active() -> bool {
    let value = native!(bool, 0x02B15662D7F8886F, native_parameters!());

    value
}

pub fn get_player_current_stealth_noise(player: i32) -> f32 {
    let value = native!(f32, 0x2F395D61F3A1F877, native_parameters!(player));

    value
}

pub fn set_player_health_recharge_multiplier(player: i32, regenRate: f32) -> () {
    let value = native!(
        (),
        0x5DB660B38DD98A31,
        native_parameters!(player, regenRate)
    );

    value
}

pub fn _get_player_health_recharge_limit(player: i32) -> f32 {
    let value = native!(f32, 0x8BC515BAE4AAF8FF, native_parameters!(player));

    value
}

pub fn _set_player_health_recharge_limit(player: i32, limit: f32) -> () {
    let value = native!((), 0xC388A0F065F5BC34, native_parameters!(player, limit));

    value
}

pub fn _set_player_fall_distance(p0: u32, p1: u32) -> () {
    let value = native!((), 0xEFD79FA81DFBA9CB, native_parameters!(p0, p1));

    value
}

pub fn set_player_weapon_damage_modifier(player: i32, modifier: f32) -> () {
    let value = native!((), 0xCE07B9F7817AADA3, native_parameters!(player, modifier));

    value
}

pub fn set_player_weapon_defense_modifier(player: i32, modifier: f32) -> () {
    let value = native!((), 0x2D83BC011CA14A3C, native_parameters!(player, modifier));

    value
}

pub fn _set_player_weapon_defense_modifier_2(player: i32, modifier: f32) -> () {
    let value = native!((), 0xBCFDE9EDE4CF27DC, native_parameters!(player, modifier));

    value
}

pub fn set_player_melee_weapon_damage_modifier(player: i32, modifier: f32, p2: bool) -> () {
    let value = native!(
        (),
        0x4A3DC7ECCC321032,
        native_parameters!(player, modifier, p2)
    );

    value
}

pub fn set_player_melee_weapon_defense_modifier(player: i32, modifier: f32) -> () {
    let value = native!((), 0xAE540335B4ABC4E2, native_parameters!(player, modifier));

    value
}

pub fn set_player_vehicle_damage_modifier(player: i32, modifier: f32) -> () {
    let value = native!((), 0xA50E117CDDF82F0C, native_parameters!(player, modifier));

    value
}

pub fn set_player_vehicle_defense_modifier(player: i32, modifier: f32) -> () {
    let value = native!((), 0x4C60E6EFDAFF2462, native_parameters!(player, modifier));

    value
}

pub fn _0x8d768602adef2245(player: i32, p1: f32) -> () {
    let value = native!((), 0x8D768602ADEF2245, native_parameters!(player, p1));

    value
}

pub fn _0xd821056b9acf8052(p0: u32, p1: u32) -> () {
    let value = native!((), 0xD821056B9ACF8052, native_parameters!(p0, p1));

    value
}

pub fn _0x31e90b8873a4cd3b(p0: u32, p1: u32) -> () {
    let value = native!((), 0x31E90B8873A4CD3B, native_parameters!(p0, p1));

    value
}

pub fn set_player_parachute_tint_index(player: i32, tintIndex: i32) -> () {
    let value = native!(
        (),
        0xA3D0E54541D9A5E5,
        native_parameters!(player, tintIndex)
    );

    value
}

pub fn get_player_parachute_tint_index(player: i32, tintIndex: *mut i32) -> () {
    let value = native!(
        (),
        0x75D3F7A1B0D9B145,
        native_parameters!(player, tintIndex)
    );

    value
}

pub fn set_player_reserve_parachute_tint_index(player: i32, index: i32) -> () {
    let value = native!((), 0xAF04C87F5DC1DF38, native_parameters!(player, index));

    value
}

pub fn get_player_reserve_parachute_tint_index(player: i32, index: *mut i32) -> () {
    let value = native!((), 0xD5A016BC3C09CF40, native_parameters!(player, index));

    value
}

pub fn set_player_parachute_pack_tint_index(player: i32, tintIndex: i32) -> () {
    let value = native!(
        (),
        0x93B0FB27C9A04060,
        native_parameters!(player, tintIndex)
    );

    value
}

pub fn get_player_parachute_pack_tint_index(player: i32, tintIndex: *mut i32) -> () {
    let value = native!(
        (),
        0x6E9C742F340CE5A2,
        native_parameters!(player, tintIndex)
    );

    value
}

pub fn set_player_has_reserve_parachute(player: i32) -> () {
    let value = native!((), 0x7DDAB28D31FAC363, native_parameters!(player));

    value
}

pub fn get_player_has_reserve_parachute(player: i32) -> bool {
    let value = native!(bool, 0x5DDFE2FF727F3CA3, native_parameters!(player));

    value
}

pub fn set_player_can_leave_parachute_smoke_trail(player: i32, enabled: bool) -> () {
    let value = native!((), 0xF401B182DBA8AF53, native_parameters!(player, enabled));

    value
}

pub fn set_player_parachute_smoke_trail_color(player: i32, r: i32, g: i32, b: i32) -> () {
    let value = native!((), 0x8217FD371A4625CF, native_parameters!(player, r, g, b));

    value
}

pub fn get_player_parachute_smoke_trail_color(
    player: i32,
    r: *mut i32,
    g: *mut i32,
    b: *mut i32,
) -> () {
    let value = native!((), 0xEF56DBABD3CD4887, native_parameters!(player, r, g, b));

    value
}

pub fn set_player_reset_flag_prefer_rear_seats(player: i32, flags: i32) -> () {
    let value = native!((), 0x11D5F725F0E780E0, native_parameters!(player, flags));

    value
}

pub fn set_player_noise_multiplier(player: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0xDB89EF50FF25FCE9,
        native_parameters!(player, multiplier)
    );

    value
}

pub fn set_player_sneaking_noise_multiplier(player: i32, multiplier: f32) -> () {
    let value = native!(
        (),
        0xB2C1A29588A9F47C,
        native_parameters!(player, multiplier)
    );

    value
}

pub fn can_ped_hear_player(player: i32, ped: i32) -> bool {
    let value = native!(bool, 0xF297383AA91DCA29, native_parameters!(player, ped));

    value
}

pub fn simulate_player_input_gait(
    player: i32,
    amount: f32,
    gaitType: i32,
    speed: f32,
    p4: bool,
    p5: bool,
) -> () {
    let value = native!(
        (),
        0x477D5D63E63ECA5D,
        native_parameters!(player, amount, gaitType, speed, p4, p5)
    );

    value
}

pub fn reset_player_input_gait(player: i32) -> () {
    let value = native!((), 0x19531C47A2ABD691, native_parameters!(player));

    value
}

pub fn set_auto_give_parachute_when_enter_plane(player: i32, toggle: bool) -> () {
    let value = native!((), 0x9F343285A00B4BB6, native_parameters!(player, toggle));

    value
}

pub fn set_auto_give_scuba_gear_when_exit_vehicle(player: i32, toggle: bool) -> () {
    let value = native!((), 0xD2B315B6689D537D, native_parameters!(player, toggle));

    value
}

pub fn set_player_stealth_perception_modifier(player: i32, value: f32) -> () {
    let value = native!((), 0x4E9021C1FCDD507A, native_parameters!(player, value));

    value
}

pub fn _0x690a61a6d13583f6(player: i32) -> bool {
    let value = native!(bool, 0x690A61A6D13583F6, native_parameters!(player));

    value
}

pub fn _0x9edd76e87d5d51ba(player: i32) -> () {
    let value = native!((), 0x9EDD76E87D5D51BA, native_parameters!(player));

    value
}

pub fn set_player_simulate_aiming(player: i32, toggle: bool) -> () {
    let value = native!((), 0xC54C95DA968EC5B5, native_parameters!(player, toggle));

    value
}

pub fn set_player_cloth_pin_frames(player: i32, p1: i32) -> () {
    let value = native!((), 0x749FADDF97DFE930, native_parameters!(player, p1));

    value
}

pub fn set_player_cloth_package_index(index: i32) -> () {
    let value = native!((), 0x9F7BBA2EA6372500, native_parameters!(index));

    value
}

pub fn set_player_cloth_lock_counter(value: i32) -> () {
    let value = native!((), 0x14D913B777DFF5DA, native_parameters!(value));

    value
}

pub fn player_attach_virtual_bound(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
) -> () {
    let value = native!(
        (),
        0xED51733DC73AED51,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn player_detach_virtual_bound() -> () {
    let value = native!((), 0x1DD5897E2FA6E7C9, native_parameters!());

    value
}

pub fn has_player_been_spotted_in_stolen_vehicle(player: i32) -> bool {
    let value = native!(bool, 0xD705740BB0A1CF4C, native_parameters!(player));

    value
}

pub fn is_player_battle_aware(player: i32) -> bool {
    let value = native!(bool, 0x38D28DA81E4E9BF9, native_parameters!(player));

    value
}

pub fn _0xbc0753c9ca14b506(player: i32, p1: i32, p2: bool) -> bool {
    let value = native!(bool, 0xBC0753C9CA14B506, native_parameters!(player, p1, p2));

    value
}

pub fn extend_world_boundary_for_player(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x5006D96C995A5827, native_parameters!(x, y, z));

    value
}

pub fn reset_world_boundary_for_player() -> () {
    let value = native!((), 0xDA1DF03D5A315F4E, native_parameters!());

    value
}

pub fn is_player_riding_train(player: i32) -> bool {
    let value = native!(bool, 0x4EC12697209F2196, native_parameters!(player));

    value
}

pub fn has_player_left_the_world(player: i32) -> bool {
    let value = native!(bool, 0xD55DDFB47991A294, native_parameters!(player));

    value
}

pub fn set_player_leave_ped_behind(player: i32, toggle: bool) -> () {
    let value = native!((), 0xFF300C7649724A0B, native_parameters!(player, toggle));

    value
}

pub fn set_player_parachute_variation_override(
    player: i32,
    p1: i32,
    p2: u32,
    p3: u32,
    p4: bool,
) -> () {
    let value = native!(
        (),
        0xD9284A8C0D48352C,
        native_parameters!(player, p1, p2, p3, p4)
    );

    value
}

pub fn clear_player_parachute_variation_override(player: i32) -> () {
    let value = native!((), 0x0F4CC924CF8C7B21, native_parameters!(player));

    value
}

pub fn set_player_parachute_model_override(player: i32, model: u32) -> () {
    let value = native!((), 0x977DB4641F6FC3DB, native_parameters!(player, model));

    value
}

pub fn clear_player_parachute_model_override(player: i32) -> () {
    let value = native!((), 0x8753997EB5F6EE3F, native_parameters!(player));

    value
}

pub fn set_player_parachute_pack_model_override(player: i32, model: u32) -> () {
    let value = native!((), 0xDC80A4C2F18A2B64, native_parameters!(player, model));

    value
}

pub fn clear_player_parachute_pack_model_override(player: i32) -> () {
    let value = native!((), 0x10C54E4389C12B42, native_parameters!(player));

    value
}

pub fn disable_player_vehicle_rewards(player: i32) -> () {
    let value = native!((), 0xC142BE3BB9CE125F, native_parameters!(player));

    value
}

pub fn _0x2f7ceb6520288061(p0: bool) -> () {
    let value = native!((), 0x2F7CEB6520288061, native_parameters!(p0));

    value
}

pub fn set_player_bluetooth_state(player: i32, state: bool) -> () {
    let value = native!((), 0x5DC40A8869C22141, native_parameters!(player, state));

    value
}

pub fn is_player_bluetooth_enable(player: i32) -> bool {
    let value = native!(bool, 0x65FAEE425DE637B0, native_parameters!(player));

    value
}

pub fn _0x5501b7a5cdb79d37(player: i32) -> () {
    let value = native!((), 0x5501B7A5CDB79D37, native_parameters!(player));

    value
}

pub fn get_player_fake_wanted_level(player: i32) -> i32 {
    let value = native!(i32, 0x56105E599CAB0EFA, native_parameters!(player));

    value
}

pub fn _0x55fcc0c390620314(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x55FCC0C390620314, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x2382ab11450ae7ba(p0: u32, p1: u32) -> () {
    let value = native!((), 0x2382AB11450AE7BA, native_parameters!(p0, p1));

    value
}

pub fn _0x6e4361ff3e8cd7ca(p0: u32) -> u32 {
    let value = native!(u32, 0x6E4361FF3E8CD7CA, native_parameters!(p0));

    value
}

pub fn _0x237440e46d918649(p0: u32) -> () {
    let value = native!((), 0x237440E46D918649, native_parameters!(p0));

    value
}

pub fn _set_player_homing_rocket_disabled(p0: u32, p1: u32) -> () {
    let value = native!((), 0xEE4EBDD2593BA844, native_parameters!(p0, p1));

    value
}

pub fn _0x9097eb6d4bb9a12a(player: i32, entity: i32) -> () {
    let value = native!((), 0x9097EB6D4BB9A12A, native_parameters!(player, entity));

    value
}

pub fn _0x9f260bfb59adbca3(player: i32, entity: i32) -> () {
    let value = native!((), 0x9F260BFB59ADBCA3, native_parameters!(player, entity));

    value
}

pub fn _0x7bae68775557ae0b(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0x7BAE68775557AE0B,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _0x7148e0f43d11f0d9() -> () {
    let value = native!((), 0x7148E0F43D11F0D9, native_parameters!());

    value
}

pub fn _0x70a382adec069dd3(coordX: f32, coordY: f32, coordZ: f32) -> () {
    let value = native!(
        (),
        0x70A382ADEC069DD3,
        native_parameters!(coordX, coordY, coordZ)
    );

    value
}
