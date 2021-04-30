use crate::types::NativeVector3;

pub fn play_ped_ringtone(ringtoneName: &std::ffi::CString, ped: i32, p2: bool) -> () {
    let value = native!(
        (),
        0xF9E56683CA8E11A5,
        native_parameters!(ringtoneName.as_ptr(), ped, p2)
    );

    value
}

pub fn is_ped_ringtone_playing(ped: i32) -> bool {
    let value = native!(bool, 0x1E8E5E20937E3137, native_parameters!(ped));

    value
}

pub fn stop_ped_ringtone(ped: i32) -> () {
    let value = native!((), 0x6C5AE23EFA885092, native_parameters!(ped));

    value
}

pub fn is_mobile_phone_call_ongoing() -> bool {
    let value = native!(bool, 0x7497D2CE2C30D24C, native_parameters!());

    value
}

pub fn _0xc8b1b2425604cdd0() -> bool {
    let value = native!(bool, 0xC8B1B2425604CDD0, native_parameters!());

    value
}

pub fn create_new_scripted_conversation() -> () {
    let value = native!((), 0xD2C91A0B572AAE56, native_parameters!());

    value
}

pub fn add_line_to_conversation(
    index: i32,
    p1: &std::ffi::CString,
    p2: &std::ffi::CString,
    p3: i32,
    p4: i32,
    p5: bool,
    p6: bool,
    p7: bool,
    p8: bool,
    p9: i32,
    p10: bool,
    p11: bool,
    p12: bool,
) -> () {
    let value = native!(
        (),
        0xC5EF963405593646,
        native_parameters!(
            index,
            p1.as_ptr(),
            p2.as_ptr(),
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
            p9,
            p10,
            p11,
            p12
        )
    );

    value
}

pub fn add_ped_to_conversation(index: i32, ped: i32, p2: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x95D9F4BC443956E7,
        native_parameters!(index, ped, p2.as_ptr())
    );

    value
}

