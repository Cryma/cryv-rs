use crate::types::NativeVector3;

pub fn set_clock_time(hour: i32, minute: i32, second: i32) -> () {
    let value = native!(
        (),
        0x47C3B5848C3E45D8,
        native_parameters!(hour, minute, second)
    );

    value
}

pub fn pause_clock(toggle: bool) -> () {
    let value = native!((), 0x4055E40BD2DBEC1D, native_parameters!(toggle));

    value
}

pub fn advance_clock_time_to(hour: i32, minute: i32, second: i32) -> () {
    let value = native!(
        (),
        0xC8CA9670B9D83B3B,
        native_parameters!(hour, minute, second)
    );

    value
}

pub fn add_to_clock_time(hours: i32, minutes: i32, seconds: i32) -> () {
    let value = native!(
        (),
        0xD716F30D8C8980E2,
        native_parameters!(hours, minutes, seconds)
    );

    value
}

pub fn get_clock_hours() -> i32 {
    let value = native!(i32, 0x25223CA6B4D20B7F, native_parameters!());

    value
}

pub fn get_clock_minutes() -> i32 {
    let value = native!(i32, 0x13D2B8ADD79640F2, native_parameters!());

    value
}

pub fn get_clock_seconds() -> i32 {
    let value = native!(i32, 0x494E97C2EF27C470, native_parameters!());

    value
}

pub fn set_clock_date(day: i32, month: i32, year: i32) -> () {
    let value = native!((), 0xB096419DF0D06CE7, native_parameters!(day, month, year));

    value
}

pub fn get_clock_day_of_week() -> i32 {
    let value = native!(i32, 0xD972E4BD7AEB235F, native_parameters!());

    value
}

pub fn get_clock_day_of_month() -> i32 {
    let value = native!(i32, 0x3D10BC92A4DB1D35, native_parameters!());

    value
}

pub fn get_clock_month() -> i32 {
    let value = native!(i32, 0xBBC72712E80257A1, native_parameters!());

    value
}

pub fn get_clock_year() -> i32 {
    let value = native!(i32, 0x961777E64BDAF717, native_parameters!());

    value
}

pub fn get_milliseconds_per_game_minute() -> i32 {
    let value = native!(i32, 0x2F8B4D1C595B11DB, native_parameters!());

    value
}

pub fn get_posix_time(
    year: *mut i32,
    month: *mut i32,
    day: *mut i32,
    hour: *mut i32,
    minute: *mut i32,
    second: *mut i32,
) -> () {
    let value = native!(
        (),
        0xDA488F299A5B164E,
        native_parameters!(year, month, day, hour, minute, second)
    );

    value
}

pub fn get_utc_time(
    year: *mut i32,
    month: *mut i32,
    day: *mut i32,
    hour: *mut i32,
    minute: *mut i32,
    second: *mut i32,
) -> () {
    let value = native!(
        (),
        0x8117E09A19EEF4D3,
        native_parameters!(year, month, day, hour, minute, second)
    );

    value
}

pub fn get_local_time(
    year: *mut i32,
    month: *mut i32,
    day: *mut i32,
    hour: *mut i32,
    minute: *mut i32,
    second: *mut i32,
) -> () {
    let value = native!(
        (),
        0x50C7A99057A69748,
        native_parameters!(year, month, day, hour, minute, second)
    );

    value
}
