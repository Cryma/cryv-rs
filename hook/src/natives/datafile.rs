use crate::types::NativeVector3;

pub fn datafile_watch_request_id(id: i32) -> () {
    let value = native!((), 0xAD6875BBC0FC899C, native_parameters!(id));

    value
}

pub fn datafile_clear_watch_list() -> () {
    let value = native!((), 0x6CC86E78358D5119, native_parameters!());

    value
}

pub fn datafile_is_valid_request_id(index: i32) -> bool {
    let value = native!(bool, 0xFCCAE5B92A830878, native_parameters!(index));

    value
}

pub fn datafile_has_loaded_file_data(p0: u32) -> bool {
    let value = native!(bool, 0x15FF52B809DB2353, native_parameters!(p0));

    value
}

pub fn datafile_has_valid_file_data(p0: u32) -> bool {
    let value = native!(bool, 0xF8CC1EBE0B62E29F, native_parameters!(p0));

    value
}

pub fn datafile_select_active_file(p0: u32) -> bool {
    let value = native!(bool, 0x22DA66936E0FFF37, native_parameters!(p0));

    value
}

pub fn datafile_delete_requested_file(p0: u32) -> bool {
    let value = native!(bool, 0x8F5EA1C01D65A100, native_parameters!(p0));

    value
}

pub fn ugc_create_content(
    data: *mut u32,
    dataCount: i32,
    contentName: &std::ffi::CString,
    description: &std::ffi::CString,
    tagsCsv: &std::ffi::CString,
    contentTypeName: &std::ffi::CString,
    publish: bool,
) -> bool {
    let value = native!(
        bool,
        0xC84527E235FCA219,
        native_parameters!(
            data,
            dataCount,
            contentName.as_ptr(),
            description.as_ptr(),
            tagsCsv.as_ptr(),
            contentTypeName.as_ptr(),
            publish
        )
    );

    value
}

pub fn ugc_create_mission(
    contentName: &std::ffi::CString,
    description: &std::ffi::CString,
    tagsCsv: &std::ffi::CString,
    contentTypeName: &std::ffi::CString,
    publish: bool,
) -> bool {
    let value = native!(
        bool,
        0xA5EFC3E847D60507,
        native_parameters!(
            contentName.as_ptr(),
            description.as_ptr(),
            tagsCsv.as_ptr(),
            contentTypeName.as_ptr(),
            publish
        )
    );

    value
}

pub fn ugc_update_content(
    contentId: &std::ffi::CString,
    data: *mut u32,
    dataCount: i32,
    contentName: &std::ffi::CString,
    description: &std::ffi::CString,
    tagsCsv: &std::ffi::CString,
    contentTypeName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x648E7A5434AF7969,
        native_parameters!(
            contentId.as_ptr(),
            data,
            dataCount,
            contentName.as_ptr(),
            description.as_ptr(),
            tagsCsv.as_ptr(),
            contentTypeName.as_ptr()
        )
    );

    value
}

pub fn ugc_update_mission(
    contentId: &std::ffi::CString,
    contentName: &std::ffi::CString,
    description: &std::ffi::CString,
    tagsCsv: &std::ffi::CString,
    contentTypeName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x4645DE9980999E93,
        native_parameters!(
            contentId.as_ptr(),
            contentName.as_ptr(),
            description.as_ptr(),
            tagsCsv.as_ptr(),
            contentTypeName.as_ptr()
        )
    );

    value
}

pub fn ugc_set_player_data(
    contentId: &std::ffi::CString,
    rating: f32,
    contentTypeName: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x692D808C34A82143,
        native_parameters!(contentId.as_ptr(), rating, contentTypeName.as_ptr())
    );

    value
}

pub fn datafile_select_ugc_data(p0: i32) -> bool {
    let value = native!(bool, 0xA69AC4ADE82B57A4, native_parameters!(p0));

    value
}

pub fn datafile_select_ugc_stats(p0: i32, p1: bool) -> bool {
    let value = native!(bool, 0x9CB0BFA7A9342C3D, native_parameters!(p0, p1));

    value
}

pub fn datafile_select_ugc_player_data(p0: i32) -> bool {
    let value = native!(bool, 0x52818819057F2B40, native_parameters!(p0));

    value
}

pub fn datafile_select_creator_stats(p0: i32) -> bool {
    let value = native!(bool, 0x01095C95CD46B624, native_parameters!(p0));

    value
}