pub fn _0x33e3c6c6f2f0b506(p0: u32, p1: f32, p2: f32, p3: f32) -> () {
    let value = native!((), 0x33E3C6C6F2F0B506, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _0x892b6ab8f33606f5(p0: i32, entity: i32) -> () {
    let value = native!((), 0x892B6AB8F33606F5, native_parameters!(p0, entity));

    value
}

pub fn set_microphone_position(
    p0: bool,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    x3: f32,
    y3: f32,
    z3: f32,
) -> () {
    let value = native!(
        (),
        0xB6AE90EDDE95C762,
        native_parameters!(p0, x1, y1, z1, x2, y2, z2, x3, y3, z3)
    );

    value
}

pub fn _0x0b568201dd99f0eb(p0: bool) -> () {
    let value = native!((), 0x0B568201DD99F0EB, native_parameters!(p0));

    value
}

pub fn _0x61631f5df50d1c34(p0: bool) -> () {
    let value = native!((), 0x61631F5DF50D1C34, native_parameters!(p0));

    value
}

pub fn start_script_phone_conversation(p0: bool, p1: bool) -> () {
    let value = native!((), 0x252E5F915EABB675, native_parameters!(p0, p1));

    value
}

pub fn preload_script_phone_conversation(p0: bool, p1: bool) -> () {
    let value = native!((), 0x6004BCB0E226AAEA, native_parameters!(p0, p1));

    value
}

pub fn start_script_conversation(p0: bool, p1: bool, p2: bool, p3: bool) -> () {
    let value = native!((), 0x6B17C62C9635D2DC, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn preload_script_conversation(p0: bool, p1: bool, p2: bool, p3: bool) -> () {
    let value = native!((), 0x3B3CAD6166916D87, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn start_preloaded_conversation() -> () {
    let value = native!((), 0x23641AFE870AF385, native_parameters!());

    value
}

pub fn get_is_preloaded_conversation_ready() -> bool {
    let value = native!(bool, 0xE73364DB90778FFA, native_parameters!());

    value
}

pub fn is_scripted_conversation_ongoing() -> bool {
    let value = native!(bool, 0x16754C556D2EDE3D, native_parameters!());

    value
}

pub fn is_scripted_conversation_loaded() -> bool {
    let value = native!(bool, 0xDF0D54BE7A776737, native_parameters!());

    value
}

pub fn get_current_scripted_conversation_line() -> i32 {
    let value = native!(i32, 0x480357EE890C295A, native_parameters!());

    value
}

pub fn pause_scripted_conversation(p0: bool) -> () {
    let value = native!((), 0x8530AD776CD72B12, native_parameters!(p0));

    value
}

pub fn restart_scripted_conversation() -> () {
    let value = native!((), 0x9AEB285D1818C9AC, native_parameters!());

    value
}

pub fn stop_scripted_conversation(p0: bool) -> i32 {
    let value = native!(i32, 0xD79DEEFB53455EBA, native_parameters!(p0));

    value
}

pub fn skip_to_next_scripted_conversation_line() -> () {
    let value = native!((), 0x9663FE6B7A61EB00, native_parameters!());

    value
}

pub fn interrupt_conversation(p0: u32, p1: *mut u32, p2: *mut u32) -> () {
    let value = native!((), 0xA018A12E5C5C2FA6, native_parameters!(p0, p1, p2));

    value
}

pub fn interrupt_conversation_and_pause(
    ped: i32,
    p1: &std::ffi::CString,
    p2: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0x8A694D7A68F8DC38,
        native_parameters!(ped, p1.as_ptr(), p2.as_ptr())
    );

    value
}

pub fn _0xaa19f5572c38b564(p0: *mut u32) -> u32 {
    let value = native!(u32, 0xAA19F5572C38B564, native_parameters!(p0));

    value
}

pub fn _0xb542de8c3d1cb210(p0: bool) -> () {
    let value = native!((), 0xB542DE8C3D1CB210, native_parameters!(p0));

    value
}

pub fn register_script_with_audio(p0: i32) -> () {
    let value = native!((), 0xC6ED9D5092438D91, native_parameters!(p0));

    value
}

pub fn unregister_script_with_audio() -> () {
    let value = native!((), 0xA8638BE228D4751A, native_parameters!());

    value
}

pub fn request_mission_audio_bank(p0: &std::ffi::CString, p1: bool, p2: u32) -> bool {
    let value = native!(
        bool,
        0x7345BDD95E62E0F2,
        native_parameters!(p0.as_ptr(), p1, p2)
    );

    value
}

pub fn request_ambient_audio_bank(p0: &std::ffi::CString, p1: bool, p2: u32) -> bool {
    let value = native!(
        bool,
        0xFE02FFBED8CA9D99,
        native_parameters!(p0.as_ptr(), p1, p2)
    );

    value
}

pub fn request_script_audio_bank(p0: &std::ffi::CString, p1: bool, p2: i32) -> bool {
    let value = native!(
        bool,
        0x2F844A8B08D76685,
        native_parameters!(p0.as_ptr(), p1, p2)
    );

    value
}

pub fn _0x40763ea7b9b783e7(p0: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(u32, 0x40763EA7B9B783E7, native_parameters!(p0, p1, p2));

    value
}

pub fn hint_ambient_audio_bank(p0: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(u32, 0x8F8C0E370AE62F5C, native_parameters!(p0, p1, p2));

    value
}

pub fn hint_script_audio_bank(p0: u32, p1: u32, p2: u32) -> u32 {
    let value = native!(u32, 0xFB380A29641EC31A, native_parameters!(p0, p1, p2));

    value
}

pub fn release_mission_audio_bank() -> () {
    let value = native!((), 0x0EC92A1BF0857187, native_parameters!());

    value
}

pub fn release_ambient_audio_bank() -> () {
    let value = native!((), 0x65475A218FFAA93D, native_parameters!());

    value
}

pub fn release_named_script_audio_bank(audioBank: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x77ED170667F50170,
        native_parameters!(audioBank.as_ptr())
    );

    value
}

pub fn release_script_audio_bank() -> () {
    let value = native!((), 0x7A2D8AD0A9EB9C3F, native_parameters!());

    value
}

pub fn _0x19af7ed9b9d23058() -> () {
    let value = native!((), 0x19AF7ED9B9D23058, native_parameters!());

    value
}

pub fn _0x9ac92eed5e4793ab() -> () {
    let value = native!((), 0x9AC92EED5E4793AB, native_parameters!());

    value
}

pub fn _0x11579d940949c49e(p0: u32) -> () {
    let value = native!((), 0x11579D940949C49E, native_parameters!(p0));

    value
}

pub fn get_sound_id() -> i32 {
    let value = native!(i32, 0x430386FE9BF80B45, native_parameters!());

    value
}

pub fn release_sound_id(soundId: i32) -> () {
    let value = native!((), 0x353FC880830B88FA, native_parameters!(soundId));

    value
}

pub fn play_sound(
    soundId: i32,
    audioName: &std::ffi::CString,
    audioRef: &std::ffi::CString,
    p3: bool,
    p4: u32,
    p5: bool,
) -> () {
    let value = native!(
        (),
        0x7FF4944CC209192D,
        native_parameters!(soundId, audioName.as_ptr(), audioRef.as_ptr(), p3, p4, p5)
    );

    value
}

pub fn play_sound_frontend(
    soundId: i32,
    audioName: &std::ffi::CString,
    audioRef: &std::ffi::CString,
    p3: bool,
) -> () {
    let value = native!(
        (),
        0x67C540AA08E4A6F5,
        native_parameters!(soundId, audioName.as_ptr(), audioRef.as_ptr(), p3)
    );

    value
}

pub fn play_deferred_sound_frontend(
    soundName: &std::ffi::CString,
    soundsetName: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0xCADA5A0D0702381E,
        native_parameters!(soundName.as_ptr(), soundsetName.as_ptr())
    );

    value
}

pub fn play_sound_from_entity(
    soundId: i32,
    audioName: &std::ffi::CString,
    entity: i32,
    audioRef: &std::ffi::CString,
    isNetwork: bool,
    p5: u32,
) -> () {
    let value = native!(
        (),
        0xE65F427EB70AB1ED,
        native_parameters!(
            soundId,
            audioName.as_ptr(),
            entity,
            audioRef.as_ptr(),
            isNetwork,
            p5
        )
    );

    value
}

pub fn _0x5b9853296731e88d(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32) -> () {
    let value = native!(
        (),
        0x5B9853296731E88D,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn play_sound_from_coord(
    soundId: i32,
    audioName: &std::ffi::CString,
    x: f32,
    y: f32,
    z: f32,
    audioRef: &std::ffi::CString,
    isNetwork: bool,
    range: i32,
    p8: bool,
) -> () {
    let value = native!(
        (),
        0x8D8686B622B88120,
        native_parameters!(
            soundId,
            audioName.as_ptr(),
            x,
            y,
            z,
            audioRef.as_ptr(),
            isNetwork,
            range,
            p8
        )
    );

    value
}

pub fn _0x7ec3c679d0e7e46b(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x7EC3C679D0E7E46B, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn stop_sound(soundId: i32) -> () {
    let value = native!((), 0xA3B0C41BA5CC0BB5, native_parameters!(soundId));

    value
}

pub fn get_network_id_from_sound_id(soundId: i32) -> i32 {
    let value = native!(i32, 0x2DE3F0A134FFBC0D, native_parameters!(soundId));

    value
}

pub fn get_sound_id_from_network_id(netId: i32) -> i32 {
    let value = native!(i32, 0x75262FD12D0A1C84, native_parameters!(netId));

    value
}

pub fn set_variable_on_sound(soundId: i32, p1: *mut u32, p2: f32) -> () {
    let value = native!((), 0xAD6B3148A78AE9B6, native_parameters!(soundId, p1, p2));

    value
}

pub fn set_variable_on_stream(p0: &std::ffi::CString, p1: f32) -> () {
    let value = native!((), 0x2F9D3834AEB9EF79, native_parameters!(p0.as_ptr(), p1));

    value
}

pub fn override_underwater_stream(p0: *mut u32, p1: bool) -> () {
    let value = native!((), 0xF2A9CDABCEA04BD6, native_parameters!(p0, p1));

    value
}

pub fn set_variable_on_under_water_stream(variableName: &std::ffi::CString, value: f32) -> () {
    let value = native!(
        (),
        0x733ADF241531E5C2,
        native_parameters!(variableName.as_ptr(), value)
    );

    value
}

pub fn has_sound_finished(soundId: i32) -> bool {
    let value = native!(bool, 0xFCBDCE714A7C88E5, native_parameters!(soundId));

    value
}

pub fn play_ped_ambient_speech_native(
    ped: i32,
    speechName: &std::ffi::CString,
    speechParam: &std::ffi::CString,
    p3: u32,
) -> () {
    let value = native!(
        (),
        0x8E04FEDD28D42462,
        native_parameters!(ped, speechName.as_ptr(), speechParam.as_ptr(), p3)
    );

    value
}

pub fn play_ped_ambient_speech_and_clone_native(
    ped: i32,
    speechName: &std::ffi::CString,
    speechParam: &std::ffi::CString,
    p3: u32,
) -> () {
    let value = native!(
        (),
        0xC6941B4A3A8FBBB9,
        native_parameters!(ped, speechName.as_ptr(), speechParam.as_ptr(), p3)
    );

    value
}

pub fn play_ped_ambient_speech_with_voice_native(
    ped: i32,
    speechName: &std::ffi::CString,
    voiceName: &std::ffi::CString,
    speechParam: &std::ffi::CString,
    p4: bool,
) -> () {
    let value = native!(
        (),
        0x3523634255FC3318,
        native_parameters!(
            ped,
            speechName.as_ptr(),
            voiceName.as_ptr(),
            speechParam.as_ptr(),
            p4
        )
    );

    value
}

pub fn play_ambient_speech_from_position_native(
    speechName: &std::ffi::CString,
    voiceName: &std::ffi::CString,
    x: f32,
    y: f32,
    z: f32,
    speechParam: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0xED640017ED337E45,
        native_parameters!(
            speechName.as_ptr(),
            voiceName.as_ptr(),
            x,
            y,
            z,
            speechParam.as_ptr()
        )
    );

    value
}

pub fn override_trevor_rage(voiceEffect: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x13AD665062541A7E,
        native_parameters!(voiceEffect.as_ptr())
    );

    value
}

pub fn reset_trevor_rage() -> () {
    let value = native!((), 0xE78503B10C4314E0, native_parameters!());

    value
}

pub fn set_player_angry(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xEA241BB04110F091, native_parameters!(ped, toggle));

    value
}

pub fn play_pain(ped: i32, painID: i32, p1: i32, p3: u32) -> () {
    let value = native!(
        (),
        0xBC9AE166038A5CEC,
        native_parameters!(ped, painID, p1, p3)
    );

    value
}

pub fn release_weapon_audio() -> () {
    let value = native!((), 0xCE4AC0439F607045, native_parameters!());

    value
}

pub fn activate_audio_slowmo_mode(p0: &std::ffi::CString) -> () {
    let value = native!((), 0xD01005D2BA2EB778, native_parameters!(p0.as_ptr()));

    value
}

pub fn deactivate_audio_slowmo_mode(p0: &std::ffi::CString) -> () {
    let value = native!((), 0xDDC635D5B3262C56, native_parameters!(p0.as_ptr()));

    value
}

pub fn set_ambient_voice_name(ped: i32, name: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x6C8065A3B780185B,
        native_parameters!(ped, name.as_ptr())
    );

    value
}

pub fn _set_ambient_voice_name_hash(ped: i32, hash: u32) -> () {
    let value = native!((), 0x9A53DED9921DE990, native_parameters!(ped, hash));

    value
}

pub fn _get_ambient_voice_name_hash(ped: i32) -> u32 {
    let value = native!(u32, 0x5E203DA2BA15D436, native_parameters!(ped));

    value
}

pub fn _set_ped_scream(ped: i32) -> () {
    let value = native!((), 0x40CF0D12D142A9E8, native_parameters!(ped));

    value
}

pub fn _0x1b7abe26cbcbf8c7(ped: i32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x1B7ABE26CBCBF8C7, native_parameters!(ped, p1, p2));

    value
}

pub fn _set_ped_voice_group(ped: i32, voiceGroupHash: u32) -> () {
    let value = native!(
        (),
        0x7CDC8C3B89F661B3,
        native_parameters!(ped, voiceGroupHash)
    );

    value
}

pub fn _set_ped_audio_gender(ped: i32, p1: bool) -> () {
    let value = native!((), 0xA5342D390CDA41D6, native_parameters!(ped, p1));

    value
}

pub fn stop_current_playing_speech(ped: i32) -> () {
    let value = native!((), 0x7A73D05A607734C7, native_parameters!(ped));

    value
}

pub fn stop_current_playing_ambient_speech(ped: i32) -> () {
    let value = native!((), 0xB8BEC0CA6F0EDB0F, native_parameters!(ped));

    value
}

pub fn is_ambient_speech_playing(ped: i32) -> bool {
    let value = native!(bool, 0x9072C8B49907BFAD, native_parameters!(ped));

    value
}

pub fn is_scripted_speech_playing(p0: u32) -> bool {
    let value = native!(bool, 0xCC9AA18DCC7084F4, native_parameters!(p0));

    value
}

pub fn is_any_speech_playing(ped: i32) -> bool {
    let value = native!(bool, 0x729072355FA39EC9, native_parameters!(ped));

    value
}

pub fn _0x30ca2ef91d15adf8() -> u32 {
    let value = native!(u32, 0x30CA2EF91D15ADF8, native_parameters!());

    value
}

pub fn _can_ped_speak(ped: i32, speechName: &std::ffi::CString, unk: bool) -> bool {
    let value = native!(
        bool,
        0x49B99BF3FDA89A7A,
        native_parameters!(ped, speechName.as_ptr(), unk)
    );

    value
}

pub fn is_ped_in_current_conversation(ped: i32) -> bool {
    let value = native!(bool, 0x049E937F18F4020C, native_parameters!(ped));

    value
}

pub fn set_ped_is_drunk(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x95D2D383D5396B8A, native_parameters!(ped, toggle));

    value
}

