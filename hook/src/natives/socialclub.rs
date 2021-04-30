use crate::types::NativeVector3;

pub fn sc_inbox_get_total_num_messages() -> i32 {
    let value = native!(i32, 0x03A93FF1A2CA0864, native_parameters!());

    value
}

pub fn _sc_inbox_message_init(p0: i32) -> u32 {
    let value = native!(u32, 0xBB8EA16ECBC976C4, native_parameters!(p0));

    value
}

pub fn _is_sc_inbox_valid(p0: i32) -> bool {
    let value = native!(bool, 0x93028F1DB42BFD08, native_parameters!(p0));

    value
}

pub fn _sc_inbox_message_pop(p0: i32) -> bool {
    let value = native!(bool, 0x2C015348CF19CA1D, native_parameters!(p0));

    value
}

pub fn sc_inbox_message_get_data_int(p0: i32, context: &std::ffi::CString, out: *mut i32) -> bool {
    let value = native!(
        bool,
        0xA00EFE4082C4056E,
        native_parameters!(p0, context.as_ptr(), out)
    );

    value
}

pub fn _sc_inbox_message_get_data_bool(p0: i32, p1: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xFFE5C16F402D851D,
        native_parameters!(p0, p1.as_ptr())
    );

    value
}

pub fn sc_inbox_message_get_data_string(
    p0: i32,
    context: &std::ffi::CString,
    out: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x7572EF42FC6A9B6D,
        native_parameters!(p0, context.as_ptr(), out.as_ptr())
    );

    value
}

pub fn sc_inbox_message_do_apply(p0: i32) -> bool {
    let value = native!(bool, 0x9A2C8064B6C1E41A, native_parameters!(p0));

    value
}

pub fn _sc_inbox_message_get_string(p0: i32) -> String {
    let value = native!(*const i8, 0xF3E31D16CBDCB304, native_parameters!(p0));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _sc_inbox_message_push_gamer_to_event_recip_list(networkHandle: *mut i32) -> () {
    let value = native!((), 0xDA024BDBD600F44A, native_parameters!(networkHandle));

    value
}

pub fn _sc_inbox_message_send_ugc_stat_update_event(data: *mut u32) -> () {
    let value = native!((), 0xA68D3D229F4F3B06, native_parameters!(data));

    value
}

pub fn sc_inbox_message_get_ugcdata(p0: u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0x69D82604A1A5A254, native_parameters!(p0, p1));

    value
}

pub fn _sc_inbox_message_send_bounty_presence_event(data: *mut u32) -> bool {
    let value = native!(bool, 0x6AFD2CD753FEEF83, native_parameters!(data));

    value
}

pub fn _sc_inbox_message_get_bounty_data(index: i32, outData: *mut u32) -> bool {
    let value = native!(bool, 0x87E0052F08BD64E6, native_parameters!(index, outData));

    value
}

pub fn _sc_inbox_get_emails(offset: i32, limit: i32) -> () {
    let value = native!((), 0x040ADDCBAFA1018A, native_parameters!(offset, limit));

    value
}

pub fn _0x16da8172459434aa() -> u32 {
    let value = native!(u32, 0x16DA8172459434AA, native_parameters!());

    value
}

pub fn _0x7db18ca8cad5b098() -> u32 {
    let value = native!(u32, 0x7DB18CA8CAD5B098, native_parameters!());

    value
}

pub fn _0x4737980e8a283806(p0: i32, p1: *mut u32) -> bool {
    let value = native!(bool, 0x4737980E8A283806, native_parameters!(p0, p1));

    value
}

pub fn _0x44aca259d67651db(p0: *mut u32, p1: u32) -> () {
    let value = native!((), 0x44ACA259D67651DB, native_parameters!(p0, p1));

    value
}

pub fn sc_email_message_push_gamer_to_recip_list(networkHandle: *mut i32) -> () {
    let value = native!((), 0x2330C12A7A605D16, native_parameters!(networkHandle));

    value
}

pub fn sc_email_message_clear_recip_list() -> () {
    let value = native!((), 0x55DF6DB45179236E, native_parameters!());

    value
}

pub fn _0x116fb94dc4b79f17(p0: &std::ffi::CString) -> () {
    let value = native!((), 0x116FB94DC4B79F17, native_parameters!(p0.as_ptr()));

    value
}

pub fn _0x07dbd622d9533857(p0: u32) -> u32 {
    let value = native!(u32, 0x07DBD622D9533857, native_parameters!(p0));

    value
}

pub fn _set_handle_rockstar_message_via_script(toggle: bool) -> () {
    let value = native!((), 0xBFA0A56A817C6C7D, native_parameters!(toggle));

    value
}

pub fn _is_rockstar_message_ready_for_script() -> bool {
    let value = native!(bool, 0xBC1CC91205EC8D6E, native_parameters!());

    value
}

