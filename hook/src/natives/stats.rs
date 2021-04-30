use crate::types::NativeVector3;

pub fn stat_clear_slot_for_reload(statSlot: i32) -> u32 {
    let value = native!(u32, 0xEB0A72181D4AA4AD, native_parameters!(statSlot));

    value
}

pub fn stat_load(p0: i32) -> bool {
    let value = native!(bool, 0xA651443F437B1CE6, native_parameters!(p0));

    value
}

pub fn stat_save(p0: i32, p1: bool, p2: i32, p3: u32) -> bool {
    let value = native!(bool, 0xE07BCA305B82D2FD, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x5688585e6d563cd8(p0: i32) -> () {
    let value = native!((), 0x5688585E6D563CD8, native_parameters!(p0));

    value
}

pub fn stat_load_pending(p0: u32) -> bool {
    let value = native!(bool, 0xA1750FFAFA181661, native_parameters!(p0));

    value
}

pub fn stat_save_pending() -> bool {
    let value = native!(bool, 0x7D3A583856F2C5AC, native_parameters!());

    value
}

pub fn stat_save_pending_or_requested() -> bool {
    let value = native!(bool, 0xBBB6AD006F1BBEA3, native_parameters!());

    value
}

pub fn stat_delete_slot(p0: u32) -> u32 {
    let value = native!(u32, 0x49A49BED12794D70, native_parameters!(p0));

    value
}

pub fn stat_slot_is_loaded(p0: u32) -> bool {
    let value = native!(bool, 0x0D0A9F0E7BD91E3C, native_parameters!(p0));

    value
}

pub fn _0x7f2c4cdf2e82df4c(p0: u32) -> bool {
    let value = native!(bool, 0x7F2C4CDF2E82DF4C, native_parameters!(p0));

    value
}

pub fn _0xe496a53ba5f50a56(p0: u32) -> u32 {
    let value = native!(u32, 0xE496A53BA5F50A56, native_parameters!(p0));

    value
}

pub fn stat_set_block_saves(toggle: bool) -> () {
    let value = native!((), 0xF434A10BA01C37D0, native_parameters!(toggle));

    value
}

pub fn _0x6a7f19756f1a9016() -> bool {
    let value = native!(bool, 0x6A7F19756F1A9016, native_parameters!());

    value
}

pub fn _0x7e6946f68a38b74f(p0: u32) -> bool {
    let value = native!(bool, 0x7E6946F68A38B74F, native_parameters!(p0));

    value
}

pub fn _0xa8733668d1047b51(p0: u32) -> () {
    let value = native!((), 0xA8733668D1047B51, native_parameters!(p0));

    value
}

pub fn _0xecb41ac6ab754401() -> bool {
    let value = native!(bool, 0xECB41AC6AB754401, native_parameters!());

    value
}

pub fn _0x9b4bd21d69b1e609() -> () {
    let value = native!((), 0x9B4BD21D69B1E609, native_parameters!());

    value
}

pub fn _0xc0e0d686ddfc6eae() -> u32 {
    let value = native!(u32, 0xC0E0D686DDFC6EAE, native_parameters!());

    value
}

pub fn stat_set_int(statName: u32, value: i32, save: bool) -> bool {
    let value = native!(
        bool,
        0xB3271D7AB655B441,
        native_parameters!(statName, value, save)
    );

    value
}

pub fn stat_set_float(statName: u32, value: f32, save: bool) -> bool {
    let value = native!(
        bool,
        0x4851997F37FE9B3C,
        native_parameters!(statName, value, save)
    );

    value
}

pub fn stat_set_bool(statName: u32, value: bool, save: bool) -> bool {
    let value = native!(
        bool,
        0x4B33C4243DE0C432,
        native_parameters!(statName, value, save)
    );

    value
}

pub fn stat_set_gxt_label(statName: u32, value: &std::ffi::CString, save: bool) -> bool {
    let value = native!(
        bool,
        0x17695002FD8B2AE0,
        native_parameters!(statName, value.as_ptr(), save)
    );

    value
}

pub fn stat_set_date(statName: u32, value: *mut u32, numFields: i32, save: bool) -> bool {
    let value = native!(
        bool,
        0x2C29BFB64F4FCBE4,
        native_parameters!(statName, value, numFields, save)
    );

    value
}

pub fn stat_set_string(statName: u32, value: &std::ffi::CString, save: bool) -> bool {
    let value = native!(
        bool,
        0xA87B2335D12531D7,
        native_parameters!(statName, value.as_ptr(), save)
    );

    value
}

pub fn stat_set_pos(statName: u32, x: f32, y: f32, z: f32, save: bool) -> bool {
    let value = native!(
        bool,
        0xDB283FDE680FE72E,
        native_parameters!(statName, x, y, z, save)
    );

    value
}

pub fn stat_set_masked_int(statName: u32, p1: u32, p2: u32, p3: i32, save: bool) -> bool {
    let value = native!(
        bool,
        0x7BBB1B54583ED410,
        native_parameters!(statName, p1, p2, p3, save)
    );

    value
}

pub fn stat_set_user_id(statName: u32, value: &std::ffi::CString, save: bool) -> bool {
    let value = native!(
        bool,
        0x8CDDF1E452BABE11,
        native_parameters!(statName, value.as_ptr(), save)
    );

    value
}

pub fn stat_set_current_posix_time(statName: u32, p1: bool) -> bool {
    let value = native!(bool, 0xC2F84B7F9C4D0C61, native_parameters!(statName, p1));

    value
}

pub fn stat_get_int(statHash: u32, outValue: *mut i32, p2: i32) -> bool {
    let value = native!(
        bool,
        0x767FBC2AC802EF3D,
        native_parameters!(statHash, outValue, p2)
    );

    value
}

pub fn stat_get_float(statHash: u32, outValue: *mut f32, p2: u32) -> bool {
    let value = native!(
        bool,
        0xD7AE6C9C9C6AC54C,
        native_parameters!(statHash, outValue, p2)
    );

    value
}

pub fn stat_get_bool(statHash: u32, outValue: *mut bool, p2: u32) -> bool {
    let value = native!(
        bool,
        0x11B5E6D2AE73F48E,
        native_parameters!(statHash, outValue, p2)
    );

    value
}

pub fn stat_get_date(statHash: u32, p1: *mut u32, p2: u32, p3: u32) -> bool {
    let value = native!(
        bool,
        0x8B0FACEFC36C824B,
        native_parameters!(statHash, p1, p2, p3)
    );

    value
}

pub fn stat_get_string(statHash: u32, p1: i32) -> String {
    let value = native!(
        *const i8,
        0xE50384ACC2C3DB74,
        native_parameters!(statHash, p1)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn stat_get_pos(p0: u32, p1: *mut u32, p2: *mut u32, p3: *mut u32, p4: u32) -> bool {
    let value = native!(
        bool,
        0x350F82CCB186AA1B,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn stat_get_masked_int(p0: u32, p1: *mut u32, p2: u32, p3: u32, p4: u32) -> bool {
    let value = native!(
        bool,
        0x655185A06D9EEAAB,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn stat_get_user_id(p0: u32) -> String {
    let value = native!(*const i8, 0x2365C388E393BBE2, native_parameters!(p0));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn stat_get_license_plate(statName: u32) -> String {
    let value = native!(*const i8, 0x5473D4195058B2E4, native_parameters!(statName));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn stat_set_license_plate(statName: u32, str: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x69FF13266D7296DA,
        native_parameters!(statName, str.as_ptr())
    );

    value
}

pub fn stat_increment(statName: u32, value: f32) -> () {
    let value = native!((), 0x9B5A68C6489E9909, native_parameters!(statName, value));

    value
}

pub fn _0x5a556b229a169402() -> bool {
    let value = native!(bool, 0x5A556B229A169402, native_parameters!());

    value
}

pub fn _0xb1d2bb1e1631f5b1() -> bool {
    let value = native!(bool, 0xB1D2BB1E1631F5B1, native_parameters!());

    value
}

pub fn _0xbed9f5693f34ed17(statName: u32, p1: i32, outValue: *mut f32) -> bool {
    let value = native!(
        bool,
        0xBED9F5693F34ED17,
        native_parameters!(statName, p1, outValue)
    );

    value
}

pub fn _0x26d7399b9587fe89(p0: i32) -> () {
    let value = native!((), 0x26D7399B9587FE89, native_parameters!(p0));

    value
}

pub fn _0xa78b8fa58200da56(p0: i32) -> () {
    let value = native!((), 0xA78B8FA58200DA56, native_parameters!(p0));

    value
}

pub fn stat_get_number_of_days(statName: u32) -> i32 {
    let value = native!(i32, 0xE0E854F5280FB769, native_parameters!(statName));

    value
}

pub fn stat_get_number_of_hours(statName: u32) -> i32 {
    let value = native!(i32, 0xF2D4B2FE415AAFC3, native_parameters!(statName));

    value
}

pub fn stat_get_number_of_minutes(statName: u32) -> i32 {
    let value = native!(i32, 0x7583B4BE4C5A41B5, native_parameters!(statName));

    value
}

pub fn stat_get_number_of_seconds(statName: u32) -> i32 {
    let value = native!(i32, 0x2CE056FF3723F00B, native_parameters!(statName));

    value
}

pub fn stat_set_profile_setting_value(profileSetting: i32, value: i32) -> () {
    let value = native!(
        (),
        0x68F01422BE1D838F,
        native_parameters!(profileSetting, value)
    );

    value
}

pub fn _stat_get_packed_bool_mask(p0: i32) -> i32 {
    let value = native!(i32, 0xF4D8E7AC2A27758C, native_parameters!(p0));

    value
}

pub fn _stat_get_packed_int_mask(p0: i32) -> i32 {
    let value = native!(i32, 0x94F12ABF9C79E339, native_parameters!(p0));

    value
}

pub fn get_packed_bool_stat_key(index: i32, spStat: bool, charStat: bool, character: i32) -> u32 {
    let value = native!(
        u32,
        0x80C75307B1C42837,
        native_parameters!(index, spStat, charStat, character)
    );

    value
}

pub fn get_packed_int_stat_key(index: i32, spStat: bool, charStat: bool, character: i32) -> u32 {
    let value = native!(
        u32,
        0x61E111E323419E07,
        native_parameters!(index, spStat, charStat, character)
    );

    value
}

pub fn get_packed_tu_bool_stat_key(
    index: i32,
    spStat: bool,
    charStat: bool,
    character: i32,
) -> u32 {
    let value = native!(
        u32,
        0xC4BB08EE7907471E,
        native_parameters!(index, spStat, charStat, character)
    );

    value
}

pub fn get_packed_tu_int_stat_key(index: i32, spStat: bool, charStat: bool, character: i32) -> u32 {
    let value = native!(
        u32,
        0xD16C2AD6B8E32854,
        native_parameters!(index, spStat, charStat, character)
    );

    value
}

pub fn _get_ngstat_bool_hash(
    index: i32,
    spStat: bool,
    charStat: bool,
    character: i32,
    section: &std::ffi::CString,
) -> u32 {
    let value = native!(
        u32,
        0xBA52FF538ED2BC71,
        native_parameters!(index, spStat, charStat, character, section.as_ptr())
    );

    value
}

pub fn _get_ngstat_int_hash(
    index: i32,
    spStat: bool,
    charStat: bool,
    character: i32,
    section: &std::ffi::CString,
) -> u32 {
    let value = native!(
        u32,
        0x2B4CDCA6F07FF3DA,
        native_parameters!(index, spStat, charStat, character, section.as_ptr())
    );

    value
}

pub fn stat_get_bool_masked(statName: u32, mask: i32, p2: i32) -> bool {
    let value = native!(
        bool,
        0x10FE3F1B79F9B071,
        native_parameters!(statName, mask, p2)
    );

    value
}

pub fn stat_set_bool_masked(statName: u32, value: bool, mask: i32, save: bool) -> bool {
    let value = native!(
        bool,
        0x5BC62EC1937B9E5B,
        native_parameters!(statName, value, mask, save)
    );

    value
}

pub fn playstats_background_script_action(action: &std::ffi::CString, value: i32) -> () {
    let value = native!(
        (),
        0x5009DFD741329729,
        native_parameters!(action.as_ptr(), value)
    );

    value
}

pub fn playstats_npc_invite(p0: *mut u32) -> () {
    let value = native!((), 0x93054C88E6AA7C44, native_parameters!(p0));

    value
}

pub fn playstats_award_xp(amount: i32, type_esc: u32, category: u32) -> () {
    let value = native!(
        (),
        0x46F917F6B4128FE4,
        native_parameters!(amount, type_esc, category)
    );

    value
}

pub fn playstats_rank_up(rank: i32) -> () {
    let value = native!((), 0xC7F2DE41D102BFB4, native_parameters!(rank));

    value
}

pub fn _playstats_start_offline_mode() -> () {
    let value = native!((), 0x098760C7461724CD, native_parameters!());

    value
}

pub fn playstats_activity_done(p0: u32, p1: u32) -> () {
    let value = native!((), 0xA071E0ED98F91286, native_parameters!(p0, p1));

    value
}

pub fn playstats_leave_job_chain(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0xC5BE134EC7BA96A0,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn playstats_mission_started(p0: *mut u32, p1: u32, p2: u32, p3: bool) -> () {
    let value = native!((), 0xC19A2925C34D2231, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn playstats_mission_over(p0: *mut u32, p1: u32, p2: u32, p3: bool, p4: bool, p5: bool) -> () {
    let value = native!(
        (),
        0x7C4BB33A8CED7324,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn playstats_mission_checkpoint(p0: *mut u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xC900596A63978C1D, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn playstats_random_mission_done(name: &std::ffi::CString, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!(
        (),
        0x71862B1D855F32E1,
        native_parameters!(name.as_ptr(), p1, p2, p3)
    );

    value
}

pub fn playstats_ros_bet(amount: i32, act: i32, player: i32, cm: f32) -> () {
    let value = native!(
        (),
        0x121FB4DDDC2D5291,
        native_parameters!(amount, act, player, cm)
    );

    value
}

pub fn playstats_race_checkpoint(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x9C375C315099DDE4,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _0x6dee77aff8c21bd1(playerAccountId: *mut i32, posixTime: *mut i32) -> bool {
    let value = native!(
        bool,
        0x6DEE77AFF8C21BD1,
        native_parameters!(playerAccountId, posixTime)
    );

    value
}

pub fn playstats_match_started(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xBC80E22DED931E3D, native_parameters!(p0, p1, p2));

    value
}

pub fn playstats_shop_item(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x176852ACAAC173D1,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn playstats_crate_drop_mission_done(
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
        0x1CAE5D2E3F9A07F0,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn _playstats_crate_created_mission_done(p0: f32, p1: f32, p2: f32) -> () {
    let value = native!((), 0xAFC7E5E075A96F46, native_parameters!(p0, p1, p2));

    value
}

pub fn playstats_hold_up_mission_done(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xCB00196B31C39EB1, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn playstats_import_export_mission_done(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x2B69F5074C894811, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn playstats_race_to_point_mission_done(
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
        0xADDD1C754E2E2914,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn playstats_acquired_hidden_package(p0: u32) -> () {
    let value = native!((), 0x79AB33F0FBFAC40C, native_parameters!(p0));

    value
}

pub fn playstats_website_visited(scaleformHash: u32, p1: i32) -> () {
    let value = native!(
        (),
        0xDDF24D535060F811,
        native_parameters!(scaleformHash, p1)
    );

    value
}

pub fn playstats_friend_activity(p0: u32, p1: u32) -> () {
    let value = native!((), 0x0F71DE29AB2258F1, native_parameters!(p0, p1));

    value
}

pub fn playstats_oddjob_done(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x69DEA3E9DB727B4C, native_parameters!(p0, p1, p2));

    value
}

pub fn playstats_prop_change(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xBA739D6D5A05D6E7, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn playstats_cloth_change(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x34B973047A2268B9,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn playstats_weapon_mode_change(
    weaponHash: u32,
    componentHashTo: u32,
    componentHashFrom: u32,
) -> () {
    let value = native!(
        (),
        0xE95C8A1875A02CA4,
        native_parameters!(weaponHash, componentHashTo, componentHashFrom)
    );

    value
}

pub fn playstats_cheat_applied(cheat: &std::ffi::CString) -> () {
    let value = native!((), 0x6058665D72302D3F, native_parameters!(cheat.as_ptr()));

    value
}

pub fn _0xf8c54a461c3e11dc(p0: *mut u32, p1: *mut u32, p2: *mut u32, p3: *mut u32) -> () {
    let value = native!((), 0xF8C54A461C3E11DC, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xf5bb8dac426a52c0(p0: *mut u32, p1: *mut u32, p2: *mut u32, p3: *mut u32) -> () {
    let value = native!((), 0xF5BB8DAC426A52C0, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xa736cf7fb7c5bff4(p0: *mut u32, p1: *mut u32, p2: *mut u32, p3: *mut u32) -> () {
    let value = native!((), 0xA736CF7FB7C5BFF4, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x14e0b2d1ad1044e0(p0: *mut u32, p1: *mut u32, p2: *mut u32, p3: *mut u32) -> () {
    let value = native!((), 0x14E0B2D1AD1044E0, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn playstats_quickfix_tool(element: i32, item: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x90D0622866E80445,
        native_parameters!(element, item.as_ptr())
    );

    value
}

pub fn playstats_idle_kick(time: i32) -> () {
    let value = native!((), 0x5DA3A8DE8CB6226F, native_parameters!(time));

    value
}

pub fn _0xd1032e482629049e(p0: i32) -> () {
    let value = native!((), 0xD1032E482629049E, native_parameters!(p0));

    value
}

pub fn _playstats_heist_save_cheat(hash: u32, p1: i32) -> () {
    let value = native!((), 0xF4FF020A08BC8863, native_parameters!(hash, p1));

    value
}

pub fn _playstats_director_mode(p0: *mut u32) -> () {
    let value = native!((), 0x46326E13DA4E0546, native_parameters!(p0));

    value
}

pub fn _playstats_award_badsport(id: i32) -> () {
    let value = native!((), 0x47B32F5611E6E483, native_parameters!(id));

    value
}

pub fn _playstats_pegasaircraft(modelHash: u32) -> () {
    let value = native!((), 0x9572BD4DD6B72122, native_parameters!(modelHash));

    value
}

pub fn _0x6a60e43998228229(p0: u32) -> () {
    let value = native!((), 0x6A60E43998228229, native_parameters!(p0));

    value
}

pub fn _0xbfafdb5faaa5c5ab(p0: u32) -> () {
    let value = native!((), 0xBFAFDB5FAAA5C5AB, native_parameters!(p0));

    value
}

pub fn _0x8c9d11605e59d955(p0: u32) -> () {
    let value = native!((), 0x8C9D11605E59D955, native_parameters!(p0));

    value
}

pub fn _0x3de3aa516fb126a4(p0: u32) -> () {
    let value = native!((), 0x3DE3AA516FB126A4, native_parameters!(p0));

    value
}

pub fn _0xbaa2f0490e146be8(p0: u32) -> () {
    let value = native!((), 0xBAA2F0490E146BE8, native_parameters!(p0));

    value
}

pub fn _0x1a7ce7cd3e653485(p0: u32) -> () {
    let value = native!((), 0x1A7CE7CD3E653485, native_parameters!(p0));

    value
}

pub fn _0x419615486bbf1956(p0: u32) -> () {
    let value = native!((), 0x419615486BBF1956, native_parameters!(p0));

    value
}

pub fn _0x84dfc579c2fc214c(p0: u32) -> () {
    let value = native!((), 0x84DFC579C2FC214C, native_parameters!(p0));

    value
}

pub fn _0x0a9c7f36e5d7b683(p0: u32) -> () {
    let value = native!((), 0x0A9C7F36E5D7B683, native_parameters!(p0));

    value
}

pub fn _0x164c5ff663790845(p0: u32) -> () {
    let value = native!((), 0x164C5FF663790845, native_parameters!(p0));

    value
}

pub fn _0xedbf6c9b0d2c65c8(p0: u32) -> () {
    let value = native!((), 0xEDBF6C9B0D2C65C8, native_parameters!(p0));

    value
}

pub fn _0x6551b1f7f6cd46ea(p0: u32) -> () {
    let value = native!((), 0x6551B1F7F6CD46EA, native_parameters!(p0));

    value
}

pub fn _0x2cd90358f67d0aa8(p0: u32) -> () {
    let value = native!((), 0x2CD90358F67D0AA8, native_parameters!(p0));

    value
}

pub fn _playstats_pi_menu_hide_settings(data: *mut u32) -> () {
    let value = native!((), 0x203B381133817079, native_parameters!(data));

    value
}

pub fn leaderboards_get_number_of_columns(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x117B45156D7EFF2E, native_parameters!(p0, p1));

    value
}

pub fn leaderboards_get_column_id(p0: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(u32, 0xC4B5467A1886EA7E, native_parameters!(p0, p1, p2));

    value
}

pub fn leaderboards_get_column_type(p0: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(u32, 0xBF4FEF46DB7894D3, native_parameters!(p0, p1, p2));

    value
}

pub fn leaderboards_read_clear_all() -> u32 {
    let value = native!(u32, 0xA34CB6E6F0DF4A0B, native_parameters!());

    value
}

pub fn leaderboards_read_clear(p0: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(u32, 0x7CCE5C737A665701, native_parameters!(p0, p1, p2));

    value
}

pub fn leaderboards_read_pending(p0: u32, p1: u32, p2: u32) -> bool {
    let value = native!(bool, 0xAC392C8483342AC2, native_parameters!(p0, p1, p2));

    value
}

pub fn leaderboards_read_any_pending() -> bool {
    let value = native!(bool, 0xA31FD15197B192BD, native_parameters!());

    value
}

pub fn leaderboards_read_successful(p0: u32, p1: u32, p2: u32) -> bool {
    let value = native!(bool, 0x2FB19228983E832C, native_parameters!(p0, p1, p2));

    value
}

pub fn leaderboards2_read_friends_by_row(
    p0: *mut u32,
    p1: *mut u32,
    p2: u32,
    p3: bool,
    p4: u32,
    p5: u32,
) -> bool {
    let value = native!(
        bool,
        0x918B101666F9CB83,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn leaderboards2_read_by_handle(p0: *mut u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0xC30713A383BFBF0E, native_parameters!(p0, p1));

    value
}

pub fn leaderboards2_read_by_row(
    p0: *mut u32,
    p1: *mut u32,
    p2: u32,
    p3: *mut u32,
    p4: u32,
    p5: *mut u32,
    p6: u32,
) -> bool {
    let value = native!(
        bool,
        0xA9CDB1E3F0A49883,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn leaderboards2_read_by_rank(p0: *mut u32, p1: u32, p2: u32) -> bool {
    let value = native!(bool, 0xBA2C7DB0C129449A, native_parameters!(p0, p1, p2));

    value
}

pub fn leaderboards2_read_by_radius(p0: *mut u32, p1: u32, p2: *mut u32) -> bool {
    let value = native!(bool, 0x5CE587FB5A42C8C4, native_parameters!(p0, p1, p2));

    value
}

pub fn leaderboards2_read_by_score_int(p0: *mut u32, p1: u32, p2: u32) -> bool {
    let value = native!(bool, 0x7EEC7E4F6984A16A, native_parameters!(p0, p1, p2));

    value
}

pub fn leaderboards2_read_by_score_float(p0: *mut u32, p1: f32, p2: u32) -> bool {
    let value = native!(bool, 0xE662C8B759D08F3C, native_parameters!(p0, p1, p2));

    value
}

pub fn leaderboards2_read_rank_prediction(p0: *mut u32, p1: *mut u32, p2: *mut u32) -> bool {
    let value = native!(bool, 0xC38DC1E90D22547C, native_parameters!(p0, p1, p2));

    value
}

pub fn _leaderboards2_read_by_platform(
    p0: *mut u32,
    gamerHandleCsv: &std::ffi::CString,
    platformName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0xF1AE5DCDBFCA2721,
        native_parameters!(p0, gamerHandleCsv.as_ptr(), platformName.as_ptr())
    );

    value
}

pub fn _0xa0f93d5465b3094d(p0: *mut u32) -> bool {
    let value = native!(bool, 0xA0F93D5465B3094D, native_parameters!(p0));

    value
}

pub fn _0x71b008056e5692d6() -> () {
    let value = native!((), 0x71B008056E5692D6, native_parameters!());

    value
}

pub fn _0x34770b9ce0e03b91(p0: u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0x34770B9CE0E03B91, native_parameters!(p0, p1));

    value
}

pub fn _0x88578f6ec36b4a3a(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x88578F6EC36B4A3A, native_parameters!(p0, p1));

    value
}

pub fn _0x38491439b6ba7f7d(p0: u32, p1: u32) -> f32 {
    let value = native!(f32, 0x38491439B6BA7F7D, native_parameters!(p0, p1));

    value
}

pub fn leaderboards2_write_data(p0: *mut u32) -> bool {
    let value = native!(bool, 0xAE2206545888AE49, native_parameters!(p0));

    value
}

pub fn leaderboards_write_add_column(p0: u32, p1: u32, p2: f32) -> () {
    let value = native!((), 0x0BCA1D2C47B0D269, native_parameters!(p0, p1, p2));

    value
}

pub fn leaderboards_write_add_column_long(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x2E65248609523599, native_parameters!(p0, p1, p2));

    value
}

pub fn leaderboards_cache_data_row(p0: *mut u32) -> bool {
    let value = native!(bool, 0xB9BB18E2C40142ED, native_parameters!(p0));

    value
}

pub fn leaderboards_clear_cache_data() -> () {
    let value = native!((), 0xD4B02A6B476E1FDC, native_parameters!());

    value
}

pub fn _0x8ec74ceb042e7cff(p0: u32) -> () {
    let value = native!((), 0x8EC74CEB042E7CFF, native_parameters!(p0));

    value
}

pub fn leaderboards_get_cache_exists(p0: u32) -> bool {
    let value = native!(bool, 0x9C51349BE6CDFE2C, native_parameters!(p0));

    value
}

pub fn leaderboards_get_cache_time(p0: u32) -> u32 {
    let value = native!(u32, 0xF04C1C27DA35F6C8, native_parameters!(p0));

    value
}

pub fn leaderboards_get_cache_number_of_rows(p0: u32) -> i32 {
    let value = native!(i32, 0x58A651CD201D89AD, native_parameters!(p0));

    value
}

pub fn leaderboards_get_cache_data_row(p0: u32, p1: u32, p2: *mut u32) -> bool {
    let value = native!(bool, 0x9120E8DBA3D69273, native_parameters!(p0, p1, p2));

    value
}

pub fn _update_stat_int(statHash: u32, value: i32, p2: i32) -> () {
    let value = native!(
        (),
        0x11FF1C80276097ED,
        native_parameters!(statHash, value, p2)
    );

    value
}

pub fn _update_stat_float(statHash: u32, value: f32, p2: i32) -> () {
    let value = native!(
        (),
        0x30A6614C1F7799B8,
        native_parameters!(statHash, value, p2)
    );

    value
}

pub fn _0x6483c25849031c4f(p0: u32, p1: u32, p2: u32, p3: *mut u32) -> () {
    let value = native!((), 0x6483C25849031C4F, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x5ead2bf6484852e4() -> bool {
    let value = native!(bool, 0x5EAD2BF6484852E4, native_parameters!());

    value
}

pub fn _0xc141b8917e0017ec() -> () {
    let value = native!((), 0xC141B8917E0017EC, native_parameters!());

    value
}

pub fn set_profile_setting_prologue_complete() -> () {
    let value = native!((), 0xB475F27C6A994D65, native_parameters!());

    value
}

pub fn _0xc67e2da1cbe759e2() -> () {
    let value = native!((), 0xC67E2DA1CBE759E2, native_parameters!());

    value
}

pub fn _0xf1a1803d3476f215(value: i32) -> () {
    let value = native!((), 0xF1A1803D3476F215, native_parameters!(value));

    value
}

pub fn _0x38baaa5dd4c9d19f(value: i32) -> () {
    let value = native!((), 0x38BAAA5DD4C9D19F, native_parameters!(value));

    value
}

pub fn _0x55384438fc55ad8e(value: i32) -> () {
    let value = native!((), 0x55384438FC55AD8E, native_parameters!(value));

    value
}

pub fn _0x723c1ce13fbfdb67(p0: u32, p1: u32) -> () {
    let value = native!((), 0x723C1CE13FBFDB67, native_parameters!(p0, p1));

    value
}

pub fn _0x0d01d20616fc73fb(p0: u32, p1: u32) -> () {
    let value = native!((), 0x0D01D20616FC73FB, native_parameters!(p0, p1));

    value
}

pub fn _0x428eaf89e24f6c36(p0: u32, p1: f32) -> () {
    let value = native!((), 0x428EAF89E24F6C36, native_parameters!(p0, p1));

    value
}

pub fn stat_set_cheat_is_active() -> () {
    let value = native!((), 0x047CBED6F6F8B63C, native_parameters!());

    value
}

pub fn leaderboards2_write_data_for_event_type(p0: *mut u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0xC980E62E33DF1D5C, native_parameters!(p0, p1));

    value
}

pub fn _0x6f361b8889a792a3() -> () {
    let value = native!((), 0x6F361B8889A792A3, native_parameters!());

    value
}

pub fn _0xc847b43f369ac0b5() -> () {
    let value = native!((), 0xC847B43F369AC0B5, native_parameters!());

    value
}

pub fn _stat_migrate_save(platformName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xA5C80D8E768A9E66,
        native_parameters!(platformName.as_ptr())
    );

    value
}

pub fn _0x9a62ec95ae10e011() -> i32 {
    let value = native!(i32, 0x9A62EC95AE10E011, native_parameters!());

    value
}

pub fn _0x4c89fe2bdeb3f169() -> u32 {
    let value = native!(u32, 0x4C89FE2BDEB3F169, native_parameters!());

    value
}

pub fn _0xc6e0e2616a7576bb() -> u32 {
    let value = native!(u32, 0xC6E0E2616A7576BB, native_parameters!());

    value
}

pub fn _0x5bd5f255321c4aaf(p0: u32) -> u32 {
    let value = native!(u32, 0x5BD5F255321C4AAF, native_parameters!(p0));

    value
}

pub fn _0xdeaaf77eb3687e97(p0: u32, p1: *mut u32) -> u32 {
    let value = native!(u32, 0xDEAAF77EB3687E97, native_parameters!(p0, p1));

    value
}

pub fn stat_save_migration_status_start() -> bool {
    let value = native!(bool, 0xC70DDCE56D0D3A99, native_parameters!());

    value
}

pub fn stat_get_save_migration_status(data: *mut u32) -> i32 {
    let value = native!(i32, 0x886913BBEACA68C1, native_parameters!(data));

    value
}

pub fn _stat_save_migration_cancel() -> bool {
    let value = native!(bool, 0x4FEF53183C3C6414, native_parameters!());

    value
}

pub fn _stat_get_cancel_save_migration_status() -> i32 {
    let value = native!(i32, 0x567384DFA67029E6, native_parameters!());

    value
}

pub fn _stat_save_migration_consume_content_unlock(
    contentId: u32,
    srcPlatform: &std::ffi::CString,
    srcGamerHandle: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x3270F67EED31FBC1,
        native_parameters!(contentId, srcPlatform.as_ptr(), srcGamerHandle.as_ptr())
    );

    value
}

pub fn _stat_get_save_migration_consume_content_unlock_status(p0: *mut i32) -> i32 {
    let value = native!(i32, 0xCE5AA445ABA8DEE0, native_parameters!(p0));

    value
}

pub fn _0x98e2bc1ca26287c3() -> () {
    let value = native!((), 0x98E2BC1CA26287C3, native_parameters!());

    value
}

pub fn _0x629526aba383bcaa() -> () {
    let value = native!((), 0x629526ABA383BCAA, native_parameters!());

    value
}

pub fn _0xbe3db208333d9844() -> u32 {
    let value = native!(u32, 0xBE3DB208333D9844, native_parameters!());

    value
}

pub fn _0x33d72899e24c3365(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x33D72899E24C3365, native_parameters!(p0, p1));

    value
}

pub fn _0xa761d4ac6115623d() -> u32 {
    let value = native!(u32, 0xA761D4AC6115623D, native_parameters!());

    value
}

pub fn _0xf11f01d98113536a(p0: u32) -> u32 {
    let value = native!(u32, 0xF11F01D98113536A, native_parameters!(p0));

    value
}

pub fn _0x8b9cdbd6c566c38c() -> u32 {
    let value = native!(u32, 0x8B9CDBD6C566C38C, native_parameters!());

    value
}

pub fn _0xe8853fbce7d8d0d6() -> u32 {
    let value = native!(u32, 0xE8853FBCE7D8D0D6, native_parameters!());

    value
}

pub fn _0xa943fd1722e11efd() -> u32 {
    let value = native!(u32, 0xA943FD1722E11EFD, native_parameters!());

    value
}

pub fn _0x84a810b375e69c0e() -> u32 {
    let value = native!(u32, 0x84A810B375E69C0E, native_parameters!());

    value
}

pub fn _0x9ec8858184cd253a() -> u32 {
    let value = native!(u32, 0x9EC8858184CD253A, native_parameters!());

    value
}

pub fn _0xba9749cc94c1fd85() -> u32 {
    let value = native!(u32, 0xBA9749CC94C1FD85, native_parameters!());

    value
}

pub fn _0x55a8becaf28a4eb7() -> u32 {
    let value = native!(u32, 0x55A8BECAF28A4EB7, native_parameters!());

    value
}

pub fn _0x32cac93c9de73d32() -> u32 {
    let value = native!(u32, 0x32CAC93C9DE73D32, native_parameters!());

    value
}

pub fn _0xaff47709f1d5dcce() -> u32 {
    let value = native!(u32, 0xAFF47709F1D5DCCE, native_parameters!());

    value
}

pub fn _0x6e0a5253375c4584() -> u32 {
    let value = native!(u32, 0x6E0A5253375C4584, native_parameters!());

    value
}

pub fn _0x1a8ea222f9c67dbb(p0: u32) -> u32 {
    let value = native!(u32, 0x1A8EA222F9C67DBB, native_parameters!(p0));

    value
}

pub fn _0xf9f2922717b819ec() -> u32 {
    let value = native!(u32, 0xF9F2922717B819EC, native_parameters!());

    value
}

pub fn _0x0b8b7f74bf061c6d() -> u32 {
    let value = native!(u32, 0x0B8B7F74BF061C6D, native_parameters!());

    value
}

pub fn _0xb3da2606774a8e2d() -> bool {
    let value = native!(bool, 0xB3DA2606774A8E2D, native_parameters!());

    value
}

pub fn _set_has_content_unlocks_flags(value: i32) -> () {
    let value = native!((), 0xDAC073C7901F9E15, native_parameters!(value));

    value
}

pub fn _set_save_migration_transaction_id(transactionId: i32) -> () {
    let value = native!((), 0xF6792800AC95350D, native_parameters!(transactionId));

    value
}

pub fn _0x6bc0acd0673acebe(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x6BC0ACD0673ACEBE, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x8d8adb562f09a245(p0: u32) -> () {
    let value = native!((), 0x8D8ADB562F09A245, native_parameters!(p0));

    value
}

pub fn _0xd1a1ee3b4fa8e760(p0: u32) -> () {
    let value = native!((), 0xD1A1EE3B4FA8E760, native_parameters!(p0));

    value
}

pub fn _0x88087ee1f28024ae(p0: u32) -> () {
    let value = native!((), 0x88087EE1F28024AE, native_parameters!(p0));

    value
}

pub fn _0xfcc228e07217fcac(p0: u32) -> () {
    let value = native!((), 0xFCC228E07217FCAC, native_parameters!(p0));

    value
}

pub fn _0x678f86d8fc040bdb(p0: u32) -> () {
    let value = native!((), 0x678F86D8FC040BDB, native_parameters!(p0));

    value
}

pub fn _0xa6f54bb2ffca35ea(p0: u32) -> () {
    let value = native!((), 0xA6F54BB2FFCA35EA, native_parameters!(p0));

    value
}

pub fn _0x5ff2c33b13a02a11(p0: u32) -> () {
    let value = native!((), 0x5FF2C33B13A02A11, native_parameters!(p0));

    value
}

pub fn _0x282b6739644f4347(p0: u32) -> () {
    let value = native!((), 0x282B6739644F4347, native_parameters!(p0));

    value
}

pub fn _0xf06a6f41cb445443(p0: u32) -> () {
    let value = native!((), 0xF06A6F41CB445443, native_parameters!(p0));

    value
}

pub fn _0x7b18da61f6bae9d5(p0: u32) -> () {
    let value = native!((), 0x7B18DA61F6BAE9D5, native_parameters!(p0));

    value
}

pub fn _0x06eaf70ae066441e(p0: u32) -> () {
    let value = native!((), 0x06EAF70AE066441E, native_parameters!(p0));

    value
}

pub fn _0x14eda9ee27bd1626(p0: u32) -> () {
    let value = native!((), 0x14EDA9EE27BD1626, native_parameters!(p0));

    value
}

pub fn _0x930f504203f561c9(p0: u32) -> () {
    let value = native!((), 0x930F504203F561C9, native_parameters!(p0));

    value
}

pub fn _0xe3261d791eb44acb(p0: u32) -> () {
    let value = native!((), 0xE3261D791EB44ACB, native_parameters!(p0));

    value
}

pub fn _0x73001e34f85137f8(p0: u32) -> () {
    let value = native!((), 0x73001E34F85137F8, native_parameters!(p0));

    value
}

pub fn _0x53cae13e9b426993(p0: u32) -> () {
    let value = native!((), 0x53CAE13E9B426993, native_parameters!(p0));

    value
}

pub fn _0x7d36291161859389(p0: u32) -> () {
    let value = native!((), 0x7D36291161859389, native_parameters!(p0));

    value
}

pub fn _playstats_spent_pi_custom_loadout(amount: i32) -> () {
    let value = native!((), 0xBE509B0A3693DE8B, native_parameters!(amount));

    value
}

pub fn _playstats_buy_contraband(data: *mut u32) -> () {
    let value = native!((), 0xD6781E42755531F7, native_parameters!(data));

    value
}

pub fn _playstats_sell_contraband(data: *mut u32) -> () {
    let value = native!((), 0xC729991A9065376E, native_parameters!(data));

    value
}

pub fn _playstats_defend_contraband(data: *mut u32) -> () {
    let value = native!((), 0x2605663BD4F23B5D, native_parameters!(data));

    value
}

pub fn _playstats_recover_contraband(data: *mut u32) -> () {
    let value = native!((), 0x04D90BA8207ADA2D, native_parameters!(data));

    value
}

pub fn _0x60eedc12af66e846(p0: u32) -> () {
    let value = native!((), 0x60EEDC12AF66E846, native_parameters!(p0));

    value
}

pub fn _0x3ebeac6c3f81f6bd(p0: u32) -> () {
    let value = native!((), 0x3EBEAC6C3F81F6BD, native_parameters!(p0));

    value
}

pub fn _0x96e6d5150dbf1c09(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x96E6D5150DBF1C09, native_parameters!(p0, p1, p2));

    value
}

pub fn _0xa3c53804bdb68ed2(p0: u32, p1: u32) -> () {
    let value = native!((), 0xA3C53804BDB68ED2, native_parameters!(p0, p1));

    value
}

pub fn _0x6bccf9948492fd85(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x6BCCF9948492FD85,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _hired_limo(p0: u32, p1: u32) -> () {
    let value = native!((), 0x792271AB35C356A4, native_parameters!(p0, p1));

    value
}

pub fn _ordered_boss_vehicle(p0: u32, p1: u32, vehicleHash: u32) -> () {
    let value = native!(
        (),
        0xCEA553E35C2246E1,
        native_parameters!(p0, p1, vehicleHash)
    );

    value
}

pub fn _0xd1c9b92bdd3f151d(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xD1C9B92BDD3F151D, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x44919cc079bb60bf(p0: u32) -> () {
    let value = native!((), 0x44919CC079BB60BF, native_parameters!(p0));

    value
}

pub fn _0x7033eefd9b28088e(p0: u32) -> () {
    let value = native!((), 0x7033EEFD9B28088E, native_parameters!(p0));

    value
}

pub fn _0xaa525dff66bb82f5(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xAA525DFF66BB82F5, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x015b03ee1c43e6ec(p0: u32) -> () {
    let value = native!((), 0x015B03EE1C43E6EC, native_parameters!(p0));

    value
}

pub fn _playstats_stunt_performed_event_allow_trigger() -> () {
    let value = native!((), 0x928DBFB892638EF3, native_parameters!());

    value
}

pub fn _playstats_stunt_performed_event_disallow_trigger() -> () {
    let value = native!((), 0x8A800DACCC0DA55D, native_parameters!());

    value
}

pub fn _0xbf371cd2b64212fd(p0: u32) -> () {
    let value = native!((), 0xBF371CD2B64212FD, native_parameters!(p0));

    value
}

pub fn _0x7d8ba05688ad64c7(p0: u32) -> () {
    let value = native!((), 0x7D8BA05688AD64C7, native_parameters!(p0));

    value
}

pub fn _0x0b565b0aae56a0e8(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32, p6: u32) -> () {
    let value = native!(
        (),
        0x0B565B0AAE56A0E8,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn _0x28ecb8ac2f607db2(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x28ECB8AC2F607DB2,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _playstats_change_mc_emblem(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x0A50D2604E05CB94,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _0xcc25a4553dfbf9ea(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0xCC25A4553DFBF9EA,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _0xf534d94dfa2ead26(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0xF534D94DFA2EAD26,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _0xd558bec0bba7e8d2(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0xD558BEC0BBA7E8D2,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _playstats_earned_mc_points(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0x501478855A6074CE,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _0x03c2eebb04b3fb72(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32, p6: u32) -> () {
    let value = native!(
        (),
        0x03C2EEBB04B3FB72,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn _0x8989cbd7b4e82534(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32, p6: u32) -> () {
    let value = native!(
        (),
        0x8989CBD7B4E82534,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn _0x27aa1c973cacfe63(
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
        0x27AA1C973CACFE63,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn _playstats_copy_rank_into_new_slot(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
) -> () {
    let value = native!(
        (),
        0xB7257BA2550EA10A,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn _playstats_dupe_detection(data: *mut u32) -> () {
    let value = native!((), 0x848B66100EE33B05, native_parameters!(data));

    value
}

pub fn _playstats_ban_alert(p0: i32) -> () {
    let value = native!((), 0x516FC96EB88EEFE5, native_parameters!(p0));

    value
}

pub fn _playstats_gunrun_mission_ended(data: *mut u32) -> () {
    let value = native!((), 0x0EACDF8487D5155A, native_parameters!(data));

    value
}

pub fn _0xdaf80797fc534bec(p0: u32) -> () {
    let value = native!((), 0xDAF80797FC534BEC, native_parameters!(p0));

    value
}

pub fn _0x316db59cd14c1774(p0: u32) -> () {
    let value = native!((), 0x316DB59CD14C1774, native_parameters!(p0));

    value
}

pub fn _0x2d7a9b577e72385e(p0: u32) -> () {
    let value = native!((), 0x2D7A9B577E72385E, native_parameters!(p0));

    value
}

pub fn _0x830c3a44eb3f2cf9(p0: u32) -> () {
    let value = native!((), 0x830C3A44EB3F2CF9, native_parameters!(p0));

    value
}

pub fn _0xb26f670685631727(p0: u32, p1: u32) -> () {
    let value = native!((), 0xB26F670685631727, native_parameters!(p0, p1));

    value
}

pub fn _0xc14bd9f5337219b2(p0: u32, p1: u32) -> () {
    let value = native!((), 0xC14BD9F5337219B2, native_parameters!(p0, p1));

    value
}

pub fn _playstats_stone_hatchet_end(data: *mut u32) -> () {
    let value = native!((), 0x35E39E5570358630, native_parameters!(data));

    value
}

pub fn _playstats_smug_mission_ended(data: *mut u32) -> () {
    let value = native!((), 0x320C35147D5B5DDD, native_parameters!(data));

    value
}

pub fn _playstats_h2_fmprep_end(data: *mut u32) -> () {
    let value = native!((), 0xD8AFB345A9C5CCBB, native_parameters!(data));

    value
}

pub fn _playstats_h2_instance_end(data: *mut u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x1E1497D0D2108115, native_parameters!(data, p1, p2, p3));

    value
}

pub fn _playstats_dar_mission_end(data: *mut u32) -> () {
    let value = native!((), 0x0BC254FF3A911501, native_parameters!(data));

    value
}

pub fn _playstats_enter_session_pack(data: *mut u32) -> () {
    let value = native!((), 0x878FF156D36E9956, native_parameters!(data));

    value
}

pub fn _playstats_drone_usage(p0: i32, p1: i32, p2: i32) -> () {
    let value = native!((), 0x66C7BB2416ED3FCE, native_parameters!(p0, p1, p2));

    value
}

pub fn _playstats_spectator_wheel_spin(p0: i32, p1: i32, p2: i32, p3: i32) -> () {
    let value = native!((), 0x6731DE84A38BFAD0, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _playstats_arena_war_spectator(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> () {
    let value = native!(
        (),
        0x6F4F599753F8200A,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _playstats_arena_wars_ended(data: *mut u32) -> () {
    let value = native!((), 0xB479D9F0D48A1BC5, native_parameters!(data));

    value
}

pub fn _playstats_passive_mode(p0: bool, p1: i32, p2: i32, p3: i32) -> () {
    let value = native!((), 0x35EEC6C2BC821A71, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _playstats_collectible(
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
        0xCD0A8A9338681CF2,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    );

    value
}

pub fn _playstats_casino_story_mission_ended(p0: u32, p1: u32) -> () {
    let value = native!((), 0xFCCCAC2BD3C1F180, native_parameters!(p0, p1));

    value
}

pub fn _playstats_casino_chip(p0: u32) -> () {
    let value = native!((), 0x0999F3F090EC5012, native_parameters!(p0));

    value
}

pub fn _playstats_casino_roulette(p0: u32) -> () {
    let value = native!((), 0x95101C443A84E7F1, native_parameters!(p0));

    value
}

pub fn _playstats_casino_blackjack(p0: u32) -> () {
    let value = native!((), 0x3EAE97309727E7AD, native_parameters!(p0));

    value
}

pub fn _playstats_casino_threecardpoker(p0: u32) -> () {
    let value = native!((), 0xF740FB339D471C35, native_parameters!(p0));

    value
}

pub fn _playstats_casino_slotmachine(p0: u32) -> () {
    let value = native!((), 0xEF5EC67D392B830A, native_parameters!(p0));

    value
}

pub fn _playstats_casino_insidetrack(p0: u32) -> () {
    let value = native!((), 0x049F059625058A86, native_parameters!(p0));

    value
}

pub fn _playstats_casino_luckyseven(p0: u32) -> () {
    let value = native!((), 0x0C432C1435F5E4FA, native_parameters!(p0));

    value
}

pub fn _playstats_casino_roulette_light(p0: u32) -> () {
    let value = native!((), 0x6572ABA3DE1197FC, native_parameters!(p0));

    value
}

pub fn _playstats_casino_blackjack_light(p0: u32) -> () {
    let value = native!((), 0xD5451C7BF151EB6F, native_parameters!(p0));

    value
}

pub fn _playstats_casino_threecardpoker_light(p0: u32) -> () {
    let value = native!((), 0xC9001364B4388F22, native_parameters!(p0));

    value
}

pub fn _playstats_casino_slotmachine_light(p0: u32) -> () {
    let value = native!((), 0xE60054A0FAE8227F, native_parameters!(p0));

    value
}

pub fn _playstats_casino_insidetrack_light(p0: u32) -> () {
    let value = native!((), 0x23A3CBCD50D54E47, native_parameters!(p0));

    value
}

pub fn _playstats_arcadegame(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0x533A7D1EA58DF958,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _0x4fcdbd3f0a813c25(p0: u32, p1: u32) -> () {
    let value = native!((), 0x4FCDBD3F0A813C25, native_parameters!(p0, p1));

    value
}

pub fn _playstats_casino_mission_ended(data: *mut u32) -> () {
    let value = native!((), 0x1A0D4A6C336B7BC5, native_parameters!(data));

    value
}

pub fn _0xdfbd93bf2943e29b(p0: u32) -> () {
    let value = native!((), 0xDFBD93BF2943E29B, native_parameters!(p0));

    value
}

pub fn _0x92fc0eedfac04a14(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0x92FC0EEDFAC04A14,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn _0x0077f15613d36993(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x0077F15613D36993, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0xf9096193df1f99d4(p0: u32) -> () {
    let value = native!((), 0xF9096193DF1F99D4, native_parameters!(p0));

    value
}

pub fn _0x2e0259babc27a327(p0: u32) -> () {
    let value = native!((), 0x2E0259BABC27A327, native_parameters!(p0));

    value
}

pub fn _0x53c31853ec9531ff(p0: u32) -> () {
    let value = native!((), 0x53C31853EC9531FF, native_parameters!(p0));

    value
}

pub fn _0x810b5fcc52ec7ff0(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x810B5FCC52EC7FF0, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x5bf29846c6527c54(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x5BF29846C6527C54,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _0xc03fab2c2f92289b(p0: u32) -> () {
    let value = native!((), 0xC03FAB2C2F92289B, native_parameters!(p0));

    value
}

pub fn _0x5cdaed54b34b0ed0(p0: u32) -> () {
    let value = native!((), 0x5CDAED54B34B0ED0, native_parameters!(p0));

    value
}

pub fn _0x4aff7e02e485e92b() -> () {
    let value = native!((), 0x4AFF7E02E485E92B, native_parameters!());

    value
}

pub fn _0xdfcdb14317a9b361(p0: u32) -> () {
    let value = native!((), 0xDFCDB14317A9B361, native_parameters!(p0));

    value
}

pub fn _0xc1e963c58664b556(p0: u32) -> () {
    let value = native!((), 0xC1E963C58664B556, native_parameters!(p0));

    value
}

pub fn _0x2fa3173480008493(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x2FA3173480008493,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _0xd4367d310f079db0(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0xD4367D310F079DB0, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x4dc416f246a41fc8(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> () {
    let value = native!(
        (),
        0x4DC416F246A41FC8,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn _0x2818ff6638cb09de(p0: u32) -> () {
    let value = native!((), 0x2818FF6638CB09DE, native_parameters!(p0));

    value
}

pub fn _0xd6ca58b3b53a0f22(p0: u32) -> () {
    let value = native!((), 0xD6CA58B3B53A0F22, native_parameters!(p0));

    value
}