pub fn play_animal_vocalization(pedHandle: i32, p1: i32, speechName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xEE066C7006C49C0A,
        native_parameters!(pedHandle, p1, speechName.as_ptr())
    );

    value
}

pub fn is_animal_vocalization_playing(pedHandle: i32) -> bool {
    let value = native!(bool, 0xC265DF9FB44A9FBD, native_parameters!(pedHandle));

    value
}

pub fn set_animal_mood(animal: i32, mood: i32) -> () {
    let value = native!((), 0xCC97B29285B1DC3B, native_parameters!(animal, mood));

    value
}

pub fn is_mobile_phone_radio_active() -> bool {
    let value = native!(bool, 0xB35CE999E8EF317E, native_parameters!());

    value
}

pub fn set_mobile_phone_radio_state(state: bool) -> () {
    let value = native!((), 0xBF286C554784F3DF, native_parameters!(state));

    value
}

pub fn get_player_radio_station_index() -> i32 {
    let value = native!(i32, 0xE8AF77C4C06ADC93, native_parameters!());

    value
}

pub fn get_player_radio_station_name() -> String {
    let value = native!(*const i8, 0xF6D733C32076AD03, native_parameters!());
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_radio_station_name(radioStation: i32) -> String {
    let value = native!(
        *const i8,
        0xB28ECA15046CA8B9,
        native_parameters!(radioStation)
    );
    let cstr = unsafe { std::ffi::CStr::from_ptr(value) };
    let value = cstr.to_str().unwrap().to_string();
    value
}

pub fn get_player_radio_station_genre() -> i32 {
    let value = native!(i32, 0xA571991A7FE6CCEB, native_parameters!());

    value
}

pub fn is_radio_retuning() -> bool {
    let value = native!(bool, 0xA151A7394A214E65, native_parameters!());

    value
}

pub fn is_radio_faded_out() -> bool {
    let value = native!(bool, 0x0626A247D2405330, native_parameters!());

    value
}

pub fn _0xff266d1d0eb1195d() -> () {
    let value = native!((), 0xFF266D1D0EB1195D, native_parameters!());

    value
}

pub fn _0xdd6bcf9e94425df9() -> () {
    let value = native!((), 0xDD6BCF9E94425DF9, native_parameters!());

    value
}

pub fn set_radio_to_station_name(stationName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xC69EDA28699D5107,
        native_parameters!(stationName.as_ptr())
    );

    value
}