pub fn _rockstar_message_get_string() -> String {
    let value = native!(*const i8, 0xDF649C4E9AFDD788, native_parameters!());
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn sc_presence_attr_set_int(attrHash: u32, value: i32) -> bool {
    let value = native!(
        bool,
        0x1F1E9682483697C7,
        native_parameters!(attrHash, value)
    );

    value
}

pub fn sc_presence_attr_set_float(attrHash: u32, value: f32) -> bool {
    let value = native!(
        bool,
        0xC4C4575F62534A24,
        native_parameters!(attrHash, value)
    );

    value
}

pub fn sc_presence_attr_set_string(attrHash: u32, value: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x287F1F75D2803595,
        native_parameters!(attrHash, value.as_ptr())
    );

    value
}

pub fn _0x487912fd248efddf(p0: u32, p1: f32) -> bool {
    let value = native!(bool, 0x487912FD248EFDDF, native_parameters!(p0, p1));

    value
}

pub fn _0xc85a7127e7ad02aa() -> u32 {
    let value = native!(u32, 0xC85A7127E7AD02AA, native_parameters!());

    value
}

pub fn _0xa770c8eec6fb2ac5() -> u32 {
    let value = native!(u32, 0xA770C8EEC6FB2AC5, native_parameters!());

    value
}

pub fn _sc_get_is_profile_attribute_set(name: &std::ffi::CString) -> bool {
    let value = native!(bool, 0x8416FE4E4629D7D7, native_parameters!(name.as_ptr()));

    value
}

pub fn _0x7ffcbfee44ecfabf() -> u32 {
    let value = native!(u32, 0x7FFCBFEE44ECFABF, native_parameters!());

    value
}

pub fn _0x2d874d4ae612a65f() -> u32 {
    let value = native!(u32, 0x2D874D4AE612A65F, native_parameters!());

    value
}

pub fn sc_profanity_check_string(string: &std::ffi::CString, token: *mut i32) -> bool {
    let value = native!(
        bool,
        0x75632C5ECD7ED843,
        native_parameters!(string.as_ptr(), token)
    );

    value
}

pub fn _sc_profanity_check_ugc_string(string: &std::ffi::CString, token: *mut i32) -> bool {
    let value = native!(
        bool,
        0xEB2BF817463DFA28,
        native_parameters!(string.as_ptr(), token)
    );

    value
}

pub fn sc_profanity_get_check_is_valid(token: i32) -> bool {
    let value = native!(bool, 0x1753344C770358AE, native_parameters!(token));

    value
}

pub fn sc_profanity_get_check_is_pending(token: i32) -> bool {
    let value = native!(bool, 0x82E4A58BABC15AE7, native_parameters!(token));

    value
}

pub fn sc_profanity_get_string_passed(token: i32) -> bool {
    let value = native!(bool, 0x85535ACF97FC0969, native_parameters!(token));

    value
}

pub fn sc_profanity_get_string_status(token: i32) -> i32 {
    let value = native!(i32, 0x930DE22F07B1CCE3, native_parameters!(token));

    value
}

pub fn _0xf6baaaf762e1bf40(p0: &std::ffi::CString, p1: *mut i32) -> bool {
    let value = native!(
        bool,
        0xF6BAAAF762E1BF40,
        native_parameters!(p0.as_ptr(), p1)
    );

    value
}

pub fn _0xf22ca0fd74b80e7a(p0: u32) -> bool {
    let value = native!(bool, 0xF22CA0FD74B80E7A, native_parameters!(p0));

    value
}

pub fn _0x9237e334f6e43156(p0: u32) -> u32 {
    let value = native!(u32, 0x9237E334F6E43156, native_parameters!(p0));

    value
}

pub fn _0x700569dba175a77c(p0: u32) -> u32 {
    let value = native!(u32, 0x700569DBA175A77C, native_parameters!(p0));

    value
}

pub fn _0x1d4446a62d35b0d0(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x1D4446A62D35B0D0, native_parameters!(p0, p1));

    value
}

pub fn _0x2e89990ddff670c3(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x2E89990DDFF670C3, native_parameters!(p0, p1));

    value
}

