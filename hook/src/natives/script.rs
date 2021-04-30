use crate::types::NativeVector3;

pub fn request_script(scriptName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x6EB5F71AA68F2E8E,
        native_parameters!(scriptName.as_ptr())
    );

    value
}

pub fn set_script_as_no_longer_needed(scriptName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xC90D2DCACD56184C,
        native_parameters!(scriptName.as_ptr())
    );

    value
}

pub fn has_script_loaded(scriptName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xE6CC9F3BA0FB9EF1,
        native_parameters!(scriptName.as_ptr())
    );

    value
}

pub fn does_script_exist(scriptName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xFC04745FBE67C19A,
        native_parameters!(scriptName.as_ptr())
    );

    value
}

pub fn request_script_with_name_hash(scriptHash: u32) -> () {
    let value = native!((), 0xD62A67D26D9653E6, native_parameters!(scriptHash));

    value
}

pub fn set_script_with_name_hash_as_no_longer_needed(scriptHash: u32) -> () {
    let value = native!((), 0xC5BC038960E9DB27, native_parameters!(scriptHash));

    value
}

pub fn has_script_with_name_hash_loaded(scriptHash: u32) -> bool {
    let value = native!(bool, 0x5F0F0C783EB16C04, native_parameters!(scriptHash));

    value
}

pub fn does_script_with_name_hash_exist(scriptHash: u32) -> bool {
    let value = native!(bool, 0xF86AA3C56BA31381, native_parameters!(scriptHash));

    value
}

pub fn terminate_thread(threadId: i32) -> () {
    let value = native!((), 0xC8B189ED9138BCD4, native_parameters!(threadId));

    value
}

pub fn is_thread_active(threadId: i32) -> bool {
    let value = native!(bool, 0x46E9AE36D8FA6417, native_parameters!(threadId));

    value
}

pub fn _get_name_of_thread(threadId: i32) -> String {
    let value = native!(*const i8, 0x05A42BA9FC8DA96B, native_parameters!(threadId));
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn script_thread_iterator_reset() -> () {
    let value = native!((), 0xDADFADA5A20143A8, native_parameters!());

    value
}

pub fn script_thread_iterator_get_next_thread_id() -> i32 {
    let value = native!(i32, 0x30B4FA1C82DD4B9F, native_parameters!());

    value
}

pub fn get_id_of_this_thread() -> i32 {
    let value = native!(i32, 0xC30338E8088E2E21, native_parameters!());

    value
}

pub fn terminate_this_thread() -> () {
    let value = native!((), 0x1090044AD1DA76FA, native_parameters!());

    value
}

pub fn _get_number_of_references_of_script_with_name_hash(scriptHash: u32) -> i32 {
    let value = native!(i32, 0x2C83A9DA6BFFC4F9, native_parameters!(scriptHash));

    value
}

pub fn get_this_script_name() -> String {
    let value = native!(*const i8, 0x442E0A7EDE4A738A, native_parameters!());
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_hash_of_this_script_name() -> u32 {
    let value = native!(u32, 0x8A1C8B1738FFE87E, native_parameters!());

    value
}

pub fn get_number_of_events(eventGroup: i32) -> i32 {
    let value = native!(i32, 0x5F92A689A06620AA, native_parameters!(eventGroup));

    value
}

pub fn get_event_exists(eventGroup: i32, eventIndex: i32) -> bool {
    let value = native!(
        bool,
        0x936E6168A9BCEDB5,
        native_parameters!(eventGroup, eventIndex)
    );

    value
}

pub fn get_event_at_index(eventGroup: i32, eventIndex: i32) -> i32 {
    let value = native!(
        i32,
        0xD8F66A3A60C62153,
        native_parameters!(eventGroup, eventIndex)
    );

    value
}

pub fn get_event_data(
    eventGroup: i32,
    eventIndex: i32,
    eventData: *mut u32,
    eventDataSize: i32,
) -> bool {
    let value = native!(
        bool,
        0x2902843FCD2B2D79,
        native_parameters!(eventGroup, eventIndex, eventData, eventDataSize)
    );

    value
}

pub fn trigger_script_event(
    eventGroup: i32,
    eventData: *mut u32,
    eventDataSize: i32,
    playerBits: i32,
) -> () {
    let value = native!(
        (),
        0x5AE99C571D5BBE5D,
        native_parameters!(eventGroup, eventData, eventDataSize, playerBits)
    );

    value
}

pub fn shutdown_loading_screen() -> () {
    let value = native!((), 0x078EBE9809CCD637, native_parameters!());

    value
}

pub fn set_no_loading_screen(toggle: bool) -> () {
    let value = native!((), 0x5262CC1995D07E09, native_parameters!(toggle));

    value
}

pub fn get_no_loading_screen() -> bool {
    let value = native!(bool, 0x18C1270EA7F199BC, native_parameters!());

    value
}

pub fn _0xb1577667c3708f9b() -> () {
    let value = native!((), 0xB1577667C3708F9B, native_parameters!());

    value
}

pub fn _0x836b62713e0534ca() -> bool {
    let value = native!(bool, 0x836B62713E0534CA, native_parameters!());

    value
}

pub fn _0x760910b49d2b98ea() -> () {
    let value = native!((), 0x760910B49D2B98EA, native_parameters!());

    value
}

pub fn bg_start_context_hash(contextHash: u32) -> () {
    let value = native!((), 0x75B18E49607874C7, native_parameters!(contextHash));

    value
}

pub fn bg_end_context_hash(contextHash: u32) -> () {
    let value = native!((), 0x107E5CC7CA942BC1, native_parameters!(contextHash));

    value
}

pub fn bg_start_context(contextName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x9D5A25BADB742ACD,
        native_parameters!(contextName.as_ptr())
    );

    value
}

pub fn bg_end_context(contextName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xDC2BACD920D0A0DD,
        native_parameters!(contextName.as_ptr())
    );

    value
}

pub fn _0x0f6f1ebbc4e1d5e6(scriptIndex: i32, p1: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x0F6F1EBBC4E1D5E6,
        native_parameters!(scriptIndex, p1.as_ptr())
    );

    value
}

pub fn _0x22e21fbcfc88c149(scriptIndex: i32, p1: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0x22E21FBCFC88C149,
        native_parameters!(scriptIndex, p1.as_ptr())
    );

    value
}

pub fn _0x829cd22e043a2577(p0: u32) -> i32 {
    let value = native!(i32, 0x829CD22E043A2577, native_parameters!(p0));

    value
}

pub fn _trigger_script_event_2(
    eventGroup: i32,
    eventData: *mut u32,
    eventDataSize: i32,
    playerBits: i32,
) -> () {
    let value = native!(
        (),
        0xA40CC53DF8E50837,
        native_parameters!(eventGroup, eventData, eventDataSize, playerBits)
    );

    value
}