pub fn set_veh_radio_station(vehicle: i32, radioStation: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x1B9C0099CB942AC6,
        native_parameters!(vehicle, radioStation.as_ptr())
    );

    value
}

pub fn _0x0be4be946463f917(vehicle: i32) -> bool {
    let value = native!(bool, 0x0BE4BE946463F917, native_parameters!(vehicle));

    value
}

pub fn _0xc1805d05e6d4fe10(vehicle: i32) -> () {
    let value = native!((), 0xC1805D05E6D4FE10, native_parameters!(vehicle));

    value
}

pub fn set_emitter_radio_station(
    emitterName: &std::ffi::CString,
    radioStation: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0xACF57305B12AF907,
        native_parameters!(emitterName.as_ptr(), radioStation.as_ptr())
    );

    value
}

pub fn set_static_emitter_enabled(emitterName: &std::ffi::CString, toggle: bool) -> () {
    let value = native!(
        (),
        0x399D2D3B33F1B8EB,
        native_parameters!(emitterName.as_ptr(), toggle)
    );

    value
}

pub fn _link_static_emitter_to_entity(emitterName: &std::ffi::CString, entity: i32) -> () {
    let value = native!(
        (),
        0x651D3228960D08AF,
        native_parameters!(emitterName.as_ptr(), entity)
    );

    value
}

pub fn set_radio_to_station_index(radioStation: i32) -> () {
    let value = native!((), 0xA619B168B8A8570F, native_parameters!(radioStation));

    value
}

pub fn set_frontend_radio_active(active: bool) -> () {
    let value = native!((), 0xF7F26C6E9CC9EBB8, native_parameters!(active));

    value
}

pub fn unlock_mission_news_story(newsStory: i32) -> () {
    let value = native!((), 0xB165AB7C248B2DC1, native_parameters!(newsStory));

    value
}

pub fn is_mission_news_story_unlocked(newsStory: i32) -> bool {
    let value = native!(bool, 0x66E49BF55B4B1874, native_parameters!(newsStory));

    value
}

pub fn get_audible_music_track_text_id() -> i32 {
    let value = native!(i32, 0x50B196FC9ED6545B, native_parameters!());

    value
}

pub fn play_end_credits_music(play: bool) -> () {
    let value = native!((), 0xCD536C4D33DCC900, native_parameters!(play));

    value
}

pub fn skip_radio_forward() -> () {
    let value = native!((), 0x6DDBBDD98E2E9C25, native_parameters!());

    value
}

pub fn freeze_radio_station(radioStation: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x344F393B027E38C3,
        native_parameters!(radioStation.as_ptr())
    );

    value
}

pub fn unfreeze_radio_station(radioStation: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xFC00454CF60B91DD,
        native_parameters!(radioStation.as_ptr())
    );

    value
}

pub fn set_radio_auto_unfreeze(toggle: bool) -> () {
    let value = native!((), 0xC1AA9F53CE982990, native_parameters!(toggle));

    value
}

pub fn set_initial_player_station(radioStation: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x88795F13FACDA88D,
        native_parameters!(radioStation.as_ptr())
    );

    value
}

pub fn set_user_radio_control_enabled(toggle: bool) -> () {
    let value = native!((), 0x19F21E63AE6EAE4E, native_parameters!(toggle));

    value
}

pub fn set_radio_track(radioStation: &std::ffi::CString, radioTrack: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xB39786F201FEE30B,
        native_parameters!(radioStation.as_ptr(), radioTrack.as_ptr())
    );

    value
}

pub fn _set_radio_track_mix(
    radioStationName: &std::ffi::CString,
    mixName: &std::ffi::CString,
    p2: i32,
) -> () {
    let value = native!(
        (),
        0x2CB0075110BE1E56,
        native_parameters!(radioStationName.as_ptr(), mixName.as_ptr(), p2)
    );

    value
}

pub fn _0x55ecf4d13d9903b0(p0: u32, p1: u32, p2: u32, p3: u32) -> () {
    let value = native!((), 0x55ECF4D13D9903B0, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn set_vehicle_radio_loud(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xBB6F1CAEC68B0BCE, native_parameters!(vehicle, toggle));

    value
}

pub fn _is_vehicle_radio_loud(vehicle: i32) -> bool {
    let value = native!(bool, 0x032A116663A4D5AC, native_parameters!(vehicle));

    value
}

pub fn set_mobile_radio_enabled_during_gameplay(toggle: bool) -> () {
    let value = native!((), 0x1098355A16064BB3, native_parameters!(toggle));

    value
}

pub fn does_player_veh_have_radio() -> bool {
    let value = native!(bool, 0x109697E2FFBAC8A1, native_parameters!());

    value
}

pub fn is_player_veh_radio_enable() -> bool {
    let value = native!(bool, 0x5F43D83FD6738741, native_parameters!());

    value
}

pub fn set_vehicle_radio_enabled(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x3B988190C0AA6C0B, native_parameters!(vehicle, toggle));

    value
}

pub fn _0xda07819e452ffe8f(p0: u32) -> () {
    let value = native!((), 0xDA07819E452FFE8F, native_parameters!(p0));

    value
}

