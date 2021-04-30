use crate::types::NativeVector3;

pub fn begin_text_command_busyspinner_on(string: &std::ffi::CString) -> () {
    let value = native!((), 0xABA17D7CE615ADBF, native_parameters!(string.as_ptr()));

    value
}

pub fn end_text_command_busyspinner_on(busySpinnerType: i32) -> () {
    let value = native!((), 0xBD12F8228410D9B4, native_parameters!(busySpinnerType));

    value
}

pub fn busyspinner_off() -> () {
    let value = native!((), 0x10D373323E5B9C0D, native_parameters!());

    value
}

pub fn preload_busyspinner() -> () {
    let value = native!((), 0xC65AB383CD91DF98, native_parameters!());

    value
}

pub fn busyspinner_is_on() -> bool {
    let value = native!(bool, 0xD422FCC5F239A915, native_parameters!());

    value
}

pub fn busyspinner_is_displaying() -> bool {
    let value = native!(bool, 0xB2A592B04648A9CB, native_parameters!());

    value
}

pub fn _0x9245e81072704b8a(p0: bool) -> () {
    let value = native!((), 0x9245E81072704B8A, native_parameters!(p0));

    value
}

pub fn _set_mouse_cursor_active_this_frame() -> () {
    let value = native!((), 0xAAE7CE1D63167423, native_parameters!());

    value
}

pub fn _set_mouse_cursor_sprite(spriteId: i32) -> () {
    let value = native!((), 0x8DB8CFFD58B62552, native_parameters!(spriteId));

    value
}

pub fn _set_mouse_cursor_visible_in_menus(toggle: bool) -> () {
    let value = native!((), 0x98215325A695E78A, native_parameters!(toggle));

    value
}

pub fn _0x3d9acb1eb139e702() -> u32 {
    let value = native!(u32, 0x3D9ACB1EB139E702, native_parameters!());

    value
}

pub fn _0x632b2940c67f4ea9(scaleformHandle: i32, p1: *mut u32, p2: *mut u32, p3: *mut u32) -> bool {
    let value = native!(
        bool,
        0x632B2940C67F4EA9,
        native_parameters!(scaleformHandle, p1, p2, p3)
    );

    value
}

pub fn thefeed_only_show_tooltips(toggle: bool) -> () {
    let value = native!((), 0x6F1554B0CC2089FA, native_parameters!(toggle));

    value
}

pub fn thefeed_set_scripted_menu_height(pos: f32) -> () {
    let value = native!((), 0x55598D21339CB998, native_parameters!(pos));

    value
}

pub fn _thefeed_disable_loading_screen_tips() -> () {
    let value = native!((), 0x32888337579A5970, native_parameters!());

    value
}

pub fn thefeed_hide_this_frame() -> () {
    let value = native!((), 0x25F87B30C382FCA7, native_parameters!());

    value
}

pub fn _thefeed_display_loading_screen_tips() -> () {
    let value = native!((), 0x15CFA549788D35EF, native_parameters!());

    value
}

pub fn thefeed_flush_queue() -> () {
    let value = native!((), 0xA8FDB297A8D25FBA, native_parameters!());

    value
}

pub fn thefeed_remove_item(notificationId: i32) -> () {
    let value = native!((), 0xBE4390CB40B3E627, native_parameters!(notificationId));

    value
}

pub fn thefeed_force_render_on() -> () {
    let value = native!((), 0xA13C11E1B5C06BFC, native_parameters!());

    value
}

pub fn thefeed_force_render_off() -> () {
    let value = native!((), 0x583049884A2EEE3C, native_parameters!());

    value
}

pub fn thefeed_pause() -> () {
    let value = native!((), 0xFDB423997FA30340, native_parameters!());

    value
}

pub fn thefeed_resume() -> () {
    let value = native!((), 0xE1CD1E48E025E661, native_parameters!());

    value
}

pub fn thefeed_is_paused() -> bool {
    let value = native!(bool, 0xA9CBFD40B3FA3010, native_parameters!());

    value
}

pub fn thefeed_sps_extend_widescreen_on() -> () {
    let value = native!((), 0xD4438C0564490E63, native_parameters!());

    value
}

pub fn thefeed_sps_extend_widescreen_off() -> () {
    let value = native!((), 0xB695E2CD0A2DA9EE, native_parameters!());

    value
}

pub fn thefeed_get_first_visible_delete_remaining() -> i32 {
    let value = native!(i32, 0x82352748437638CA, native_parameters!());

    value
}

pub fn thefeed_comment_teleport_pool_on() -> () {
    let value = native!((), 0x56C8B608CFD49854, native_parameters!());

    value
}

pub fn thefeed_comment_teleport_pool_off() -> () {
    let value = native!((), 0xADED7F5748ACAFE6, native_parameters!());

    value
}

pub fn _thefeed_set_next_post_background_color(hudColorIndex: i32) -> () {
    let value = native!((), 0x92F0DA1E27DB96DC, native_parameters!(hudColorIndex));

    value
}

pub fn _thefeed_set_animpostfx_color(red: i32, green: i32, blue: i32, alpha: i32) -> () {
    let value = native!(
        (),
        0x17430B918701C342,
        native_parameters!(red, green, blue, alpha)
    );

    value
}

pub fn _thefeed_set_animpostfx_count(count: i32) -> () {
    let value = native!((), 0x17AD8C9706BDD88A, native_parameters!(count));

    value
}

pub fn _thefeed_set_animpostfx_sound(toggle: bool) -> () {
    let value = native!((), 0x4A0C7C9BB10ABB36, native_parameters!(toggle));

    value
}

pub fn thefeed_reset_all_parameters() -> () {
    let value = native!((), 0xFDD85225B2DEA55E, native_parameters!());

    value
}

pub fn thefeed_freeze_next_post() -> () {
    let value = native!((), 0xFDEC055AB549E328, native_parameters!());

    value
}

pub fn thefeed_clear_frozen_post() -> () {
    let value = native!((), 0x80FE4F3AB4E1B62A, native_parameters!());

    value
}

pub fn _thefeed_set_flush_animpostfx(p0: bool) -> () {
    let value = native!((), 0xBAE4F9B97CD43B30, native_parameters!(p0));

    value
}