pub fn datafile_load_offline_ugc(filename: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0xC5238C011AF405E4,
        native_parameters!(filename.as_ptr())
    );

    value
}

pub fn datafile_create() -> () {
    let value = native!((), 0xD27058A1CA2B13EE, native_parameters!());

    value
}

pub fn datafile_delete() -> () {
    let value = native!((), 0x9AB9C1CFC8862DFB, native_parameters!());

    value
}

pub fn datafile_store_mission_header() -> () {
    let value = native!((), 0x2ED61456317B8178, native_parameters!());

    value
}

pub fn datafile_flush_mission_header() -> () {
    let value = native!((), 0xC55854C7D7274882, native_parameters!());

    value
}

pub fn datafile_get_file_dict() -> String {
    let value = native!(*const i8, 0x906B778CA1DC72B6, native_parameters!());
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn datafile_start_save_to_cloud(filename: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x83BCCE3224735F05,
        native_parameters!(filename.as_ptr())
    );

    value
}

pub fn datafile_update_save_to_cloud(p0: *mut bool) -> bool {
    let value = native!(bool, 0x4DFDD9EB705F8140, native_parameters!(p0));

    value
}

pub fn datafile_is_save_pending() -> bool {
    let value = native!(bool, 0xBEDB96A7584AA8CF, native_parameters!());

    value
}

pub fn _0xa6eef01087181edd(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0xA6EEF01087181EDD, native_parameters!(p0, p1));

    value
}

pub fn _0x6ad0bd5e087866cb(p0: u32) -> () {
    let value = native!((), 0x6AD0BD5E087866CB, native_parameters!(p0));

    value
}

pub fn _0xdbf860cf1db8e599(p0: u32) -> u32 {
    let value = native!(u32, 0xDBF860CF1DB8E599, native_parameters!(p0));

    value
}

pub fn datadict_set_bool(objectData: *mut u32, key: &std::ffi::CString, value: bool) -> () {
    let value = native!(
        (),
        0x35124302A556A325,
        native_parameters!(objectData, key.as_ptr(), value)
    );

    value
}

pub fn datadict_set_int(objectData: *mut u32, key: &std::ffi::CString, value: i32) -> () {
    let value = native!(
        (),
        0xE7E035450A7948D5,
        native_parameters!(objectData, key.as_ptr(), value)
    );

    value
}

pub fn datadict_set_float(objectData: *mut u32, key: &std::ffi::CString, value: f32) -> () {
    let value = native!(
        (),
        0xC27E1CC2D795105E,
        native_parameters!(objectData, key.as_ptr(), value)
    );

    value
}

pub fn datadict_set_string(
    objectData: *mut u32,
    key: &std::ffi::CString,
    value: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0x8FF3847DADD8E30C,
        native_parameters!(objectData, key.as_ptr(), value.as_ptr())
    );

    value
}

pub fn datadict_set_vector(
    objectData: *mut u32,
    key: &std::ffi::CString,
    valueX: f32,
    valueY: f32,
    valueZ: f32,
) -> () {
    let value = native!(
        (),
        0x4CD49B76338C7DEE,
        native_parameters!(objectData, key.as_ptr(), valueX, valueY, valueZ)
    );

    value
}

pub fn datadict_create_dict(objectData: *mut u32, key: &std::ffi::CString) -> *mut u32 {
    let value = native!(
        *mut u32,
        0xA358F56F10732EE1,
        native_parameters!(objectData, key.as_ptr())
    );

    value
}

pub fn datadict_create_array(objectData: *mut u32, key: &std::ffi::CString) -> *mut u32 {
    let value = native!(
        *mut u32,
        0x5B11728527CA6E5F,
        native_parameters!(objectData, key.as_ptr())
    );

    value
}

pub fn datadict_get_bool(objectData: *mut u32, key: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x1186940ED72FFEEC,
        native_parameters!(objectData, key.as_ptr())
    );

    value
}

pub fn datadict_get_int(objectData: *mut u32, key: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0x78F06F6B1FB5A80C,
        native_parameters!(objectData, key.as_ptr())
    );

    value
}

pub fn datadict_get_float(objectData: *mut u32, key: &std::ffi::CString) -> f32 {
    let value = native!(
        f32,
        0x06610343E73B9727,
        native_parameters!(objectData, key.as_ptr())
    );

    value
}