pub fn set_custom_radio_track_list(
    radioStation: &std::ffi::CString,
    trackListName: &std::ffi::CString,
    p2: bool,
) -> () {
    let value = native!(
        (),
        0x4E404A9361F75BB2,
        native_parameters!(radioStation.as_ptr(), trackListName.as_ptr(), p2)
    );

    value
}

pub fn clear_custom_radio_track_list(radioStation: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x1654F24A88A8E3FE,
        native_parameters!(radioStation.as_ptr())
    );

    value
}

pub fn get_num_unlocked_radio_stations() -> i32 {
    let value = native!(i32, 0xF1620ECB50E01DE7, native_parameters!());

    value
}

pub fn find_radio_station_index(station: i32) -> i32 {
    let value = native!(i32, 0x8D67489793FF428B, native_parameters!(station));

    value
}

pub fn set_radio_station_music_only(radioStation: &std::ffi::CString, toggle: bool) -> () {
    let value = native!(
        (),
        0x774BD811F656A122,
        native_parameters!(radioStation.as_ptr(), toggle)
    );

    value
}

pub fn set_radio_frontend_fade_time(fadeTime: f32) -> () {
    let value = native!((), 0x2C96CDB04FCA358E, native_parameters!(fadeTime));

    value
}

pub fn unlock_radio_station_track_list(
    radioStation: &std::ffi::CString,
    trackListName: &std::ffi::CString,
) -> () {
    let value = native!(
        (),
        0x031ACB6ABA18C729,
        native_parameters!(radioStation.as_ptr(), trackListName.as_ptr())
    );

    value
}

pub fn _update_lsur(enableMixes: bool) -> () {
    let value = native!((), 0x47AED84213A47510, native_parameters!(enableMixes));

    value
}

pub fn _lock_radio_station(radioStationName: &std::ffi::CString, toggle: bool) -> () {
    let value = native!(
        (),
        0x477D9DB48F889591,
        native_parameters!(radioStationName.as_ptr(), toggle)
    );

    value
}

pub fn _0xc64a06d939f826f5(p0: *mut f32, p1: *mut f32, p2: *mut i32) -> bool {
    let value = native!(bool, 0xC64A06D939F826F5, native_parameters!(p0, p1, p2));

    value
}

pub fn _get_current_radio_station_hash(radioStationName: &std::ffi::CString) -> i32 {
    let value = native!(
        i32,
        0x3E65CDE5215832C1,
        native_parameters!(radioStationName.as_ptr())
    );

    value
}

pub fn _0x34d66bc058019ce0(radioStationName: &std::ffi::CString) -> u32 {
    let value = native!(
        u32,
        0x34D66BC058019CE0,
        native_parameters!(radioStationName.as_ptr())
    );

    value
}

pub fn _0xf3365489e0dd50f9(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xF3365489E0DD50F9, native_parameters!(vehicle, toggle));

    value
}

pub fn set_ambient_zone_state(zoneName: &std::ffi::CString, p1: bool, p2: bool) -> () {
    let value = native!(
        (),
        0xBDA07E5950085E46,
        native_parameters!(zoneName.as_ptr(), p1, p2)
    );

    value
}

pub fn clear_ambient_zone_state(zoneName: &std::ffi::CString, p1: bool) -> () {
    let value = native!(
        (),
        0x218DD44AAAC964FF,
        native_parameters!(zoneName.as_ptr(), p1)
    );

    value
}

pub fn set_ambient_zone_list_state(p0: *mut u32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x9748FA4DE50CCE3E, native_parameters!(p0, p1, p2));

    value
}

pub fn clear_ambient_zone_list_state(p0: *mut u32, p1: bool) -> () {
    let value = native!((), 0x120C48C614909FA4, native_parameters!(p0, p1));

    value
}

pub fn set_ambient_zone_state_persistent(
    ambientZone: &std::ffi::CString,
    p1: bool,
    p2: bool,
) -> () {
    let value = native!(
        (),
        0x1D6650420CEC9D3B,
        native_parameters!(ambientZone.as_ptr(), p1, p2)
    );

    value
}

pub fn set_ambient_zone_list_state_persistent(
    ambientZone: &std::ffi::CString,
    p1: bool,
    p2: bool,
) -> () {
    let value = native!(
        (),
        0xF3638DAE8C4045E1,
        native_parameters!(ambientZone.as_ptr(), p1, p2)
    );

    value
}

pub fn is_ambient_zone_enabled(ambientZone: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x01E2817A479A7F9B,
        native_parameters!(ambientZone.as_ptr())
    );

    value
}

pub fn _0x5d2bfaab8d956e0e() -> () {
    let value = native!((), 0x5D2BFAAB8D956E0E, native_parameters!());

    value
}

pub fn set_cutscene_audio_override(name: &std::ffi::CString) -> () {
    let value = native!((), 0x3B4BF5F0859204D9, native_parameters!(name.as_ptr()));

    value
}

pub fn _set_variable_on_cutscene_audio(variableName: &std::ffi::CString, value: f32) -> () {
    let value = native!(
        (),
        0xBCC29F935ED07688,
        native_parameters!(variableName.as_ptr(), value)
    );

    value
}

pub fn play_police_report(name: &std::ffi::CString, p1: f32) -> i32 {
    let value = native!(
        i32,
        0xDFEBD56D9BD1EB16,
        native_parameters!(name.as_ptr(), p1)
    );

    value
}

pub fn _cancel_current_police_report() -> () {
    let value = native!((), 0xB4F90FAF7670B16F, native_parameters!());

    value
}

pub fn blip_siren(vehicle: i32) -> () {
    let value = native!((), 0x1B9025BDA76822B6, native_parameters!(vehicle));

    value
}

pub fn override_veh_horn(vehicle: i32, r#override: bool, hornHash: i32) -> () {
    let value = native!(
        (),
        0x3CDC1E622CCE0356,
        native_parameters!(vehicle, r#override, hornHash)
    );

    value
}

pub fn is_horn_active(vehicle: i32) -> bool {
    let value = native!(bool, 0x9D6BFC12B05C6121, native_parameters!(vehicle));

    value
}

pub fn set_aggressive_horns(toggle: bool) -> () {
    let value = native!((), 0x395BF71085D1B1D9, native_parameters!(toggle));

    value
}

pub fn _0x02e93c796abd3a97(p0: bool) -> () {
    let value = native!((), 0x02E93C796ABD3A97, native_parameters!(p0));

    value
}

pub fn _0x58bb377bec7cd5f4(p0: bool, p1: bool) -> () {
    let value = native!((), 0x58BB377BEC7CD5F4, native_parameters!(p0, p1));

    value
}

pub fn _0x9bd7bd55e4533183(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0x9BD7BD55E4533183, native_parameters!(p0, p1, p2));

    value
}

