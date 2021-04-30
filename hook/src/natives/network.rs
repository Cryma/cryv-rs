use crate::types::NativeVector3;

pub fn _get_online_version() -> String {
    let value = native!(*const i8, 0xFCA9373EF340AC0A, native_parameters!());
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn network_is_signed_in() -> bool {
    let value = native!(bool, 0x054354A99211EB96, native_parameters!());

    value
}

pub fn network_is_signed_online() -> bool {
    let value = native!(bool, 0x1077788E268557C2, native_parameters!());

    value
}

pub fn _0xbd545d44cce70597() -> bool {
    let value = native!(bool, 0xBD545D44CCE70597, native_parameters!());

    value
}

pub fn _0xebcab9e5048434f4() -> u32 {
    let value = native!(u32, 0xEBCAB9E5048434F4, native_parameters!());

    value
}

pub fn _0x74fb3e29e6d10fa9() -> u32 {
    let value = native!(u32, 0x74FB3E29E6D10FA9, native_parameters!());

    value
}

pub fn _0x7808619f31ff22db() -> u32 {
    let value = native!(u32, 0x7808619F31FF22DB, native_parameters!());

    value
}

pub fn _0xa0fa4ec6a05da44e() -> u32 {
    let value = native!(u32, 0xA0FA4EC6A05DA44E, native_parameters!());

    value
}

pub fn network_has_valid_ros_credentials() -> bool {
    let value = native!(bool, 0x85443FF4C328F53B, native_parameters!());

    value
}

pub fn _0x8d11e61a4abf49cc() -> bool {
    let value = native!(bool, 0x8D11E61A4ABF49CC, native_parameters!());

    value
}

pub fn network_is_cloud_available() -> bool {
    let value = native!(bool, 0x9A4CF4F48AD77302, native_parameters!());

    value
}

pub fn network_has_social_club_account() -> bool {
    let value = native!(bool, 0x67A5589628E0CFF6, native_parameters!());

    value
}

pub fn network_are_social_club_policies_current() -> bool {
    let value = native!(bool, 0xBA9775570DB788CF, native_parameters!());

    value
}

pub fn network_is_host() -> bool {
    let value = native!(bool, 0x8DB296B814EDDA07, native_parameters!());

    value
}

pub fn _0x4237e822315d8ba9() -> bool {
    let value = native!(bool, 0x4237E822315D8BA9, native_parameters!());

    value
}

pub fn network_have_online_privileges() -> bool {
    let value = native!(bool, 0x25CB5A9F37BFD063, native_parameters!());

    value
}

pub fn _network_has_age_restricted_profile() -> bool {
    let value = native!(bool, 0x1353F87E89946207, native_parameters!());

    value
}

pub fn network_have_user_content_privileges(p0: i32) -> bool {
    let value = native!(bool, 0x72D918C99BCACC54, native_parameters!(p0));

    value
}

pub fn network_have_communication_privileges(p0: i32, player: i32) -> bool {
    let value = native!(bool, 0xAEEF48CDF5B6CE7C, native_parameters!(p0, player));

    value
}

pub fn _0x78321bea235fd8cd(p0: u32, p1: bool) -> bool {
    let value = native!(bool, 0x78321BEA235FD8CD, native_parameters!(p0, p1));

    value
}

pub fn network_check_user_content_privileges(p0: i32, p1: i32, p2: bool) -> bool {
    let value = native!(bool, 0x595F028698072DD9, native_parameters!(p0, p1, p2));

    value
}

pub fn network_check_communication_privileges(p0: i32, p1: i32, p2: bool) -> bool {
    let value = native!(bool, 0x83F28CE49FBBFFBA, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x07eab372c8841d99(p0: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(u32, 0x07EAB372C8841D99, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x906ca41a4b74eca4() -> u32 {
    let value = native!(u32, 0x906CA41A4B74ECA4, native_parameters!());

    value
}

pub fn _0x023acab2dc9dc4a4() -> u32 {
    let value = native!(u32, 0x023ACAB2DC9DC4A4, native_parameters!());

    value
}

pub fn network_has_social_networking_sharing_priv() -> bool {
    let value = native!(bool, 0x76BF03FADBF154F5, native_parameters!());

    value
}

pub fn network_get_age_group() -> i32 {
    let value = native!(i32, 0x9614B71F8ADB982B, native_parameters!());

    value
}

pub fn _0x0cf6cc51aa18f0f8(p0: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(u32, 0x0CF6CC51AA18F0F8, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x64e5c4cc82847b73() -> bool {
    let value = native!(bool, 0x64E5C4CC82847B73, native_parameters!());

    value
}

pub fn _0x1f7bc3539f9e0224() -> () {
    let value = native!((), 0x1F7BC3539F9E0224, native_parameters!());

    value
}

pub fn _network_have_online_privilege_2() -> bool {
    let value = native!(bool, 0x5EA784D197556507, native_parameters!());

    value
}

pub fn _0xa8acb6459542a8c8() -> u32 {
    let value = native!(u32, 0xA8ACB6459542A8C8, native_parameters!());

    value
}

pub fn _0x83fe8d7229593017() -> () {
    let value = native!((), 0x83FE8D7229593017, native_parameters!());

    value
}

pub fn _0x53c10c8bd774f2c9() -> u32 {
    let value = native!(u32, 0x53C10C8BD774F2C9, native_parameters!());

    value
}

pub fn network_can_bail() -> bool {
    let value = native!(bool, 0x580CE4438479CC61, native_parameters!());

    value
}

pub fn network_bail(p0: i32, p1: i32, p2: i32) -> () {
    let value = native!((), 0x95914459A87EBA28, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x283b6062a2c01e9b() -> () {
    let value = native!((), 0x283B6062A2C01E9B, native_parameters!());

    value
}

pub fn _0x8b4ffc790ca131ef(p0: u32, p1: u32, p2: u32, p3: u32) -> u32 {
    let value = native!(u32, 0x8B4FFC790CA131EF, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _network_transition_track(hash: u32, p1: i32, p2: i32, state: i32, p4: i32) -> () {
    let value = native!(
        (),
        0xC3BFED92026A2AAD,
        native_parameters!(hash, p1, p2, state, p4)
    );

    value
}

pub fn _0x04918a41bc9b8157(p0: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(u32, 0x04918A41BC9B8157, native_parameters!(p0, p1, p2));

    value
}

pub fn network_can_access_multiplayer(loadingState: *mut i32) -> bool {
    let value = native!(bool, 0xAF50DA1A3F8B1BA4, native_parameters!(loadingState));

    value
}

pub fn network_is_multiplayer_disabled() -> bool {
    let value = native!(bool, 0x9747292807126EDA, native_parameters!());

    value
}

pub fn network_can_enter_multiplayer() -> bool {
    let value = native!(bool, 0x7E782A910C362C25, native_parameters!());

    value
}

pub fn network_session_enter(p0: u32, p1: u32, p2: u32, maxPlayers: i32, p4: u32, p5: u32) -> u32 {
    let value = native!(
        u32,
        0x330ED4D05491934F,
        native_parameters!(p0, p1, p2, maxPlayers, p4, p5)
    );

    value
}

pub fn network_session_friend_matchmaking(p0: i32, p1: i32, maxPlayers: i32, p3: bool) -> bool {
    let value = native!(
        bool,
        0x2CFC76E0D087C994,
        native_parameters!(p0, p1, maxPlayers, p3)
    );

    value
}

pub fn network_session_crew_matchmaking(
    p0: i32,
    p1: i32,
    p2: i32,
    maxPlayers: i32,
    p4: bool,
) -> bool {
    let value = native!(
        bool,
        0x94BC51E9449D917F,
        native_parameters!(p0, p1, p2, maxPlayers, p4)
    );

    value
}

pub fn network_session_activity_quickmatch(p0: u32, p1: u32, p2: u32, p3: u32) -> bool {
    let value = native!(bool, 0xBE3E347A87ACEB82, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn network_session_host(p0: i32, maxPlayers: i32, p2: bool) -> bool {
    let value = native!(
        bool,
        0x6F3D4ED9BEE4E61D,
        native_parameters!(p0, maxPlayers, p2)
    );

    value
}

pub fn network_session_host_closed(p0: i32, maxPlayers: i32) -> bool {
    let value = native!(bool, 0xED34C0C02C098BB7, native_parameters!(p0, maxPlayers));

    value
}

pub fn network_session_host_friends_only(p0: i32, maxPlayers: i32) -> bool {
    let value = native!(bool, 0xB9CFD27A5D578D83, native_parameters!(p0, maxPlayers));

    value
}

pub fn network_session_is_closed_friends() -> bool {
    let value = native!(bool, 0xFBCFA2EA2E206890, native_parameters!());

    value
}

pub fn network_session_is_closed_crew() -> bool {
    let value = native!(bool, 0x74732C6CA90DA2B4, native_parameters!());

    value
}

pub fn network_session_is_solo() -> bool {
    let value = native!(bool, 0xF3929C2379B60CCE, native_parameters!());

    value
}

pub fn network_session_is_private() -> bool {
    let value = native!(bool, 0xCEF70AA5B3F89BA1, native_parameters!());

    value
}

pub fn network_session_end(p0: bool, p1: bool) -> bool {
    let value = native!(bool, 0xA02E59562D711006, native_parameters!(p0, p1));

    value
}

pub fn _0xb9351a07a0d458b1(p0: u32) -> u32 {
    let value = native!(u32, 0xB9351A07A0D458B1, native_parameters!(p0));

    value
}

pub fn network_session_kick_player(player: i32) -> () {
    let value = native!((), 0xFA8904DC5F304220, native_parameters!(player));

    value
}

pub fn network_session_get_kick_vote(player: i32) -> bool {
    let value = native!(bool, 0xD6D09A6F32F49EF1, native_parameters!(player));

    value
}

pub fn _0x041c7f2a6c9894e6(p0: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(u32, 0x041C7F2A6C9894E6, native_parameters!(p0, p1, p2));

    value
}

pub fn network_join_previously_failed_session() -> bool {
    let value = native!(bool, 0x59DF79317F85A7E0, native_parameters!());

    value
}

pub fn network_join_previously_failed_transition() -> bool {
    let value = native!(bool, 0xFFE1E5B792D92B34, native_parameters!());

    value
}

pub fn network_session_set_matchmaking_group(matchmakingGroup: i32) -> () {
    let value = native!((), 0x49EC8030F5015F8B, native_parameters!(matchmakingGroup));

    value
}

pub fn network_session_set_matchmaking_group_max(playerType: i32, playerCount: i32) -> () {
    let value = native!(
        (),
        0x8B6A4DD0AF9CE215,
        native_parameters!(playerType, playerCount)
    );

    value
}

pub fn network_session_get_matchmaking_group_free(p0: i32) -> i32 {
    let value = native!(i32, 0x56CE820830EF040B, native_parameters!(p0));

    value
}

pub fn _0xcae55f48d3d7875c(p0: i32) -> () {
    let value = native!((), 0xCAE55F48D3D7875C, native_parameters!(p0));

    value
}

pub fn _0xf49abc20d8552257(p0: u32) -> () {
    let value = native!((), 0xF49ABC20D8552257, native_parameters!(p0));

    value
}

pub fn _0x4811bbac21c5fcd5(p0: u32) -> () {
    let value = native!((), 0x4811BBAC21C5FCD5, native_parameters!(p0));

    value
}

pub fn _0x5539c3ebf104a53a(p0: bool) -> () {
    let value = native!((), 0x5539C3EBF104A53A, native_parameters!(p0));

    value
}

pub fn _0x702bc4d605522539(p0: u32) -> () {
    let value = native!((), 0x702BC4D605522539, native_parameters!(p0));

    value
}

pub fn network_session_set_matchmaking_property_id(p0: bool) -> () {
    let value = native!((), 0x3F52E880AAF6C8CA, native_parameters!(p0));

    value
}

pub fn network_session_set_matchmaking_mental_state(p0: u32) -> () {
    let value = native!((), 0xF1EEA2DDA9FFA69D, native_parameters!(p0));

    value
}

pub fn _0x5ecd378ee64450ab(p0: u32) -> () {
    let value = native!((), 0x5ECD378EE64450AB, native_parameters!(p0));

    value
}

pub fn _0x59d421683d31835a(p0: u32) -> () {
    let value = native!((), 0x59D421683D31835A, native_parameters!(p0));

    value
}

pub fn _0x1153fa02a659051c() -> () {
    let value = native!((), 0x1153FA02A659051C, native_parameters!());

    value
}

pub fn network_session_validate_join(p0: bool) -> () {
    let value = native!((), 0xC19F6C8E7865A6FF, native_parameters!(p0));

    value
}

pub fn network_add_followers(p0: *mut i32, p1: i32) -> () {
    let value = native!((), 0x236406F60CF216D6, native_parameters!(p0, p1));

    value
}

pub fn network_clear_followers() -> () {
    let value = native!((), 0x058F43EC59A8631A, native_parameters!());

    value
}

pub fn network_get_global_multiplayer_clock(
    hours: *mut i32,
    minutes: *mut i32,
    seconds: *mut i32,
) -> () {
    let value = native!(
        (),
        0x6D03BFBD643B2A02,
        native_parameters!(hours, minutes, seconds)
    );

    value
}

pub fn _0x600f8cb31c7aab6e(p0: u32) -> () {
    let value = native!((), 0x600F8CB31C7AAB6E, native_parameters!(p0));

    value
}

pub fn _network_get_targeting_mode() -> i32 {
    let value = native!(i32, 0xDFFA5BE8381C3314, native_parameters!());

    value
}

pub fn network_find_gamers_in_crew(p0: u32) -> bool {
    let value = native!(bool, 0xE532D6811B3A4D2A, native_parameters!(p0));

    value
}

pub fn network_find_matched_gamers(p0: u32, p1: f32, p2: f32, p3: f32) -> bool {
    let value = native!(bool, 0xF7B2CFDE5C9F700D, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn network_is_finding_gamers() -> bool {
    let value = native!(bool, 0xDDDF64C91BFCF0AA, native_parameters!());

    value
}

pub fn _0xf9b83b77929d8863() -> u32 {
    let value = native!(u32, 0xF9B83B77929D8863, native_parameters!());

    value
}

pub fn network_get_num_found_gamers() -> i32 {
    let value = native!(i32, 0xA1B043EE79A916FB, native_parameters!());

    value
}

pub fn network_get_found_gamer(p0: *mut u32, p1: u32) -> bool {
    let value = native!(bool, 0x9DCFF2AFB68B3476, native_parameters!(p0, p1));

    value
}

pub fn network_clear_found_gamers() -> () {
    let value = native!((), 0x6D14CCEE1B40381A, native_parameters!());

    value
}

pub fn _network_get_gamer_status(p0: *mut u32) -> bool {
    let value = native!(bool, 0x85A0EF54A500882C, native_parameters!(p0));

    value
}

pub fn _0x2cc848a861d01493() -> u32 {
    let value = native!(u32, 0x2CC848A861D01493, native_parameters!());

    value
}

pub fn _0x94a8394d150b013a() -> u32 {
    let value = native!(u32, 0x94A8394D150B013A, native_parameters!());

    value
}

pub fn _0x5ae17c6b0134b7f1() -> u32 {
    let value = native!(u32, 0x5AE17C6B0134B7F1, native_parameters!());

    value
}

pub fn network_get_gamer_status_result(p0: *mut u32, p1: u32) -> bool {
    let value = native!(bool, 0x02A8BEC6FD9AF660, native_parameters!(p0, p1));

    value
}

pub fn network_clear_get_gamer_status() -> () {
    let value = native!((), 0x86E0660E4F5C956D, native_parameters!());

    value
}

pub fn network_session_join_invite() -> () {
    let value = native!((), 0xC6F8AB8A4189CF3A, native_parameters!());

    value
}

pub fn network_session_cancel_invite() -> () {
    let value = native!((), 0x2FBF47B1B36D36F9, native_parameters!());

    value
}

pub fn network_session_force_cancel_invite() -> () {
    let value = native!((), 0xA29177F7703B5644, native_parameters!());

    value
}

pub fn network_has_pending_invite() -> bool {
    let value = native!(bool, 0xAC8C7B9B88C4A668, native_parameters!());

    value
}

pub fn _0xc42dd763159f3461() -> bool {
    let value = native!(bool, 0xC42DD763159F3461, native_parameters!());

    value
}

pub fn _network_accept_invite() -> bool {
    let value = native!(bool, 0x62A0296C1BB1CEB3, native_parameters!());

    value
}

pub fn network_session_was_invited() -> bool {
    let value = native!(bool, 0x23DFB504655D0CE4, native_parameters!());

    value
}

pub fn network_session_get_inviter(networkHandle: *mut i32) -> () {
    let value = native!((), 0xE57397B4A3429DD0, native_parameters!(networkHandle));

    value
}

pub fn _0xd313de83394af134() -> bool {
    let value = native!(bool, 0xD313DE83394AF134, native_parameters!());

    value
}

pub fn _0xbdb6f89c729cf388() -> bool {
    let value = native!(bool, 0xBDB6F89C729CF388, native_parameters!());

    value
}

pub fn network_suppress_invite(toggle: bool) -> () {
    let value = native!((), 0xA0682D67EF1FBA3D, native_parameters!(toggle));

    value
}

pub fn network_block_invites(toggle: bool) -> () {
    let value = native!((), 0x34F9E9049454A7A0, native_parameters!(toggle));

    value
}

pub fn network_block_join_queue_invites(toggle: bool) -> () {
    let value = native!((), 0xCFEB8AF24FC1D0BB, native_parameters!(toggle));

    value
}

pub fn _0xf814fec6a19fd6e0() -> () {
    let value = native!((), 0xF814FEC6A19FD6E0, native_parameters!());

    value
}

pub fn _network_block_kicked_players(p0: bool) -> () {
    let value = native!((), 0x6B07B9CE4D390375, native_parameters!(p0));

    value
}

pub fn network_set_script_ready_for_events(toggle: bool) -> () {
    let value = native!((), 0x7AC752103856FB20, native_parameters!(toggle));

    value
}

pub fn network_is_offline_invite_pending() -> bool {
    let value = native!(bool, 0x74698374C45701D2, native_parameters!());

    value
}

pub fn _0x140e6a44870a11ce() -> () {
    let value = native!((), 0x140E6A44870A11CE, native_parameters!());

    value
}

pub fn network_session_host_single_player(p0: i32) -> () {
    let value = native!((), 0xC74C33FCA52856D5, native_parameters!(p0));

    value
}

pub fn network_session_leave_single_player() -> () {
    let value = native!((), 0x3442775428FD2DAA, native_parameters!());

    value
}

pub fn network_is_game_in_progress() -> bool {
    let value = native!(bool, 0x10FAB35428CCC9D7, native_parameters!());

    value
}

pub fn network_is_session_active() -> bool {
    let value = native!(bool, 0xD83C2B94E7508980, native_parameters!());

    value
}

pub fn network_is_in_session() -> bool {
    let value = native!(bool, 0xCA97246103B63917, native_parameters!());

    value
}

pub fn network_is_session_started() -> bool {
    let value = native!(bool, 0x9DE624D2FC4B603F, native_parameters!());

    value
}

pub fn network_is_session_busy() -> bool {
    let value = native!(bool, 0xF4435D66A8E2905E, native_parameters!());

    value
}

pub fn network_can_session_end() -> bool {
    let value = native!(bool, 0x4EEBC3694E49C572, native_parameters!());

    value
}

pub fn _0x4c9034162368e206() -> u32 {
    let value = native!(u32, 0x4C9034162368E206, native_parameters!());

    value
}

pub fn network_session_mark_visible(toggle: bool) -> () {
    let value = native!((), 0x271CC6AB59EBF9A5, native_parameters!(toggle));

    value
}

pub fn network_session_is_visible() -> bool {
    let value = native!(bool, 0xBA416D68C631496A, native_parameters!());

    value
}

pub fn network_session_block_join_requests(toggle: bool) -> () {
    let value = native!((), 0xA73667484D7037C3, native_parameters!(toggle));

    value
}

pub fn network_session_change_slots(p0: i32, p1: bool) -> () {
    let value = native!((), 0xB4AB419E0D86ACAE, native_parameters!(p0, p1));

    value
}

pub fn network_session_get_private_slots() -> i32 {
    let value = native!(i32, 0x53AFD64C6758F2F9, native_parameters!());

    value
}

pub fn network_session_voice_host() -> () {
    let value = native!((), 0x9C1556705F864230, native_parameters!());

    value
}

pub fn network_session_voice_leave() -> () {
    let value = native!((), 0x6793E42BE02B575D, native_parameters!());

    value
}

pub fn network_session_voice_connect_to_player(p0: *mut u32) -> () {
    let value = native!((), 0xABD5E88B8A2D3DB2, native_parameters!(p0));

    value
}

pub fn network_session_voice_respond_to_request(p0: bool, p1: i32) -> () {
    let value = native!((), 0x7F8413B7FC2AA6B9, native_parameters!(p0, p1));

    value
}

pub fn network_session_voice_set_timeout(timeout: i32) -> () {
    let value = native!((), 0x5B8ED3DB018927B1, native_parameters!(timeout));

    value
}

pub fn network_session_is_in_voice_session() -> bool {
    let value = native!(bool, 0x855BC38818F6F684, native_parameters!());

    value
}

pub fn _0xb5d3453c98456528() -> u32 {
    let value = native!(u32, 0xB5D3453C98456528, native_parameters!());

    value
}

pub fn network_session_is_voice_session_busy() -> bool {
    let value = native!(bool, 0xEF0912DDF7C4CB4B, native_parameters!());

    value
}

pub fn network_send_text_message(message: &std::ffi::CString, networkHandle: *mut i32) -> bool {
    let value = native!(
        bool,
        0x3A214F2EC889B100,
        native_parameters!(message.as_ptr(), networkHandle)
    );

    value
}

pub fn network_set_activity_spectator(toggle: bool) -> () {
    let value = native!((), 0x75138790B4359A74, native_parameters!(toggle));

    value
}

pub fn network_is_activity_spectator() -> bool {
    let value = native!(bool, 0x12103B9E0C9F92FB, native_parameters!());

    value
}

pub fn _0x0e4f77f7b9d74d84(p0: u32) -> () {
    let value = native!((), 0x0E4F77F7B9D74D84, native_parameters!(p0));

    value
}

pub fn network_set_activity_spectator_max(maxSpectators: i32) -> () {
    let value = native!((), 0x9D277B76D1D12222, native_parameters!(maxSpectators));

    value
}

pub fn network_get_activity_player_num(p0: bool) -> i32 {
    let value = native!(i32, 0x73E2B500410DA5A2, native_parameters!(p0));

    value
}

pub fn network_is_activity_spectator_from_handle(networkHandle: *mut i32) -> bool {
    let value = native!(bool, 0x2763BBAA72A7BCB9, native_parameters!(networkHandle));

    value
}

pub fn network_host_transition(
    p0: i32,
    p1: i32,
    p2: i32,
    p3: i32,
    p4: u32,
    p5: bool,
    p6: bool,
    p7: i32,
    p8: u32,
    p9: i32,
) -> bool {
    let value = native!(
        bool,
        0xA60BB5CE242BB254,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn network_do_transition_quickmatch(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
) -> bool {
    let value = native!(
        bool,
        0x71FB0EBCD4915D56,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn network_do_transition_quickmatch_async(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
) -> bool {
    let value = native!(
        bool,
        0xA091A5E44F0072E5,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn network_do_transition_quickmatch_with_group(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: *mut u32,
    p5: u32,
    p6: u32,
    p7: u32,
) -> bool {
    let value = native!(
        bool,
        0x9C4AB58491FDC98A,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn network_join_group_activity() -> u32 {
    let value = native!(u32, 0xA06509A691D12BE4, native_parameters!());

    value
}

pub fn _0x1888694923ef4591() -> () {
    let value = native!((), 0x1888694923EF4591, native_parameters!());

    value
}

pub fn _0xb13e88e655e5a3bc() -> () {
    let value = native!((), 0xB13E88E655E5A3BC, native_parameters!());

    value
}

pub fn network_is_transition_closed_friends() -> bool {
    let value = native!(bool, 0x6512765E3BE78C50, native_parameters!());

    value
}

pub fn network_is_transition_closed_crew() -> bool {
    let value = native!(bool, 0x0DBD5D7E3C5BEC3B, native_parameters!());

    value
}

pub fn network_is_transition_solo() -> bool {
    let value = native!(bool, 0x5DC577201723960A, native_parameters!());

    value
}

pub fn network_is_transition_private() -> bool {
    let value = native!(bool, 0x5A6AA44FF8E931E6, native_parameters!());

    value
}

pub fn _0x617f49c2668e6155() -> i32 {
    let value = native!(i32, 0x617F49C2668E6155, native_parameters!());

    value
}

pub fn _0x261e97ad7bcf3d40(p0: bool) -> () {
    let value = native!((), 0x261E97AD7BCF3D40, native_parameters!(p0));

    value
}

pub fn _0x39917e1b4cb0f911(p0: bool) -> () {
    let value = native!((), 0x39917E1B4CB0F911, native_parameters!(p0));

    value
}

pub fn _0x2ce9d95e4051aecd(p0: u32) -> () {
    let value = native!((), 0x2CE9D95E4051AECD, native_parameters!(p0));

    value
}

pub fn network_set_transition_creator_handle(p0: *mut u32) -> () {
    let value = native!((), 0xEF26739BCD9907D5, native_parameters!(p0));

    value
}

pub fn network_clear_transition_creator_handle() -> () {
    let value = native!((), 0xFB3272229A82C759, native_parameters!());

    value
}

pub fn network_invite_gamers_to_transition(p0: *mut u32, p1: u32) -> bool {
    let value = native!(bool, 0x4A595C32F77DFF76, native_parameters!(p0, p1));

    value
}

pub fn network_set_gamer_invited_to_transition(networkHandle: *mut i32) -> () {
    let value = native!((), 0xCA2C8073411ECDB6, native_parameters!(networkHandle));

    value
}

pub fn network_leave_transition() -> bool {
    let value = native!(bool, 0xD23A1A815D21DB19, native_parameters!());

    value
}

pub fn network_launch_transition() -> bool {
    let value = native!(bool, 0x2DCF46CB1A4F0884, native_parameters!());

    value
}

pub fn _0xa2e9c1ab8a92e8cd(toggle: bool) -> () {
    let value = native!((), 0xA2E9C1AB8A92E8CD, native_parameters!(toggle));

    value
}

pub fn network_bail_transition(p0: i32, p1: i32, p2: i32) -> () {
    let value = native!((), 0xEAA572036990CD1B, native_parameters!(p0, p1, p2));

    value
}

pub fn network_do_transition_to_game(p0: bool, maxPlayers: i32) -> bool {
    let value = native!(bool, 0x3E9BB38102A589B0, native_parameters!(p0, maxPlayers));

    value
}

pub fn network_do_transition_to_new_game(p0: bool, maxPlayers: i32, p2: bool) -> bool {
    let value = native!(
        bool,
        0x4665F51EFED00034,
        native_parameters!(p0, maxPlayers, p2)
    );

    value
}

pub fn network_do_transition_to_freemode(
    p0: *mut u32,
    p1: u32,
    p2: bool,
    players: i32,
    p4: bool,
) -> bool {
    let value = native!(
        bool,
        0x3AAD8B2FCA1E289F,
        native_parameters!(p0, p1, p2, players, p4)
    );

    value
}

pub fn network_do_transition_to_new_freemode(
    p0: *mut u32,
    p1: *mut u32,
    players: i32,
    p3: bool,
    p4: bool,
    p5: bool,
) -> bool {
    let value = native!(
        bool,
        0x9E80A5BA8109F974,
        native_parameters!(p0, p1, players, p3, p4, p5)
    );

    value
}

pub fn network_is_transition_to_game() -> bool {
    let value = native!(bool, 0x9D7696D8F4FA6CB7, native_parameters!());

    value
}

pub fn network_get_transition_members(data: *mut u32, dataCount: i32) -> i32 {
    let value = native!(i32, 0x73B000F7FBC55829, native_parameters!(data, dataCount));

    value
}

pub fn network_apply_transition_parameter(p0: i32, p1: i32) -> () {
    let value = native!((), 0x521638ADA1BA0D18, native_parameters!(p0, p1));

    value
}

pub fn network_apply_transition_parameter_string(
    p0: i32,
    string: &std::ffi::CString,
    p2: bool,
) -> () {
    let value = native!(
        (),
        0xEBEFC2E77084F599,
        native_parameters!(p0, string.as_ptr(), p2)
    );

    value
}

pub fn network_send_transition_gamer_instruction(
    networkHandle: *mut i32,
    p1: &std::ffi::CString,
    p2: i32,
    p3: i32,
    p4: bool,
) -> bool {
    let value = native!(
        bool,
        0x31D1D2B858D25E6B,
        native_parameters!(networkHandle, p1.as_ptr(), p2, p3, p4)
    );

    value
}

pub fn network_mark_transition_gamer_as_fully_joined(p0: *mut u32) -> bool {
    let value = native!(bool, 0x5728BB6D63E3FF1D, native_parameters!(p0));

    value
}

pub fn network_is_transition_host() -> bool {
    let value = native!(bool, 0x0B824797C9BF2159, native_parameters!());

    value
}

pub fn network_is_transition_host_from_handle(networkHandle: *mut i32) -> bool {
    let value = native!(bool, 0x6B5C83BA3EFE6A10, native_parameters!(networkHandle));

    value
}

pub fn network_get_transition_host(networkHandle: *mut i32) -> bool {
    let value = native!(bool, 0x65042B9774C4435E, native_parameters!(networkHandle));

    value
}

pub fn network_is_in_transition() -> bool {
    let value = native!(bool, 0x68049AEFF83D8F0A, native_parameters!());

    value
}

pub fn network_is_transition_started() -> bool {
    let value = native!(bool, 0x53FA83401D9C07FE, native_parameters!());

    value
}

pub fn network_is_transition_busy() -> bool {
    let value = native!(bool, 0x520F3282A53D26B7, native_parameters!());

    value
}

pub fn network_is_transition_matchmaking() -> bool {
    let value = native!(bool, 0x292564C735375EDF, native_parameters!());

    value
}

pub fn _0xc571d0e77d8bbc29() -> bool {
    let value = native!(bool, 0xC571D0E77D8BBC29, native_parameters!());

    value
}

pub fn _0x1398582b7f72b3ed(p0: u32) -> () {
    let value = native!((), 0x1398582B7F72B3ED, native_parameters!(p0));

    value
}

pub fn _0x1f8e00fb18239600(p0: u32) -> () {
    let value = native!((), 0x1F8E00FB18239600, native_parameters!(p0));

    value
}

pub fn _0xf6f4383b7c92f11a(p0: u32) -> () {
    let value = native!((), 0xF6F4383B7C92F11A, native_parameters!(p0));

    value
}

pub fn network_open_transition_matchmaking() -> () {
    let value = native!((), 0x2B3A8F7CA3A38FDE, native_parameters!());

    value
}

pub fn network_close_transition_matchmaking() -> () {
    let value = native!((), 0x43F4DBA69710E01E, native_parameters!());

    value
}

pub fn network_is_transition_open_to_matchmaking() -> bool {
    let value = native!(bool, 0x37A4494483B9F5C9, native_parameters!());

    value
}

pub fn network_set_transition_visibility_lock(p0: bool, p1: bool) -> () {
    let value = native!((), 0x0C978FDA19692C2C, native_parameters!(p0, p1));

    value
}

pub fn network_is_transition_visibility_locked() -> bool {
    let value = native!(bool, 0xD0A484CB2F829FBE, native_parameters!());

    value
}

pub fn network_set_transition_activity_id(p0: u32) -> () {
    let value = native!((), 0x30DE938B516F0AD2, native_parameters!(p0));

    value
}

pub fn network_change_transition_slots(p0: u32, p1: u32) -> () {
    let value = native!((), 0xEEEDA5E6D7080987, native_parameters!(p0, p1));

    value
}

pub fn _0x973d76aa760a6cb6(p0: bool) -> () {
    let value = native!((), 0x973D76AA760A6CB6, native_parameters!(p0));

    value
}

pub fn network_has_player_started_transition(player: i32) -> bool {
    let value = native!(bool, 0x9AC9CCBFA8C29795, native_parameters!(player));

    value
}

pub fn network_are_transition_details_valid(p0: u32) -> bool {
    let value = native!(bool, 0x2615AA2A695930C1, native_parameters!(p0));

    value
}

pub fn network_join_transition(player: i32) -> bool {
    let value = native!(bool, 0x9D060B08CD63321A, native_parameters!(player));

    value
}

pub fn network_has_invited_gamer_to_transition(p0: *mut u32) -> bool {
    let value = native!(bool, 0x7284A47B3540E6CF, native_parameters!(p0));

    value
}

pub fn _0x3f9990bf5f22759c(p0: *mut u32) -> bool {
    let value = native!(bool, 0x3F9990BF5F22759C, native_parameters!(p0));

    value
}

pub fn network_is_activity_session() -> bool {
    let value = native!(bool, 0x05095437424397FA, native_parameters!());

    value
}

pub fn _0x4a9fde3a5a6d0437(toggle: bool) -> () {
    let value = native!((), 0x4A9FDE3A5A6D0437, native_parameters!(toggle));

    value
}

pub fn network_send_invite_via_presence(
    networkHandle: *mut i32,
    p1: *mut u32,
    p2: u32,
    p3: u32,
) -> bool {
    let value = native!(
        bool,
        0xC3C7A6AFDB244624,
        native_parameters!(networkHandle, p1, p2, p3)
    );

    value
}

pub fn _network_send_presence_transition_invite(
    p0: *mut u32,
    p1: *mut u32,
    p2: u32,
    p3: u32,
) -> bool {
    let value = native!(bool, 0xC116FF9B4D488291, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x1171a97a3d3981b6(p0: *mut u32, p1: *mut u32, p2: u32, p3: u32) -> bool {
    let value = native!(bool, 0x1171A97A3D3981B6, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x742b58f723233ed9(p0: u32) -> u32 {
    let value = native!(u32, 0x742B58F723233ED9, native_parameters!(p0));

    value
}

pub fn network_get_num_presence_invites() -> i32 {
    let value = native!(i32, 0xCEFA968912D0F78D, native_parameters!());

    value
}

pub fn network_accept_presence_invite(p0: u32) -> bool {
    let value = native!(bool, 0xFA91550DF9318B22, native_parameters!(p0));

    value
}

pub fn network_remove_presence_invite(p0: u32) -> bool {
    let value = native!(bool, 0xF0210268DB0974B1, native_parameters!(p0));

    value
}

pub fn network_get_presence_invite_id(p0: u32) -> u32 {
    let value = native!(u32, 0xDFF09646E12EC386, native_parameters!(p0));

    value
}

pub fn network_get_presence_invite_inviter(p0: u32) -> u32 {
    let value = native!(u32, 0x4962CC4AA2F345B7, native_parameters!(p0));

    value
}

pub fn network_get_presence_invite_handle(p0: u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0x38D5B0FEBB086F75, native_parameters!(p0, p1));

    value
}

pub fn network_get_presence_invite_session_id(p0: u32) -> u32 {
    let value = native!(u32, 0x26E1CD96B0903D60, native_parameters!(p0));

    value
}

pub fn network_get_presence_invite_content_id(p0: u32) -> u32 {
    let value = native!(u32, 0x24409FC4C55CB22D, native_parameters!(p0));

    value
}

pub fn network_get_presence_invite_playlist_length(p0: u32) -> u32 {
    let value = native!(u32, 0xD39B3FFF8FFDD5BF, native_parameters!(p0));

    value
}

pub fn network_get_presence_invite_playlist_current(p0: u32) -> u32 {
    let value = native!(u32, 0x728C4CC7920CD102, native_parameters!(p0));

    value
}

pub fn network_get_presence_invite_from_admin(p0: u32) -> bool {
    let value = native!(bool, 0x3DBF2DF0AEB7D289, native_parameters!(p0));

    value
}

pub fn network_get_presence_invite_is_tournament(p0: u32) -> bool {
    let value = native!(bool, 0x8806CEBFABD3CE05, native_parameters!(p0));

    value
}

pub fn network_has_follow_invite() -> bool {
    let value = native!(bool, 0x76D9B976C4C09FDE, native_parameters!());

    value
}

pub fn network_action_follow_invite() -> u32 {
    let value = native!(u32, 0xC88156EBB786F8D5, native_parameters!());

    value
}

pub fn network_clear_follow_invite() -> u32 {
    let value = native!(u32, 0x439BFDE3CD0610F6, native_parameters!());

    value
}

pub fn _0xebf8284d8cadeb53() -> () {
    let value = native!((), 0xEBF8284D8CADEB53, native_parameters!());

    value
}

pub fn network_remove_transition_invite(p0: *mut u32) -> () {
    let value = native!((), 0x7524B431B2E6F7EE, native_parameters!(p0));

    value
}

pub fn network_remove_all_transition_invite() -> () {
    let value = native!((), 0x726E0375C7A26368, native_parameters!());

    value
}

pub fn _0xf083835b70ba9bfe() -> () {
    let value = native!((), 0xF083835B70BA9BFE, native_parameters!());

    value
}

pub fn network_invite_gamers(p0: *mut u32, p1: u32, p2: *mut u32, p3: *mut u32) -> bool {
    let value = native!(bool, 0x9D80CD1D0E6327DE, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn network_has_invited_gamer(p0: *mut u32) -> bool {
    let value = native!(bool, 0x4D86CD31E8976ECE, native_parameters!(p0));

    value
}

pub fn _0x71dc455f5cd1c2b1(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0x71DC455F5CD1C2B1, native_parameters!(networkHandle));

    value
}

pub fn _0x3855fb5eb2c5e8b2(p0: u32) -> u32 {
    let value = native!(u32, 0x3855FB5EB2C5E8B2, native_parameters!(p0));

    value
}

pub fn network_get_currently_selected_gamer_handle_from_invite_menu(p0: *mut u32) -> bool {
    let value = native!(bool, 0x74881E6BCAE2327C, native_parameters!(p0));

    value
}

pub fn network_set_currently_selected_gamer_handle_from_invite_menu(p0: *mut u32) -> bool {
    let value = native!(bool, 0x7206F674F2A3B1BB, native_parameters!(p0));

    value
}

pub fn network_set_invite_on_call_for_invite_menu(p0: *mut u32) -> () {
    let value = native!((), 0x66F010A4B031A331, native_parameters!(p0));

    value
}

pub fn network_check_data_manager_succeeded_for_handle(p0: u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0x44B37CDCAE765AAE, native_parameters!(p0, p1));

    value
}

pub fn _0x4ad490ae1536933b(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x4AD490AE1536933B, native_parameters!(p0, p1));

    value
}

pub fn _0x0d77a82dc2d0da59(p0: *mut u32, p1: *mut u32) -> () {
    let value = native!((), 0x0D77A82DC2D0DA59, native_parameters!(p0, p1));

    value
}

pub fn fillout_pm_player_list(networkHandle: *mut i32, p1: u32, p2: u32) -> bool {
    let value = native!(
        bool,
        0xCBBD7C4991B64809,
        native_parameters!(networkHandle, p1, p2)
    );

    value
}

pub fn fillout_pm_player_list_with_names(p0: *mut u32, p1: *mut u32, p2: u32, p3: u32) -> bool {
    let value = native!(bool, 0x716B6DB9D1886106, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn refresh_player_list_stats(p0: i32) -> bool {
    let value = native!(bool, 0xE26CCFF8094D8C74, native_parameters!(p0));

    value
}

pub fn network_set_current_data_manager_handle(p0: *mut u32) -> bool {
    let value = native!(bool, 0x796A87B3B68D1F3D, native_parameters!(p0));

    value
}

pub fn network_is_in_platform_party() -> bool {
    let value = native!(bool, 0x2FC5650B0271CB57, native_parameters!());

    value
}

pub fn _network_get_platform_party_unk() -> i32 {
    let value = native!(i32, 0x01ABCE5E7CBDA196, native_parameters!());

    value
}

pub fn network_get_platform_party_members(data: *mut u32, dataSize: i32) -> i32 {
    let value = native!(i32, 0x120364DE2845DAF8, native_parameters!(data, dataSize));

    value
}

pub fn network_is_in_platform_party_chat() -> bool {
    let value = native!(bool, 0xFD8B834A8BA05048, native_parameters!());

    value
}

pub fn network_is_chatting_in_platform_party(networkHandle: *mut i32) -> bool {
    let value = native!(bool, 0x8DE9945BCC9AEC52, native_parameters!(networkHandle));

    value
}

pub fn _0x2bf66d2e7414f686() -> u32 {
    let value = native!(u32, 0x2BF66D2E7414F686, native_parameters!());

    value
}

pub fn _0x14922ed3e38761f0() -> bool {
    let value = native!(bool, 0x14922ED3E38761F0, native_parameters!());

    value
}

pub fn _0x6ce50e47f5543d0c() -> () {
    let value = native!((), 0x6CE50E47F5543D0C, native_parameters!());

    value
}

pub fn _0xfa2888e3833c8e96() -> () {
    let value = native!((), 0xFA2888E3833C8E96, native_parameters!());

    value
}

pub fn _0x25d990f8e0e3f13c() -> () {
    let value = native!((), 0x25D990F8E0E3F13C, native_parameters!());

    value
}

pub fn network_seed_random_number_generator(seed: i32) -> () {
    let value = native!((), 0xF1B84178F8674195, native_parameters!(seed));

    value
}

pub fn network_get_random_int() -> i32 {
    let value = native!(i32, 0x599E4FA1F87EB5FF, native_parameters!());

    value
}

pub fn network_get_random_int_ranged(rangeStart: i32, rangeEnd: i32) -> i32 {
    let value = native!(
        i32,
        0xE30CF56F1EFA5F43,
        native_parameters!(rangeStart, rangeEnd)
    );

    value
}

pub fn network_player_is_cheater() -> bool {
    let value = native!(bool, 0x655B91F1495A9090, native_parameters!());

    value
}

pub fn network_player_get_cheater_reason() -> i32 {
    let value = native!(i32, 0x172F75B6EE2233BA, native_parameters!());

    value
}

pub fn network_player_is_badsport() -> bool {
    let value = native!(bool, 0x19D8DA0E5A68045A, native_parameters!());

    value
}

pub fn _trigger_script_crc_check_on_player(player: i32, p1: i32, scriptHash: u32) -> bool {
    let value = native!(
        bool,
        0x46FB3ED415C7641C,
        native_parameters!(player, p1, scriptHash)
    );

    value
}

pub fn _0xa12d3a5a3753cc23() -> u32 {
    let value = native!(u32, 0xA12D3A5A3753CC23, native_parameters!());

    value
}

pub fn _0xf287f506767cc8a9() -> u32 {
    let value = native!(u32, 0xF287F506767CC8A9, native_parameters!());

    value
}

pub fn _remote_cheat_detected(player: i32, a: i32, b: i32) -> bool {
    let value = native!(bool, 0x472841A026D26D8B, native_parameters!(player, a, b));

    value
}

pub fn bad_sport_player_left_detected(
    networkHandle: *mut i32,
    event: i32,
    amountReceived: i32,
) -> bool {
    let value = native!(
        bool,
        0xEC5E3AF5289DCA81,
        native_parameters!(networkHandle, event, amountReceived)
    );

    value
}

pub fn network_apply_ped_scar_data(ped: i32, p1: i32) -> () {
    let value = native!((), 0xE66C690248F11150, native_parameters!(ped, p1));

    value
}

pub fn network_set_this_script_is_network_script(lobbySize: i32, p1: bool, playerId: i32) -> () {
    let value = native!(
        (),
        0x1CA59E306ECB80A5,
        native_parameters!(lobbySize, p1, playerId)
    );

    value
}

pub fn _network_is_this_script_marked(p0: u32, p1: bool, p2: u32) -> bool {
    let value = native!(bool, 0xD1110739EEADB592, native_parameters!(p0, p1, p2));

    value
}

pub fn network_get_this_script_is_network_script() -> bool {
    let value = native!(bool, 0x2910669969E9535E, native_parameters!());

    value
}

pub fn network_get_max_num_participants() -> i32 {
    let value = native!(i32, 0xA6C90FBC38E395EE, native_parameters!());

    value
}

pub fn network_get_num_participants() -> i32 {
    let value = native!(i32, 0x18D0456E86604654, native_parameters!());

    value
}

pub fn network_get_script_status() -> i32 {
    let value = native!(i32, 0x57D158647A6BFABF, native_parameters!());

    value
}

pub fn network_register_host_broadcast_variables(vars: *mut i32, numVars: i32) -> () {
    let value = native!((), 0x3E9B2F01C50DF595, native_parameters!(vars, numVars));

    value
}

pub fn network_register_player_broadcast_variables(vars: *mut i32, numVars: i32) -> () {
    let value = native!((), 0x3364AA97340CA215, native_parameters!(vars, numVars));

    value
}

pub fn _0xea8c0ddb10e2822a(p0: u32, p1: u32) -> () {
    let value = native!((), 0xEA8C0DDB10E2822A, native_parameters!(p0, p1));

    value
}

pub fn _0xd6d7478ca62b8d41(p0: u32, p1: u32) -> () {
    let value = native!((), 0xD6D7478CA62B8D41, native_parameters!(p0, p1));

    value
}

pub fn network_finish_broadcasting_data() -> () {
    let value = native!((), 0x64F62AFB081E260D, native_parameters!());

    value
}

pub fn network_has_received_host_broadcast_data() -> bool {
    let value = native!(bool, 0x5D10B3795F3FC886, native_parameters!());

    value
}

pub fn network_get_player_index(player: i32) -> i32 {
    let value = native!(i32, 0x24FB80D107371267, native_parameters!(player));

    value
}

pub fn network_get_participant_index(index: i32) -> i32 {
    let value = native!(i32, 0x1B84DF6AF2A46938, native_parameters!(index));

    value
}

pub fn network_get_player_index_from_ped(ped: i32) -> i32 {
    let value = native!(i32, 0x6C0E2E0125610278, native_parameters!(ped));

    value
}

pub fn network_get_num_connected_players() -> i32 {
    let value = native!(i32, 0xA4A79DD2D9600654, native_parameters!());

    value
}

pub fn network_is_player_connected(player: i32) -> bool {
    let value = native!(bool, 0x93DC1BE4E1ABE9D1, native_parameters!(player));

    value
}

pub fn network_get_total_num_players() -> i32 {
    let value = native!(i32, 0xCF61D4B4702EE9EB, native_parameters!());

    value
}

pub fn network_is_participant_active(p0: i32) -> bool {
    let value = native!(bool, 0x6FF8FF40B6357D45, native_parameters!(p0));

    value
}

pub fn network_is_player_active(player: i32) -> bool {
    let value = native!(bool, 0xB8DFD30D6973E135, native_parameters!(player));

    value
}

pub fn network_is_player_a_participant(player: i32) -> bool {
    let value = native!(bool, 0x3CA58F6CB7CBD784, native_parameters!(player));

    value
}

pub fn network_is_host_of_this_script() -> bool {
    let value = native!(bool, 0x83CD99A1E6061AB5, native_parameters!());

    value
}

pub fn network_get_host_of_this_script() -> i32 {
    let value = native!(i32, 0xC7B4D79B01FA7A5C, native_parameters!());

    value
}

pub fn network_get_host_of_script(scriptName: &std::ffi::CString, p1: i32, p2: i32) -> i32 {
    let value = native!(
        i32,
        0x1D6A14F1F9A736FC,
        native_parameters!(scriptName.as_ptr(), p1, p2)
    );

    value
}

pub fn network_set_mission_finished() -> () {
    let value = native!((), 0x3B3D11CD9FFCDFC9, native_parameters!());

    value
}

pub fn network_is_script_active(
    scriptName: &std::ffi::CString,
    player: i32,
    p2: bool,
    p3: u32,
) -> bool {
    let value = native!(
        bool,
        0x9D40DF90FAD26098,
        native_parameters!(scriptName.as_ptr(), player, p2, p3)
    );

    value
}

pub fn network_is_script_active_by_hash(scriptHash: u32, p1: i32, p2: bool, p3: i32) -> bool {
    let value = native!(
        bool,
        0xDA7DE67F5FE5EE13,
        native_parameters!(scriptHash, p1, p2, p3)
    );

    value
}

pub fn _0x560b423d73015e77(p0: u32) -> u32 {
    let value = native!(u32, 0x560B423D73015E77, native_parameters!(p0));

    value
}

pub fn network_get_num_script_participants(p0: *mut u32, p1: u32, p2: u32) -> i32 {
    let value = native!(i32, 0x3658E8CD94FC121A, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x638a3a81733086db() -> u32 {
    let value = native!(u32, 0x638A3A81733086DB, native_parameters!());

    value
}

pub fn network_is_player_a_participant_on_script(
    player1: i32,
    script: &std::ffi::CString,
    player2: i32,
) -> bool {
    let value = native!(
        bool,
        0x1AD5B71586B94820,
        native_parameters!(player1, script.as_ptr(), player2)
    );

    value
}

pub fn _0x2302c0264ea58d31() -> () {
    let value = native!((), 0x2302C0264EA58D31, native_parameters!());

    value
}

pub fn _0x741a3d8380319a81() -> () {
    let value = native!((), 0x741A3D8380319A81, native_parameters!());

    value
}

pub fn participant_id() -> i32 {
    let value = native!(i32, 0x90986E8876CE0A83, native_parameters!());

    value
}

pub fn participant_id_to_int() -> i32 {
    let value = native!(i32, 0x57A3BDDAD8E5AA0A, native_parameters!());

    value
}

pub fn _0x2da41ed6e1fcd7a5(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x2DA41ED6E1FCD7A5, native_parameters!(p0, p1));

    value
}

pub fn network_get_destroyer_of_network_id(netId: i32, weaponHash: *mut u32) -> i32 {
    let value = native!(
        i32,
        0x7A1ADEEF01740A24,
        native_parameters!(netId, weaponHash)
    );

    value
}

pub fn _0xc434133d9ba52777(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0xC434133D9BA52777, native_parameters!(p0, p1));

    value
}

pub fn _0x83660b734994124d(p0: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(u32, 0x83660B734994124D, native_parameters!(p0, p1, p2));

    value
}

pub fn _network_get_destroyer_of_entity(p0: u32, p1: u32, weaponHash: *mut u32) -> bool {
    let value = native!(
        bool,
        0x4CACA84440FA26F6,
        native_parameters!(p0, p1, weaponHash)
    );

    value
}

pub fn network_get_entity_killer_of_player(player: i32, weaponHash: *mut u32) -> i32 {
    let value = native!(
        i32,
        0x42B2DAA6B596F5F8,
        native_parameters!(player, weaponHash)
    );

    value
}

pub fn network_resurrect_local_player(
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    unk: bool,
    changetime: bool,
    p6: u32,
) -> () {
    let value = native!(
        (),
        0xEA23C49EAA83ACFB,
        native_parameters!(x, y, z, heading, unk, changetime, p6)
    );

    value
}

pub fn network_set_local_player_invincible_time(time: i32) -> () {
    let value = native!((), 0x2D95C7E2D7E07307, native_parameters!(time));

    value
}

pub fn network_is_local_player_invincible() -> bool {
    let value = native!(bool, 0x8A8694B48715B000, native_parameters!());

    value
}

pub fn network_disable_invincible_flashing(player: i32, toggle: bool) -> () {
    let value = native!((), 0x9DD368BF06983221, native_parameters!(player, toggle));

    value
}

pub fn network_set_local_player_sync_look_at(toggle: bool) -> () {
    let value = native!((), 0x524FF0AEFF9C3973, native_parameters!(toggle));

    value
}

pub fn network_has_entity_been_registered_with_this_thread(entity: i32) -> bool {
    let value = native!(bool, 0xB07D3185E11657A5, native_parameters!(entity));

    value
}

pub fn network_get_network_id_from_entity(entity: i32) -> i32 {
    let value = native!(i32, 0xA11700682F3AD45C, native_parameters!(entity));

    value
}

pub fn network_get_entity_from_network_id(netId: i32) -> i32 {
    let value = native!(i32, 0xCE4E5D9B0A4FF560, native_parameters!(netId));

    value
}

pub fn network_get_entity_is_networked(entity: i32) -> bool {
    let value = native!(bool, 0xC7827959479DCC78, native_parameters!(entity));

    value
}

pub fn network_get_entity_is_local(entity: i32) -> bool {
    let value = native!(bool, 0x0991549DE4D64762, native_parameters!(entity));

    value
}

pub fn network_register_entity_as_networked(entity: i32) -> () {
    let value = native!((), 0x06FAACD625D80CAA, native_parameters!(entity));

    value
}

pub fn network_unregister_networked_entity(entity: i32) -> () {
    let value = native!((), 0x7368E683BB9038D6, native_parameters!(entity));

    value
}

pub fn network_does_network_id_exist(netId: i32) -> bool {
    let value = native!(bool, 0x38CE16C96BD11344, native_parameters!(netId));

    value
}

pub fn network_does_entity_exist_with_network_id(netId: i32) -> bool {
    let value = native!(bool, 0x18A47D074708FD68, native_parameters!(netId));

    value
}

pub fn network_request_control_of_network_id(netId: i32) -> bool {
    let value = native!(bool, 0xA670B3662FAFFBD0, native_parameters!(netId));

    value
}

pub fn network_has_control_of_network_id(netId: i32) -> bool {
    let value = native!(bool, 0x4D36070FE0215186, native_parameters!(netId));

    value
}

pub fn _0x7242f8b741ce1086(netId: i32) -> bool {
    let value = native!(bool, 0x7242F8B741CE1086, native_parameters!(netId));

    value
}

pub fn network_request_control_of_entity(entity: i32) -> bool {
    let value = native!(bool, 0xB69317BF5E782347, native_parameters!(entity));

    value
}

pub fn network_request_control_of_door(doorID: i32) -> bool {
    let value = native!(bool, 0x870DDFD5A4A796E4, native_parameters!(doorID));

    value
}

pub fn network_has_control_of_entity(entity: i32) -> bool {
    let value = native!(bool, 0x01BF60A500E28887, native_parameters!(entity));

    value
}

pub fn network_has_control_of_pickup(pickup: i32) -> bool {
    let value = native!(bool, 0x5BC9495F0B3B6FA6, native_parameters!(pickup));

    value
}

pub fn network_has_control_of_door(doorHash: u32) -> bool {
    let value = native!(bool, 0xCB3C68ADB06195DF, native_parameters!(doorHash));

    value
}

pub fn network_is_door_networked(doorHash: u32) -> bool {
    let value = native!(bool, 0xC01E93FAC20C3346, native_parameters!(doorHash));

    value
}

pub fn veh_to_net(vehicle: i32) -> i32 {
    let value = native!(i32, 0xB4C94523F023419C, native_parameters!(vehicle));

    value
}

pub fn ped_to_net(ped: i32) -> i32 {
    let value = native!(i32, 0x0EDEC3C276198689, native_parameters!(ped));

    value
}

pub fn obj_to_net(object: i32) -> i32 {
    let value = native!(i32, 0x99BFDC94A603E541, native_parameters!(object));

    value
}

pub fn net_to_veh(netHandle: i32) -> i32 {
    let value = native!(i32, 0x367B936610BA360C, native_parameters!(netHandle));

    value
}

pub fn net_to_ped(netHandle: i32) -> i32 {
    let value = native!(i32, 0xBDCD95FC216A8B3E, native_parameters!(netHandle));

    value
}

pub fn net_to_obj(netHandle: i32) -> i32 {
    let value = native!(i32, 0xD8515F5FEA14CB3F, native_parameters!(netHandle));

    value
}

pub fn net_to_ent(netHandle: i32) -> i32 {
    let value = native!(i32, 0xBFFEAB45A9A9094A, native_parameters!(netHandle));

    value
}

pub fn network_get_local_handle(networkHandle: *mut i32, bufferSize: i32) -> () {
    let value = native!(
        (),
        0xE86051786B66CD8E,
        native_parameters!(networkHandle, bufferSize)
    );

    value
}

pub fn network_handle_from_user_id(
    userId: &std::ffi::CString,
    networkHandle: *mut i32,
    bufferSize: i32,
) -> () {
    let value = native!(
        (),
        0xDCD51DD8F87AEC5C,
        native_parameters!(userId.as_ptr(), networkHandle, bufferSize)
    );

    value
}

pub fn network_handle_from_member_id(
    memberId: &std::ffi::CString,
    networkHandle: *mut i32,
    bufferSize: i32,
) -> () {
    let value = native!(
        (),
        0xA0FD21BED61E5C4C,
        native_parameters!(memberId.as_ptr(), networkHandle, bufferSize)
    );

    value
}

pub fn network_handle_from_player(player: i32, networkHandle: *mut i32, bufferSize: i32) -> () {
    let value = native!(
        (),
        0x388EB2B86C73B6B3,
        native_parameters!(player, networkHandle, bufferSize)
    );

    value
}

pub fn network_hash_from_player_handle(player: i32) -> u32 {
    let value = native!(u32, 0xBC1D768F2F5D6C05, native_parameters!(player));

    value
}

pub fn network_hash_from_gamer_handle(networkHandle: *mut i32) -> u32 {
    let value = native!(u32, 0x58575AC3CF2CA8EC, native_parameters!(networkHandle));

    value
}

pub fn network_handle_from_friend(
    friendIndex: i32,
    networkHandle: *mut i32,
    bufferSize: i32,
) -> () {
    let value = native!(
        (),
        0xD45CB817D7E177D2,
        native_parameters!(friendIndex, networkHandle, bufferSize)
    );

    value
}

pub fn network_gamertag_from_handle_start(networkHandle: *mut i32) -> bool {
    let value = native!(bool, 0x9F0C0A981D73FA56, native_parameters!(networkHandle));

    value
}

pub fn network_gamertag_from_handle_pending() -> bool {
    let value = native!(bool, 0xB071E27958EF4CF0, native_parameters!());

    value
}

pub fn network_gamertag_from_handle_succeeded() -> bool {
    let value = native!(bool, 0xFD00798DBA7523DD, native_parameters!());

    value
}

pub fn network_get_gamertag_from_handle(networkHandle: *mut i32) -> String {
    let value = native!(
        *const i8,
        0x426141162EBE5CDB,
        native_parameters!(networkHandle)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _0xd66c9e72b3cc4982(p0: *mut u32, p1: u32) -> i32 {
    let value = native!(i32, 0xD66C9E72B3CC4982, native_parameters!(p0, p1));

    value
}

pub fn network_get_displaynames_from_handles(p0: u32, p1: u32, p2: u32) -> i32 {
    let value = native!(i32, 0x58CC181719256197, native_parameters!(p0, p1, p2));

    value
}

pub fn network_are_handles_the_same(netHandle1: *mut i32, netHandle2: *mut i32) -> bool {
    let value = native!(
        bool,
        0x57DBA049E110F217,
        native_parameters!(netHandle1, netHandle2)
    );

    value
}

pub fn network_is_handle_valid(networkHandle: *mut i32, bufferSize: i32) -> bool {
    let value = native!(
        bool,
        0x6F79B93B0A8E4133,
        native_parameters!(networkHandle, bufferSize)
    );

    value
}

pub fn network_get_player_from_gamer_handle(networkHandle: *mut i32) -> i32 {
    let value = native!(i32, 0xCE5F689CF5A0A49D, native_parameters!(networkHandle));

    value
}

pub fn network_member_id_from_gamer_handle(networkHandle: *mut i32) -> String {
    let value = native!(
        *const i8,
        0xC82630132081BB6F,
        native_parameters!(networkHandle)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn network_is_gamer_in_my_session(networkHandle: *mut i32) -> bool {
    let value = native!(bool, 0x0F10B05DDF8D16E9, native_parameters!(networkHandle));

    value
}

pub fn network_show_profile_ui(networkHandle: *mut i32) -> () {
    let value = native!((), 0x859ED1CEA343FCA8, native_parameters!(networkHandle));

    value
}

pub fn network_player_get_name(player: i32) -> String {
    let value = native!(*const i8, 0x7718D2E2060837D2, native_parameters!(player));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn network_player_get_userid(player: i32, userID: *mut i32) -> String {
    let value = native!(
        *const i8,
        0x4927FC39CD0869A0,
        native_parameters!(player, userID)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn network_player_is_rockstar_dev(player: i32) -> bool {
    let value = native!(bool, 0x544ABDDA3B409B6D, native_parameters!(player));

    value
}

pub fn network_player_index_is_cheater(player: i32) -> bool {
    let value = native!(bool, 0x565E430DB3B05BEC, native_parameters!(player));

    value
}

pub fn _network_get_entity_net_script_id(entity: i32) -> i32 {
    let value = native!(i32, 0x815F18AD865F057F, native_parameters!(entity));

    value
}

pub fn _0x37d5f739fd494675(p0: u32) -> i32 {
    let value = native!(i32, 0x37D5F739FD494675, native_parameters!(p0));

    value
}

pub fn network_is_inactive_profile(p0: *mut u32) -> bool {
    let value = native!(bool, 0x7E58745504313A2E, native_parameters!(p0));

    value
}

pub fn network_get_max_friends() -> i32 {
    let value = native!(i32, 0xAFEBB0D5D8F687D2, native_parameters!());

    value
}

pub fn network_get_friend_count() -> i32 {
    let value = native!(i32, 0x203F1CFD823B27A4, native_parameters!());

    value
}

pub fn network_get_friend_name(friendIndex: i32) -> String {
    let value = native!(
        *const i8,
        0xE11EBBB2A783FE8B,
        native_parameters!(friendIndex)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _network_get_friend_name_from_index(friendIndex: i32) -> String {
    let value = native!(
        *const i8,
        0x4164F227D052E293,
        native_parameters!(friendIndex)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn network_is_friend_online(name: &std::ffi::CString) -> bool {
    let value = native!(bool, 0x425A44533437B64D, native_parameters!(name.as_ptr()));

    value
}

pub fn network_is_friend_handle_online(networkHandle: *mut i32) -> bool {
    let value = native!(bool, 0x87EB7A3FFCB314DB, native_parameters!(networkHandle));

    value
}

pub fn network_is_friend_in_same_title(friendName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x2EA9A3BEDF3F17B8,
        native_parameters!(friendName.as_ptr())
    );

    value
}

pub fn network_is_friend_in_multiplayer(friendName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x57005C18827F3A28,
        native_parameters!(friendName.as_ptr())
    );

    value
}

pub fn network_is_friend(networkHandle: *mut i32) -> bool {
    let value = native!(bool, 0x1A24A179F9B31654, native_parameters!(networkHandle));

    value
}

pub fn network_is_pending_friend(p0: u32) -> u32 {
    let value = native!(u32, 0x0BE73DA6984A6E33, native_parameters!(p0));

    value
}

pub fn network_is_adding_friend() -> u32 {
    let value = native!(u32, 0x6EA101606F6E4D81, native_parameters!());

    value
}

pub fn network_add_friend(networkHandle: *mut i32, message: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x8E02D73914064223,
        native_parameters!(networkHandle, message.as_ptr())
    );

    value
}

pub fn network_is_friend_index_online(friendIndex: i32) -> bool {
    let value = native!(bool, 0xBAD8F2A42B844821, native_parameters!(friendIndex));

    value
}

pub fn network_set_player_is_passive(toggle: bool) -> () {
    let value = native!((), 0x1B857666604B1A74, native_parameters!(toggle));

    value
}

pub fn network_get_player_owns_waypoint(player: i32) -> bool {
    let value = native!(bool, 0x82377B65E943F72D, native_parameters!(player));

    value
}

pub fn network_can_set_waypoint() -> bool {
    let value = native!(bool, 0xC927EC229934AF60, native_parameters!());

    value
}

pub fn _0x4c2a9fdc22377075() -> () {
    let value = native!((), 0x4C2A9FDC22377075, native_parameters!());

    value
}

pub fn _0xb309ebea797e001f(p0: u32) -> u32 {
    let value = native!(u32, 0xB309EBEA797E001F, native_parameters!(p0));

    value
}

pub fn _0x26f07dd83a5f7f98() -> u32 {
    let value = native!(u32, 0x26F07DD83A5F7F98, native_parameters!());

    value
}

pub fn network_has_headset() -> bool {
    let value = native!(bool, 0xE870F9F1F7B4F1FA, native_parameters!());

    value
}

pub fn _0x7d395ea61622e116(p0: bool) -> () {
    let value = native!((), 0x7D395EA61622E116, native_parameters!(p0));

    value
}

pub fn network_is_local_talking() -> bool {
    let value = native!(bool, 0xC0D2AF00BCC234CA, native_parameters!());

    value
}

pub fn network_gamer_has_headset(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0xF2FD55CB574BCC55, native_parameters!(networkHandle));

    value
}

pub fn network_is_gamer_talking(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0x71C33B22606CD88A, native_parameters!(networkHandle));

    value
}

pub fn _network_can_communicate_with_gamer_2(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0x8F5D1AD832AEB06C, native_parameters!(networkHandle));

    value
}

pub fn network_can_communicate_with_gamer(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0xA150A4F065806B1F, native_parameters!(networkHandle));

    value
}

pub fn network_is_gamer_muted_by_me(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0xCE60DE011B6C7978, native_parameters!(networkHandle));

    value
}

pub fn network_am_i_muted_by_gamer(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0xDF02A2C93F1F26DA, native_parameters!(networkHandle));

    value
}

pub fn network_is_gamer_blocked_by_me(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0xE944C4F5AF1B5883, native_parameters!(networkHandle));

    value
}

pub fn network_am_i_blocked_by_gamer(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0x15337C7C268A27B2, native_parameters!(networkHandle));

    value
}

pub fn network_can_view_gamer_user_content(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0xB57A49545BA53CE7, native_parameters!(networkHandle));

    value
}

pub fn network_has_view_gamer_user_content_result(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0xCCA4318E1AB03F1F, native_parameters!(networkHandle));

    value
}

pub fn network_can_play_multiplayer_with_gamer(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0x07DD29D5E22763F1, native_parameters!(networkHandle));

    value
}

pub fn network_can_gamer_play_multiplayer_with_me(networkHandle: *mut u32) -> bool {
    let value = native!(bool, 0x135F9B7B7ADD2185, native_parameters!(networkHandle));

    value
}

pub fn network_is_player_talking(player: i32) -> bool {
    let value = native!(bool, 0x031E11F3D447647E, native_parameters!(player));

    value
}

pub fn network_player_has_headset(player: i32) -> bool {
    let value = native!(bool, 0x3FB99A8B08D18FD6, native_parameters!(player));

    value
}

pub fn network_is_player_muted_by_me(player: i32) -> bool {
    let value = native!(bool, 0x8C71288AE68EDE39, native_parameters!(player));

    value
}

pub fn network_am_i_muted_by_player(player: i32) -> bool {
    let value = native!(bool, 0x9D6981DFC91A8604, native_parameters!(player));

    value
}

pub fn network_is_player_blocked_by_me(player: i32) -> bool {
    let value = native!(bool, 0x57AF1F8E27483721, native_parameters!(player));

    value
}

pub fn network_am_i_blocked_by_player(player: i32) -> bool {
    let value = native!(bool, 0x87F395D957D4353D, native_parameters!(player));

    value
}

pub fn network_get_player_loudness(player: i32) -> f32 {
    let value = native!(f32, 0x21A1684A25C2867F, native_parameters!(player));

    value
}

pub fn network_set_talker_proximity(value: f32) -> () {
    let value = native!((), 0xCBF12D65F95AD686, native_parameters!(value));

    value
}

pub fn network_get_talker_proximity() -> f32 {
    let value = native!(f32, 0x84F0F13120B4E098, native_parameters!());

    value
}

pub fn network_set_voice_active(toggle: bool) -> () {
    let value = native!((), 0xBABEC9E69A91C57B, native_parameters!(toggle));

    value
}

pub fn _0xcfeb46dcd7d8d5eb(p0: bool) -> () {
    let value = native!((), 0xCFEB46DCD7D8D5EB, native_parameters!(p0));

    value
}

pub fn network_override_transition_chat(p0: bool) -> () {
    let value = native!((), 0xAF66059A131AA269, native_parameters!(p0));

    value
}

pub fn network_set_team_only_chat(toggle: bool) -> () {
    let value = native!((), 0xD5B4883AC32F24C3, native_parameters!(toggle));

    value
}

pub fn _0x265559da40b3f327(p0: u32) -> () {
    let value = native!((), 0x265559DA40B3F327, native_parameters!(p0));

    value
}

pub fn _0x4348bfda56023a2f(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x4348BFDA56023A2F, native_parameters!(p0, p1));

    value
}

pub fn network_override_team_restrictions(team: i32, toggle: bool) -> () {
    let value = native!((), 0x6F697A66CE78674E, native_parameters!(team, toggle));

    value
}

pub fn network_set_override_spectator_mode(toggle: bool) -> () {
    let value = native!((), 0x70DA3BF8DACD3210, native_parameters!(toggle));

    value
}

pub fn _0x3c5c1e2c2ff814b1(toggle: bool) -> () {
    let value = native!((), 0x3C5C1E2C2FF814B1, native_parameters!(toggle));

    value
}

pub fn _0x9d7afcbf21c51712(toggle: bool) -> () {
    let value = native!((), 0x9D7AFCBF21C51712, native_parameters!(toggle));

    value
}

pub fn network_set_no_spectator_chat(toggle: bool) -> () {
    let value = native!((), 0xF46A1E03E8755980, native_parameters!(toggle));

    value
}

pub fn _0x6a5d89d7769a40d8(toggle: bool) -> () {
    let value = native!((), 0x6A5D89D7769A40D8, native_parameters!(toggle));

    value
}

pub fn network_override_chat_restrictions(player: i32, toggle: bool) -> () {
    let value = native!((), 0x3039AE5AD2C9C0C4, native_parameters!(player, toggle));

    value
}

pub fn network_override_send_restrictions(player: i32, toggle: bool) -> () {
    let value = native!((), 0x97DD4C5944CC2E6A, native_parameters!(player, toggle));

    value
}

pub fn network_override_send_restrictions_all(toggle: bool) -> () {
    let value = native!((), 0x57B192B4D4AD23D5, native_parameters!(toggle));

    value
}

pub fn network_override_receive_restrictions(player: i32, toggle: bool) -> () {
    let value = native!((), 0xDDF73E2B1FEC5AB4, native_parameters!(player, toggle));

    value
}

pub fn network_override_receive_restrictions_all(toggle: bool) -> () {
    let value = native!((), 0x0FF2862B61A58AF9, native_parameters!(toggle));

    value
}

pub fn network_set_voice_channel(channel: i32) -> () {
    let value = native!((), 0xEF6212C2EFEF1A23, native_parameters!(channel));

    value
}

pub fn network_clear_voice_channel() -> () {
    let value = native!((), 0xE036A705F989E049, native_parameters!());

    value
}

pub fn network_apply_voice_proximity_override(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xDBD2056652689917, native_parameters!(x, y, z));

    value
}

pub fn network_clear_voice_proximity_override() -> () {
    let value = native!((), 0xF03755696450470C, native_parameters!());

    value
}

pub fn _0x5e3aa4ca2b6fb0ee(p0: u32) -> () {
    let value = native!((), 0x5E3AA4CA2B6FB0EE, native_parameters!(p0));

    value
}

pub fn _0xca575c391fea25cc(p0: u32) -> () {
    let value = native!((), 0xCA575C391FEA25CC, native_parameters!(p0));

    value
}

pub fn _0xadb57e5b663cca8b(p0: i32, p1: *mut f32, p2: *mut f32) -> () {
    let value = native!((), 0xADB57E5B663CCA8B, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x8ef52acaecc51d9c(toggle: bool) -> () {
    let value = native!((), 0x8EF52ACAECC51D9C, native_parameters!(toggle));

    value
}

pub fn _network_is_text_chat_active() -> bool {
    let value = native!(bool, 0x5FCF4D7069B09026, native_parameters!());

    value
}

pub fn shutdown_and_launch_single_player_game() -> () {
    let value = native!((), 0x593850C16A36B692, native_parameters!());

    value
}

pub fn _shutdown_and_load_most_recent_save() -> bool {
    let value = native!(bool, 0x9ECA15ADFE141431, native_parameters!());

    value
}

pub fn network_set_friendly_fire_option(toggle: bool) -> () {
    let value = native!((), 0xF808475FA571D823, native_parameters!(toggle));

    value
}

pub fn network_set_rich_presence(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x1DCCACDCFC569362, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn network_set_rich_presence_string(p0: i32, textLabel: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x3E200C2BCF4164EB,
        native_parameters!(p0, textLabel.as_ptr())
    );

    value
}

pub fn network_get_timeout_time() -> i32 {
    let value = native!(i32, 0x5ED0356A0CE3A34F, native_parameters!());

    value
}

pub fn _network_respawn_coords(player: i32, x: f32, y: f32, z: f32, p4: bool, p5: bool) -> () {
    let value = native!(
        (),
        0x9769F811D1785B03,
        native_parameters!(player, x, y, z, p4, p5)
    );

    value
}

pub fn _0xbf22e0f32968e967(player: i32, p1: bool) -> () {
    let value = native!((), 0xBF22E0F32968E967, native_parameters!(player, p1));

    value
}

pub fn remove_all_sticky_bombs_from_entity(entity: i32, ped: i32) -> () {
    let value = native!((), 0x715135F4B82AC90D, native_parameters!(entity, ped));

    value
}

pub fn _0x17c9e241111a674d(p0: u32, p1: u32) -> () {
    let value = native!((), 0x17C9E241111A674D, native_parameters!(p0, p1));

    value
}

pub fn _0x2e4c123d1c8a710e(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32, p6: u32) -> u32 {
    let value = native!(
        u32,
        0x2E4C123D1C8A710E,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn network_clan_service_is_valid() -> bool {
    let value = native!(bool, 0x579CCED0265D4896, native_parameters!());

    value
}

pub fn network_clan_player_is_active(networkHandle: *mut i32) -> bool {
    let value = native!(bool, 0xB124B57F571D8F18, native_parameters!(networkHandle));

    value
}

pub fn network_clan_player_get_desc(
    clanDesc: *mut i32,
    bufferSize: i32,
    networkHandle: *mut i32,
) -> bool {
    let value = native!(
        bool,
        0xEEE6EACBE8874FBA,
        native_parameters!(clanDesc, bufferSize, networkHandle)
    );

    value
}

pub fn network_clan_is_rockstar_clan(clanDesc: *mut i32, bufferSize: i32) -> bool {
    let value = native!(
        bool,
        0x7543BB439F63792B,
        native_parameters!(clanDesc, bufferSize)
    );

    value
}

pub fn network_clan_get_ui_formatted_tag(
    clanDesc: *mut i32,
    bufferSize: i32,
    formattedTag: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0xF45352426FF3A4F0,
        native_parameters!(clanDesc, bufferSize, formattedTag.as_ptr())
    );

    value
}

pub fn network_clan_get_local_memberships_count() -> i32 {
    let value = native!(i32, 0x1F471B79ACC90BEF, native_parameters!());

    value
}

pub fn network_clan_get_membership_desc(memberDesc: *mut i32, p1: i32) -> bool {
    let value = native!(bool, 0x48DE78AF2C8885B8, native_parameters!(memberDesc, p1));

    value
}

pub fn network_clan_download_membership(networkHandle: *mut i32) -> bool {
    let value = native!(bool, 0xA989044E70010ABE, native_parameters!(networkHandle));

    value
}

pub fn network_clan_download_membership_pending(p0: *mut u32) -> bool {
    let value = native!(bool, 0x5B9E023DC6EBEDC0, native_parameters!(p0));

    value
}

pub fn _network_is_clan_membership_finished_downloading() -> bool {
    let value = native!(bool, 0xB3F64A6A91432477, native_parameters!());

    value
}

pub fn network_clan_remote_memberships_are_in_cache(p0: *mut i32) -> bool {
    let value = native!(bool, 0xBB6E6FEE99D866B2, native_parameters!(p0));

    value
}

pub fn network_clan_get_membership_count(p0: *mut i32) -> i32 {
    let value = native!(i32, 0xAAB11F6C4ADBC2C1, native_parameters!(p0));

    value
}

pub fn network_clan_get_membership_valid(p0: *mut i32, p1: u32) -> bool {
    let value = native!(bool, 0x48A59CF88D43DF0E, native_parameters!(p0, p1));

    value
}

pub fn network_clan_get_membership(p0: *mut i32, clanMembership: *mut i32, p2: i32) -> bool {
    let value = native!(
        bool,
        0xC8BC2011F67B3411,
        native_parameters!(p0, clanMembership, p2)
    );

    value
}

pub fn network_clan_join(clanDesc: i32) -> bool {
    let value = native!(bool, 0x9FAAA4F4FC71F87F, native_parameters!(clanDesc));

    value
}

pub fn _network_clan_animation(animDict: &std::ffi::CString, animName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x729E3401F0430686,
        native_parameters!(animDict.as_ptr(), animName.as_ptr())
    );

    value
}

pub fn _0x2b51edbefc301339(p0: i32, p1: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x2B51EDBEFC301339,
        native_parameters!(p0, p1.as_ptr())
    );

    value
}

pub fn _0xc32ea7a2f6ca7557() -> u32 {
    let value = native!(u32, 0xC32EA7A2F6CA7557, native_parameters!());

    value
}

pub fn network_clan_get_emblem_txd_name(netHandle: *mut u32, txdName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x5835D9CD92E83184,
        native_parameters!(netHandle, txdName.as_ptr())
    );

    value
}

pub fn network_clan_request_emblem(p0: u32) -> bool {
    let value = native!(bool, 0x13518FF1C6B28938, native_parameters!(p0));

    value
}

pub fn network_clan_is_emblem_ready(p0: u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0xA134777FF7F33331, native_parameters!(p0, p1));

    value
}

pub fn network_clan_release_emblem(p0: u32) -> () {
    let value = native!((), 0x113E6E3E50E286B0, native_parameters!(p0));

    value
}

pub fn network_get_primary_clan_data_clear() -> u32 {
    let value = native!(u32, 0x9AA46BADAD0E27ED, native_parameters!());

    value
}

pub fn network_get_primary_clan_data_cancel() -> () {
    let value = native!((), 0x042E4B70B93E6054, native_parameters!());

    value
}

pub fn network_get_primary_clan_data_start(p0: *mut u32, p1: u32) -> bool {
    let value = native!(bool, 0xCE86D8191B762107, native_parameters!(p0, p1));

    value
}

pub fn network_get_primary_clan_data_pending() -> u32 {
    let value = native!(u32, 0xB5074DB804E28CE7, native_parameters!());

    value
}

pub fn network_get_primary_clan_data_success() -> u32 {
    let value = native!(u32, 0x5B4F04F19376A0BA, native_parameters!());

    value
}

pub fn network_get_primary_clan_data_new(p0: *mut u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0xC080FF658B2E41DA, native_parameters!(p0, p1));

    value
}

pub fn set_network_id_can_migrate(netId: i32, toggle: bool) -> () {
    let value = native!((), 0x299EEB23175895FC, native_parameters!(netId, toggle));

    value
}

pub fn set_network_id_exists_on_all_machines(netId: i32, toggle: bool) -> () {
    let value = native!((), 0xE05E81A888FA63C8, native_parameters!(netId, toggle));

    value
}

pub fn set_network_id_always_exists_for_player(netId: i32, player: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0xA8A024587329F36A,
        native_parameters!(netId, player, toggle)
    );

    value
}

pub fn _0x9d724b400a7e8ffc(p0: u32, p1: u32) -> () {
    let value = native!((), 0x9D724B400A7E8FFC, native_parameters!(p0, p1));

    value
}

pub fn network_set_entity_can_blend(entity: i32, toggle: bool) -> () {
    let value = native!((), 0xD830567D88A1E873, native_parameters!(entity, toggle));

    value
}

pub fn _0x0379daf89ba09aa5(p0: u32, p1: u32) -> () {
    let value = native!((), 0x0379DAF89BA09AA5, native_parameters!(p0, p1));

    value
}

pub fn _network_set_entity_invisible_to_network(entity: i32, toggle: bool) -> () {
    let value = native!((), 0xF1CA12B18AEF5298, native_parameters!(entity, toggle));

    value
}

pub fn set_network_id_visible_in_cutscene(netId: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xA6928482543022B4, native_parameters!(netId, p1, p2));

    value
}

pub fn _0x32ebd154cb6b8b99(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x32EBD154CB6B8B99, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x76b3f29d3f967692(p0: u32, p1: u32) -> () {
    let value = native!((), 0x76B3F29D3F967692, native_parameters!(p0, p1));

    value
}

pub fn set_network_cutscene_entities(toggle: bool) -> () {
    let value = native!((), 0xAAA553E7DD28A457, native_parameters!(toggle));

    value
}

pub fn _0x3fa36981311fa4ff(netId: i32, state: bool) -> () {
    let value = native!((), 0x3FA36981311FA4FF, native_parameters!(netId, state));

    value
}

pub fn is_network_id_owned_by_participant(netId: i32) -> bool {
    let value = native!(bool, 0xA1607996431332DF, native_parameters!(netId));

    value
}

pub fn set_local_player_visible_in_cutscene(p0: bool, p1: bool) -> () {
    let value = native!((), 0xD1065D68947E7B6E, native_parameters!(p0, p1));

    value
}

pub fn set_local_player_invisible_locally(p0: bool) -> () {
    let value = native!((), 0xE5F773C1A1D9D168, native_parameters!(p0));

    value
}

pub fn set_local_player_visible_locally(p0: bool) -> () {
    let value = native!((), 0x7619364C82D3BF14, native_parameters!(p0));

    value
}

pub fn set_player_invisible_locally(player: i32, toggle: bool) -> () {
    let value = native!((), 0x12B37D54667DB0B8, native_parameters!(player, toggle));

    value
}

pub fn set_player_visible_locally(player: i32, toggle: bool) -> () {
    let value = native!((), 0xFAA10F1FAFB11AF2, native_parameters!(player, toggle));

    value
}

pub fn fade_out_local_player(p0: bool) -> () {
    let value = native!((), 0x416DBD4CD6ED8DD2, native_parameters!(p0));

    value
}

pub fn network_fade_out_entity(entity: i32, normal: bool, slow: bool) -> () {
    let value = native!(
        (),
        0xDE564951F95E09ED,
        native_parameters!(entity, normal, slow)
    );

    value
}

pub fn network_fade_in_entity(entity: i32, state: bool, p2: u32) -> () {
    let value = native!(
        (),
        0x1F4ED342ACEFE62D,
        native_parameters!(entity, state, p2)
    );

    value
}

pub fn network_is_player_fading(player: i32) -> bool {
    let value = native!(bool, 0x631DC5DFF4B110E3, native_parameters!(player));

    value
}

pub fn network_is_entity_fading(entity: i32) -> bool {
    let value = native!(bool, 0x422F32CC7E56ABAD, native_parameters!(entity));

    value
}

pub fn is_player_in_cutscene(player: i32) -> bool {
    let value = native!(bool, 0xE73092F4157CD126, native_parameters!(player));

    value
}

pub fn set_entity_visible_in_cutscene(p0: u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xE0031D3C8F36AB82, native_parameters!(p0, p1, p2));

    value
}

pub fn set_entity_locally_invisible(entity: i32) -> () {
    let value = native!((), 0xE135A9FF3F5D05D8, native_parameters!(entity));

    value
}

pub fn set_entity_locally_visible(entity: i32) -> () {
    let value = native!((), 0x241E289B5C059EDC, native_parameters!(entity));

    value
}

pub fn is_damage_tracker_active_on_network_id(netID: i32) -> bool {
    let value = native!(bool, 0x6E192E33AD436366, native_parameters!(netID));

    value
}

pub fn activate_damage_tracker_on_network_id(netID: i32, toggle: bool) -> () {
    let value = native!((), 0xD45B1FFCCD52FF19, native_parameters!(netID, toggle));

    value
}

pub fn _is_damage_tracker_active_on_player(player: i32) -> bool {
    let value = native!(bool, 0xB2092A1EAA7FD45F, native_parameters!(player));

    value
}

pub fn _activate_damage_tracker_on_player(player: i32, toggle: bool) -> () {
    let value = native!((), 0xBEC0816FF5ACBCDA, native_parameters!(player, toggle));

    value
}

pub fn is_sphere_visible_to_another_machine(p0: f32, p1: f32, p2: f32, p3: f32) -> bool {
    let value = native!(bool, 0xD82CF8E64C8729D8, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn is_sphere_visible_to_player(p0: u32, p1: f32, p2: f32, p3: f32, p4: f32) -> bool {
    let value = native!(
        bool,
        0xDC3A310219E5DA62,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn reserve_network_mission_objects(amount: i32) -> () {
    let value = native!((), 0x4E5C93BD0C32FBF8, native_parameters!(amount));

    value
}

pub fn reserve_network_mission_peds(amount: i32) -> () {
    let value = native!((), 0xB60FEBA45333D36F, native_parameters!(amount));

    value
}

pub fn reserve_network_mission_vehicles(amount: i32) -> () {
    let value = native!((), 0x76B02E21ED27A469, native_parameters!(amount));

    value
}

pub fn _reserve_network_local_objects(amount: i32) -> () {
    let value = native!((), 0x797F9C5E661D920E, native_parameters!(amount));

    value
}

pub fn _reserve_network_local_peds(amount: i32) -> () {
    let value = native!((), 0x2C8DF5D129595281, native_parameters!(amount));

    value
}

pub fn _reserve_network_local_vehicles(amount: i32) -> () {
    let value = native!((), 0x42613035157E4208, native_parameters!(amount));

    value
}

pub fn can_register_mission_objects(amount: i32) -> bool {
    let value = native!(bool, 0x800DD4721A8B008B, native_parameters!(amount));

    value
}

pub fn can_register_mission_peds(amount: i32) -> bool {
    let value = native!(bool, 0xBCBF4FEF9FA5D781, native_parameters!(amount));

    value
}

pub fn can_register_mission_vehicles(amount: i32) -> bool {
    let value = native!(bool, 0x7277F1F2E085EE74, native_parameters!(amount));

    value
}

pub fn _can_register_mission_pickups(amount: i32) -> bool {
    let value = native!(bool, 0x0A49D1CB6E34AF72, native_parameters!(amount));

    value
}

pub fn _0xe16aa70ce9beedc3(p0: u32) -> u32 {
    let value = native!(u32, 0xE16AA70CE9BEEDC3, native_parameters!(p0));

    value
}

pub fn can_register_mission_entities(
    ped_amt: i32,
    vehicle_amt: i32,
    object_amt: i32,
    pickup_amt: i32,
) -> bool {
    let value = native!(
        bool,
        0x69778E7564BADE6D,
        native_parameters!(ped_amt, vehicle_amt, object_amt, pickup_amt)
    );

    value
}

pub fn get_num_reserved_mission_objects(p0: bool, p1: u32) -> i32 {
    let value = native!(i32, 0xAA81B5F10BC43AC2, native_parameters!(p0, p1));

    value
}

pub fn get_num_reserved_mission_peds(p0: bool, p1: u32) -> i32 {
    let value = native!(i32, 0x1F13D5AE5CB17E17, native_parameters!(p0, p1));

    value
}

pub fn get_num_reserved_mission_vehicles(p0: bool, p1: u32) -> i32 {
    let value = native!(i32, 0xCF3A965906452031, native_parameters!(p0, p1));

    value
}

pub fn get_num_created_mission_objects(p0: bool) -> i32 {
    let value = native!(i32, 0x12B6281B6C6706C0, native_parameters!(p0));

    value
}

pub fn get_num_created_mission_peds(p0: bool) -> i32 {
    let value = native!(i32, 0xCB215C4B56A7FAE7, native_parameters!(p0));

    value
}

pub fn get_num_created_mission_vehicles(p0: bool) -> i32 {
    let value = native!(i32, 0x0CD9AB83489430EA, native_parameters!(p0));

    value
}

pub fn _0xe42d626eec94e5d9(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32, p6: u32) -> () {
    let value = native!(
        (),
        0xE42D626EEC94E5D9,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn get_max_num_network_objects() -> i32 {
    let value = native!(i32, 0xC7BE335216B5EC7C, native_parameters!());

    value
}

pub fn get_max_num_network_peds() -> i32 {
    let value = native!(i32, 0x0C1F7D49C39D2289, native_parameters!());

    value
}

pub fn get_max_num_network_vehicles() -> i32 {
    let value = native!(i32, 0x0AFCE529F69B21FF, native_parameters!());

    value
}

pub fn get_max_num_network_pickups() -> i32 {
    let value = native!(i32, 0xA72835064DD63E4C, native_parameters!());

    value
}

pub fn _0xba7f0b77d80a4eb7(p0: u32, p1: u32) -> () {
    let value = native!((), 0xBA7F0B77D80A4EB7, native_parameters!(p0, p1));

    value
}

pub fn _0x0f1a4b45b7693b95(p0: u32, p1: u32) -> () {
    let value = native!((), 0x0F1A4B45B7693B95, native_parameters!(p0, p1));

    value
}

pub fn get_network_time() -> i32 {
    let value = native!(i32, 0x7A5487FE9FAA6B48, native_parameters!());

    value
}

pub fn get_network_time_accurate() -> i32 {
    let value = native!(i32, 0x89023FBBF9200E9F, native_parameters!());

    value
}

pub fn has_network_time_started() -> bool {
    let value = native!(bool, 0x46718ACEEDEAFC84, native_parameters!());

    value
}

pub fn get_time_offset(timeA: i32, timeB: i32) -> i32 {
    let value = native!(i32, 0x017008CCDAD48503, native_parameters!(timeA, timeB));

    value
}

pub fn is_time_less_than(timeA: i32, timeB: i32) -> bool {
    let value = native!(bool, 0xCB2CF5148012C8D0, native_parameters!(timeA, timeB));

    value
}

pub fn is_time_more_than(timeA: i32, timeB: i32) -> bool {
    let value = native!(bool, 0xDE350F8651E4346C, native_parameters!(timeA, timeB));

    value
}

pub fn is_time_equal_to(timeA: i32, timeB: i32) -> bool {
    let value = native!(bool, 0xF5BC95857BD6D512, native_parameters!(timeA, timeB));

    value
}

pub fn get_time_difference(timeA: i32, timeB: i32) -> i32 {
    let value = native!(i32, 0xA2C6FC031D46FFF0, native_parameters!(timeA, timeB));

    value
}

pub fn get_time_as_string(time: i32) -> String {
    let value = native!(*const i8, 0x9E23B1777A927DAD, native_parameters!(time));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _get_cloud_time_as_string() -> String {
    let value = native!(*const i8, 0xF12E6CD06C73D69E, native_parameters!());
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_cloud_time_as_int() -> i32 {
    let value = native!(i32, 0x9A73240B49945C76, native_parameters!());

    value
}

pub fn convert_posix_time(posixTime: i32, timeStructure: *mut u32) -> () {
    let value = native!(
        (),
        0xAC97AF97FA68E5D5,
        native_parameters!(posixTime, timeStructure)
    );

    value
}

pub fn network_set_in_spectator_mode(toggle: bool, playerPed: i32) -> () {
    let value = native!(
        (),
        0x423DE3854BB50894,
        native_parameters!(toggle, playerPed)
    );

    value
}

pub fn network_set_in_spectator_mode_extended(toggle: bool, playerPed: i32, p2: bool) -> () {
    let value = native!(
        (),
        0x419594E137637120,
        native_parameters!(toggle, playerPed, p2)
    );

    value
}

pub fn network_set_in_free_cam_mode(toggle: bool) -> () {
    let value = native!((), 0xFC18DB55AE19E046, native_parameters!(toggle));

    value
}

pub fn network_set_choice_migrate_options(toggle: bool, player: i32) -> () {
    let value = native!((), 0x5C707A667DF8B9FA, native_parameters!(toggle, player));

    value
}

pub fn network_is_in_spectator_mode() -> bool {
    let value = native!(bool, 0x048746E388762E11, native_parameters!());

    value
}

pub fn network_set_in_mp_cutscene(p0: bool, p1: bool) -> () {
    let value = native!((), 0x9CA5DE655269FEC4, native_parameters!(p0, p1));

    value
}

pub fn network_is_in_mp_cutscene() -> bool {
    let value = native!(bool, 0x6CC27C9FA2040220, native_parameters!());

    value
}

pub fn network_is_player_in_mp_cutscene(player: i32) -> bool {
    let value = native!(bool, 0x63F9EE203C3619F2, native_parameters!(player));

    value
}

pub fn _0xfac18e7356bd3210() -> () {
    let value = native!((), 0xFAC18E7356BD3210, native_parameters!());

    value
}

pub fn set_network_vehicle_respot_timer(netId: i32, time: i32, p2: u32, p3: u32) -> () {
    let value = native!(
        (),
        0xEC51713AB6EC36E8,
        native_parameters!(netId, time, p2, p3)
    );

    value
}

pub fn set_network_vehicle_as_ghost(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x6274C4712850841E, native_parameters!(vehicle, toggle));

    value
}

pub fn _0xa2a707979fe754dc(p0: u32, p1: u32) -> () {
    let value = native!((), 0xA2A707979FE754DC, native_parameters!(p0, p1));

    value
}

pub fn _0x838da0936a24ed4d(p0: u32, p1: u32) -> () {
    let value = native!((), 0x838DA0936A24ED4D, native_parameters!(p0, p1));

    value
}

pub fn _set_local_player_as_ghost(toggle: bool, p1: bool) -> () {
    let value = native!((), 0x5FFE9B4144F9712F, native_parameters!(toggle, p1));

    value
}

pub fn _is_entity_ghosted_to_local_player(entity: i32) -> bool {
    let value = native!(bool, 0x21D04D7BC538C146, native_parameters!(entity));

    value
}

pub fn _0x13f1fcb111b820b0(p0: bool) -> () {
    let value = native!((), 0x13F1FCB111B820B0, native_parameters!(p0));

    value
}

pub fn _set_relationship_to_player(player: i32, p1: bool) -> () {
    let value = native!((), 0xA7C511FA1C5BDA38, native_parameters!(player, p1));

    value
}

pub fn _set_ghosted_entity_alpha(alpha: i32) -> () {
    let value = native!((), 0x658500AE6D723A7E, native_parameters!(alpha));

    value
}

pub fn _reset_ghosted_entity_alpha() -> () {
    let value = native!((), 0x17330EBF2F2124A8, native_parameters!());

    value
}

pub fn _network_set_entity_ghosted_with_owner(entity: i32, p1: bool) -> () {
    let value = native!((), 0x4BA166079D658ED4, native_parameters!(entity, p1));

    value
}

pub fn _0xd7b6c73cad419bcf(p0: bool) -> () {
    let value = native!((), 0xD7B6C73CAD419BCF, native_parameters!(p0));

    value
}

pub fn _0x7ef7649b64d7ff10(entity: i32) -> bool {
    let value = native!(bool, 0x7EF7649B64D7FF10, native_parameters!(entity));

    value
}

pub fn use_player_colour_instead_of_team_colour(toggle: bool) -> () {
    let value = native!((), 0x77758139EC9B66C7, native_parameters!(toggle));

    value
}

pub fn network_create_synchronised_scene(
    x: f32,
    y: f32,
    z: f32,
    xRot: f32,
    yRot: f32,
    zRot: f32,
    rotationOrder: i32,
    useOcclusionPortal: bool,
    looped: bool,
    p9: f32,
    animTime: f32,
    p11: f32,
) -> i32 {
    let value = native!(
        i32,
        0x7CD6BC4C2BBDD526,
        native_parameters!(
            x,
            y,
            z,
            xRot,
            yRot,
            zRot,
            rotationOrder,
            useOcclusionPortal,
            looped,
            p9,
            animTime,
            p11
        )
    );

    value
}

pub fn network_add_ped_to_synchronised_scene(
    ped: i32,
    netScene: i32,
    animDict: &std::ffi::CString,
    animnName: &std::ffi::CString,
    speed: f32,
    speedMultiplier: f32,
    duration: i32,
    flag: i32,
    playbackRate: f32,
    p9: u32,
) -> () {
    let value = native!(
        (),
        0x742A637471BCECD9,
        native_parameters!(
            ped,
            netScene,
            animDict.as_ptr(),
            animnName.as_ptr(),
            speed,
            speedMultiplier,
            duration,
            flag,
            playbackRate,
            p9
        )
    );

    value
}

pub fn _0xa5eafe473e45c442(
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
        0xA5EAFE473E45C442,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn network_add_entity_to_synchronised_scene(
    entity: i32,
    netScene: i32,
    animDict: &std::ffi::CString,
    animName: &std::ffi::CString,
    speed: f32,
    speedMulitiplier: f32,
    flag: i32,
) -> () {
    let value = native!(
        (),
        0xF2404D68CBC855FA,
        native_parameters!(
            entity,
            netScene,
            animDict.as_ptr(),
            animName.as_ptr(),
            speed,
            speedMulitiplier,
            flag
        )
    );

    value
}

pub fn _0x45f35c0edc33b03b(
    netScene: i32,
    modelHash: u32,
    x: f32,
    y: f32,
    z: f32,
    p5: f32,
    p6: &std::ffi::CString,
    p7: f32,
    p8: f32,
    flags: i32,
) -> () {
    let value = native!(
        (),
        0x45F35C0EDC33B03B,
        native_parameters!(netScene, modelHash, x, y, z, p5, p6.as_ptr(), p7, p8, flags)
    );

    value
}

pub fn _network_force_local_use_of_synced_scene_camera(
    netScene: i32,
    animDict: &std::ffi::CString,
    animName: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0xCF8BD3B0BD6D42D7,
        native_parameters!(netScene, animDict.as_ptr(), animName.as_ptr())
    );

    value
}

pub fn network_attach_synchronised_scene_to_entity(netScene: i32, entity: i32, bone: i32) -> () {
    let value = native!(
        (),
        0x478DCBD2A98B705A,
        native_parameters!(netScene, entity, bone)
    );

    value
}

pub fn network_start_synchronised_scene(netScene: i32) -> () {
    let value = native!((), 0x9A1B3FCDB36C8697, native_parameters!(netScene));

    value
}

pub fn network_stop_synchronised_scene(netScene: i32) -> () {
    let value = native!((), 0xC254481A4574CB2F, native_parameters!(netScene));

    value
}

pub fn _network_convert_synchronised_scene_to_synchronized_scene(netScene: i32) -> i32 {
    let value = native!(i32, 0x02C40BF885C567B6, native_parameters!(netScene));

    value
}

pub fn _0xc9b43a33d09cada7(p0: u32) -> () {
    let value = native!((), 0xC9B43A33D09CADA7, native_parameters!(p0));

    value
}

pub fn _0x144da052257ae7d8(p0: u32) -> () {
    let value = native!((), 0x144DA052257AE7D8, native_parameters!(p0));

    value
}

pub fn _0xfb1f9381e80fa13f(p0: i32, p1: u32) -> u32 {
    let value = native!(u32, 0xFB1F9381E80FA13F, native_parameters!(p0, p1));

    value
}

pub fn network_start_respawn_search_for_player(
    player: i32,
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    p5: f32,
    p6: f32,
    p7: f32,
    flags: i32,
) -> bool {
    let value = native!(
        bool,
        0x5A6FFA2433E2F14C,
        native_parameters!(player, x, y, z, radius, p5, p6, p7, flags)
    );

    value
}

pub fn network_start_respawn_search_in_angled_area_for_player(
    player: i32,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    width: f32,
    p8: f32,
    p9: f32,
    p10: f32,
    flags: i32,
) -> bool {
    let value = native!(
        bool,
        0x4BA92A18502BCA61,
        native_parameters!(player, x1, y1, z1, x2, y2, z2, width, p8, p9, p10, flags)
    );

    value
}

pub fn network_query_respawn_results(p0: *mut u32) -> u32 {
    let value = native!(u32, 0x3C891A251567DFCE, native_parameters!(p0));

    value
}

pub fn network_cancel_respawn_search() -> () {
    let value = native!((), 0xFB8F2A6F3DF08CBE, native_parameters!());

    value
}

pub fn network_get_respawn_result(
    randomInt: i32,
    coordinates: *mut NativeVector3,
    heading: *mut f32,
) -> () {
    let value = native!(
        (),
        0x371EA43692861CF1,
        native_parameters!(randomInt, coordinates, heading)
    );

    value
}

pub fn network_get_respawn_result_flags(p0: u32) -> u32 {
    let value = native!(u32, 0x6C34F1208B8923FD, native_parameters!(p0));

    value
}

pub fn network_start_solo_tutorial_session() -> () {
    let value = native!((), 0x17E0198B3882C2CB, native_parameters!());

    value
}

pub fn _0xfb680d403909dc70(p0: u32, p1: u32) -> () {
    let value = native!((), 0xFB680D403909DC70, native_parameters!(p0, p1));

    value
}

pub fn network_end_tutorial_session() -> () {
    let value = native!((), 0xD0AFAFF5A51D72F7, native_parameters!());

    value
}

pub fn network_is_in_tutorial_session() -> bool {
    let value = native!(bool, 0xADA24309FE08DACF, native_parameters!());

    value
}

pub fn _0xb37e4e6a2388ca7b() -> bool {
    let value = native!(bool, 0xB37E4E6A2388CA7B, native_parameters!());

    value
}

pub fn network_is_tutorial_session_change_pending() -> bool {
    let value = native!(bool, 0x35F0B98A8387274D, native_parameters!());

    value
}

pub fn network_get_player_tutorial_session_instance(player: i32) -> i32 {
    let value = native!(i32, 0x3B39236746714134, native_parameters!(player));

    value
}

pub fn _network_is_player_equal_to_index(player: i32, index: i32) -> bool {
    let value = native!(bool, 0x9DE986FC9A87C474, native_parameters!(player, index));

    value
}

pub fn network_conceal_player(player: i32, toggle: bool, p2: bool) -> () {
    let value = native!(
        (),
        0xBBDF066252829606,
        native_parameters!(player, toggle, p2)
    );

    value
}

pub fn network_is_player_concealed(player: i32) -> bool {
    let value = native!(bool, 0x919B3C98ED8292F9, native_parameters!(player));

    value
}

pub fn _network_conceal_entity(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x1632BE0AC1E62876, native_parameters!(entity, toggle));

    value
}

pub fn _network_is_entity_concealed(entity: i32) -> bool {
    let value = native!(bool, 0x71302EC70689052A, native_parameters!(entity));

    value
}

pub fn network_override_clock_time(hours: i32, minutes: i32, seconds: i32) -> () {
    let value = native!(
        (),
        0xE679E3E06E363892,
        native_parameters!(hours, minutes, seconds)
    );

    value
}

pub fn _network_override_clock_milliseconds_per_game_minute(ms: i32) -> () {
    let value = native!((), 0x42BF1D2E723B6D7E, native_parameters!(ms));

    value
}

pub fn network_clear_clock_time_override() -> () {
    let value = native!((), 0xD972DF67326F966E, native_parameters!());

    value
}

pub fn network_is_clock_time_overridden() -> bool {
    let value = native!(bool, 0xD7C95D322FF57522, native_parameters!());

    value
}

pub fn network_add_entity_area(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32) -> u32 {
    let value = native!(
        u32,
        0x494C8FB299290269,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn network_add_entity_angled_area(
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
        0x376C6375BA60293A,
        native_parameters!(x1, y1, z1, x2, y2, z2, width)
    );

    value
}

pub fn network_add_entity_displayed_boundaries(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
) -> u32 {
    let value = native!(
        u32,
        0x25B99872D588A101,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _0x2b1c623823db0d9d(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32, p6: u32) -> u32 {
    let value = native!(
        u32,
        0x2B1C623823DB0D9D,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn network_remove_entity_area(p0: u32) -> bool {
    let value = native!(bool, 0x93CF869BAA0C4874, native_parameters!(p0));

    value
}

pub fn network_entity_area_does_exist(areaHandle: i32) -> bool {
    let value = native!(bool, 0xE64A3CA08DFA37A9, native_parameters!(areaHandle));

    value
}

pub fn _0x4df7cfff471a7fb1(p0: u32) -> bool {
    let value = native!(bool, 0x4DF7CFFF471A7FB1, native_parameters!(p0));

    value
}

pub fn network_entity_area_is_occupied(areaHandle: i32) -> bool {
    let value = native!(bool, 0x4A2D4E8BF4265B0F, native_parameters!(areaHandle));

    value
}

pub fn _network_set_network_id_dynamic(netID: i32, toggle: bool) -> () {
    let value = native!((), 0x2B1813ABA29016C5, native_parameters!(netID, toggle));

    value
}

pub fn _0xa6fceccf4721d679(p0: u32) -> () {
    let value = native!((), 0xA6FCECCF4721D679, native_parameters!(p0));

    value
}

pub fn _0x95baf97c82464629(p0: u32, p1: u32) -> () {
    let value = native!((), 0x95BAF97C82464629, native_parameters!(p0, p1));

    value
}

pub fn network_request_cloud_background_scripts() -> bool {
    let value = native!(bool, 0x924426BFFD82E915, native_parameters!());

    value
}

pub fn network_is_cloud_background_script_request_pending() -> bool {
    let value = native!(bool, 0x8132C0EB8B2B3293, native_parameters!());

    value
}

pub fn network_request_cloud_tunables() -> () {
    let value = native!((), 0x42FB3B532D526E6C, native_parameters!());

    value
}

pub fn network_is_tunable_cloud_request_pending() -> bool {
    let value = native!(bool, 0x0467C11ED88B7D28, native_parameters!());

    value
}

pub fn network_get_tunable_cloud_crc() -> i32 {
    let value = native!(i32, 0x10BD227A753B0D84, native_parameters!());

    value
}

pub fn network_does_tunable_exist(
    tunableContext: &std::ffi::CString,
    tunableName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x85E5F8B9B898B20A,
        native_parameters!(tunableContext.as_ptr(), tunableName.as_ptr())
    );

    value
}

pub fn network_access_tunable_int(
    tunableContext: &std::ffi::CString,
    tunableName: &std::ffi::CString,
    value: *mut i32,
) -> bool {
    let value = native!(
        bool,
        0x8BE1146DFD5D4468,
        native_parameters!(tunableContext.as_ptr(), tunableName.as_ptr(), value)
    );

    value
}

pub fn network_access_tunable_float(
    tunableContext: &std::ffi::CString,
    tunableName: &std::ffi::CString,
    value: *mut f32,
) -> bool {
    let value = native!(
        bool,
        0xE5608CA7BC163A5F,
        native_parameters!(tunableContext.as_ptr(), tunableName.as_ptr(), value)
    );

    value
}

pub fn network_access_tunable_bool(
    tunableContext: &std::ffi::CString,
    tunableName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0xAA6A47A573ABB75A,
        native_parameters!(tunableContext.as_ptr(), tunableName.as_ptr())
    );

    value
}

pub fn network_does_tunable_exist_hash(tunableContext: u32, tunableName: u32) -> bool {
    let value = native!(
        bool,
        0xE4E53E1419D81127,
        native_parameters!(tunableContext, tunableName)
    );

    value
}

pub fn _network_allocate_tunables_registration_data_map() -> bool {
    let value = native!(bool, 0xFAFC23AEE23868DB, native_parameters!());

    value
}

pub fn network_access_tunable_int_hash(
    tunableContext: u32,
    tunableName: u32,
    value: *mut i32,
) -> bool {
    let value = native!(
        bool,
        0x40FCE03E50E8DBE8,
        native_parameters!(tunableContext, tunableName, value)
    );

    value
}

pub fn _network_register_tunable_int_hash(
    contextHash: u32,
    nameHash: u32,
    value: *mut i32,
) -> bool {
    let value = native!(
        bool,
        0x3A8B55FDA4C8DDEF,
        native_parameters!(contextHash, nameHash, value)
    );

    value
}

pub fn network_access_tunable_float_hash(
    tunableContext: u32,
    tunableName: u32,
    value: *mut f32,
) -> bool {
    let value = native!(
        bool,
        0x972BC203BBC4C4D5,
        native_parameters!(tunableContext, tunableName, value)
    );

    value
}

pub fn _network_register_tunable_float_hash(
    contextHash: u32,
    nameHash: u32,
    value: *mut f32,
) -> bool {
    let value = native!(
        bool,
        0x1950DAE9848A4739,
        native_parameters!(contextHash, nameHash, value)
    );

    value
}

pub fn network_access_tunable_bool_hash(tunableContext: u32, tunableName: u32) -> bool {
    let value = native!(
        bool,
        0xEA16B69D93D71A45,
        native_parameters!(tunableContext, tunableName)
    );

    value
}

pub fn _network_register_tunable_bool_hash(
    contextHash: u32,
    nameHash: u32,
    value: *mut bool,
) -> bool {
    let value = native!(
        bool,
        0x697F508861875B42,
        native_parameters!(contextHash, nameHash, value)
    );

    value
}

pub fn network_try_access_tunable_bool_hash(
    tunableContext: u32,
    tunableName: u32,
    defaultValue: bool,
) -> bool {
    let value = native!(
        bool,
        0xC7420099936CE286,
        native_parameters!(tunableContext, tunableName, defaultValue)
    );

    value
}

pub fn network_get_content_modifier_list_id(contentHash: u32) -> i32 {
    let value = native!(i32, 0x187382F8A3E0A6C3, native_parameters!(contentHash));

    value
}

pub fn _0x7db53b37a2f211a0() -> i32 {
    let value = native!(i32, 0x7DB53B37A2F211A0, native_parameters!());

    value
}

pub fn network_reset_body_tracker() -> () {
    let value = native!((), 0x72433699B4E6DD64, native_parameters!());

    value
}

pub fn _network_get_num_body_trackers() -> i32 {
    let value = native!(i32, 0xD38C4A6D047C019D, native_parameters!());

    value
}

pub fn _0x2e0bf682cc778d49(p0: u32) -> bool {
    let value = native!(bool, 0x2E0BF682CC778D49, native_parameters!(p0));

    value
}

pub fn _0x0ede326d47cd0f3e(ped: i32, player: i32) -> bool {
    let value = native!(bool, 0x0EDE326D47CD0F3E, native_parameters!(ped, player));

    value
}

pub fn _network_set_vehicle_wheels_destructible(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x890E2C5ABED7236D, native_parameters!(entity, toggle));

    value
}

pub fn _0x38b7c51ab1edc7d8(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x38B7C51AB1EDC7D8, native_parameters!(entity, toggle));

    value
}

pub fn _0x3fc795691834481d(p0: u32, p1: u32) -> () {
    let value = native!((), 0x3FC795691834481D, native_parameters!(p0, p1));

    value
}

pub fn network_explode_vehicle(vehicle: i32, isAudible: bool, isInvisible: bool, p3: bool) -> () {
    let value = native!(
        (),
        0x301A42153C9AD707,
        native_parameters!(vehicle, isAudible, isInvisible, p3)
    );

    value
}

pub fn _0x2a5e0621dd815a9a(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x2A5E0621DD815A9A, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xcd71a4ecab22709e(entity: i32) -> () {
    let value = native!((), 0xCD71A4ECAB22709E, native_parameters!(entity));

    value
}

pub fn network_override_coords_and_heading(
    entity: i32,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
) -> () {
    let value = native!(
        (),
        0xA7E30DE9272B6D49,
        native_parameters!(entity, x, y, z, heading)
    );

    value
}

pub fn _0xe6717e652b8c8d8a(p0: u32, p1: u32) -> () {
    let value = native!((), 0xE6717E652B8C8D8A, native_parameters!(p0, p1));

    value
}

pub fn network_disable_proximity_migration(netID: i32) -> () {
    let value = native!((), 0x407091CF6037118E, native_parameters!(netID));

    value
}

pub fn network_set_property_id(id: i32) -> () {
    let value = native!((), 0x1775961C2FBBCB5C, native_parameters!(id));

    value
}

pub fn network_clear_property_id() -> () {
    let value = native!((), 0xC2B82527CA77053E, native_parameters!());

    value
}

pub fn _0x367ef5e2f439b4c6(p0: i32) -> () {
    let value = native!((), 0x367EF5E2F439B4C6, native_parameters!(p0));

    value
}

pub fn _0x94538037ee44f5cf(p0: bool) -> () {
    let value = native!((), 0x94538037EE44F5CF, native_parameters!(p0));

    value
}

pub fn network_cache_local_player_head_blend_data() -> () {
    let value = native!((), 0xBD0BE0BFC927EAC1, native_parameters!());

    value
}

pub fn network_has_cached_player_head_blend_data(player: i32) -> bool {
    let value = native!(bool, 0x237D5336A9A54108, native_parameters!(player));

    value
}

pub fn network_apply_cached_player_head_blend_data(ped: i32, player: i32) -> bool {
    let value = native!(bool, 0x99B72C7ABDE5C910, native_parameters!(ped, player));

    value
}

pub fn get_num_commerce_items() -> i32 {
    let value = native!(i32, 0xF2EAC213D5EA0623, native_parameters!());

    value
}

pub fn is_commerce_data_valid() -> bool {
    let value = native!(bool, 0xEA14EEF5B7CD2C30, native_parameters!());

    value
}

pub fn _0xb606e6cc59664972(p0: u32) -> () {
    let value = native!((), 0xB606E6CC59664972, native_parameters!(p0));

    value
}

pub fn _0x1d4dc17c38feaff0() -> bool {
    let value = native!(bool, 0x1D4DC17C38FEAFF0, native_parameters!());

    value
}

pub fn get_commerce_item_id(index: i32) -> String {
    let value = native!(*const i8, 0x662635855957C411, native_parameters!(index));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_commerce_item_name(index: i32) -> String {
    let value = native!(*const i8, 0xB4271092CA7EDF48, native_parameters!(index));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_commerce_product_price(index: i32) -> String {
    let value = native!(*const i8, 0xCA94551B50B4932C, native_parameters!(index));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_commerce_item_num_cats(index: i32) -> i32 {
    let value = native!(i32, 0x2A7776C709904AB0, native_parameters!(index));

    value
}

pub fn get_commerce_item_cat(index: i32, index2: i32) -> String {
    let value = native!(
        *const i8,
        0x6F44CBF56D79FAC0,
        native_parameters!(index, index2)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn open_commerce_store(p0: &std::ffi::CString, p1: &std::ffi::CString, p2: i32) -> () {
    let value = native!(
        (),
        0x58C21165F6545892,
        native_parameters!(p0.as_ptr(), p1.as_ptr(), p2)
    );

    value
}

pub fn is_commerce_store_open() -> bool {
    let value = native!(bool, 0x2EAC52B4019E2782, native_parameters!());

    value
}

pub fn set_store_enabled(toggle: bool) -> () {
    let value = native!((), 0x9641A9FF718E9C5E, native_parameters!(toggle));

    value
}

pub fn request_commerce_item_image(index: i32) -> bool {
    let value = native!(bool, 0xA2F952104FC6DD4B, native_parameters!(index));

    value
}

pub fn release_all_commerce_item_images() -> () {
    let value = native!((), 0x72D0706CD6CCDB58, native_parameters!());

    value
}

pub fn get_commerce_item_texturename(index: i32) -> String {
    let value = native!(*const i8, 0x722F5D28B61C5EA8, native_parameters!(index));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn is_store_available_to_user() -> bool {
    let value = native!(bool, 0x883D79C4071E18B3, native_parameters!());

    value
}

pub fn _0x265635150fb0d82e() -> () {
    let value = native!((), 0x265635150FB0D82E, native_parameters!());

    value
}

pub fn _0x444c4525ece0a4b9() -> () {
    let value = native!((), 0x444C4525ECE0A4B9, native_parameters!());

    value
}

pub fn _0x59328eb08c5ceb2b() -> bool {
    let value = native!(bool, 0x59328EB08C5CEB2B, native_parameters!());

    value
}

pub fn _0xfae628f1e9adb239(p0: u32, p1: i32, p2: u32) -> () {
    let value = native!((), 0xFAE628F1E9ADB239, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x754615490a029508() -> i32 {
    let value = native!(i32, 0x754615490A029508, native_parameters!());

    value
}

pub fn _0x155467aca0f55705() -> i32 {
    let value = native!(i32, 0x155467ACA0F55705, native_parameters!());

    value
}

pub fn cloud_delete_member_file(p0: &std::ffi::CString) -> i32 {
    let value = native!(i32, 0xC64DED7EF0D2FE37, native_parameters!(p0.as_ptr()));

    value
}

pub fn cloud_has_request_completed(handle: i32) -> bool {
    let value = native!(bool, 0x4C61B39930D045DA, native_parameters!(handle));

    value
}

pub fn cloud_did_request_succeed(handle: i32) -> bool {
    let value = native!(bool, 0x3A3D5568AF297CD5, native_parameters!(handle));

    value
}

pub fn cloud_check_availability() -> () {
    let value = native!((), 0x4F18196C8D38768D, native_parameters!());

    value
}

pub fn cloud_is_checking_availability() -> bool {
    let value = native!(bool, 0xC7ABAC5DE675EE3B, native_parameters!());

    value
}

pub fn cloud_get_availability_check_result() -> bool {
    let value = native!(bool, 0x0B0CC10720653F3B, native_parameters!());

    value
}

pub fn _0x8b0c2964ba471961() -> u32 {
    let value = native!(u32, 0x8B0C2964BA471961, native_parameters!());

    value
}

pub fn _0x88b588b41ff7868e() -> u32 {
    let value = native!(u32, 0x88B588B41FF7868E, native_parameters!());

    value
}

pub fn _0x67fc09bc554a75e5() -> u32 {
    let value = native!(u32, 0x67FC09BC554A75E5, native_parameters!());

    value
}

pub fn _clear_launch_params() -> () {
    let value = native!((), 0x966DD84FB6A46017, native_parameters!());

    value
}

pub fn ugc_copy_content(p0: *mut u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0x152D90E4C1B4738A, native_parameters!(p0, p1));

    value
}

pub fn _0x9fedf86898f100e9() -> u32 {
    let value = native!(u32, 0x9FEDF86898F100E9, native_parameters!());

    value
}

pub fn ugc_has_create_finished() -> bool {
    let value = native!(bool, 0x5E24341A7F92A74B, native_parameters!());

    value
}

pub fn _0x24e4e51fc16305f9() -> u32 {
    let value = native!(u32, 0x24E4E51FC16305F9, native_parameters!());

    value
}

pub fn ugc_get_create_result() -> u32 {
    let value = native!(u32, 0xFBC5E768C7A77A6A, native_parameters!());

    value
}

pub fn ugc_get_create_content_id() -> u32 {
    let value = native!(u32, 0xC55A0B40FFB1ED23, native_parameters!());

    value
}

pub fn ugc_clear_create_result() -> () {
    let value = native!((), 0x17440AA15D1D3739, native_parameters!());

    value
}

pub fn ugc_query_my_content(p0: u32, p1: u32, p2: *mut u32, p3: u32, p4: u32, p5: u32) -> bool {
    let value = native!(
        bool,
        0x9BF438815F5D96EA,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _0x692d58df40657e8c(p0: u32, p1: u32, p2: u32, p3: *mut u32, p4: u32, p5: bool) -> bool {
    let value = native!(
        bool,
        0x692D58DF40657E8C,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn ugc_query_by_content_id(
    contentId: &std::ffi::CString,
    latestVersion: bool,
    contentTypeName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x158EC424F35EC469,
        native_parameters!(contentId.as_ptr(), latestVersion, contentTypeName.as_ptr())
    );

    value
}

pub fn ugc_query_by_content_ids(
    data: *mut u32,
    count: i32,
    latestVersion: bool,
    contentTypeName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0xC7397A83F7A2A462,
        native_parameters!(data, count, latestVersion, contentTypeName.as_ptr())
    );

    value
}

pub fn _ugc_query_recently_created_content(
    offset: i32,
    count: i32,
    contentTypeName: &std::ffi::CString,
    p3: i32,
) -> bool {
    let value = native!(
        bool,
        0x6D4CB481FAC835E8,
        native_parameters!(offset, count, contentTypeName.as_ptr(), p3)
    );

    value
}

pub fn ugc_get_bookmarked_content(p0: u32, p1: u32, p2: *mut u32, p3: *mut u32) -> bool {
    let value = native!(bool, 0xD5A4B59980401588, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn ugc_get_my_content(p0: u32, p1: u32, p2: *mut u32, p3: *mut u32) -> bool {
    let value = native!(bool, 0x3195F8DD0D531052, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn ugc_get_friend_content(p0: u32, p1: u32, p2: *mut u32, p3: *mut u32) -> bool {
    let value = native!(bool, 0xF9E1CCAE8BA4C281, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn ugc_get_crew_content(p0: u32, p1: u32, p2: u32, p3: *mut u32, p4: *mut u32) -> bool {
    let value = native!(
        bool,
        0x9F6E2821885CAEE2,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn ugc_get_get_by_category(p0: u32, p1: u32, p2: u32, p3: *mut u32, p4: *mut u32) -> bool {
    let value = native!(
        bool,
        0x678BB03C1A3BD51E,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn set_balance_add_machine(
    contentId: &std::ffi::CString,
    contentTypeName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x815E5E3073DA1D67,
        native_parameters!(contentId.as_ptr(), contentTypeName.as_ptr())
    );

    value
}

pub fn set_balance_add_machines(
    data: *mut u32,
    dataCount: i32,
    contentTypeName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0xB8322EEB38BE7C26,
        native_parameters!(data, dataCount, contentTypeName.as_ptr())
    );

    value
}

pub fn _0xa7862bc5ed1dfd7e(p0: u32, p1: u32, p2: *mut u32, p3: *mut u32) -> bool {
    let value = native!(bool, 0xA7862BC5ED1DFD7E, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x97a770beef227e2b(p0: u32, p1: u32, p2: *mut u32, p3: *mut u32) -> bool {
    let value = native!(bool, 0x97A770BEEF227E2B, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x5324a0e3e4ce3570(p0: u32, p1: u32, p2: *mut u32, p3: *mut u32) -> bool {
    let value = native!(bool, 0x5324A0E3E4CE3570, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn ugc_cancel_query() -> () {
    let value = native!((), 0xE9B99B6853181409, native_parameters!());

    value
}

pub fn ugc_is_getting() -> bool {
    let value = native!(bool, 0xD53ACDBEF24A46E8, native_parameters!());

    value
}

pub fn ugc_has_get_finished() -> bool {
    let value = native!(bool, 0x02ADA21EA2F6918F, native_parameters!());

    value
}

pub fn ugc_did_get_succeed() -> u32 {
    let value = native!(u32, 0x941E5306BCD7C2C7, native_parameters!());

    value
}

pub fn _0xc87e740d9f3872cc() -> u32 {
    let value = native!(u32, 0xC87E740D9F3872CC, native_parameters!());

    value
}

pub fn ugc_get_query_result() -> u32 {
    let value = native!(u32, 0xEDF7F927136C224B, native_parameters!());

    value
}

pub fn ugc_get_content_num() -> u32 {
    let value = native!(u32, 0xE0A6138401BCB837, native_parameters!());

    value
}

pub fn ugc_get_content_total() -> u32 {
    let value = native!(u32, 0x769951E2455E2EB5, native_parameters!());

    value
}

pub fn ugc_get_content_hash() -> u32 {
    let value = native!(u32, 0x3A17A27D75C74887, native_parameters!());

    value
}

pub fn ugc_clear_query_results() -> () {
    let value = native!((), 0xBA96394A0EECFA65, native_parameters!());

    value
}

pub fn ugc_get_content_user_id(p0: i32) -> String {
    let value = native!(*const i8, 0xCD67AD041A394C9C, native_parameters!(p0));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _0x584770794d758c18(p0: u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0x584770794D758C18, native_parameters!(p0, p1));

    value
}

pub fn _0x8c8d2739ba44af0f(p0: u32) -> bool {
    let value = native!(bool, 0x8C8D2739BA44AF0F, native_parameters!(p0));

    value
}

pub fn ugc_get_content_user_name(p0: u32) -> u32 {
    let value = native!(u32, 0x703F12425ECA8BF5, native_parameters!(p0));

    value
}

pub fn _0xaeab987727c5a8a4(p0: u32) -> bool {
    let value = native!(bool, 0xAEAB987727C5A8A4, native_parameters!(p0));

    value
}

pub fn ugc_get_content_category(p0: i32) -> i32 {
    let value = native!(i32, 0xA7BAB11E7C9C6C5A, native_parameters!(p0));

    value
}

pub fn ugc_get_content_id(p0: i32) -> String {
    let value = native!(*const i8, 0x55AA95F481D694D2, native_parameters!(p0));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn ugc_get_root_content_id(p0: i32) -> String {
    let value = native!(*const i8, 0xC0173D6BFF4E0348, native_parameters!(p0));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn ugc_get_content_name(p0: u32) -> u32 {
    let value = native!(u32, 0xBF09786A7FCAB582, native_parameters!(p0));

    value
}

pub fn ugc_get_content_description_hash(p0: u32) -> i32 {
    let value = native!(i32, 0x7CF0448787B23758, native_parameters!(p0));

    value
}

pub fn ugc_get_content_path(p0: i32, p1: i32) -> String {
    let value = native!(*const i8, 0xBAF6BABF9E7CCC13, native_parameters!(p0, p1));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn ugc_get_content_updated_date(p0: u32, p1: *mut u32) -> () {
    let value = native!((), 0xCFD115B373C0DF63, native_parameters!(p0, p1));

    value
}

pub fn ugc_get_content_file_version(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x37025B27D9B658B1, native_parameters!(p0, p1));

    value
}

pub fn _0x1d610eb0fea716d9(p0: i32) -> bool {
    let value = native!(bool, 0x1D610EB0FEA716D9, native_parameters!(p0));

    value
}

pub fn _0x7fcc39c46c3c03bd(p0: i32) -> bool {
    let value = native!(bool, 0x7FCC39C46C3C03BD, native_parameters!(p0));

    value
}

pub fn ugc_get_content_language(p0: u32) -> u32 {
    let value = native!(u32, 0x32DD916F3F7C9672, native_parameters!(p0));

    value
}

pub fn ugc_get_content_is_published(p0: u32) -> bool {
    let value = native!(bool, 0x3054F114121C21EA, native_parameters!(p0));

    value
}

pub fn ugc_get_content_is_verified(p0: u32) -> bool {
    let value = native!(bool, 0xA9240A96C74CCA13, native_parameters!(p0));

    value
}

pub fn ugc_get_content_rating(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x1ACCFBA3D8DAB2EE, native_parameters!(p0, p1));

    value
}

pub fn ugc_get_content_rating_count(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x759299C5BB31D2A9, native_parameters!(p0, p1));

    value
}

pub fn ugc_get_content_rating_positive_count(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x87E5C46C187FE0AE, native_parameters!(p0, p1));

    value
}

pub fn ugc_get_content_rating_negative_count(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x4E548C0D7AE39FF9, native_parameters!(p0, p1));

    value
}

pub fn ugc_get_content_has_player_record(p0: u32) -> bool {
    let value = native!(bool, 0x70EA8DA57840F9BE, native_parameters!(p0));

    value
}

pub fn ugc_get_content_has_player_bookmarked(p0: u32) -> bool {
    let value = native!(bool, 0x993CBE59D350D225, native_parameters!(p0));

    value
}

pub fn ugc_request_content_data_from_index(p0: i32, p1: i32) -> i32 {
    let value = native!(i32, 0x171DF6A0C07FB3DC, native_parameters!(p0, p1));

    value
}

pub fn ugc_request_content_data_from_params(
    contentTypeName: &std::ffi::CString,
    contentId: &std::ffi::CString,
    p2: i32,
    p3: i32,
    p4: i32,
) -> i32 {
    let value = native!(
        i32,
        0x7FD2990AF016795E,
        native_parameters!(contentTypeName.as_ptr(), contentId.as_ptr(), p2, p3, p4)
    );

    value
}

pub fn ugc_request_cached_description(p0: i32) -> i32 {
    let value = native!(i32, 0x5E0165278F6339EE, native_parameters!(p0));

    value
}

pub fn _0x2d5dc831176d0114(p0: u32) -> bool {
    let value = native!(bool, 0x2D5DC831176D0114, native_parameters!(p0));

    value
}

pub fn _0xebfa8d50addc54c4(p0: u32) -> bool {
    let value = native!(bool, 0xEBFA8D50ADDC54C4, native_parameters!(p0));

    value
}

pub fn _0x162c23ca83ed0a62(p0: u32) -> bool {
    let value = native!(bool, 0x162C23CA83ED0A62, native_parameters!(p0));

    value
}

pub fn ugc_get_cached_description(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x40F7E66472DF3E5C, native_parameters!(p0, p1));

    value
}

pub fn _0x5a34cd9c3c5bec44(p0: u32) -> bool {
    let value = native!(bool, 0x5A34CD9C3C5BEC44, native_parameters!(p0));

    value
}

pub fn _0x68103e2247887242() -> () {
    let value = native!((), 0x68103E2247887242, native_parameters!());

    value
}

pub fn ugc_publish(
    contentId: &std::ffi::CString,
    baseContentId: &std::ffi::CString,
    contentTypeName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x1DE0F5F50D723CAA,
        native_parameters!(
            contentId.as_ptr(),
            baseContentId.as_ptr(),
            contentTypeName.as_ptr()
        )
    );

    value
}

pub fn ugc_set_bookmarked(
    contentId: &std::ffi::CString,
    bookmarked: bool,
    contentTypeName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x274A1519DFC1094F,
        native_parameters!(contentId.as_ptr(), bookmarked, contentTypeName.as_ptr())
    );

    value
}

pub fn ugc_set_deleted(p0: *mut u32, p1: bool, p2: *mut u32) -> bool {
    let value = native!(bool, 0xD05D1A6C74DA3498, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x45e816772e93a9db() -> u32 {
    let value = native!(u32, 0x45E816772E93A9DB, native_parameters!());

    value
}

pub fn ugc_has_modify_finished() -> bool {
    let value = native!(bool, 0x299EF3C576773506, native_parameters!());

    value
}

pub fn _0x793ff272d5b365f4() -> u32 {
    let value = native!(u32, 0x793FF272D5B365F4, native_parameters!());

    value
}

pub fn ugc_get_modify_result() -> u32 {
    let value = native!(u32, 0x5A0A3D1A186A5508, native_parameters!());

    value
}

pub fn ugc_clear_modify_result() -> () {
    let value = native!((), 0xA1E5E0204A6FCC70, native_parameters!());

    value
}

pub fn _0xb746d20b17f2a229(p0: *mut u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0xB746D20B17F2A229, native_parameters!(p0, p1));

    value
}

pub fn _0x63b406d7884bfa95() -> u32 {
    let value = native!(u32, 0x63B406D7884BFA95, native_parameters!());

    value
}

pub fn _0x4d02279c83be69fe() -> u32 {
    let value = native!(u32, 0x4D02279C83BE69FE, native_parameters!());

    value
}

pub fn ugc_get_creator_num() -> u32 {
    let value = native!(u32, 0x597F8DBA9B206FC7, native_parameters!());

    value
}

pub fn ugc_policies_make_private(p0: u32) -> bool {
    let value = native!(bool, 0x5CAE833B0EE0C500, native_parameters!(p0));

    value
}

pub fn ugc_clear_offline_query() -> () {
    let value = native!((), 0x61A885D3F7CFEE9A, native_parameters!());

    value
}

pub fn ugc_set_query_data_from_offline(p0: bool) -> () {
    let value = native!((), 0xF98DDE0A8ED09323, native_parameters!(p0));

    value
}

pub fn _0xfd75dabc0957bf33(p0: bool) -> () {
    let value = native!((), 0xFD75DABC0957BF33, native_parameters!(p0));

    value
}

pub fn ugc_is_language_supported(p0: u32) -> bool {
    let value = native!(bool, 0xF53E48461B71EECB, native_parameters!(p0));

    value
}

pub fn _facebook_set_heist_complete(
    heistName: &std::ffi::CString,
    cashEarned: i32,
    xpEarned: i32,
) -> bool {
    let value = native!(
        bool,
        0x098AB65B9ED9A9EC,
        native_parameters!(heistName.as_ptr(), cashEarned, xpEarned)
    );

    value
}

pub fn _facebook_set_create_character_complete() -> bool {
    let value = native!(bool, 0xDC48473142545431, native_parameters!());

    value
}

pub fn _facebook_set_milestone_complete(milestoneId: i32) -> bool {
    let value = native!(bool, 0x0AE1F1653B554AB9, native_parameters!(milestoneId));

    value
}

pub fn _facebook_is_sending_data() -> bool {
    let value = native!(bool, 0x62B9FEC9A11F10EF, native_parameters!());

    value
}

pub fn _facebook_do_unk_check() -> bool {
    let value = native!(bool, 0xA75E2B6733DA5142, native_parameters!());

    value
}

pub fn _facebook_is_available() -> bool {
    let value = native!(bool, 0x43865688AE10F0D7, native_parameters!());

    value
}

pub fn texture_download_request(
    PlayerHandle: *mut i32,
    FilePath: &std::ffi::CString,
    Name: &std::ffi::CString,
    p3: bool,
) -> i32 {
    let value = native!(
        i32,
        0x16160DA74A8E74A2,
        native_parameters!(PlayerHandle, FilePath.as_ptr(), Name.as_ptr(), p3)
    );

    value
}

pub fn title_texture_download_request(
    FilePath: &std::ffi::CString,
    Name: &std::ffi::CString,
    p2: bool,
) -> i32 {
    let value = native!(
        i32,
        0x0B203B4AFDE53A4F,
        native_parameters!(FilePath.as_ptr(), Name.as_ptr(), p2)
    );

    value
}

pub fn ugc_texture_download_request(
    p0: *mut u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: *mut u32,
    p5: bool,
) -> u32 {
    let value = native!(
        u32,
        0x308F96458B7087CC,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn texture_download_release(p0: i32) -> () {
    let value = native!((), 0x487EB90B98E9FB19, native_parameters!(p0));

    value
}

pub fn texture_download_has_failed(p0: i32) -> bool {
    let value = native!(bool, 0x5776ED562C134687, native_parameters!(p0));

    value
}

pub fn texture_download_get_name(p0: i32) -> String {
    let value = native!(*const i8, 0x3448505B6E35262D, native_parameters!(p0));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_status_of_texture_download(p0: i32) -> i32 {
    let value = native!(i32, 0x8BD6C6DEA20E82C6, native_parameters!(p0));

    value
}

pub fn _0x60edd13eb3ac1ff3() -> bool {
    let value = native!(bool, 0x60EDD13EB3AC1FF3, native_parameters!());

    value
}

pub fn _network_should_show_connectivity_troubleshooting() -> bool {
    let value = native!(bool, 0x82A2B386716608F1, native_parameters!());

    value
}

pub fn network_is_cable_connected() -> bool {
    let value = native!(bool, 0xEFFB25453D8600F9, native_parameters!());

    value
}

pub fn _network_get_ros_privilege_9() -> bool {
    let value = native!(bool, 0x66B59CFFD78467AF, native_parameters!());

    value
}

pub fn network_have_ros_social_club_priv() -> bool {
    let value = native!(bool, 0x606E4D3E3CCCF3EB, native_parameters!());

    value
}

pub fn network_have_ros_banned_priv() -> bool {
    let value = native!(bool, 0x8020A73847E0CA7D, native_parameters!());

    value
}

pub fn network_have_ros_create_ticket_priv() -> bool {
    let value = native!(bool, 0xA0AD7E2AF5349F61, native_parameters!());

    value
}

pub fn network_have_ros_multiplayer_priv() -> bool {
    let value = native!(bool, 0x5F91D5D0B36AA310, native_parameters!());

    value
}

pub fn network_have_ros_leaderboard_write_priv() -> bool {
    let value = native!(bool, 0x422D396F80A96547, native_parameters!());

    value
}

pub fn network_has_ros_privilege(index: i32) -> bool {
    let value = native!(bool, 0xA699957E60D80214, native_parameters!(index));

    value
}

pub fn network_has_ros_privilege_end_date(
    privilege: i32,
    banType: *mut i32,
    timeData: *mut u32,
) -> bool {
    let value = native!(
        bool,
        0xC22912B1D85F26B1,
        native_parameters!(privilege, banType, timeData)
    );

    value
}

pub fn _network_get_ros_privilege_24() -> bool {
    let value = native!(bool, 0x593570C289A77688, native_parameters!());

    value
}

pub fn _network_get_ros_privilege_25() -> bool {
    let value = native!(bool, 0x91B87C55093DE351, native_parameters!());

    value
}

pub fn _0x36391f397731595d(p0: u32) -> u32 {
    let value = native!(u32, 0x36391F397731595D, native_parameters!(p0));

    value
}

pub fn network_start_user_content_permissions_check(netHandle: *mut u32) -> i32 {
    let value = native!(i32, 0xDEB2B99A1AF1A2A6, native_parameters!(netHandle));

    value
}

pub fn _0x9465e683b12d3f6b() -> () {
    let value = native!((), 0x9465E683B12D3F6B, native_parameters!());

    value
}

pub fn _0xca59ccae5d01e4ce() -> () {
    let value = native!((), 0xCA59CCAE5D01E4CE, native_parameters!());

    value
}

pub fn _network_has_game_been_altered() -> bool {
    let value = native!(bool, 0x659CF2EF7F550C4F, native_parameters!());

    value
}

pub fn _network_update_player_scars() -> () {
    let value = native!((), 0xB7C7F6AD6424304B, native_parameters!());

    value
}

pub fn network_disable_leave_remote_ped_behind(toggle: bool) -> () {
    let value = native!((), 0xC505036A35AFD01B, native_parameters!(toggle));

    value
}

pub fn _network_allow_local_entity_attachment(entity: i32, toggle: bool) -> () {
    let value = native!((), 0x267C78C60E806B9A, native_parameters!(entity, toggle));

    value
}

pub fn _0x6bff5f84102df80a(player: i32) -> () {
    let value = native!((), 0x6BFF5F84102DF80A, native_parameters!(player));

    value
}

pub fn _0x5c497525f803486b() -> () {
    let value = native!((), 0x5C497525F803486B, native_parameters!());

    value
}

pub fn _0x6fb7bb3607d27fa2() -> u32 {
    let value = native!(u32, 0x6FB7BB3607D27FA2, native_parameters!());

    value
}

pub fn _0x45a83257ed02d9bc() -> () {
    let value = native!((), 0x45A83257ED02D9BC, native_parameters!());

    value
}

pub fn _0x16d3d49902f697bb(player: i32) -> bool {
    let value = native!(bool, 0x16D3D49902F697BB, native_parameters!(player));

    value
}

pub fn _0xd414be129bb81b32(player: i32) -> f32 {
    let value = native!(f32, 0xD414BE129BB81B32, native_parameters!(player));

    value
}

pub fn _0x0e3a041ed6ac2b45(player: i32) -> f32 {
    let value = native!(f32, 0x0E3A041ED6AC2B45, native_parameters!(player));

    value
}

pub fn _0x350c23949e43686c(player: i32) -> f32 {
    let value = native!(f32, 0x350C23949E43686C, native_parameters!(player));

    value
}

pub fn _network_get_num_unacked_for_player(player: i32) -> i32 {
    let value = native!(i32, 0xFF8FCF9FFC458A1C, native_parameters!(player));

    value
}

pub fn _network_get_unreliable_resend_count_for_player(player: i32) -> i32 {
    let value = native!(i32, 0x3765C3A3E8192E10, native_parameters!(player));

    value
}

pub fn _network_get_oldest_resend_count_for_player(player: i32) -> i32 {
    let value = native!(i32, 0x52C1EADAF7B10302, native_parameters!(player));

    value
}

pub fn _network_report_myself() -> () {
    let value = native!((), 0x5626D9D6810730D5, native_parameters!());

    value
}

pub fn _0x64d779659bc37b19(entity: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x64D779659BC37B19,
        native_parameters!(entity)
    );

    value
}

pub fn _network_get_player_coords(player: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x125E6D638B8605D4,
        native_parameters!(player)
    );

    value
}

pub fn _0x33de49edf4dde77a(entity: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x33DE49EDF4DDE77A,
        native_parameters!(entity)
    );

    value
}

pub fn _0xaa5fafcd2c5f5e47(entity: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0xAA5FAFCD2C5F5E47,
        native_parameters!(entity)
    );

    value
}

pub fn _0xaedf1bc1c133d6e3() -> u32 {
    let value = native!(u32, 0xAEDF1BC1C133D6E3, native_parameters!());

    value
}

pub fn _0x2555cf7da5473794() -> u32 {
    let value = native!(u32, 0x2555CF7DA5473794, native_parameters!());

    value
}

pub fn _0x6fd992c4a1c1b986() -> u32 {
    let value = native!(u32, 0x6FD992C4A1C1B986, native_parameters!());

    value
}

pub fn _0xdb663cc9ff3407a9(player: i32) -> i32 {
    let value = native!(i32, 0xDB663CC9FF3407A9, native_parameters!(player));

    value
}