pub fn _thefeed_add_txd_ref(p0: *mut u32, p1: *mut u32, p2: *mut u32, p3: *mut u32) -> () {
    let value = native!((), 0x317EBA71D7543F52, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn begin_text_command_thefeed_post(text: &std::ffi::CString) -> () {
    let value = native!((), 0x202709F4C58A0424, native_parameters!(text.as_ptr()));

    value
}

pub fn end_text_command_thefeed_post_stats(
    statTitle: &std::ffi::CString,
    iconEnum: i32,
    stepVal: bool,
    barValue: i32,
    isImportant: bool,
    pictureTextureDict: &std::ffi::CString,
    pictureTextureName: &std::ffi::CString,
) -> i32 {
    let value = native!(
        i32,
        0x2B7E9A4EAAA93C89,
        native_parameters!(
            statTitle.as_ptr(),
            iconEnum,
            stepVal,
            barValue,
            isImportant,
            pictureTextureDict.as_ptr(),
            pictureTextureName.as_ptr()
        )
    );

    value
}

pub fn end_text_command_thefeed_post_messagetext(
    txdName: &std::ffi::CString,
    textureName: &std::ffi::CString,
    flash: bool,
    iconType: i32,
    sender: &std::ffi::CString,
    subject: &std::ffi::CString,
) -> i32 {
    let value = native!(
        i32,
        0x1CCD9A37359072CF,
        native_parameters!(
            txdName.as_ptr(),
            textureName.as_ptr(),
            flash,
            iconType,
            sender.as_ptr(),
            subject.as_ptr()
        )
    );

    value
}

pub fn _end_text_command_thefeed_post_messagetext_gxt_entry(
    txdName: &std::ffi::CString,
    textureName: &std::ffi::CString,
    flash: bool,
    iconType: i32,
    sender: &std::ffi::CString,
    subject: &std::ffi::CString,
) -> i32 {
    let value = native!(
        i32,
        0xC6F580E4C94926AC,
        native_parameters!(
            txdName.as_ptr(),
            textureName.as_ptr(),
            flash,
            iconType,
            sender.as_ptr(),
            subject.as_ptr()
        )
    );

    value
}

pub fn end_text_command_thefeed_post_messagetext_tu(
    txdName: &std::ffi::CString,
    textureName: &std::ffi::CString,
    flash: bool,
    iconType: i32,
    sender: &std::ffi::CString,
    subject: &std::ffi::CString,
    duration: f32,
) -> i32 {
    let value = native!(
        i32,
        0x1E6611149DB3DB6B,
        native_parameters!(
            txdName.as_ptr(),
            textureName.as_ptr(),
            flash,
            iconType,
            sender.as_ptr(),
            subject.as_ptr(),
            duration
        )
    );

    value
}

pub fn end_text_command_thefeed_post_messagetext_with_crew_tag(
    txdName: &std::ffi::CString,
    textureName: &std::ffi::CString,
    flash: bool,
    iconType: i32,
    sender: &std::ffi::CString,
    subject: &std::ffi::CString,
    duration: f32,
    clanTag: &std::ffi::CString,
) -> i32 {
    let value = native!(
        i32,
        0x5CBF7BADE20DB93E,
        native_parameters!(
            txdName.as_ptr(),
            textureName.as_ptr(),
            flash,
            iconType,
            sender.as_ptr(),
            subject.as_ptr(),
            duration,
            clanTag.as_ptr()
        )
    );

    value
}

pub fn end_text_command_thefeed_post_messagetext_with_crew_tag_and_additional_icon(
    txdName: &std::ffi::CString,
    textureName: &std::ffi::CString,
    flash: bool,
    iconType1: i32,
    sender: &std::ffi::CString,
    subject: &std::ffi::CString,
    duration: f32,
    clanTag: &std::ffi::CString,
    iconType2: i32,
    p9: i32,
) -> i32 {
    let value = native!(
        i32,
        0x531B84E7DA981FB6,
        native_parameters!(
            txdName.as_ptr(),
            textureName.as_ptr(),
            flash,
            iconType1,
            sender.as_ptr(),
            subject.as_ptr(),
            duration,
            clanTag.as_ptr(),
            iconType2,
            p9
        )
    );

    value
}

pub fn end_text_command_thefeed_post_ticker(blink: bool, p1: bool) -> i32 {
    let value = native!(i32, 0x2ED7843F8F801023, native_parameters!(blink, p1));

    value
}

pub fn end_text_command_thefeed_post_ticker_forced(blink: bool, p1: bool) -> i32 {
    let value = native!(i32, 0x44FA03975424A0EE, native_parameters!(blink, p1));

    value
}

pub fn end_text_command_thefeed_post_ticker_with_tokens(blink: bool, p1: bool) -> i32 {
    let value = native!(i32, 0x378E809BF61EC840, native_parameters!(blink, p1));

    value
}

pub fn end_text_command_thefeed_post_award(
    textureDict: &std::ffi::CString,
    textureName: &std::ffi::CString,
    rpBonus: i32,
    colorOverlay: i32,
    titleLabel: &std::ffi::CString,
) -> i32 {
    let value = native!(
        i32,
        0xAA295B6F28BD587D,
        native_parameters!(
            textureDict.as_ptr(),
            textureName.as_ptr(),
            rpBonus,
            colorOverlay,
            titleLabel.as_ptr()
        )
    );

    value
}

pub fn end_text_command_thefeed_post_crewtag(
    p0: bool,
    p1: bool,
    p2: *mut i32,
    p3: i32,
    isLeader: bool,
    unk0: bool,
    clanDesc: i32,
    R: i32,
    G: i32,
    B: i32,
) -> i32 {
    let value = native!(
        i32,
        0x97C9E4E7024A8F2C,
        native_parameters!(p0, p1, p2, p3, isLeader, unk0, clanDesc, R, G, B)
    );

    value
}

pub fn end_text_command_thefeed_post_crewtag_with_game_name(
    p0: bool,
    p1: bool,
    p2: *mut i32,
    p3: i32,
    isLeader: bool,
    unk0: bool,
    clanDesc: i32,
    playerName: &std::ffi::CString,
    R: i32,
    G: i32,
    B: i32,
) -> i32 {
    let value = native!(
        i32,
        0x137BC35589E34E1E,
        native_parameters!(
            p0,
            p1,
            p2,
            p3,
            isLeader,
            unk0,
            clanDesc,
            playerName.as_ptr(),
            R,
            G,
            B
        )
    );

    value
}

pub fn end_text_command_thefeed_post_unlock(p0: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(u32, 0x33EE12743CCD6343, native_parameters!(p0, p1, p2));

    value
}

pub fn end_text_command_thefeed_post_unlock_tu(p0: u32, p1: u32, p2: u32, p3: u32) -> u32 {
    let value = native!(u32, 0xC8F3AAF93D0600BF, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn end_text_command_thefeed_post_unlock_tu_with_color(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
) -> u32 {
    let value = native!(
        u32,
        0x7AE0589093A2E088,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn end_text_command_thefeed_post_mpticker(blink: bool, p1: bool) -> i32 {
    let value = native!(i32, 0xF020C96915705B3A, native_parameters!(blink, p1));

    value
}

pub fn end_text_command_thefeed_post_crew_rankup(
    p0: &std::ffi::CString,
    p1: &std::ffi::CString,
    p2: &std::ffi::CString,
    p3: bool,
    p4: bool,
) -> i32 {
    let value = native!(
        i32,
        0x8EFCCF6EC66D85E4,
        native_parameters!(p0.as_ptr(), p1.as_ptr(), p2.as_ptr(), p3, p4)
    );

    value
}

pub fn end_text_command_thefeed_post_versus_tu(
    p0: *mut u32,
    p1: *mut u32,
    p2: u32,
    p3: *mut u32,
    p4: *mut u32,
    p5: u32,
    p6: u32,
    p7: u32,
) -> u32 {
    let value = native!(
        u32,
        0xB6871B0555B02996,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7)
    );

    value
}

pub fn _end_text_command_thefeed_post_replay_icon(
    type_esc: i32,
    image: i32,
    text: &std::ffi::CString,
) -> i32 {
    let value = native!(
        i32,
        0xD202B92CBF1D816F,
        native_parameters!(type_esc, image, text.as_ptr())
    );

    value
}

pub fn _end_text_command_thefeed_post_replay_input(
    type_esc: i32,
    button: &std::ffi::CString,
    text: &std::ffi::CString,
) -> i32 {
    let value = native!(
        i32,
        0xDD6CB2CCE7C2735C,
        native_parameters!(type_esc, button.as_ptr(), text.as_ptr())
    );

    value
}

pub fn begin_text_command_print(GxtEntry: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xB87A37EEB7FAA67D,
        native_parameters!(GxtEntry.as_ptr())
    );

    value
}

pub fn end_text_command_print(duration: i32, drawImmediately: bool) -> () {
    let value = native!(
        (),
        0x9D77056A530643F6,
        native_parameters!(duration, drawImmediately)
    );

    value
}

pub fn begin_text_command_is_message_displayed(text: &std::ffi::CString) -> () {
    let value = native!((), 0x853648FD1063A213, native_parameters!(text.as_ptr()));

    value
}

pub fn end_text_command_is_message_displayed() -> bool {
    let value = native!(bool, 0x8A9BA1AB3E237613, native_parameters!());

    value
}

pub fn begin_text_command_display_text(text: &std::ffi::CString) -> () {
    let value = native!((), 0x25FBB336DF1804CB, native_parameters!(text.as_ptr()));

    value
}

pub fn end_text_command_display_text(x: f32, y: f32, p2: i32) -> () {
    let value = native!((), 0xCD015E5BB0D96A57, native_parameters!(x, y, p2));

    value
}

pub fn _begin_text_command_get_width(text: &std::ffi::CString) -> () {
    let value = native!((), 0x54CE8AC98E120CAB, native_parameters!(text.as_ptr()));

    value
}

pub fn _end_text_command_get_width(p0: bool) -> f32 {
    let value = native!(f32, 0x85F061DA64ED2F67, native_parameters!(p0));

    value
}

pub fn _begin_text_command_line_count(entry: &std::ffi::CString) -> () {
    let value = native!((), 0x521FB041D93DD0E4, native_parameters!(entry.as_ptr()));

    value
}

pub fn _end_text_command_line_count(x: f32, y: f32) -> i32 {
    let value = native!(i32, 0x9040DFB09BE75706, native_parameters!(x, y));

    value
}

pub fn begin_text_command_display_help(inputType: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x8509B634FBE7DA11,
        native_parameters!(inputType.as_ptr())
    );

    value
}

pub fn end_text_command_display_help(p0: i32, loop_esc: bool, beep: bool, shape: i32) -> () {
    let value = native!(
        (),
        0x238FFE5C7B0498A6,
        native_parameters!(p0, loop_esc, beep, shape)
    );

    value
}

pub fn begin_text_command_is_this_help_message_being_displayed(
    labelName: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0x0A24DA3A41B718F5,
        native_parameters!(labelName.as_ptr())
    );

    value
}

pub fn end_text_command_is_this_help_message_being_displayed(p0: i32) -> bool {
    let value = native!(bool, 0x10BDDBFC529428DD, native_parameters!(p0));

    value
}

pub fn begin_text_command_set_blip_name(textLabel: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xF9113A30DE5C6670,
        native_parameters!(textLabel.as_ptr())
    );

    value
}

pub fn end_text_command_set_blip_name(blip: i32) -> () {
    let value = native!((), 0xBC38B49BCB83BC9B, native_parameters!(blip));

    value
}

pub fn _begin_text_command_objective(p0: &std::ffi::CString) -> () {
    let value = native!((), 0x23D69E0465570028, native_parameters!(p0.as_ptr()));

    value
}

pub fn _end_text_command_objective(p0: bool) -> () {
    let value = native!((), 0xCFDBDF5AE59BA0F4, native_parameters!(p0));

    value
}

pub fn begin_text_command_clear_print(text: &std::ffi::CString) -> () {
    let value = native!((), 0xE124FA80A759019C, native_parameters!(text.as_ptr()));

    value
}

pub fn end_text_command_clear_print() -> () {
    let value = native!((), 0xFCC75460ABA29378, native_parameters!());

    value
}

pub fn begin_text_command_override_button_text(gxtEntry: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x8F9EE5687F8EECCD,
        native_parameters!(gxtEntry.as_ptr())
    );

    value
}

pub fn end_text_command_override_button_text(p0: i32) -> () {
    let value = native!((), 0xA86911979638106F, native_parameters!(p0));

    value
}

pub fn add_text_component_integer(value: i32) -> () {
    let value = native!((), 0x03B504CF259931BC, native_parameters!(value));

    value
}

pub fn add_text_component_float(value: f32, decimalPlaces: i32) -> () {
    let value = native!(
        (),
        0xE7DCB5B874BCD96E,
        native_parameters!(value, decimalPlaces)
    );

    value
}

pub fn add_text_component_substring_text_label(labelName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xC63CD5D2920ACBE7,
        native_parameters!(labelName.as_ptr())
    );

    value
}

pub fn add_text_component_substring_text_label_hash_key(gxtEntryHash: u32) -> () {
    let value = native!((), 0x17299B63C7683A2B, native_parameters!(gxtEntryHash));

    value
}

pub fn add_text_component_substring_blip_name(blip: i32) -> () {
    let value = native!((), 0x80EAD8E2E1D5D52E, native_parameters!(blip));

    value
}

pub fn add_text_component_substring_player_name(text: &std::ffi::CString) -> () {
    let value = native!((), 0x6C188BE134E074AA, native_parameters!(text.as_ptr()));

    value
}

pub fn add_text_component_substring_time(timestamp: i32, flags: i32) -> () {
    let value = native!((), 0x1115F16B8AB9E8BF, native_parameters!(timestamp, flags));

    value
}

pub fn add_text_component_formatted_integer(value: i32, commaSeparated: bool) -> () {
    let value = native!(
        (),
        0x0E4C749FF9DE9CC4,
        native_parameters!(value, commaSeparated)
    );

    value
}

pub fn add_text_component_substring_phone_number(p0: &std::ffi::CString, p1: i32) -> () {
    let value = native!((), 0x761B77454205A61D, native_parameters!(p0.as_ptr(), p1));

    value
}

pub fn add_text_component_substring_website(website: &std::ffi::CString) -> () {
    let value = native!((), 0x94CF4AC034C9C986, native_parameters!(website.as_ptr()));

    value
}

pub fn add_text_component_substring_keyboard_display(string: &std::ffi::CString) -> () {
    let value = native!((), 0x5F68520888E69014, native_parameters!(string.as_ptr()));

    value
}

pub fn set_colour_of_next_text_component(hudColor: i32) -> () {
    let value = native!((), 0x39BBF623FC803EAC, native_parameters!(hudColor));

    value
}