pub fn is_stream_playing() -> bool {
    let value = native!(bool, 0xD11FA52EB849D978, native_parameters!());

    value
}

pub fn get_stream_play_time() -> i32 {
    let value = native!(i32, 0x4E72BBDBCA58A3DB, native_parameters!());

    value
}

pub fn load_stream(streamName: &std::ffi::CString, soundSet: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x1F1F957154EC51DF,
        native_parameters!(streamName.as_ptr(), soundSet.as_ptr())
    );

    value
}

pub fn load_stream_with_start_offset(
    streamName: &std::ffi::CString,
    startOffset: i32,
    soundSet: &std::ffi::CString,
) -> bool {
    let value = native!(
        bool,
        0x59C16B79F53B3712,
        native_parameters!(streamName.as_ptr(), startOffset, soundSet.as_ptr())
    );

    value
}

pub fn play_stream_from_ped(ped: i32) -> () {
    let value = native!((), 0x89049DD63C08B5D1, native_parameters!(ped));

    value
}

pub fn play_stream_from_vehicle(vehicle: i32) -> () {
    let value = native!((), 0xB70374A758007DFA, native_parameters!(vehicle));

    value
}

pub fn play_stream_from_object(object: i32) -> () {
    let value = native!((), 0xEBAA9B64D76356FD, native_parameters!(object));

    value
}

pub fn play_stream_frontend() -> () {
    let value = native!((), 0x58FCE43488F9F5F4, native_parameters!());

    value
}

pub fn play_stream_from_position(x: f32, y: f32, z: f32) -> () {
    let value = native!((), 0x21442F412E8DE56B, native_parameters!(x, y, z));

    value
}

pub fn stop_stream() -> () {
    let value = native!((), 0xA4718A1419D18151, native_parameters!());

    value
}

pub fn stop_ped_speaking(ped: i32, shaking: bool) -> () {
    let value = native!((), 0x9D64D7405520E3D3, native_parameters!(ped, shaking));

    value
}

pub fn _0xf8ad2eed7c47e8fe(ped: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0xF8AD2EED7C47E8FE, native_parameters!(ped, p1, p2));

    value
}

pub fn _0xab6781a5f3101470(p0: u32, p1: u32) -> () {
    let value = native!((), 0xAB6781A5F3101470, native_parameters!(p0, p1));

    value
}

pub fn disable_ped_pain_audio(ped: i32, toggle: bool) -> () {
    let value = native!((), 0xA9A41C1E940FB0E8, native_parameters!(ped, toggle));

    value
}

pub fn is_ambient_speech_disabled(ped: i32) -> bool {
    let value = native!(bool, 0x932C2D096A2C3FFF, native_parameters!(ped));

    value
}

pub fn _0xa8a7d434afb4b97b(p0: &std::ffi::CString, p1: i32) -> () {
    let value = native!((), 0xA8A7D434AFB4B97B, native_parameters!(p0.as_ptr(), p1));

    value
}

pub fn _0x2acabed337622df2(p0: &std::ffi::CString) -> () {
    let value = native!((), 0x2ACABED337622DF2, native_parameters!(p0.as_ptr()));

    value
}

pub fn set_siren_with_no_driver(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x1FEF0683B96EBCF2, native_parameters!(vehicle, toggle));

    value
}

pub fn _trigger_siren(vehicle: i32) -> () {
    let value = native!((), 0x66C3FB05206041BA, native_parameters!(vehicle));

    value
}

pub fn _sound_vehicle_horn_this_frame(vehicle: i32) -> () {
    let value = native!((), 0x9C11908013EA4715, native_parameters!(vehicle));

    value
}

pub fn set_horn_enabled(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x76D683C108594D0E, native_parameters!(vehicle, toggle));

    value
}

pub fn set_audio_vehicle_priority(vehicle: i32, p1: u32) -> () {
    let value = native!((), 0xE5564483E407F914, native_parameters!(vehicle, p1));

    value
}

pub fn _0x9d3af56e94c9ae98(vehicle: i32, p1: f32) -> () {
    let value = native!((), 0x9D3AF56E94C9AE98, native_parameters!(vehicle, p1));

    value
}

pub fn use_siren_as_horn(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xFA932DE350266EF8, native_parameters!(vehicle, toggle));

    value
}

pub fn _force_vehicle_engine_audio(vehicle: i32, audioName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x4F0C413926060B38,
        native_parameters!(vehicle, audioName.as_ptr())
    );

    value
}

pub fn _preload_vehicle_audio(vehicleModel: u32) -> () {
    let value = native!((), 0xCA4CEA6AE0000A7E, native_parameters!(vehicleModel));

    value
}

pub fn _0xf1f8157b8c3f171c(vehicle: i32, p1: &std::ffi::CString, p2: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xF1F8157B8C3F171C,
        native_parameters!(vehicle, p1.as_ptr(), p2.as_ptr())
    );

    value
}

pub fn _0xd2dccd8e16e20997(p0: u32) -> () {
    let value = native!((), 0xD2DCCD8E16E20997, native_parameters!(p0));

    value
}

pub fn _0x5db8010ee71fdef2(vehicle: i32) -> bool {
    let value = native!(bool, 0x5DB8010EE71FDEF2, native_parameters!(vehicle));

    value
}

pub fn set_vehicle_audio_engine_damage_factor(vehicle: i32, damageFactor: f32) -> () {
    let value = native!(
        (),
        0x59E7B488451F4D3A,
        native_parameters!(vehicle, damageFactor)
    );

    value
}

pub fn set_vehicle_audio_body_damage_factor(vehicle: i32, p1: f32) -> () {
    let value = native!((), 0x01BB4D577D38BD9E, native_parameters!(vehicle, p1));

    value
}

pub fn _0x1c073274e065c6d2(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x1C073274E065C6D2, native_parameters!(vehicle, toggle));

    value
}

pub fn enable_vehicle_exhaust_pops(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x2BE4BC731D039D5A, native_parameters!(vehicle, toggle));

    value
}

pub fn set_vehicle_boost_active(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x4A04DE7CAB2739A1, native_parameters!(vehicle, toggle));

    value
}

