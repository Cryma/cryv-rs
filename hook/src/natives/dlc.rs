use crate::types::NativeVector3;

pub fn _0x241fca5b1aa14f75() -> bool {
    let value = native!(bool, 0x241FCA5B1AA14F75, native_parameters!());

    value
}

pub fn is_dlc_present(dlcHash: u32) -> bool {
    let value = native!(bool, 0x812595A0644CE1DE, native_parameters!(dlcHash));

    value
}

pub fn _0xf2e07819ef1a5289() -> bool {
    let value = native!(bool, 0xF2E07819EF1A5289, native_parameters!());

    value
}

pub fn _0x9489659372a81585() -> bool {
    let value = native!(bool, 0x9489659372A81585, native_parameters!());

    value
}

pub fn _0xa213b11dff526300() -> bool {
    let value = native!(bool, 0xA213B11DFF526300, native_parameters!());

    value
}

pub fn _get_extra_content_pack_has_been_installed() -> bool {
    let value = native!(bool, 0x8D30F648014A92B5, native_parameters!());

    value
}

pub fn get_is_loading_screen_active() -> bool {
    let value = native!(bool, 0x10D0A8F259E93EC9, native_parameters!());

    value
}

pub fn _0xc4637a6d03c24cc3() -> bool {
    let value = native!(bool, 0xC4637A6D03C24CC3, native_parameters!());

    value
}

pub fn has_cloud_requests_finished(p0: *mut bool, unused: u32) -> bool {
    let value = native!(bool, 0x46E2B844905BC5F0, native_parameters!(p0, unused));

    value
}

pub fn on_enter_sp() -> () {
    let value = native!((), 0xD7C10C4A637992C9, native_parameters!());

    value
}

pub fn on_enter_mp() -> () {
    let value = native!((), 0x0888C3502DBBEEF5, native_parameters!());

    value
}