pub fn _get_text_substring(text: &std::ffi::CString, position: i32, length: i32) -> String {
    let value = native!(
        *const i8,
        0x169BD9382084C8C0,
        native_parameters!(text.as_ptr(), position, length)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _get_text_substring_safe(
    text: &std::ffi::CString,
    position: i32,
    length: i32,
    maxLength: i32,
) -> String {
    let value = native!(
        *const i8,
        0xB2798643312205C5,
        native_parameters!(text.as_ptr(), position, length, maxLength)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _get_text_substring_slice(
    text: &std::ffi::CString,
    startPosition: i32,
    endPosition: i32,
) -> String {
    let value = native!(
        *const i8,
        0xCE94AEBA5D82908A,
        native_parameters!(text.as_ptr(), startPosition, endPosition)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _get_label_text(labelName: &std::ffi::CString) -> String {
    let value = native!(
        *const i8,
        0x7B5280EBA9840C72,
        native_parameters!(labelName.as_ptr())
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn clear_prints() -> () {
    let value = native!((), 0xCC33FA791322B9D9, native_parameters!());

    value
}

pub fn clear_brief() -> () {
    let value = native!((), 0x9D292F73ADBD9313, native_parameters!());

    value
}

pub fn clear_all_help_messages() -> () {
    let value = native!((), 0x6178F68A87A4D3A0, native_parameters!());

    value
}

pub fn clear_this_print(p0: &std::ffi::CString) -> () {
    let value = native!((), 0xCF708001E1E536DD, native_parameters!(p0.as_ptr()));

    value
}

pub fn clear_small_prints() -> () {
    let value = native!((), 0x2CEA2839313C09AC, native_parameters!());

    value
}

pub fn does_text_block_exist(gxt: &std::ffi::CString) -> bool {
    let value = native!(bool, 0x1C7302E725259789, native_parameters!(gxt.as_ptr()));

    value
}

pub fn request_additional_text(gxt: &std::ffi::CString, slot: i32) -> () {
    let value = native!(
        (),
        0x71A78003C8E71424,
        native_parameters!(gxt.as_ptr(), slot)
    );

    value
}

pub fn request_additional_text_for_dlc(gxt: &std::ffi::CString, slot: i32) -> () {
    let value = native!(
        (),
        0x6009F9F1AE90D8A6,
        native_parameters!(gxt.as_ptr(), slot)
    );

    value
}

pub fn has_additional_text_loaded(slot: i32) -> bool {
    let value = native!(bool, 0x02245FE4BED318B8, native_parameters!(slot));

    value
}

pub fn clear_additional_text(p0: i32, p1: bool) -> () {
    let value = native!((), 0x2A179DF17CCF04CD, native_parameters!(p0, p1));

    value
}

pub fn is_streaming_additional_text(p0: i32) -> bool {
    let value = native!(bool, 0x8B6817B71B85EBF0, native_parameters!(p0));

    value
}

pub fn has_this_additional_text_loaded(gxt: &std::ffi::CString, slot: i32) -> bool {
    let value = native!(
        bool,
        0xADBF060E2B30C5BC,
        native_parameters!(gxt.as_ptr(), slot)
    );

    value
}

pub fn is_message_being_displayed() -> bool {
    let value = native!(bool, 0x7984C03AA5CC2F41, native_parameters!());

    value
}

pub fn does_text_label_exist(gxt: &std::ffi::CString) -> bool {
    let value = native!(bool, 0xAC09CA973C564252, native_parameters!(gxt.as_ptr()));

    value
}

pub fn _0x98c3cf913d895111(string: &std::ffi::CString, length: i32) -> String {
    let value = native!(
        *const i8,
        0x98C3CF913D895111,
        native_parameters!(string.as_ptr(), length)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_length_of_string_with_this_text_label(gxt: &std::ffi::CString) -> i32 {
    let value = native!(i32, 0x801BD273D3A23F74, native_parameters!(gxt.as_ptr()));

    value
}

pub fn get_length_of_literal_string(string: &std::ffi::CString) -> i32 {
    let value = native!(i32, 0xF030907CCBB8A9FD, native_parameters!(string.as_ptr()));

    value
}

pub fn get_length_of_literal_string_in_bytes(string: &std::ffi::CString) -> i32 {
    let value = native!(i32, 0x43E4111189E54F0E, native_parameters!(string.as_ptr()));

    value
}

pub fn get_street_name_from_hash_key(hash: u32) -> String {
    let value = native!(*const i8, 0xD0EF8A959B8A4CB9, native_parameters!(hash));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn is_hud_preference_switched_on() -> bool {
    let value = native!(bool, 0x1930DFA731813EC4, native_parameters!());

    value
}

pub fn is_radar_preference_switched_on() -> bool {
    let value = native!(bool, 0x9EB6522EA68F22FE, native_parameters!());

    value
}

pub fn is_subtitle_preference_switched_on() -> bool {
    let value = native!(bool, 0xAD6DACA4BA53E0A4, native_parameters!());

    value
}

pub fn display_hud(toggle: bool) -> () {
    let value = native!((), 0xA6294919E56FF02A, native_parameters!(toggle));

    value
}

pub fn _display_hud_when_dead_this_frame() -> () {
    let value = native!((), 0x7669F9E39DC17063, native_parameters!());

    value
}

pub fn display_hud_when_paused_this_frame() -> () {
    let value = native!((), 0x402F9ED62087E898, native_parameters!());

    value
}

pub fn display_radar(toggle: bool) -> () {
    let value = native!((), 0xA0EBB943C300E693, native_parameters!(toggle));

    value
}

pub fn _0xcd74233600c4ea6b(toggle: bool) -> () {
    let value = native!((), 0xCD74233600C4EA6B, native_parameters!(toggle));

    value
}

pub fn _0xc2d2ad9eaae265b8() -> bool {
    let value = native!(bool, 0xC2D2AD9EAAE265B8, native_parameters!());

    value
}

pub fn is_hud_hidden() -> bool {
    let value = native!(bool, 0xA86478C6958735C5, native_parameters!());

    value
}

pub fn is_radar_hidden() -> bool {
    let value = native!(bool, 0x157F93B036700462, native_parameters!());

    value
}

pub fn is_minimap_rendering() -> bool {
    let value = native!(bool, 0xAF754F20EB5CD51A, native_parameters!());

    value
}

pub fn _0x0c698d8f099174c7(p0: u32) -> () {
    let value = native!((), 0x0C698D8F099174C7, native_parameters!(p0));

    value
}

pub fn _0xe4c3b169876d33d7(p0: u32) -> () {
    let value = native!((), 0xE4C3B169876D33D7, native_parameters!(p0));

    value
}

pub fn _0xeb81a3dadd503187() -> () {
    let value = native!((), 0xEB81A3DADD503187, native_parameters!());

    value
}

pub fn set_blip_route(blip: i32, enabled: bool) -> () {
    let value = native!((), 0x4F7D8A9BFB0B43E9, native_parameters!(blip, enabled));

    value
}

pub fn _clear_all_blip_routes() -> () {
    let value = native!((), 0xD12882D3FF82BF11, native_parameters!());

    value
}

pub fn set_blip_route_colour(blip: i32, colour: i32) -> () {
    let value = native!((), 0x837155CD2F63DA09, native_parameters!(blip, colour));

    value
}

pub fn _0x2790f4b17d098e26(toggle: bool) -> () {
    let value = native!((), 0x2790F4B17D098E26, native_parameters!(toggle));

    value
}

pub fn _0x6cdd58146a436083(p0: u32) -> () {
    let value = native!((), 0x6CDD58146A436083, native_parameters!(p0));

    value
}

pub fn _0xd1942374085c8469(p0: u32) -> () {
    let value = native!((), 0xD1942374085C8469, native_parameters!(p0));

    value
}

pub fn add_next_message_to_previous_briefs(p0: bool) -> () {
    let value = native!((), 0x60296AF4BA14ABC5, native_parameters!(p0));

    value
}

pub fn _0x57d760d55f54e071(p0: i32) -> () {
    let value = native!((), 0x57D760D55F54E071, native_parameters!(p0));

    value
}

pub fn set_radar_zoom_precise(zoom: f32) -> () {
    let value = native!((), 0xBD12C5EEE184C337, native_parameters!(zoom));

    value
}

pub fn set_radar_zoom(zoomLevel: i32) -> () {
    let value = native!((), 0x096EF57A0C999BBA, native_parameters!(zoomLevel));

    value
}

pub fn set_radar_zoom_to_blip(blip: i32, zoom: f32) -> () {
    let value = native!((), 0xF98E4B3E56AFC7B1, native_parameters!(blip, zoom));

    value
}

pub fn set_radar_zoom_to_distance(zoom: f32) -> () {
    let value = native!((), 0xCB7CC0D58405AD41, native_parameters!(zoom));

    value
}

pub fn _0xd2049635deb9c375() -> () {
    let value = native!((), 0xD2049635DEB9C375, native_parameters!());

    value
}

pub fn get_hud_colour(
    hudColorIndex: i32,
    r: *mut i32,
    g: *mut i32,
    b: *mut i32,
    a: *mut i32,
) -> () {
    let value = native!(
        (),
        0x7C9C91AB74A0360F,
        native_parameters!(hudColorIndex, r, g, b, a)
    );

    value
}

pub fn set_script_variable_hud_colour(r: i32, g: i32, b: i32, a: i32) -> () {
    let value = native!((), 0xD68A5FF8A3A89874, native_parameters!(r, g, b, a));

    value
}

pub fn _set_script_variable_2_hud_colour(r: i32, g: i32, b: i32, a: i32) -> () {
    let value = native!((), 0x16A304E6CB2BFAB9, native_parameters!(r, g, b, a));

    value
}

pub fn replace_hud_colour(hudColorIndex: i32, hudColorIndex2: i32) -> () {
    let value = native!(
        (),
        0x1CCC708F0F850613,
        native_parameters!(hudColorIndex, hudColorIndex2)
    );

    value
}

pub fn replace_hud_colour_with_rgba(hudColorIndex: i32, r: i32, g: i32, b: i32, a: i32) -> () {
    let value = native!(
        (),
        0xF314CF4F0211894E,
        native_parameters!(hudColorIndex, r, g, b, a)
    );

    value
}

pub fn _set_ability_bar_visibility_in_multiplayer(visible: bool) -> () {
    let value = native!((), 0x1DFEDD15019315A9, native_parameters!(visible));

    value
}

pub fn _set_allow_ability_bar_in_multiplayer(toggle: bool) -> () {
    let value = native!((), 0x889329C80FE5963C, native_parameters!(toggle));

    value
}

pub fn flash_ability_bar(millisecondsToFlash: i32) -> () {
    let value = native!(
        (),
        0x02CFBA0C9E9275CE,
        native_parameters!(millisecondsToFlash)
    );

    value
}

pub fn set_ability_bar_value(p0: f32, p1: f32) -> () {
    let value = native!((), 0x9969599CCFF5D85E, native_parameters!(p0, p1));

    value
}

pub fn flash_wanted_display(p0: bool) -> () {
    let value = native!((), 0xA18AFB39081B6A1F, native_parameters!(p0));

    value
}

pub fn _0xba8d65c1c65702e5(toggle: bool) -> () {
    let value = native!((), 0xBA8D65C1C65702E5, native_parameters!(toggle));

    value
}

pub fn get_rendered_character_height(size: f32, font: i32) -> f32 {
    let value = native!(f32, 0xDB88A37483346780, native_parameters!(size, font));

    value
}

pub fn set_text_scale(scale: f32, size: f32) -> () {
    let value = native!((), 0x07C837F9A01C34C9, native_parameters!(scale, size));

    value
}

pub fn set_text_colour(red: i32, green: i32, blue: i32, alpha: i32) -> () {
    let value = native!(
        (),
        0xBE6B23FFA53FB442,
        native_parameters!(red, green, blue, alpha)
    );

    value
}

pub fn set_text_centre(align: bool) -> () {
    let value = native!((), 0xC02F4DBFB51D988B, native_parameters!(align));

    value
}

pub fn set_text_right_justify(toggle: bool) -> () {
    let value = native!((), 0x6B3C4650BC8BEE47, native_parameters!(toggle));

    value
}

pub fn set_text_justification(justifyType: i32) -> () {
    let value = native!((), 0x4E096588B13FFECA, native_parameters!(justifyType));

    value
}

pub fn set_text_wrap(start: f32, end: f32) -> () {
    let value = native!((), 0x63145D9C883A1A70, native_parameters!(start, end));

    value
}

pub fn set_text_leading(p0: i32) -> () {
    let value = native!((), 0xA50ABC31E3CDFAFF, native_parameters!(p0));

    value
}

pub fn set_text_proportional(p0: bool) -> () {
    let value = native!((), 0x038C1F517D7FDCF8, native_parameters!(p0));

    value
}

pub fn set_text_font(fontType: i32) -> () {
    let value = native!((), 0x66E0276CC5F6B9DA, native_parameters!(fontType));

    value
}

pub fn set_text_drop_shadow() -> () {
    let value = native!((), 0x1CA3E9EAC9D93E5E, native_parameters!());

    value
}

pub fn set_text_dropshadow(distance: i32, r: i32, g: i32, b: i32, a: i32) -> () {
    let value = native!(
        (),
        0x465C84BC39F1C351,
        native_parameters!(distance, r, g, b, a)
    );

    value
}

pub fn set_text_outline() -> () {
    let value = native!((), 0x2513DFB0FB8400FE, native_parameters!());

    value
}

pub fn set_text_edge(p0: i32, r: i32, g: i32, b: i32, a: i32) -> () {
    let value = native!((), 0x441603240D202FA6, native_parameters!(p0, r, g, b, a));

    value
}

pub fn set_text_render_id(renderId: i32) -> () {
    let value = native!((), 0x5F15302936E07111, native_parameters!(renderId));

    value
}

pub fn get_default_script_rendertarget_render_id() -> i32 {
    let value = native!(i32, 0x52F0982D7FD156B6, native_parameters!());

    value
}

pub fn register_named_rendertarget(name: &std::ffi::CString, p1: bool) -> bool {
    let value = native!(
        bool,
        0x57D9C12635E25CE3,
        native_parameters!(name.as_ptr(), p1)
    );

    value
}

pub fn is_named_rendertarget_registered(name: &std::ffi::CString) -> bool {
    let value = native!(bool, 0x78DCDC15C9F116B4, native_parameters!(name.as_ptr()));

    value
}

pub fn release_named_rendertarget(name: &std::ffi::CString) -> bool {
    let value = native!(bool, 0xE9F6FFE837354DD4, native_parameters!(name.as_ptr()));

    value
}

pub fn link_named_rendertarget(modelHash: u32) -> () {
    let value = native!((), 0xF6C09E276AEB3F2D, native_parameters!(modelHash));

    value
}

pub fn get_named_rendertarget_render_id(name: &std::ffi::CString) -> i32 {
    let value = native!(i32, 0x1A6478B61C6BDC3B, native_parameters!(name.as_ptr()));

    value
}

pub fn is_named_rendertarget_linked(modelHash: u32) -> bool {
    let value = native!(bool, 0x113750538FA31298, native_parameters!(modelHash));

    value
}

pub fn clear_help(toggle: bool) -> () {
    let value = native!((), 0x8DFCED7A656F8802, native_parameters!(toggle));

    value
}

pub fn is_help_message_on_screen() -> bool {
    let value = native!(bool, 0xDAD37F45428801AE, native_parameters!());

    value
}

pub fn _0x214cd562a939246a() -> bool {
    let value = native!(bool, 0x214CD562A939246A, native_parameters!());

    value
}

pub fn is_help_message_being_displayed() -> bool {
    let value = native!(bool, 0x4D79439A6B55AC67, native_parameters!());

    value
}

pub fn is_help_message_fading_out() -> bool {
    let value = native!(bool, 0x327EDEEEAC55C369, native_parameters!());

    value
}

pub fn _set_help_message_text_style(style: i32, hudColor: i32, alpha: i32, p3: i32, p4: i32) -> () {
    let value = native!(
        (),
        0xB9C362BABECDDC7A,
        native_parameters!(style, hudColor, alpha, p3, p4)
    );

    value
}

pub fn get_standard_blip_enum_id() -> bool {
    let value = native!(bool, 0x4A9923385BDB9DAD, native_parameters!());

    value
}

pub fn get_waypoint_blip_enum_id() -> i32 {
    let value = native!(i32, 0x186E5D252FA50E7D, native_parameters!());

    value
}

pub fn get_number_of_active_blips() -> i32 {
    let value = native!(i32, 0x9A3FF3DE163034E8, native_parameters!());

    value
}

pub fn get_next_blip_info_id(blipSprite: i32) -> i32 {
    let value = native!(i32, 0x14F96AA50D6FBEA7, native_parameters!(blipSprite));

    value
}

pub fn get_first_blip_info_id(blipSprite: i32) -> i32 {
    let value = native!(i32, 0x1BEDE233E6CD2A1F, native_parameters!(blipSprite));

    value
}

pub fn _get_closest_blip_of_type(blipSprite: i32) -> i32 {
    let value = native!(i32, 0xD484BF71050CA1EE, native_parameters!(blipSprite));

    value
}

pub fn get_blip_info_id_coord(blip: i32) -> NativeVector3 {
    let value = native!(NativeVector3, 0xFA7C7F0AADF25D09, native_parameters!(blip));

    value
}

pub fn get_blip_info_id_display(blip: i32) -> i32 {
    let value = native!(i32, 0x1E314167F701DC3B, native_parameters!(blip));

    value
}

pub fn get_blip_info_id_type(blip: i32) -> i32 {
    let value = native!(i32, 0xBE9B0959FFD0779B, native_parameters!(blip));

    value
}

pub fn get_blip_info_id_entity_index(blip: i32) -> i32 {
    let value = native!(i32, 0x4BA4E2553AFEDC2C, native_parameters!(blip));

    value
}

pub fn get_blip_info_id_pickup_index(blip: i32) -> i32 {
    let value = native!(i32, 0x9B6786E4C03DD382, native_parameters!(blip));

    value
}

pub fn get_blip_from_entity(entity: i32) -> i32 {
    let value = native!(i32, 0xBC8DBDCA2436F7E8, native_parameters!(entity));

    value
}

pub fn add_blip_for_radius(posX: f32, posY: f32, posZ: f32, radius: f32) -> i32 {
    let value = native!(
        i32,
        0x46818D79B1F7499A,
        native_parameters!(posX, posY, posZ, radius)
    );

    value
}

pub fn _add_blip_for_area(x: f32, y: f32, z: f32, width: f32, height: f32) -> i32 {
    let value = native!(
        i32,
        0xCE5D0E5E315DB238,
        native_parameters!(x, y, z, width, height)
    );

    value
}

pub fn add_blip_for_entity(entity: i32) -> i32 {
    let value = native!(i32, 0x5CDE92C702A8FCE7, native_parameters!(entity));

    value
}

pub fn add_blip_for_pickup(pickup: i32) -> i32 {
    let value = native!(i32, 0xBE339365C863BD36, native_parameters!(pickup));

    value
}

pub fn add_blip_for_coord(x: f32, y: f32, z: f32) -> i32 {
    let value = native!(i32, 0x5A039BB0BCA604B6, native_parameters!(x, y, z));

    value
}

pub fn trigger_sonar_blip(posX: f32, posY: f32, posZ: f32, radius: f32, p4: i32) -> () {
    let value = native!(
        (),
        0x72DD432F3CDFC0EE,
        native_parameters!(posX, posY, posZ, radius, p4)
    );

    value
}

pub fn allow_sonar_blips(toggle: bool) -> () {
    let value = native!((), 0x60734CC207C9833C, native_parameters!(toggle));

    value
}

pub fn set_blip_coords(blip: i32, posX: f32, posY: f32, posZ: f32) -> () {
    let value = native!(
        (),
        0xAE2AF67E9D9AF65D,
        native_parameters!(blip, posX, posY, posZ)
    );

    value
}

pub fn get_blip_coords(blip: i32) -> NativeVector3 {
    let value = native!(NativeVector3, 0x586AFE3FF72D996E, native_parameters!(blip));

    value
}

pub fn set_blip_sprite(blip: i32, spriteId: i32) -> () {
    let value = native!((), 0xDF735600A4696DAF, native_parameters!(blip, spriteId));

    value
}

pub fn get_blip_sprite(blip: i32) -> i32 {
    let value = native!(i32, 0x1FC877464A04FC4F, native_parameters!(blip));

    value
}

pub fn _0x9fcb3cbfb3ead69a(p0: i32, p1: f32) -> () {
    let value = native!((), 0x9FCB3CBFB3EAD69A, native_parameters!(p0, p1));

    value
}

pub fn _0xb7b873520c84c118() -> () {
    let value = native!((), 0xB7B873520C84C118, native_parameters!());

    value
}

pub fn set_blip_name_from_text_file(blip: i32, gxtEntry: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xEAA0FFE120D92784,
        native_parameters!(blip, gxtEntry.as_ptr())
    );

    value
}

pub fn set_blip_name_to_player_name(blip: i32, player: i32) -> () {
    let value = native!((), 0x127DE7B20C60A6A3, native_parameters!(blip, player));

    value
}

pub fn set_blip_alpha(blip: i32, alpha: i32) -> () {
    let value = native!((), 0x45FF974EEE1C8734, native_parameters!(blip, alpha));

    value
}

pub fn get_blip_alpha(blip: i32) -> i32 {
    let value = native!(i32, 0x970F608F0EE6C885, native_parameters!(blip));

    value
}

pub fn set_blip_fade(blip: i32, opacity: i32, duration: i32) -> () {
    let value = native!(
        (),
        0x2AEE8F8390D2298C,
        native_parameters!(blip, opacity, duration)
    );

    value
}

pub fn _0x2c173ae2bdb9385e(blip: i32) -> i32 {
    let value = native!(i32, 0x2C173AE2BDB9385E, native_parameters!(blip));

    value
}

pub fn set_blip_rotation(blip: i32, rotation: i32) -> () {
    let value = native!((), 0xF87683CDF73C3F6E, native_parameters!(blip, rotation));

    value
}

pub fn _set_blip_squared_rotation(blip: i32, heading: f32) -> () {
    let value = native!((), 0xA8B6AFDAC320AC87, native_parameters!(blip, heading));

    value
}

pub fn _0x003e92ba477f9d7f(blip: i32) -> i32 {
    let value = native!(i32, 0x003E92BA477F9D7F, native_parameters!(blip));

    value
}

pub fn set_blip_flash_timer(blip: i32, duration: i32) -> () {
    let value = native!((), 0xD3CD6FD297AE87CC, native_parameters!(blip, duration));

    value
}

pub fn set_blip_flash_interval(blip: i32, p1: u32) -> () {
    let value = native!((), 0xAA51DB313C010A7E, native_parameters!(blip, p1));

    value
}

pub fn set_blip_colour(blip: i32, color: i32) -> () {
    let value = native!((), 0x03D7FB09E75D6B7E, native_parameters!(blip, color));

    value
}

pub fn set_blip_secondary_colour(blip: i32, r: i32, g: i32, b: i32) -> () {
    let value = native!((), 0x14892474891E09EB, native_parameters!(blip, r, g, b));

    value
}

pub fn get_blip_colour(blip: i32) -> i32 {
    let value = native!(i32, 0xDF729E8D20CF7327, native_parameters!(blip));

    value
}

pub fn get_blip_hud_colour(blip: i32) -> i32 {
    let value = native!(i32, 0x729B5F1EFBC0AAEE, native_parameters!(blip));

    value
}

pub fn is_blip_short_range(blip: i32) -> bool {
    let value = native!(bool, 0xDA5F8727EB75B926, native_parameters!(blip));

    value
}

pub fn is_blip_on_minimap(blip: i32) -> bool {
    let value = native!(bool, 0xE41CA53051197A27, native_parameters!(blip));

    value
}

pub fn does_blip_have_gps_route(blip: i32) -> bool {
    let value = native!(bool, 0xDD2238F57B977751, native_parameters!(blip));

    value
}

pub fn set_blip_hidden_on_legend(blip: i32, toggle: bool) -> () {
    let value = native!((), 0x54318C915D27E4CE, native_parameters!(blip, toggle));

    value
}

pub fn set_blip_high_detail(blip: i32, toggle: bool) -> () {
    let value = native!((), 0xE2590BC29220CEBB, native_parameters!(blip, toggle));

    value
}

pub fn set_blip_as_mission_creator_blip(blip: i32, toggle: bool) -> () {
    let value = native!((), 0x24AC0137444F9FD5, native_parameters!(blip, toggle));

    value
}

pub fn is_mission_creator_blip(blip: i32) -> bool {
    let value = native!(bool, 0x26F49BF3381D933D, native_parameters!(blip));

    value
}

pub fn get_new_selected_mission_creator_blip() -> i32 {
    let value = native!(i32, 0x5C90988E7C8E1AF4, native_parameters!());

    value
}

pub fn is_hovering_over_mission_creator_blip() -> bool {
    let value = native!(bool, 0x4167EFE0527D706E, native_parameters!());

    value
}

pub fn show_start_mission_instructional_button(p0: bool) -> () {
    let value = native!((), 0xF1A6C18B35BCADE6, native_parameters!(p0));

    value
}

pub fn _0x2916a928514c9827() -> () {
    let value = native!((), 0x2916A928514C9827, native_parameters!());

    value
}

pub fn _0xb552929b85fc27ec(p0: u32, p1: u32) -> () {
    let value = native!((), 0xB552929B85FC27EC, native_parameters!(p0, p1));

    value
}

pub fn set_blip_flashes(blip: i32, toggle: bool) -> () {
    let value = native!((), 0xB14552383D39CE3E, native_parameters!(blip, toggle));

    value
}

pub fn set_blip_flashes_alternate(blip: i32, toggle: bool) -> () {
    let value = native!((), 0x2E8D9498C56DD0D1, native_parameters!(blip, toggle));

    value
}

pub fn is_blip_flashing(blip: i32) -> bool {
    let value = native!(bool, 0xA5E41FD83AD6CEF0, native_parameters!(blip));

    value
}

pub fn set_blip_as_short_range(blip: i32, toggle: bool) -> () {
    let value = native!((), 0xBE8BE4FE60E27B72, native_parameters!(blip, toggle));

    value
}

pub fn set_blip_scale(blip: i32, scale: f32) -> () {
    let value = native!((), 0xD38744167B2FA257, native_parameters!(blip, scale));

    value
}

pub fn _set_blip_scale_transformation(blip: i32, xScale: f32, yScale: f32) -> () {
    let value = native!(
        (),
        0xCD6524439909C979,
        native_parameters!(blip, xScale, yScale)
    );

    value
}

pub fn set_blip_priority(blip: i32, priority: i32) -> () {
    let value = native!((), 0xAE9FC9EF6A9FAC79, native_parameters!(blip, priority));

    value
}

pub fn set_blip_display(blip: i32, displayId: i32) -> () {
    let value = native!((), 0x9029B2F3DA924928, native_parameters!(blip, displayId));

    value
}

pub fn set_blip_category(blip: i32, index: i32) -> () {
    let value = native!((), 0x234CDD44D996FD9A, native_parameters!(blip, index));

    value
}

pub fn remove_blip(blip: *mut i32) -> () {
    let value = native!((), 0x86A652570E5F25DD, native_parameters!(blip));

    value
}

pub fn set_blip_as_friendly(blip: i32, toggle: bool) -> () {
    let value = native!((), 0x6F6F290102C02AB4, native_parameters!(blip, toggle));

    value
}

pub fn pulse_blip(blip: i32) -> () {
    let value = native!((), 0x742D6FD43115AF73, native_parameters!(blip));

    value
}

pub fn show_number_on_blip(blip: i32, number: i32) -> () {
    let value = native!((), 0xA3C0B359DCB848B6, native_parameters!(blip, number));

    value
}

pub fn hide_number_on_blip(blip: i32) -> () {
    let value = native!((), 0x532CFF637EF80148, native_parameters!(blip));

    value
}

pub fn show_height_on_blip(blip: i32, toggle: bool) -> () {
    let value = native!((), 0x75A16C3DA34F1245, native_parameters!(blip, toggle));

    value
}

pub fn show_tick_on_blip(blip: i32, toggle: bool) -> () {
    let value = native!((), 0x74513EA3E505181E, native_parameters!(blip, toggle));

    value
}

pub fn show_heading_indicator_on_blip(blip: i32, toggle: bool) -> () {
    let value = native!((), 0x5FBCA48327B914DF, native_parameters!(blip, toggle));

    value
}

pub fn show_outline_indicator_on_blip(blip: i32, toggle: bool) -> () {
    let value = native!((), 0xB81656BC81FE24D1, native_parameters!(blip, toggle));

    value
}

pub fn show_friend_indicator_on_blip(blip: i32, toggle: bool) -> () {
    let value = native!((), 0x23C3EB807312F01A, native_parameters!(blip, toggle));

    value
}

pub fn show_crew_indicator_on_blip(blip: i32, toggle: bool) -> () {
    let value = native!((), 0xDCFB5D4DB8BF367E, native_parameters!(blip, toggle));

    value
}

pub fn _set_blip_display_indicator_on_blip(blip: i32, toggle: bool) -> () {
    let value = native!((), 0xC4278F70131BAA6D, native_parameters!(blip, toggle));

    value
}

pub fn _0x4b5b620c9b59ed34(p0: u32, p1: u32) -> () {
    let value = native!((), 0x4B5B620C9B59ED34, native_parameters!(p0, p1));

    value
}

pub fn _0x2c9f302398e13141(p0: u32, p1: u32) -> () {
    let value = native!((), 0x2C9F302398E13141, native_parameters!(p0, p1));

    value
}

pub fn _set_blip_shrink(blip: i32, toggle: bool) -> () {
    let value = native!((), 0x2B6D467DAB714E8D, native_parameters!(blip, toggle));

    value
}

pub fn set_radius_blip_edge(blip: i32, toggle: bool) -> () {
    let value = native!((), 0x25615540D894B814, native_parameters!(blip, toggle));

    value
}

pub fn does_blip_exist(blip: i32) -> bool {
    let value = native!(bool, 0xA6DB27D19ECBB7DA, native_parameters!(blip));

    value
}

pub fn set_waypoint_off() -> () {
    let value = native!((), 0xA7E4E2D361C2627F, native_parameters!());

    value
}

pub fn _delete_waypoint() -> () {
    let value = native!((), 0xD8E694757BCEA8E9, native_parameters!());

    value
}

pub fn refresh_waypoint() -> () {
    let value = native!((), 0x81FA173F170560D1, native_parameters!());

    value
}

pub fn is_waypoint_active() -> bool {
    let value = native!(bool, 0x1DD1F58F493F1DA5, native_parameters!());

    value
}

pub fn set_new_waypoint(x: f32, y: f32) -> () {
    let value = native!((), 0xFE43368D2AA4F2FC, native_parameters!(x, y));

    value
}

pub fn set_blip_bright(blip: i32, toggle: bool) -> () {
    let value = native!((), 0xB203913733F27884, native_parameters!(blip, toggle));

    value
}

pub fn set_blip_show_cone(blip: i32, toggle: bool, p2: u32) -> () {
    let value = native!((), 0x13127EC3665E8EE1, native_parameters!(blip, toggle, p2));

    value
}

pub fn _0xc594b315edf2d4af(ped: i32) -> () {
    let value = native!((), 0xC594B315EDF2D4AF, native_parameters!(ped));

    value
}

pub fn _0xf83d0febe75e62c9(
    p0: u32,
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
    p7: u32,
    p8: u32,
) -> () {
    let value = native!(
        (),
        0xF83D0FEBE75E62C9,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn _0x35a3cd97b2c0a6d2(blip: i32) -> () {
    let value = native!((), 0x35A3CD97B2C0A6D2, native_parameters!(blip));

    value
}

pub fn _0x8410c5e0cd847b9d() -> () {
    let value = native!((), 0x8410C5E0CD847B9D, native_parameters!());

    value
}

pub fn set_minimap_component(componentId: i32, toggle: bool, overrideColor: i32) -> u32 {
    let value = native!(
        u32,
        0x75A9A10948D1DEA6,
        native_parameters!(componentId, toggle, overrideColor)
    );

    value
}

pub fn _set_minimap_sonar_enabled(toggle: bool) -> () {
    let value = native!((), 0x6B50FC8749632EC1, native_parameters!(toggle));

    value
}

pub fn _show_signin_ui() -> () {
    let value = native!((), 0x60E892BA4F5BDCA4, native_parameters!());

    value
}

pub fn get_main_player_blip_id() -> i32 {
    let value = native!(i32, 0xDCD4EC3F419D02FA, native_parameters!());

    value
}

pub fn _0x41350b4fc28e3941(p0: bool) -> () {
    let value = native!((), 0x41350B4FC28E3941, native_parameters!(p0));

    value
}

pub fn hide_loading_on_fade_this_frame() -> () {
    let value = native!((), 0x4B0311D3CDC4648F, native_parameters!());

    value
}

pub fn set_radar_as_interior_this_frame(interior: u32, x: f32, y: f32, z: i32, zoom: i32) -> () {
    let value = native!(
        (),
        0x59E727A1C9D3E31A,
        native_parameters!(interior, x, y, z, zoom)
    );

    value
}

pub fn _0x504dfe62a1692296(toggle: bool) -> () {
    let value = native!((), 0x504DFE62A1692296, native_parameters!(toggle));

    value
}

pub fn set_radar_as_exterior_this_frame() -> () {
    let value = native!((), 0xE81B7D2A3DAB2D81, native_parameters!());

    value
}

pub fn _set_player_blip_position_this_frame(x: f32, y: f32) -> () {
    let value = native!((), 0x77E2DD177910E1CF, native_parameters!(x, y));

    value
}

pub fn _0xa17784fca9548d15(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xA17784FCA9548D15, native_parameters!(p0, p1, p2));

    value
}

pub fn _is_minimap_in_interior() -> bool {
    let value = native!(bool, 0x9049FE339D5F6F6F, native_parameters!());

    value
}

pub fn hide_minimap_exterior_map_this_frame() -> () {
    let value = native!((), 0x5FBAE526203990C9, native_parameters!());

    value
}

pub fn hide_minimap_interior_map_this_frame() -> () {
    let value = native!((), 0x20FE7FDFEEAD38C0, native_parameters!());

    value
}

pub fn _set_toggle_minimap_heist_island(toggle: bool) -> () {
    let value = native!((), 0x5E1460624D194A38, native_parameters!(toggle));

    value
}

pub fn dont_tilt_minimap_this_frame() -> () {
    let value = native!((), 0x6D14BFDC33B34F55, native_parameters!());

    value
}

pub fn _0x55f5a5f07134de60() -> () {
    let value = native!((), 0x55F5A5F07134DE60, native_parameters!());

    value
}

pub fn set_widescreen_format(p0: u32) -> () {
    let value = native!((), 0xC3B07BA00A83B0F1, native_parameters!(p0));

    value
}

pub fn display_area_name(toggle: bool) -> () {
    let value = native!((), 0x276B6CE369C33678, native_parameters!(toggle));

    value
}

pub fn display_cash(toggle: bool) -> () {
    let value = native!((), 0x96DEC8D5430208B7, native_parameters!(toggle));

    value
}

pub fn _0x170f541e1cadd1de(p0: bool) -> () {
    let value = native!((), 0x170F541E1CADD1DE, native_parameters!(p0));

    value
}

pub fn _set_player_cash_change(cash: i32, bank: i32) -> () {
    let value = native!((), 0x0772DF77852C2E30, native_parameters!(cash, bank));

    value
}

pub fn display_ammo_this_frame(display: bool) -> () {
    let value = native!((), 0xA5E78BA2B1331C55, native_parameters!(display));

    value
}

pub fn display_sniper_scope_this_frame() -> () {
    let value = native!((), 0x73115226F4814E62, native_parameters!());

    value
}

pub fn hide_hud_and_radar_this_frame() -> () {
    let value = native!((), 0x719FF505F097FD20, native_parameters!());

    value
}

pub fn _0xe67c6dfd386ea5e7(p0: bool) -> () {
    let value = native!((), 0xE67C6DFD386EA5E7, native_parameters!(p0));

    value
}

pub fn set_multiplayer_wallet_cash() -> () {
    let value = native!((), 0xC2D15BEF167E27BC, native_parameters!());

    value
}

pub fn remove_multiplayer_wallet_cash() -> () {
    let value = native!((), 0x95CF81BD06EE1887, native_parameters!());

    value
}

pub fn set_multiplayer_bank_cash() -> () {
    let value = native!((), 0xDD21B55DF695CD0A, native_parameters!());

    value
}

pub fn remove_multiplayer_bank_cash() -> () {
    let value = native!((), 0xC7C6789AA1CFEDD0, native_parameters!());

    value
}

pub fn set_multiplayer_hud_cash(p0: i32, p1: i32) -> () {
    let value = native!((), 0xFD1D220394BCB824, native_parameters!(p0, p1));

    value
}

pub fn remove_multiplayer_hud_cash() -> () {
    let value = native!((), 0x968F270E39141ECA, native_parameters!());

    value
}

pub fn hide_help_text_this_frame() -> () {
    let value = native!((), 0xD46923FC481CA285, native_parameters!());

    value
}

pub fn _0x801879a9b4f4b2fb() -> bool {
    let value = native!(bool, 0x801879A9B4F4B2FB, native_parameters!());

    value
}

pub fn display_help_text_this_frame(message: &std::ffi::CString, p1: bool) -> () {
    let value = native!(
        (),
        0x960C9FF8F616E41C,
        native_parameters!(message.as_ptr(), p1)
    );

    value
}

pub fn hud_force_weapon_wheel(show: bool) -> () {
    let value = native!((), 0xEB354E5376BC81A7, native_parameters!(show));

    value
}

pub fn _hud_display_loading_screen_tips() -> () {
    let value = native!((), 0x488043841BBE156F, native_parameters!());

    value
}

pub fn _hud_weapon_wheel_ignore_selection() -> () {
    let value = native!((), 0x0AFC4AF510774B47, native_parameters!());

    value
}

pub fn _hud_weapon_wheel_get_selected_hash() -> u32 {
    let value = native!(u32, 0xA48931185F0536FE, native_parameters!());

    value
}

pub fn hud_set_weapon_wheel_top_slot(weaponHash: u32) -> () {
    let value = native!((), 0x72C1056D678BB7D8, native_parameters!(weaponHash));

    value
}

pub fn _hud_weapon_wheel_get_slot_hash(weaponTypeIndex: i32) -> u32 {
    let value = native!(u32, 0xA13E93403F26C812, native_parameters!(weaponTypeIndex));

    value
}

pub fn _hud_weapon_wheel_ignore_control_input(toggle: bool) -> () {
    let value = native!((), 0x14C9FDCC41F81F63, native_parameters!(toggle));

    value
}

pub fn set_gps_flags(p0: i32, p1: f32) -> () {
    let value = native!((), 0x5B440763A4C8D15B, native_parameters!(p0, p1));

    value
}

pub fn clear_gps_flags() -> () {
    let value = native!((), 0x21986729D6A3A830, native_parameters!());

    value
}

pub fn set_race_track_render(toggle: bool) -> () {
    let value = native!((), 0x1EAC5F91BCBC5073, native_parameters!(toggle));

    value
}

pub fn clear_gps_race_track() -> () {
    let value = native!((), 0x7AA5B4CE533C858B, native_parameters!());

    value
}

pub fn start_gps_custom_route(hudColor: i32, displayOnFoot: bool, followPlayer: bool) -> () {
    let value = native!(
        (),
        0xDB34E8D56FC13B08,
        native_parameters!(hudColor, displayOnFoot, followPlayer)
    );

    value
}

pub fn add_point_to_gps_custom_route(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x311438A071DD9B1A, native_parameters!(x, y, z));

    value
}

pub fn set_gps_custom_route_render(toggle: bool, radarThickness: i32, mapThickness: i32) -> () {
    let value = native!(
        (),
        0x900086F371220B6F,
        native_parameters!(toggle, radarThickness, mapThickness)
    );

    value
}

pub fn clear_gps_custom_route() -> () {
    let value = native!((), 0xE6DE0561D9232A64, native_parameters!());

    value
}

pub fn start_gps_multi_route(hudColor: i32, routeFromPlayer: bool, displayOnFoot: bool) -> () {
    let value = native!(
        (),
        0x3D3D15AF7BCAAF83,
        native_parameters!(hudColor, routeFromPlayer, displayOnFoot)
    );

    value
}

pub fn add_point_to_gps_multi_route(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0xA905192A6781C41B, native_parameters!(x, y, z));

    value
}

pub fn set_gps_multi_route_render(toggle: bool) -> () {
    let value = native!((), 0x3DDA37128DD1ACA8, native_parameters!(toggle));

    value
}

pub fn clear_gps_multi_route() -> () {
    let value = native!((), 0x67EEDEA1B9BAFD94, native_parameters!());

    value
}

pub fn clear_gps_player_waypoint() -> () {
    let value = native!((), 0xFF4FB7C8CDFA3DA7, native_parameters!());

    value
}

pub fn set_gps_flashes(toggle: bool) -> () {
    let value = native!((), 0x320D0E0D936A0E9B, native_parameters!(toggle));

    value
}

pub fn _0x7b21e0bb01e8224a(p0: u32) -> () {
    let value = native!((), 0x7B21E0BB01E8224A, native_parameters!(p0));

    value
}

pub fn flash_minimap_display() -> () {
    let value = native!((), 0xF2DD778C22B15BDA, native_parameters!());

    value
}

pub fn flash_minimap_display_with_color(hudColorIndex: i32) -> () {
    let value = native!((), 0x6B1DE27EE78E6A19, native_parameters!(hudColorIndex));

    value
}

pub fn toggle_stealth_radar(toggle: bool) -> () {
    let value = native!((), 0x6AFDFB93754950C7, native_parameters!(toggle));

    value
}

pub fn set_minimap_in_spectator_mode(toggle: bool, ped: i32) -> () {
    let value = native!((), 0x1A5CD7752DD28CD3, native_parameters!(toggle, ped));

    value
}

pub fn set_mission_name(p0: bool, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x5F28ECF5FC84772F,
        native_parameters!(p0, name.as_ptr())
    );

    value
}

pub fn _set_mission_name_2(p0: bool, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xE45087D85F468BC2,
        native_parameters!(p0, name.as_ptr())
    );

    value
}

pub fn _0x817b86108eb94e51(
    p0: bool,
    p1: *mut u32,
    p2: *mut u32,
    p3: *mut u32,
    p4: *mut u32,
    p5: *mut u32,
    p6: *mut u32,
    p7: *mut u32,
    p8: *mut u32,
) -> () {
    let value = native!(
        (),
        0x817B86108EB94E51,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn set_minimap_block_waypoint(toggle: bool) -> () {
    let value = native!((), 0x58FADDED207897DC, native_parameters!(toggle));

    value
}

pub fn set_minimap_in_prologue(toggle: bool) -> () {
    let value = native!((), 0x9133955F1A2DA957, native_parameters!(toggle));

    value
}

pub fn set_minimap_hide_fow(toggle: bool) -> () {
    let value = native!((), 0xF8DEE0A5600CBB93, native_parameters!(toggle));

    value
}

pub fn get_minimap_fow_discovery_ratio() -> f32 {
    let value = native!(f32, 0xE0130B41D3CF4574, native_parameters!());

    value
}

pub fn get_minimap_fow_coordinate_is_revealed(x: f32, y: f32, z: f32) -> bool {
    let value = native!(bool, 0x6E31B91145873922, native_parameters!(x, y, z));

    value
}

pub fn _0x62e849b7eb28e770(p0: bool) -> () {
    let value = native!((), 0x62E849B7EB28E770, native_parameters!(p0));

    value
}

pub fn set_minimap_fow_reveal_coordinate(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x0923DBF87DFF735E, native_parameters!(x, y, z));

    value
}

pub fn set_minimap_golf_course(hole: i32) -> () {
    let value = native!((), 0x71BDB63DBAF8DA59, native_parameters!(hole));

    value
}

pub fn set_minimap_golf_course_off() -> () {
    let value = native!((), 0x35EDD5B2E3FF01C0, native_parameters!());

    value
}

pub fn lock_minimap_angle(angle: i32) -> () {
    let value = native!((), 0x299FAEBB108AE05B, native_parameters!(angle));

    value
}

pub fn unlock_minimap_angle() -> () {
    let value = native!((), 0x8183455E16C42E3A, native_parameters!());

    value
}

pub fn lock_minimap_position(x: f32, y: f32) -> () {
    let value = native!((), 0x1279E861A329E73F, native_parameters!(x, y));

    value
}

pub fn unlock_minimap_position() -> () {
    let value = native!((), 0x3E93E06DB8EF1F30, native_parameters!());

    value
}

pub fn _set_minimap_altitude_indicator_level(altitude: f32, p1: bool, p2: u32) -> () {
    let value = native!((), 0xD201F3FF917A506D, native_parameters!(altitude, p1, p2));

    value
}

pub fn set_health_hud_display_values(health: i32, capacity: i32, wasAdded: bool) -> () {
    let value = native!(
        (),
        0x3F5CC444DCAAA8F2,
        native_parameters!(health, capacity, wasAdded)
    );

    value
}

pub fn set_max_health_hud_display(maximumValue: i32) -> () {
    let value = native!((), 0x975D66A0BC17064C, native_parameters!(maximumValue));

    value
}

pub fn set_max_armour_hud_display(maximumValue: i32) -> () {
    let value = native!((), 0x06A320535F5F0248, native_parameters!(maximumValue));

    value
}

pub fn set_bigmap_active(toggleBigMap: bool, showFullMap: bool) -> () {
    let value = native!(
        (),
        0x231C8F89D0539D8F,
        native_parameters!(toggleBigMap, showFullMap)
    );

    value
}

pub fn is_hud_component_active(id: i32) -> bool {
    let value = native!(bool, 0xBC4C9EA5391ECC0D, native_parameters!(id));

    value
}

pub fn is_scripted_hud_component_active(id: i32) -> bool {
    let value = native!(bool, 0xDD100EB17A94FF65, native_parameters!(id));

    value
}

pub fn hide_scripted_hud_component_this_frame(id: i32) -> () {
    let value = native!((), 0xE374C498D8BADC14, native_parameters!(id));

    value
}

pub fn _show_scripted_hud_component_this_frame(id: i32) -> () {
    let value = native!((), 0x4F38DCA127DAAEA2, native_parameters!(id));

    value
}

pub fn is_scripted_hud_component_hidden_this_frame(id: i32) -> bool {
    let value = native!(bool, 0x09C0403ED9A751C2, native_parameters!(id));

    value
}

pub fn hide_hud_component_this_frame(id: i32) -> () {
    let value = native!((), 0x6806C51AD12B83B8, native_parameters!(id));

    value
}

pub fn show_hud_component_this_frame(id: i32) -> () {
    let value = native!((), 0x0B4DF1FA60C0E664, native_parameters!(id));

    value
}

pub fn _hide_area_and_vehicle_name_this_frame() -> () {
    let value = native!((), 0xA4DEDE28B1814289, native_parameters!());

    value
}

pub fn reset_reticule_values() -> () {
    let value = native!((), 0x12782CE0A636E9F0, native_parameters!());

    value
}

pub fn reset_hud_component_values(id: i32) -> () {
    let value = native!((), 0x450930E616475D0D, native_parameters!(id));

    value
}

pub fn set_hud_component_position(id: i32, x: f32, y: f32) -> () {
    let value = native!((), 0xAABB1F56E2A17CED, native_parameters!(id, x, y));

    value
}

pub fn get_hud_component_position(id: i32) -> NativeVector3 {
    let value = native!(NativeVector3, 0x223CA69A8C4417FD, native_parameters!(id));

    value
}

pub fn clear_reminder_message() -> () {
    let value = native!((), 0xB57D8DD645CFA2CF, native_parameters!());

    value
}

pub fn get_hud_screen_position_from_world_position(
    worldX: f32,
    worldY: f32,
    worldZ: f32,
    screenX: *mut f32,
    screenY: *mut f32,
) -> bool {
    let value = native!(
        bool,
        0xF9904D11F1ACBEC3,
        native_parameters!(worldX, worldY, worldZ, screenX, screenY)
    );

    value
}

pub fn open_reportugc_menu() -> () {
    let value = native!((), 0x523A590C1A3CC0D3, native_parameters!());

    value
}

pub fn force_close_reportugc_menu() -> () {
    let value = native!((), 0xEE4C0E6DBC6F2C6F, native_parameters!());

    value
}

pub fn is_reportugc_menu_open() -> bool {
    let value = native!(bool, 0x9135584D09A3437E, native_parameters!());

    value
}

pub fn is_floating_help_text_on_screen(hudIndex: i32) -> bool {
    let value = native!(bool, 0x2432784ACA090DA4, native_parameters!(hudIndex));

    value
}

pub fn set_floating_help_text_screen_position(hudIndex: i32, x: f32, y: f32) -> () {
    let value = native!((), 0x7679CC1BCEBE3D4C, native_parameters!(hudIndex, x, y));

    value
}

pub fn set_floating_help_text_world_position(hudIndex: i32, x: f32, y: f32, z: f32) -> () {
    let value = native!(
        (),
        0x784BA7E0ECEB4178,
        native_parameters!(hudIndex, x, y, z)
    );

    value
}

pub fn set_floating_help_text_to_entity(
    hudIndex: i32,
    entity: i32,
    offsetX: f32,
    offsetY: f32,
) -> () {
    let value = native!(
        (),
        0xB094BC1DB4018240,
        native_parameters!(hudIndex, entity, offsetX, offsetY)
    );

    value
}

pub fn set_floating_help_text_style(
    hudIndex: i32,
    p1: i32,
    p2: i32,
    p3: i32,
    p4: i32,
    p5: i32,
) -> () {
    let value = native!(
        (),
        0x788E7FD431BD67F1,
        native_parameters!(hudIndex, p1, p2, p3, p4, p5)
    );

    value
}

pub fn clear_floating_help(hudIndex: i32, p1: bool) -> () {
    let value = native!((), 0x50085246ABD3FEFA, native_parameters!(hudIndex, p1));

    value
}

pub fn create_mp_gamer_tag_with_crew_color(
    player: i32,
    username: &std::ffi::CString,
    pointedClanTag: bool,
    isRockstarClan: bool,
    clanTag: &std::ffi::CString,
    clanFlag: i32,
    r: i32,
    g: i32,
    b: i32,
) -> () {
    let value = native!(
        (),
        0x6DD05E9D83EFA4C9,
        native_parameters!(
            player,
            username.as_ptr(),
            pointedClanTag,
            isRockstarClan,
            clanTag.as_ptr(),
            clanFlag,
            r,
            g,
            b
        )
    );

    value
}

pub fn is_mp_gamer_tag_movie_active() -> bool {
    let value = native!(bool, 0x6E0EB3EB47C8D7AA, native_parameters!());

    value
}

pub fn create_fake_mp_gamer_tag(
    ped: i32,
    username: &std::ffi::CString,
    pointedClanTag: bool,
    isRockstarClan: bool,
    clanTag: &std::ffi::CString,
    clanFlag: i32,
) -> i32 {
    let value = native!(
        i32,
        0xBFEFE3321A3F5015,
        native_parameters!(
            ped,
            username.as_ptr(),
            pointedClanTag,
            isRockstarClan,
            clanTag.as_ptr(),
            clanFlag
        )
    );

    value
}

pub fn remove_mp_gamer_tag(gamerTagId: i32) -> () {
    let value = native!((), 0x31698AA80E0223F8, native_parameters!(gamerTagId));

    value
}

pub fn is_mp_gamer_tag_active(gamerTagId: i32) -> bool {
    let value = native!(bool, 0x4E929E7A5796FD26, native_parameters!(gamerTagId));

    value
}

pub fn is_mp_gamer_tag_free(gamerTagId: i32) -> bool {
    let value = native!(bool, 0x595B5178E412E199, native_parameters!(gamerTagId));

    value
}

pub fn set_mp_gamer_tag_visibility(gamerTagId: i32, component: i32, toggle: bool, p3: u32) -> () {
    let value = native!(
        (),
        0x63BB75ABEDC1F6A0,
        native_parameters!(gamerTagId, component, toggle, p3)
    );

    value
}

pub fn _set_mp_gamer_tag_enabled(gamerTagId: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0xEE76FF7E6A0166B0,
        native_parameters!(gamerTagId, toggle)
    );

    value
}

pub fn _set_mp_gamer_tag_icons(gamerTagId: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0xA67F9C46D612B6F1,
        native_parameters!(gamerTagId, toggle)
    );

    value
}

pub fn _set_mp_gamer_health_bar_display(gamerTagId: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0xD29EC58C2F6B5014,
        native_parameters!(gamerTagId, toggle)
    );

    value
}

pub fn _set_mp_gamer_health_bar_max(gamerTagId: i32, value: i32, maximumValue: i32) -> () {
    let value = native!(
        (),
        0x1563FE35E9928E67,
        native_parameters!(gamerTagId, value, maximumValue)
    );

    value
}

pub fn set_mp_gamer_tag_colour(gamerTagId: i32, component: i32, hudColorIndex: i32) -> () {
    let value = native!(
        (),
        0x613ED644950626AE,
        native_parameters!(gamerTagId, component, hudColorIndex)
    );

    value
}

pub fn set_mp_gamer_tag_health_bar_colour(gamerTagId: i32, hudColorIndex: i32) -> () {
    let value = native!(
        (),
        0x3158C77A7E888AB4,
        native_parameters!(gamerTagId, hudColorIndex)
    );

    value
}

pub fn set_mp_gamer_tag_alpha(gamerTagId: i32, component: i32, alpha: i32) -> () {
    let value = native!(
        (),
        0xD48FE545CD46F857,
        native_parameters!(gamerTagId, component, alpha)
    );

    value
}

pub fn set_mp_gamer_tag_wanted_level(gamerTagId: i32, wantedlvl: i32) -> () {
    let value = native!(
        (),
        0xCF228E2AA03099C3,
        native_parameters!(gamerTagId, wantedlvl)
    );

    value
}

pub fn _set_mp_gamer_tag_unk(gamerTagId: i32, p1: i32) -> () {
    let value = native!((), 0x9C16459B2324B2CF, native_parameters!(gamerTagId, p1));

    value
}

pub fn set_mp_gamer_tag_name(gamerTagId: i32, string: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xDEA2B8283BAA3944,
        native_parameters!(gamerTagId, string.as_ptr())
    );

    value
}

pub fn _is_valid_mp_gamer_tag_movie(gamerTagId: i32) -> bool {
    let value = native!(bool, 0xEB709A36958ABE0D, native_parameters!(gamerTagId));

    value
}

pub fn set_mp_gamer_tag_big_text(gamerTagId: i32, string: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x7B7723747CCB55B6,
        native_parameters!(gamerTagId, string.as_ptr())
    );

    value
}

pub fn get_current_webpage_id() -> i32 {
    let value = native!(i32, 0x01A358D9128B7A86, native_parameters!());

    value
}

pub fn get_current_website_id() -> i32 {
    let value = native!(i32, 0x97D47996FC48CBAD, native_parameters!());

    value
}

pub fn get_global_actionscript_flag(flagIndex: i32) -> i32 {
    let value = native!(i32, 0xE3B05614DCE1D014, native_parameters!(flagIndex));

    value
}

pub fn reset_global_actionscript_flag(flagIndex: i32) -> () {
    let value = native!((), 0xB99C4E4D9499DF29, native_parameters!(flagIndex));

    value
}

pub fn _is_warning_message_active_2() -> bool {
    let value = native!(bool, 0xAF42195A42C63BBA, native_parameters!());

    value
}

pub fn set_warning_message(
    titleMsg: &std::ffi::CString,
    flags: i32,
    promptMsg: &std::ffi::CString,
    p3: bool,
    p4: i32,
    p5: &std::ffi::CString,
    p6: &std::ffi::CString,
    showBackground: bool,
    p8: u32,
) -> () {
    let value = native!(
        (),
        0x7B1776B3B53F8D74,
        native_parameters!(
            titleMsg.as_ptr(),
            flags,
            promptMsg.as_ptr(),
            p3,
            p4,
            p5.as_ptr(),
            p6.as_ptr(),
            showBackground,
            p8
        )
    );

    value
}

pub fn set_warning_message_with_header(
    entryHeader: &std::ffi::CString,
    entryLine1: &std::ffi::CString,
    instructionalKey: i32,
    entryLine2: &std::ffi::CString,
    p4: bool,
    p5: u32,
    showBackground: *mut u32,
    p7: *mut u32,
    p8: bool,
    p9: u32,
) -> () {
    let value = native!(
        (),
        0xDC38CC1E35B6A5D7,
        native_parameters!(
            entryHeader.as_ptr(),
            entryLine1.as_ptr(),
            instructionalKey,
            entryLine2.as_ptr(),
            p4,
            p5,
            showBackground,
            p7,
            p8,
            p9
        )
    );

    value
}

pub fn set_warning_message_with_header_and_substring_flags(
    entryHeader: &std::ffi::CString,
    entryLine1: &std::ffi::CString,
    instructionalKey: u32,
    entryLine2: &std::ffi::CString,
    p4: bool,
    p5: u32,
    p6: u32,
    p7: *mut u32,
    p8: *mut u32,
    p9: bool,
    p10: u32,
) -> () {
    let value = native!(
        (),
        0x701919482C74B5AB,
        native_parameters!(
            entryHeader.as_ptr(),
            entryLine1.as_ptr(),
            instructionalKey,
            entryLine2.as_ptr(),
            p4,
            p5,
            p6,
            p7,
            p8,
            p9,
            p10
        )
    );

    value
}

pub fn _set_warning_message_with_header_unk(
    entryHeader: &std::ffi::CString,
    entryLine1: &std::ffi::CString,
    flags: i32,
    entryLine2: &std::ffi::CString,
    p4: bool,
    p5: u32,
    p6: *mut u32,
    p7: *mut u32,
    showBg: bool,
    p9: u32,
    p10: u32,
) -> () {
    let value = native!(
        (),
        0x38B55259C2E078ED,
        native_parameters!(
            entryHeader.as_ptr(),
            entryLine1.as_ptr(),
            flags,
            entryLine2.as_ptr(),
            p4,
            p5,
            p6,
            p7,
            showBg,
            p9,
            p10
        )
    );

    value
}

pub fn _set_warning_message_with_alert(
    labelTitle: &std::ffi::CString,
    labelMessage: &std::ffi::CString,
    p2: i32,
    p3: i32,
    labelMessage2: &std::ffi::CString,
    p5: bool,
    p6: i32,
    p7: i32,
    p8: &std::ffi::CString,
    p9: &std::ffi::CString,
    background: bool,
    errorCode: i32,
) -> () {
    let value = native!(
        (),
        0x15803FEC3B9A872B,
        native_parameters!(
            labelTitle.as_ptr(),
            labelMessage.as_ptr(),
            p2,
            p3,
            labelMessage2.as_ptr(),
            p5,
            p6,
            p7,
            p8.as_ptr(),
            p9.as_ptr(),
            background,
            errorCode
        )
    );

    value
}

pub fn _get_warning_message_title_hash() -> u32 {
    let value = native!(u32, 0x81DF9ABA6C83DFF9, native_parameters!());

    value
}

pub fn _set_warning_message_list_row(
    index: i32,
    name: &std::ffi::CString,
    cash: i32,
    rp: i32,
    lvl: i32,
    colour: i32,
) -> bool {
    let value = native!(
        bool,
        0x0C5A80A9E096D529,
        native_parameters!(index, name.as_ptr(), cash, rp, lvl, colour)
    );

    value
}

pub fn _0xdaf87174be7454ff(p0: u32) -> bool {
    let value = native!(bool, 0xDAF87174BE7454FF, native_parameters!(p0));

    value
}

pub fn _remove_warning_message_list_items() -> () {
    let value = native!((), 0x6EF54AB721DC6242, native_parameters!());

    value
}

pub fn is_warning_message_active() -> bool {
    let value = native!(bool, 0xE18B138FABC53103, native_parameters!());

    value
}

pub fn clear_dynamic_pause_menu_error_message() -> () {
    let value = native!((), 0x7792424AA0EAC32E, native_parameters!());

    value
}

pub fn _race_gallery_fullscreen(toggle: bool) -> () {
    let value = native!((), 0x5354C5BA2EA868A4, native_parameters!(toggle));

    value
}

pub fn _race_gallery_next_blip_sprite(spriteId: i32) -> () {
    let value = native!((), 0x1EAE6DD17B7A5EFA, native_parameters!(spriteId));

    value
}

pub fn _race_gallery_add_blip(x: f32, y: f32, z: f32) -> u32 {
    let value = native!(u32, 0x551DF99658DB6EE8, native_parameters!(x, y, z));

    value
}

pub fn _clear_race_gallery_blips() -> () {
    let value = native!((), 0x2708FC083123F9FF, native_parameters!());

    value
}

pub fn force_sonar_blips_this_frame() -> u32 {
    let value = native!(u32, 0x1121BFA1A1A522A8, native_parameters!());

    value
}

pub fn _get_north_radar_blip() -> i32 {
    let value = native!(i32, 0x3F0CF9CB7E589B88, native_parameters!());

    value
}

pub fn display_player_name_tags_on_blips(toggle: bool) -> () {
    let value = native!((), 0x82CEDC33687E1F50, native_parameters!(toggle));

    value
}

pub fn _0x211c4ef450086857() -> () {
    let value = native!((), 0x211C4EF450086857, native_parameters!());

    value
}

pub fn _0xbf4f34a85ca2970c() -> () {
    let value = native!((), 0xBF4F34A85CA2970C, native_parameters!());

    value
}

pub fn activate_frontend_menu(menuhash: u32, togglePause: bool, component: i32) -> () {
    let value = native!(
        (),
        0xEF01D36B9C9D0C7B,
        native_parameters!(menuhash, togglePause, component)
    );

    value
}

pub fn restart_frontend_menu(menuHash: u32, p1: i32) -> () {
    let value = native!((), 0x10706DC6AD2D49C0, native_parameters!(menuHash, p1));

    value
}

pub fn get_current_frontend_menu_version() -> u32 {
    let value = native!(u32, 0x2309595AD6145265, native_parameters!());

    value
}

pub fn set_pause_menu_active(toggle: bool) -> () {
    let value = native!((), 0xDF47FC56C71569CF, native_parameters!(toggle));

    value
}

pub fn disable_frontend_this_frame() -> () {
    let value = native!((), 0x6D3465A73092F0E6, native_parameters!());

    value
}

pub fn suppress_frontend_rendering_this_frame() -> () {
    let value = native!((), 0xBA751764F0821256, native_parameters!());

    value
}

pub fn _allow_pause_menu_when_dead_this_frame() -> () {
    let value = native!((), 0xCC3FDDED67BCFC63, native_parameters!());

    value
}

pub fn set_frontend_active(active: bool) -> () {
    let value = native!((), 0x745711A75AB09277, native_parameters!(active));

    value
}

pub fn is_pause_menu_active() -> bool {
    let value = native!(bool, 0xB0034A223497FFCB, native_parameters!());

    value
}

pub fn _0x2f057596f2bd0061() -> bool {
    let value = native!(bool, 0x2F057596F2BD0061, native_parameters!());

    value
}

pub fn get_pause_menu_state() -> i32 {
    let value = native!(i32, 0x272ACD84970869C5, native_parameters!());

    value
}

pub fn _0x5bff36d6ed83e0ae() -> NativeVector3 {
    let value = native!(NativeVector3, 0x5BFF36D6ED83E0AE, native_parameters!());

    value
}

pub fn is_pause_menu_restarting() -> bool {
    let value = native!(bool, 0x1C491717107431C7, native_parameters!());

    value
}

pub fn _log_debug_info(p0: &std::ffi::CString) -> () {
    let value = native!((), 0x2162C446DFDF38FD, native_parameters!(p0.as_ptr()));

    value
}

pub fn _0x77f16b447824da6c(p0: u32) -> () {
    let value = native!((), 0x77F16B447824DA6C, native_parameters!(p0));

    value
}

pub fn _0xcdca26e80faecb8f() -> () {
    let value = native!((), 0xCDCA26E80FAECB8F, native_parameters!());

    value
}

pub fn _0x2de6c5e2e996f178(p0: u32) -> () {
    let value = native!((), 0x2DE6C5E2E996F178, native_parameters!(p0));

    value
}

pub fn pause_menu_activate_context(contextHash: u32) -> () {
    let value = native!((), 0xDD564BDD0472C936, native_parameters!(contextHash));

    value
}

pub fn pause_menu_deactivate_context(contextHash: u32) -> () {
    let value = native!((), 0x444D8CF241EC25C5, native_parameters!(contextHash));

    value
}

pub fn pause_menu_is_context_active(contextHash: u32) -> bool {
    let value = native!(bool, 0x84698AB38D0C6636, native_parameters!(contextHash));

    value
}

pub fn pause_menu_is_context_menu_active() -> u32 {
    let value = native!(u32, 0x2A25ADC48F87841F, native_parameters!());

    value
}

pub fn _0xde03620f8703a9df() -> u32 {
    let value = native!(u32, 0xDE03620F8703A9DF, native_parameters!());

    value
}

pub fn _0x359af31a4b52f5ed() -> u32 {
    let value = native!(u32, 0x359AF31A4B52F5ED, native_parameters!());

    value
}

pub fn _0x13c4b962653a5280() -> u32 {
    let value = native!(u32, 0x13C4B962653A5280, native_parameters!());

    value
}

pub fn _0xc8e1071177a23be5(p0: *mut u32, p1: *mut u32, p2: *mut u32) -> bool {
    let value = native!(bool, 0xC8E1071177A23BE5, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x4895bdea16e7c080(p0: i32) -> () {
    let value = native!((), 0x4895BDEA16E7C080, native_parameters!(p0));

    value
}

pub fn pause_menu_set_busy_spinner(p0: bool, p1: u32, p2: u32) -> () {
    let value = native!((), 0xC78E239AC5B2DDB9, native_parameters!(p0, p1, p2));

    value
}

pub fn _0xf06ebb91a81e09e3(p0: bool) -> () {
    let value = native!((), 0xF06EBB91A81E09E3, native_parameters!(p0));

    value
}

pub fn is_frontend_ready_for_control() -> bool {
    let value = native!(bool, 0x3BAB9A4E4F2FF5C7, native_parameters!());

    value
}

pub fn take_control_of_frontend() -> () {
    let value = native!((), 0xEC9264727EEC0F28, native_parameters!());

    value
}

pub fn release_control_of_frontend() -> () {
    let value = native!((), 0x14621BB1DF14E2B2, native_parameters!());

    value
}

pub fn _0x66e7cb63c97b7d20() -> u32 {
    let value = native!(u32, 0x66E7CB63C97B7D20, native_parameters!());

    value
}

pub fn _0x593feae1f73392d4() -> u32 {
    let value = native!(u32, 0x593FEAE1F73392D4, native_parameters!());

    value
}

pub fn is_navigating_menu_content() -> u32 {
    let value = native!(u32, 0x4E3CD0EF8A489541, native_parameters!());

    value
}

pub fn _0xf284ac67940c6812() -> u32 {
    let value = native!(u32, 0xF284AC67940C6812, native_parameters!());

    value
}

pub fn _0x2e22fefa0100275e() -> bool {
    let value = native!(bool, 0x2E22FEFA0100275E, native_parameters!());

    value
}

pub fn _0x0cf54f20de43879c(p0: u32) -> () {
    let value = native!((), 0x0CF54F20DE43879C, native_parameters!(p0));

    value
}

pub fn _get_pause_menu_selection(lastItemMenuId: *mut i32, selectedItemUniqueId: *mut i32) -> () {
    let value = native!(
        (),
        0x36C1451A88A09630,
        native_parameters!(lastItemMenuId, selectedItemUniqueId)
    );

    value
}

pub fn _get_pause_menu_selection_data(
    lastItemMenuId: *mut i32,
    selectedItemMenuId: *mut i32,
    selectedItemUniqueId: *mut i32,
) -> () {
    let value = native!(
        (),
        0x7E17BE53E1AAABAF,
        native_parameters!(lastItemMenuId, selectedItemMenuId, selectedItemUniqueId)
    );

    value
}

pub fn _0xa238192f33110615(p0: *mut i32, p1: *mut i32, p2: *mut i32) -> bool {
    let value = native!(bool, 0xA238192F33110615, native_parameters!(p0, p1, p2));

    value
}

pub fn get_menu_ped_int_stat(p0: u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0xEF4CED81CEBEDC6D, native_parameters!(p0, p1));

    value
}

pub fn _0xca6b2f7ce32ab653(p0: u32, p1: *mut u32, p2: u32) -> bool {
    let value = native!(bool, 0xCA6B2F7CE32AB653, native_parameters!(p0, p1, p2));

    value
}

pub fn get_menu_ped_masked_int_stat(p0: u32, p1: *mut u32, p2: u32, p3: u32) -> bool {
    let value = native!(bool, 0x90A6526CF0381030, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x24a49beaf468dc90(p0: u32, p1: *mut u32, p2: u32, p3: u32, p4: u32) -> bool {
    let value = native!(
        bool,
        0x24A49BEAF468DC90,
        native_parameters!(p0, p1, p2, p3, p4)
    );

    value
}

pub fn get_menu_ped_float_stat(p0: u32, p1: *mut f32) -> bool {
    let value = native!(bool, 0x5FBD7095FE7AE57F, native_parameters!(p0, p1));

    value
}

pub fn _0x8f08017f9d7c47bd(p0: u32, p1: *mut u32, p2: u32) -> bool {
    let value = native!(bool, 0x8F08017F9D7C47BD, native_parameters!(p0, p1, p2));

    value
}

pub fn get_menu_ped_bool_stat(p0: u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0x052991E59076E4E4, native_parameters!(p0, p1));

    value
}

pub fn clear_ped_in_pause_menu() -> () {
    let value = native!((), 0x5E62BE5DC58E9E06, native_parameters!());

    value
}

pub fn give_ped_to_pause_menu(ped: i32, p1: i32) -> () {
    let value = native!((), 0xAC0BFBDC3BE00E14, native_parameters!(ped, p1));

    value
}

pub fn set_pause_menu_ped_lighting(state: bool) -> () {
    let value = native!((), 0x3CA6050692BC61B0, native_parameters!(state));

    value
}

pub fn set_pause_menu_ped_sleep_state(state: bool) -> () {
    let value = native!((), 0xECF128344E9FF9F1, native_parameters!(state));

    value
}

pub fn open_online_policies_menu() -> () {
    let value = native!((), 0x805D7CBB36FD6C4C, native_parameters!());

    value
}

pub fn _0xf13fe2a80c05c561() -> bool {
    let value = native!(bool, 0xF13FE2A80C05C561, native_parameters!());

    value
}

pub fn is_online_policies_menu_active() -> bool {
    let value = native!(bool, 0x6F72CD94F7B5B68C, native_parameters!());

    value
}

pub fn open_social_club_menu() -> () {
    let value = native!((), 0x75D3691713C3B05A, native_parameters!());

    value
}

pub fn close_social_club_menu() -> () {
    let value = native!((), 0xD2B32BE3FC1626C6, native_parameters!());

    value
}

pub fn set_social_club_tour(name: &std::ffi::CString) -> () {
    let value = native!((), 0x9E778248D6685FE0, native_parameters!(name.as_ptr()));

    value
}

pub fn is_social_club_active() -> bool {
    let value = native!(bool, 0xC406BE343FC4B9AF, native_parameters!());

    value
}

pub fn _0x1185a8087587322c(p0: bool) -> () {
    let value = native!((), 0x1185A8087587322C, native_parameters!(p0));

    value
}

pub fn _force_close_text_input_box() -> () {
    let value = native!((), 0x8817605C2BA76200, native_parameters!());

    value
}

pub fn _0x577599cced639ca2(p0: u32) -> () {
    let value = native!((), 0x577599CCED639CA2, native_parameters!(p0));

    value
}

pub fn _override_multiplayer_chat_prefix(gxtEntryHash: u32) -> () {
    let value = native!((), 0x6A1738B4323FE2D9, native_parameters!(gxtEntryHash));

    value
}

pub fn _is_multiplayer_chat_active() -> bool {
    let value = native!(bool, 0xB118AF58B5F332A1, native_parameters!());

    value
}

pub fn _close_multiplayer_chat() -> () {
    let value = native!((), 0x1AC8F4AD40E22127, native_parameters!());

    value
}

pub fn _0x7c226d5346d4d10a(p0: u32) -> () {
    let value = native!((), 0x7C226D5346D4D10A, native_parameters!(p0));

    value
}

pub fn _override_multiplayer_chat_colour(p0: i32, hudColor: i32) -> () {
    let value = native!((), 0xF47E567B3630DD12, native_parameters!(p0, hudColor));

    value
}

pub fn _set_text_chat_unk(p0: bool) -> () {
    let value = native!((), 0x1DB21A44B09E8BA3, native_parameters!(p0));

    value
}

pub fn flag_player_context_in_tournament(toggle: bool) -> () {
    let value = native!((), 0xCEF214315D276FD1, native_parameters!(toggle));

    value
}

pub fn set_ped_has_ai_blip(ped: i32, hasCone: bool) -> () {
    let value = native!((), 0xD30C50DF888D58B5, native_parameters!(ped, hasCone));

    value
}

pub fn _set_ped_has_ai_blip_with_color(ped: i32, hasCone: bool, color: i32) -> () {
    let value = native!(
        (),
        0xB13DCB4C6FAAD238,
        native_parameters!(ped, hasCone, color)
    );

    value
}

pub fn does_ped_have_ai_blip(ped: i32) -> bool {
    let value = native!(bool, 0x15B8ECF844EE67ED, native_parameters!(ped));

    value
}

pub fn set_ped_ai_blip_gang_id(ped: i32, gangId: i32) -> () {
    let value = native!((), 0xE52B8E7F85D39A08, native_parameters!(ped, gangId));

    value
}

pub fn set_ped_ai_blip_has_cone(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x3EED80DFF7325CAA, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_ai_blip_forced_on(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x0C4BBF625CA98C4E, native_parameters!(ped, toggle));

    value
}

pub fn set_ped_ai_blip_notice_range(ped: i32, range: f32) -> () {
    let value = native!((), 0x97C65887D4B37FA9, native_parameters!(ped, range));

    value
}

pub fn _set_ped_ai_blip_sprite(ped: i32, spriteId: i32) -> () {
    let value = native!((), 0xFCFACD0DB9D7A57D, native_parameters!(ped, spriteId));

    value
}

pub fn _get_ai_blip_2(ped: i32) -> i32 {
    let value = native!(i32, 0x7CD934010E115C2C, native_parameters!(ped));

    value
}

pub fn _get_ai_blip(ped: i32) -> i32 {
    let value = native!(i32, 0x56176892826A4FE8, native_parameters!(ped));

    value
}

pub fn _has_director_mode_been_triggered() -> bool {
    let value = native!(bool, 0xA277800A9EAE340E, native_parameters!());

    value
}

pub fn _set_director_mode_clear_triggered_flag() -> () {
    let value = native!((), 0x2632482FD6B9AB87, native_parameters!());

    value
}

pub fn _set_player_is_in_director_mode(toggle: bool) -> () {
    let value = native!((), 0x808519373FD336A3, native_parameters!(toggle));

    value
}

pub fn _0x04655f9d075d0ae5(toggle: bool) -> () {
    let value = native!((), 0x04655F9D075D0AE5, native_parameters!(toggle));

    value
}

pub fn _0x243296a510b562b6() -> () {
    let value = native!((), 0x243296A510B562B6, native_parameters!());

    value
}