pub fn _0x6fddad856e36988a(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0x6FDDAD856E36988A, native_parameters!(vehicle, toggle));

    value
}

pub fn set_script_update_door_audio(doorHash: u32, toggle: bool) -> () {
    let value = native!((), 0x06C0023BED16DD6B, native_parameters!(doorHash, toggle));

    value
}

pub fn play_vehicle_door_open_sound(vehicle: i32, doorIndex: i32) -> () {
    let value = native!(
        (),
        0x3A539D52857EA82D,
        native_parameters!(vehicle, doorIndex)
    );

    value
}

pub fn play_vehicle_door_close_sound(vehicle: i32, doorIndex: i32) -> () {
    let value = native!(
        (),
        0x62A456AA4769EF34,
        native_parameters!(vehicle, doorIndex)
    );

    value
}

pub fn enable_stall_warning_sounds(vehicle: i32, toggle: bool) -> () {
    let value = native!((), 0xC15907D667F7CFB2, native_parameters!(vehicle, toggle));

    value
}

pub fn is_game_in_control_of_music() -> bool {
    let value = native!(bool, 0x6D28DC1671E334FD, native_parameters!());

    value
}

pub fn set_gps_active(active: bool) -> () {
    let value = native!((), 0x3BD3F52BA9B1E4E8, native_parameters!(active));

    value
}

pub fn play_mission_complete_audio(audioName: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0xB138AAB8A70D3C69,
        native_parameters!(audioName.as_ptr())
    );

    value
}

pub fn is_mission_complete_playing() -> bool {
    let value = native!(bool, 0x19A30C23F5827F8A, native_parameters!());

    value
}

pub fn is_mission_complete_ready_for_ui() -> bool {
    let value = native!(bool, 0x6F259F82D873B8B8, native_parameters!());

    value
}

pub fn block_death_jingle(toggle: bool) -> () {
    let value = native!((), 0xF154B8D1775B2DEC, native_parameters!(toggle));

    value
}

pub fn start_audio_scene(scene: &std::ffi::CString) -> bool {
    let value = native!(bool, 0x013A80FC08F6E4F2, native_parameters!(scene.as_ptr()));

    value
}

pub fn stop_audio_scene(scene: &std::ffi::CString) -> () {
    let value = native!((), 0xDFE8422B3B94E688, native_parameters!(scene.as_ptr()));

    value
}

pub fn stop_audio_scenes() -> () {
    let value = native!((), 0xBAC7FC81A75EC1A1, native_parameters!());

    value
}

pub fn is_audio_scene_active(scene: &std::ffi::CString) -> bool {
    let value = native!(bool, 0xB65B60556E2A9225, native_parameters!(scene.as_ptr()));

    value
}

pub fn set_audio_scene_variable(
    scene: &std::ffi::CString,
    variable: &std::ffi::CString,
    value: f32,
) -> () {
    let value = native!(
        (),
        0xEF21A9EF089A2668,
        native_parameters!(scene.as_ptr(), variable.as_ptr(), value)
    );

    value
}

pub fn set_audio_script_cleanup_time(time: i32) -> () {
    let value = native!((), 0xA5F377B175A699C5, native_parameters!(time));

    value
}

pub fn add_entity_to_audio_mix_group(entity: i32, groupName: &std::ffi::CString, p2: f32) -> () {
    let value = native!(
        (),
        0x153973AB99FE8980,
        native_parameters!(entity, groupName.as_ptr(), p2)
    );

    value
}

pub fn remove_entity_from_audio_mix_group(entity: i32, p1: f32) -> () {
    let value = native!((), 0x18EB48CFC41F2EA0, native_parameters!(entity, p1));

    value
}

pub fn audio_is_scripted_music_playing() -> bool {
    let value = native!(bool, 0x845FFC3A4FEEFA3E, native_parameters!());

    value
}

pub fn _0x2dd39bf3e2f9c47f() -> u32 {
    let value = native!(u32, 0x2DD39BF3E2F9C47F, native_parameters!());

    value
}

pub fn prepare_music_event(eventName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x1E5185B72EF5158A,
        native_parameters!(eventName.as_ptr())
    );

    value
}

pub fn cancel_music_event(eventName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x5B17A90291133DA5,
        native_parameters!(eventName.as_ptr())
    );

    value
}

pub fn trigger_music_event(eventName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x706D57B0F50DA710,
        native_parameters!(eventName.as_ptr())
    );

    value
}

pub fn is_music_oneshot_playing() -> bool {
    let value = native!(bool, 0xA097AB275061FB21, native_parameters!());

    value
}

pub fn get_music_playtime() -> i32 {
    let value = native!(i32, 0xE7A0D23DC414507B, native_parameters!());

    value
}

pub fn _0x159b7318403a1cd8(p0: u32) -> () {
    let value = native!((), 0x159B7318403A1CD8, native_parameters!(p0));

    value
}

pub fn record_broken_glass(x: f32, y: f32, z: f32, radius: f32) -> () {
    let value = native!((), 0xFBE20329593DEC9D, native_parameters!(x, y, z, radius));

    value
}

pub fn clear_all_broken_glass() -> () {
    let value = native!((), 0xB32209EFFDC04913, native_parameters!());

    value
}

pub fn _0x70b8ec8fc108a634(p0: bool, p1: u32) -> () {
    let value = native!((), 0x70B8EC8FC108A634, native_parameters!(p0, p1));

    value
}

pub fn _0x149aee66f0cb3a99(p0: f32, p1: f32) -> () {
    let value = native!((), 0x149AEE66F0CB3A99, native_parameters!(p0, p1));

    value
}

pub fn _0x8bf907833be275de(p0: f32, p1: f32) -> () {
    let value = native!((), 0x8BF907833BE275DE, native_parameters!(p0, p1));

    value
}

pub fn _0x062d5ead4da2fa6a() -> () {
    let value = native!((), 0x062D5EAD4DA2FA6A, native_parameters!());

    value
}

pub fn prepare_alarm(alarmName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x9D74AE343DB65533,
        native_parameters!(alarmName.as_ptr())
    );

    value
}

pub fn start_alarm(alarmName: &std::ffi::CString, p2: bool) -> () {
    let value = native!(
        (),
        0x0355EF116C4C97B2,
        native_parameters!(alarmName.as_ptr(), p2)
    );

    value
}

pub fn stop_alarm(alarmName: &std::ffi::CString, toggle: bool) -> () {
    let value = native!(
        (),
        0xA1CADDCD98415A41,
        native_parameters!(alarmName.as_ptr(), toggle)
    );

    value
}