pub fn _0xd0ee05fe193646ea(p0: *mut u32, p1: *mut u32, p2: *mut u32) -> bool {
    let value = native!(bool, 0xD0EE05FE193646EA, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x1989c6e6f67e76a8(p0: *mut u32, p1: *mut u32, p2: *mut u32) -> bool {
    let value = native!(bool, 0x1989C6E6F67E76A8, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x07c61676e5bb52cd(p0: u32) -> u32 {
    let value = native!(u32, 0x07C61676E5BB52CD, native_parameters!(p0));

    value
}

pub fn _0x8147fff6a718e1ad(p0: u32) -> u32 {
    let value = native!(u32, 0x8147FFF6A718E1AD, native_parameters!(p0));

    value
}

pub fn _0x0f73393bac7e6730(p0: *mut u32, p1: *mut i32) -> bool {
    let value = native!(bool, 0x0F73393BAC7E6730, native_parameters!(p0, p1));

    value
}

pub fn _0xd302e99edf0449cf(p0: u32) -> u32 {
    let value = native!(u32, 0xD302E99EDF0449CF, native_parameters!(p0));

    value
}

pub fn _0x5c4ebffa98bdb41c(p0: u32) -> u32 {
    let value = native!(u32, 0x5C4EBFFA98BDB41C, native_parameters!(p0));

    value
}

pub fn _0xff8f3a92b75ed67a() -> u32 {
    let value = native!(u32, 0xFF8F3A92B75ED67A, native_parameters!());

    value
}

pub fn _0x4ed9c8d6da297639() -> u32 {
    let value = native!(u32, 0x4ED9C8D6DA297639, native_parameters!());

    value
}

pub fn _0x710bcda8071eded1() -> u32 {
    let value = native!(u32, 0x710BCDA8071EDED1, native_parameters!());

    value
}

pub fn _0x50a8a36201dbf83e() -> u32 {
    let value = native!(u32, 0x50A8A36201DBF83E, native_parameters!());

    value
}

pub fn _0x9de5d2f723575ed0() -> u32 {
    let value = native!(u32, 0x9DE5D2F723575ED0, native_parameters!());

    value
}

pub fn _0xc2c97ea97711d1ae() -> u32 {
    let value = native!(u32, 0xC2C97EA97711D1AE, native_parameters!());

    value
}

pub fn _0x450819d8cf90c416() -> u32 {
    let value = native!(u32, 0x450819D8CF90C416, native_parameters!());

    value
}

pub fn _0x4a7d6e727f941747(p0: *mut u32) -> u32 {
    let value = native!(u32, 0x4A7D6E727F941747, native_parameters!(p0));

    value
}

pub fn _0xe75a4a2e5e316d86() -> u32 {
    let value = native!(u32, 0xE75A4A2E5E316D86, native_parameters!());

    value
}

pub fn _0x2570e26be63964e3() -> u32 {
    let value = native!(u32, 0x2570E26BE63964E3, native_parameters!());

    value
}

pub fn _0x1d12a56fc95be92e() -> u32 {
    let value = native!(u32, 0x1D12A56FC95BE92E, native_parameters!());

    value
}

pub fn _0x33df47cc0642061b() -> u32 {
    let value = native!(u32, 0x33DF47CC0642061B, native_parameters!());

    value
}

pub fn _0xa468e0be12b12c70() -> u32 {
    let value = native!(u32, 0xA468E0BE12B12C70, native_parameters!());

    value
}

pub fn _0x8cc469ab4d349b7c(p0: i32, p1: &std::ffi::CString, p2: *mut u32) -> bool {
    let value = native!(
        bool,
        0x8CC469AB4D349B7C,
        native_parameters!(p0, p1.as_ptr(), p2)
    );

    value
}

pub fn _0xc5a35c73b68f3c49() -> u32 {
    let value = native!(u32, 0xC5A35C73B68F3C49, native_parameters!());

    value
}

pub fn _0x699e4a5c8c893a18(p0: i32, p1: &std::ffi::CString, p2: *mut u32) -> bool {
    let value = native!(
        bool,
        0x699E4A5C8C893A18,
        native_parameters!(p0, p1.as_ptr(), p2)
    );

    value
}

pub fn _0x19853b5b17d77bca(p0: u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0x19853B5B17D77BCA, native_parameters!(p0, p1));

    value
}

pub fn _0x6bfb12ce158e3dd4(p0: u32) -> bool {
    let value = native!(bool, 0x6BFB12CE158E3DD4, native_parameters!(p0));

    value
}

pub fn _0xfe4c1d0d3b9cc17e(p0: u32, p1: u32) -> bool {
    let value = native!(bool, 0xFE4C1D0D3B9CC17E, native_parameters!(p0, p1));

    value
}

pub fn _0xd8122c407663b995() -> u32 {
    let value = native!(u32, 0xD8122C407663B995, native_parameters!());

    value
}

pub fn _0x3001bef2feca3680() -> bool {
    let value = native!(bool, 0x3001BEF2FECA3680, native_parameters!());

    value
}

pub fn _0x92da6e70ef249bd1(p0: &std::ffi::CString, p1: *mut i32) -> bool {
    let value = native!(
        bool,
        0x92DA6E70EF249BD1,
        native_parameters!(p0.as_ptr(), p1)
    );

    value
}

pub fn _0x675721c9f644d161() -> () {
    let value = native!((), 0x675721C9F644D161, native_parameters!());

    value
}

pub fn _0xe4f6e8d07a2f0f51(p0: u32) -> u32 {
    let value = native!(u32, 0xE4F6E8D07A2F0F51, native_parameters!(p0));

    value
}

pub fn _0x8a4416c0db05fa66(p0: u32) -> bool {
    let value = native!(bool, 0x8A4416C0DB05FA66, native_parameters!(p0));

    value
}

pub fn _0xea95c0853a27888e() -> () {
    let value = native!((), 0xEA95C0853A27888E, native_parameters!());

    value
}

pub fn _sc_get_nickname() -> String {
    let value = native!(*const i8, 0x198D161F458ECC7F, native_parameters!());
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn _0x225798743970412b(p0: *mut i32) -> bool {
    let value = native!(bool, 0x225798743970412B, native_parameters!(p0));

    value
}

pub fn _sc_get_has_achievement_been_passed(achievement: i32) -> bool {
    let value = native!(bool, 0x418DC16FAE452C1C, native_parameters!(achievement));

    value
}