pub fn datadict_get_string(objectData: *mut u32, key: &std::ffi::CString) -> String {
    let value = native!(
        *const i8,
        0x3D2FD9E763B24472,
        native_parameters!(objectData, key.as_ptr())
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn datadict_get_vector(objectData: *mut u32, key: &std::ffi::CString) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x46CD3CB66E0825CC,
        native_parameters!(objectData, key.as_ptr())
    );

    value
}

pub fn datadict_get_dict(objectData: *mut u32, key: &std::ffi::CString) -> *mut u32 {
    let value = native!(
        *mut u32,
        0xB6B9DDC412FCEEE2,
        native_parameters!(objectData, key.as_ptr())
    );

    value
}

pub fn datadict_get_array(objectData: *mut u32, key: &std::ffi::CString) -> *mut u32 {
    let value = native!(
        *mut u32,
        0x7A983AA9DA2659ED,
        native_parameters!(objectData, key.as_ptr())
    );

    value
}

pub fn datadict_get_type(objectData: *mut u32, key: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0x031C55ED33227371,
        native_parameters!(objectData, key.as_ptr())
    );

    value
}

pub fn dataarray_add_bool(arrayData: *mut u32, value: bool) -> () {
    let value = native!((), 0xF8B0F5A43E928C76, native_parameters!(arrayData, value));

    value
}

pub fn dataarray_add_int(arrayData: *mut u32, value: i32) -> () {
    let value = native!((), 0xCABDB751D86FE93B, native_parameters!(arrayData, value));

    value
}

pub fn dataarray_add_float(arrayData: *mut u32, value: f32) -> () {
    let value = native!((), 0x57A995FD75D37F56, native_parameters!(arrayData, value));

    value
}

pub fn dataarray_add_string(arrayData: *mut u32, value: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x2F0661C155AEEEAA,
        native_parameters!(arrayData, value.as_ptr())
    );

    value
}

pub fn dataarray_add_vector(arrayData: *mut u32, valueX: f32, valueY: f32, valueZ: f32) -> () {
    let value = native!(
        (),
        0x407F8D034F70F0C2,
        native_parameters!(arrayData, valueX, valueY, valueZ)
    );

    value
}

pub fn dataarray_add_dict(arrayData: *mut u32) -> *mut u32 {
    let value = native!(*mut u32, 0x6889498B3E19C797, native_parameters!(arrayData));

    value
}

pub fn dataarray_get_bool(arrayData: *mut u32, arrayIndex: i32) -> bool {
    let value = native!(
        bool,
        0x50C1B2874E50C114,
        native_parameters!(arrayData, arrayIndex)
    );

    value
}

pub fn dataarray_get_int(arrayData: *mut u32, arrayIndex: i32) -> i32 {
    let value = native!(
        i32,
        0x3E5AE19425CD74BE,
        native_parameters!(arrayData, arrayIndex)
    );

    value
}

pub fn dataarray_get_float(arrayData: *mut u32, arrayIndex: i32) -> f32 {
    let value = native!(
        f32,
        0xC0C527B525D7CFB5,
        native_parameters!(arrayData, arrayIndex)
    );

    value
}

pub fn dataarray_get_string(arrayData: *mut u32, arrayIndex: i32) -> String {
    let value = native!(
        *const i8,
        0xD3F2FFEB8D836F52,
        native_parameters!(arrayData, arrayIndex)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn dataarray_get_vector(arrayData: *mut u32, arrayIndex: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x8D2064E5B64A628A,
        native_parameters!(arrayData, arrayIndex)
    );

    value
}

pub fn dataarray_get_dict(arrayData: *mut u32, arrayIndex: i32) -> *mut u32 {
    let value = native!(
        *mut u32,
        0x8B5FADCC4E3A145F,
        native_parameters!(arrayData, arrayIndex)
    );

    value
}

pub fn dataarray_get_count(arrayData: *mut u32) -> i32 {
    let value = native!(i32, 0x065DB281590CEA2D, native_parameters!(arrayData));

    value
}

pub fn dataarray_get_type(arrayData: *mut u32, arrayIndex: i32) -> i32 {
    let value = native!(
        i32,
        0x3A0014ADB172A3C5,
        native_parameters!(arrayData, arrayIndex)
    );

    value
}