pub fn stop_all_alarms(stop: bool) -> () {
    let value = native!((), 0x2F794A877ADD4C92, native_parameters!(stop));

    value
}

pub fn is_alarm_playing(alarmName: &std::ffi::CString) -> bool {
    let value = native!(
        bool,
        0x226435CB96CCFC8C,
        native_parameters!(alarmName.as_ptr())
    );

    value
}

pub fn get_vehicle_default_horn(vehicle: i32) -> u32 {
    let value = native!(u32, 0x02165D55000219AC, native_parameters!(vehicle));

    value
}

pub fn get_vehicle_default_horn_ignore_mods(vehicle: i32) -> u32 {
    let value = native!(u32, 0xACB5DCCA1EC76840, native_parameters!(vehicle));

    value
}

pub fn reset_ped_audio_flags(ped: i32) -> () {
    let value = native!((), 0xF54BB7B61036F335, native_parameters!(ped));

    value
}

pub fn _set_ped_audio_footstep_loud(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x0653B735BFBDFE87, native_parameters!(ped, toggle));

    value
}

pub fn _set_ped_audio_footstep_quiet(ped: i32, toggle: bool) -> () {
    let value = native!((), 0x29DA3CA8D8B2692D, native_parameters!(ped, toggle));

    value
}

pub fn override_player_ground_material(hash: u32, toggle: bool) -> () {
    let value = native!((), 0xD2CC78CD3D0B50F9, native_parameters!(hash, toggle));

    value
}

pub fn _0xbf4dc1784be94dfa(ped: i32, p1: bool, hash: u32) -> () {
    let value = native!((), 0xBF4DC1784BE94DFA, native_parameters!(ped, p1, hash));

    value
}

pub fn _override_microphone_settings(hash: u32, toggle: bool) -> () {
    let value = native!((), 0x75773E11BA459E90, native_parameters!(hash, toggle));

    value
}

pub fn freeze_microphone() -> () {
    let value = native!((), 0xD57AAAE0E2214D11, native_parameters!());

    value
}

pub fn distant_cop_car_sirens(value: bool) -> () {
    let value = native!((), 0x552369F549563AD5, native_parameters!(value));

    value
}

pub fn _0x43fa0dfc5df87815(vehicle: i32, p1: bool) -> () {
    let value = native!((), 0x43FA0DFC5DF87815, native_parameters!(vehicle, p1));

    value
}

pub fn _0xb81cf134aeb56ffb() -> () {
    let value = native!((), 0xB81CF134AEB56FFB, native_parameters!());

    value
}

pub fn set_audio_flag(flagName: &std::ffi::CString, toggle: bool) -> () {
    let value = native!(
        (),
        0xB9EFD5C25018725A,
        native_parameters!(flagName.as_ptr(), toggle)
    );

    value
}

pub fn prepare_synchronized_audio_event(p0: &std::ffi::CString, p1: u32) -> u32 {
    let value = native!(u32, 0xC7ABCACA4985A766, native_parameters!(p0.as_ptr(), p1));

    value
}

pub fn prepare_synchronized_audio_event_for_scene(p0: u32, p1: *mut u32) -> bool {
    let value = native!(bool, 0x029FE7CD1B7E2E75, native_parameters!(p0, p1));

    value
}

pub fn play_synchronized_audio_event(p0: u32) -> bool {
    let value = native!(bool, 0x8B2FD4560E55DD2D, native_parameters!(p0));

    value
}

pub fn stop_synchronized_audio_event(p0: u32) -> bool {
    let value = native!(bool, 0x92D6A88E64A94430, native_parameters!(p0));

    value
}

pub fn _0xc8ede9bdbccba6d4(p0: *mut u32, p1: f32, p2: f32, p3: f32) -> () {
    let value = native!((), 0xC8EDE9BDBCCBA6D4, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn _set_synchronized_audio_event_position_this_frame(p0: &std::ffi::CString, p1: i32) -> () {
    let value = native!((), 0x950A154B8DAB6185, native_parameters!(p0.as_ptr(), p1));

    value
}

pub fn set_audio_special_effect_mode(mode: i32) -> () {
    let value = native!((), 0x12561FCBB62D5B9C, native_parameters!(mode));

    value
}

pub fn set_portal_settings_override(p0: &std::ffi::CString, p1: &std::ffi::CString) -> () {
    let value = native!(
        (),
        0x044DBAD7A7FA2BE5,
        native_parameters!(p0.as_ptr(), p1.as_ptr())
    );

    value
}

pub fn remove_portal_settings_override(p0: &std::ffi::CString) -> () {
    let value = native!((), 0xB4BBFD9CD8B3922B, native_parameters!(p0.as_ptr()));

    value
}

pub fn _0xe4e6dd5566d28c82() -> () {
    let value = native!((), 0xE4E6DD5566D28C82, native_parameters!());

    value
}

pub fn _0x3a48ab4445d499be() -> u32 {
    let value = native!(u32, 0x3A48AB4445D499BE, native_parameters!());

    value
}

pub fn _set_ped_talk(ped: i32) -> () {
    let value = native!((), 0x4ADA3F19BE4A6047, native_parameters!(ped));

    value
}

pub fn _0x0150b6ff25a9e2e5() -> () {
    let value = native!((), 0x0150B6FF25A9E2E5, native_parameters!());

    value
}

pub fn _0xbef34b1d9624d5dd(p0: bool) -> () {
    let value = native!((), 0xBEF34B1D9624D5DD, native_parameters!(p0));

    value
}

pub fn stop_cutscene_audio() -> () {
    let value = native!((), 0x806058BBDC136E06, native_parameters!());

    value
}

pub fn _has_multiplayer_audio_data_loaded() -> bool {
    let value = native!(bool, 0x544810ED9DB6BBE6, native_parameters!());

    value
}

pub fn _has_multiplayer_audio_data_unloaded() -> bool {
    let value = native!(bool, 0x5B50ABB1FE3746F4, native_parameters!());

    value
}

pub fn _get_vehicle_default_horn_variation(vehicle: i32) -> i32 {
    let value = native!(i32, 0xD53F3A29BCE2580E, native_parameters!(vehicle));

    value
}

pub fn _set_vehicle_horn_variation(vehicle: i32, value: i32) -> () {
    let value = native!((), 0x0350E7E17BA767D0, native_parameters!(vehicle, value));

    value
}
