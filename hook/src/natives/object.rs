use crate::types::NativeVector3;

pub fn create_object(
    modelHash: u32,
    x: f32,
    y: f32,
    z: f32,
    isNetwork: bool,
    bScriptHostObj: bool,
    dynamic: bool,
) -> i32 {
    let value = native!(
        i32,
        0x509D5878EB39E842,
        native_parameters!(modelHash, x, y, z, isNetwork, bScriptHostObj, dynamic)
    );

    value
}

pub fn create_object_no_offset(
    modelHash: u32,
    x: f32,
    y: f32,
    z: f32,
    isNetwork: bool,
    bScriptHostObj: bool,
    dynamic: bool,
) -> i32 {
    let value = native!(
        i32,
        0x9A294B2138ABB884,
        native_parameters!(modelHash, x, y, z, isNetwork, bScriptHostObj, dynamic)
    );

    value
}

pub fn delete_object(object: *mut i32) -> () {
    let value = native!((), 0x539E0AE3E6634B9F, native_parameters!(object));

    value
}

pub fn place_object_on_ground_properly(object: i32) -> bool {
    let value = native!(bool, 0x58A850EAEE20FAA3, native_parameters!(object));

    value
}

pub fn _place_object_on_ground_properly_2(object: i32) -> bool {
    let value = native!(bool, 0xD76EEEF746057FD6, native_parameters!(object));

    value
}

pub fn _0xafe24e4d29249e4a(object: i32, p1: f32, p2: f32, p3: bool) -> bool {
    let value = native!(
        bool,
        0xAFE24E4D29249E4A,
        native_parameters!(object, p1, p2, p3)
    );

    value
}

pub fn slide_object(
    object: i32,
    toX: f32,
    toY: f32,
    toZ: f32,
    speedX: f32,
    speedY: f32,
    speedZ: f32,
    collision: bool,
) -> bool {
    let value = native!(
        bool,
        0x2FDFF4107B8C1147,
        native_parameters!(object, toX, toY, toZ, speedX, speedY, speedZ, collision)
    );

    value
}

pub fn set_object_targettable(object: i32, targettable: bool) -> () {
    let value = native!(
        (),
        0x8A7391690F5AFD81,
        native_parameters!(object, targettable)
    );

    value
}

pub fn _set_object_something(object: i32, p1: bool) -> () {
    let value = native!((), 0x77F33F2CCF64B3AA, native_parameters!(object, p1));

    value
}

pub fn get_closest_object_of_type(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    modelHash: u32,
    isMission: bool,
    p6: bool,
    p7: bool,
) -> i32 {
    let value = native!(
        i32,
        0xE143FA2249364369,
        native_parameters!(x, y, z, radius, modelHash, isMission, p6, p7)
    );

    value
}

pub fn has_object_been_broken(object: i32, p1: u32) -> bool {
    let value = native!(bool, 0x8ABFB70C49CC43E2, native_parameters!(object, p1));

    value
}

pub fn has_closest_object_of_type_been_broken(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    modelHash: u32,
    p5: u32,
) -> bool {
    let value = native!(
        bool,
        0x761B0E69AC4D007E,
        native_parameters!(p0, p1, p2, p3, modelHash, p5)
    );

    value
}

pub fn has_closest_object_of_type_been_completely_destroyed(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    modelHash: u32,
    p5: bool,
) -> bool {
    let value = native!(
        bool,
        0x46494A2475701343,
        native_parameters!(x, y, z, radius, modelHash, p5)
    );

    value
}

pub fn _0x2542269291c6ac84(p0: u32) -> u32 {
    let value = native!(u32, 0x2542269291C6AC84, native_parameters!(p0));

    value
}

pub fn _get_object_offset_from_coords(
    xPos: f32,
    yPos: f32,
    zPos: f32,
    heading: f32,
    xOffset: f32,
    yOffset: f32,
    zOffset: f32,
) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x163E252DE035A133,
        native_parameters!(xPos, yPos, zPos, heading, xOffset, yOffset, zOffset)
    );

    value
}

pub fn get_coords_and_rotation_of_closest_object_of_type(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    modelHash: u32,
    outPosition: *mut NativeVector3,
    outRotation: *mut NativeVector3,
    rotationOrder: i32,
) -> u32 {
    let value = native!(
        u32,
        0x163F8B586BC95F2A,
        native_parameters!(
            x,
            y,
            z,
            radius,
            modelHash,
            outPosition,
            outRotation,
            rotationOrder
        )
    );

    value
}

pub fn set_state_of_closest_door_of_type(
    type_esc: u32,
    x: f32,
    y: f32,
    z: f32,
    locked: bool,
    heading: f32,
    p6: bool,
) -> () {
    let value = native!(
        (),
        0xF82D8F1926A02C3D,
        native_parameters!(type_esc, x, y, z, locked, heading, p6)
    );

    value
}

pub fn get_state_of_closest_door_of_type(
    type_esc: u32,
    x: f32,
    y: f32,
    z: f32,
    locked: *mut bool,
    heading: *mut f32,
) -> () {
    let value = native!(
        (),
        0xEDC1A5B84AEF33FF,
        native_parameters!(type_esc, x, y, z, locked, heading)
    );

    value
}

pub fn _door_control(
    modelHash: u32,
    x: f32,
    y: f32,
    z: f32,
    locked: bool,
    xRotMult: f32,
    yRotMult: f32,
    zRotMult: f32,
) -> () {
    let value = native!(
        (),
        0x9B12F9A24FABEDB0,
        native_parameters!(modelHash, x, y, z, locked, xRotMult, yRotMult, zRotMult)
    );

    value
}

pub fn _0x006e4b040ed37ec3(p0: u32) -> () {
    let value = native!((), 0x006E4B040ED37EC3, native_parameters!(p0));

    value
}

pub fn add_door_to_system(
    doorHash: u32,
    modelHash: u32,
    x: f32,
    y: f32,
    z: f32,
    p5: bool,
    scriptDoor: bool,
    isLocal: bool,
) -> () {
    let value = native!(
        (),
        0x6F8838D03D1DC226,
        native_parameters!(doorHash, modelHash, x, y, z, p5, scriptDoor, isLocal)
    );

    value
}

pub fn remove_door_from_system(doorHash: u32) -> () {
    let value = native!((), 0x464D8E1427156FE4, native_parameters!(doorHash));

    value
}

pub fn door_system_set_door_state(
    doorHash: u32,
    state: i32,
    requestDoor: bool,
    forceUpdate: bool,
) -> () {
    let value = native!(
        (),
        0x6BAB9442830C7F53,
        native_parameters!(doorHash, state, requestDoor, forceUpdate)
    );

    value
}

pub fn door_system_get_door_state(doorHash: u32) -> i32 {
    let value = native!(i32, 0x160AA1B32F6139B8, native_parameters!(doorHash));

    value
}

pub fn door_system_get_door_pending_state(doorHash: u32) -> i32 {
    let value = native!(i32, 0x4BC2854478F3A749, native_parameters!(doorHash));

    value
}

pub fn door_system_set_automatic_rate(
    doorHash: u32,
    rate: f32,
    requestDoor: bool,
    forceUpdate: bool,
) -> () {
    let value = native!(
        (),
        0x03C27E13B42A0E82,
        native_parameters!(doorHash, rate, requestDoor, forceUpdate)
    );

    value
}

pub fn door_system_set_automatic_distance(
    doorHash: u32,
    distance: f32,
    requestDoor: bool,
    forceUpdate: bool,
) -> () {
    let value = native!(
        (),
        0x9BA001CB45CBF627,
        native_parameters!(doorHash, distance, requestDoor, forceUpdate)
    );

    value
}

pub fn door_system_set_open_ratio(
    doorHash: u32,
    ajar: f32,
    requestDoor: bool,
    forceUpdate: bool,
) -> () {
    let value = native!(
        (),
        0xB6E6FBA95C7324AC,
        native_parameters!(doorHash, ajar, requestDoor, forceUpdate)
    );

    value
}

pub fn _0xe851471aefc3374f(p0: u32) -> u32 {
    let value = native!(u32, 0xE851471AEFC3374F, native_parameters!(p0));

    value
}

pub fn door_system_get_open_ratio(doorHash: u32) -> f32 {
    let value = native!(f32, 0x65499865FCA6E5EC, native_parameters!(doorHash));

    value
}

pub fn door_system_set_spring_removed(
    doorHash: u32,
    removed: bool,
    requestDoor: bool,
    forceUpdate: bool,
) -> () {
    let value = native!(
        (),
        0xC485E07E4F0B7958,
        native_parameters!(doorHash, removed, requestDoor, forceUpdate)
    );

    value
}

pub fn door_system_set_hold_open(doorHash: u32, toggle: bool) -> () {
    let value = native!((), 0xD9B71952F78A2640, native_parameters!(doorHash, toggle));

    value
}

pub fn _0xa85a21582451e951(doorHash: u32, p1: bool) -> () {
    let value = native!((), 0xA85A21582451E951, native_parameters!(doorHash, p1));

    value
}

pub fn is_door_registered_with_system(doorHash: u32) -> bool {
    let value = native!(bool, 0xC153C43EA202C8C1, native_parameters!(doorHash));

    value
}

pub fn is_door_closed(doorHash: u32) -> bool {
    let value = native!(bool, 0xC531EE8A1145A149, native_parameters!(doorHash));

    value
}

pub fn _0xc7f29ca00f46350e(p0: bool) -> () {
    let value = native!((), 0xC7F29CA00F46350E, native_parameters!(p0));

    value
}

pub fn _0x701fda1e82076ba4() -> () {
    let value = native!((), 0x701FDA1E82076BA4, native_parameters!());

    value
}

pub fn door_system_get_is_physics_loaded(p0: u32) -> bool {
    let value = native!(bool, 0xDF97CDD4FC08FD34, native_parameters!(p0));

    value
}

pub fn door_system_find_existing_door(
    x: f32,
    y: f32,
    z: f32,
    modelHash: u32,
    outDoorHash: *mut u32,
) -> bool {
    let value = native!(
        bool,
        0x589F80B325CC82C5,
        native_parameters!(x, y, z, modelHash, outDoorHash)
    );

    value
}

pub fn is_garage_empty(garageHash: u32, p1: bool, p2: i32) -> bool {
    let value = native!(
        bool,
        0x90E47239EA1980B8,
        native_parameters!(garageHash, p1, p2)
    );

    value
}

pub fn is_player_entirely_inside_garage(garageHash: u32, player: i32, p2: f32, p3: i32) -> bool {
    let value = native!(
        bool,
        0x024A60DEB0EA69F0,
        native_parameters!(garageHash, player, p2, p3)
    );

    value
}

pub fn is_player_partially_inside_garage(garageHash: u32, player: i32, p2: i32) -> bool {
    let value = native!(
        bool,
        0x1761DC5D8471CBAA,
        native_parameters!(garageHash, player, p2)
    );

    value
}

pub fn are_entities_entirely_inside_garage(
    garageHash: u32,
    p1: bool,
    p2: bool,
    p3: bool,
    p4: u32,
) -> bool {
    let value = native!(
        bool,
        0x85B6C850546FDDE2,
        native_parameters!(garageHash, p1, p2, p3, p4)
    );

    value
}

pub fn is_any_entity_entirely_inside_garage(
    garageHash: u32,
    p1: bool,
    p2: bool,
    p3: bool,
    p4: u32,
) -> bool {
    let value = native!(
        bool,
        0x673ED815D6E323B7,
        native_parameters!(garageHash, p1, p2, p3, p4)
    );

    value
}

pub fn is_object_entirely_inside_garage(garageHash: u32, entity: i32, p2: f32, p3: i32) -> bool {
    let value = native!(
        bool,
        0x372EF6699146A1E4,
        native_parameters!(garageHash, entity, p2, p3)
    );

    value
}

pub fn is_object_partially_inside_garage(garageHash: u32, entity: i32, p2: i32) -> bool {
    let value = native!(
        bool,
        0xF0EED5A6BC7B237A,
        native_parameters!(garageHash, entity, p2)
    );

    value
}

pub fn _clear_garage_area(garageHash: u32, isNetwork: bool) -> () {
    let value = native!(
        (),
        0xDA05194260CDCDF9,
        native_parameters!(garageHash, isNetwork)
    );

    value
}

pub fn _0x190428512b240692(
    garageHash: u32,
    vehicles: bool,
    peds: bool,
    objects: bool,
    isNetwork: bool,
) -> () {
    let value = native!(
        (),
        0x190428512B240692,
        native_parameters!(garageHash, vehicles, peds, objects, isNetwork)
    );

    value
}

pub fn _0x659f9d71f52843f8(id: i32, toggle: bool) -> () {
    let value = native!((), 0x659F9D71F52843F8, native_parameters!(id, toggle));

    value
}

pub fn enable_saving_in_garage(garageHash: u32, toggle: bool) -> () {
    let value = native!(
        (),
        0xF2E1A7133DD356A6,
        native_parameters!(garageHash, toggle)
    );

    value
}

pub fn _0x66a49d021870fe88() -> () {
    let value = native!((), 0x66A49D021870FE88, native_parameters!());

    value
}

pub fn does_object_of_type_exist_at_coords(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    hash: u32,
    p5: bool,
) -> bool {
    let value = native!(
        bool,
        0xBFA48E2FF417213F,
        native_parameters!(x, y, z, radius, hash, p5)
    );

    value
}

pub fn is_point_in_angled_area(
    xPos: f32,
    yPos: f32,
    zPos: f32,
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    width: f32,
    debug: bool,
    includeZ: bool,
) -> bool {
    let value = native!(
        bool,
        0x2A70BAE8883E4C81,
        native_parameters!(xPos, yPos, zPos, x1, y1, z1, x2, y2, z2, width, debug, includeZ)
    );

    value
}

pub fn set_object_allow_low_lod_buoyancy(object: i32, toggle: bool) -> () {
    let value = native!((), 0x4D89D607CB3DD1D2, native_parameters!(object, toggle));

    value
}

pub fn set_object_physics_params(
    object: i32,
    weight: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    gravity: f32,
    p7: f32,
    p8: f32,
    p9: f32,
    p10: f32,
    buoyancy: f32,
) -> () {
    let value = native!(
        (),
        0xF6DF6E90DE7DF90F,
        native_parameters!(object, weight, p2, p3, p4, p5, gravity, p7, p8, p9, p10, buoyancy)
    );

    value
}

pub fn get_object_fragment_damage_health(p0: u32, p1: bool) -> f32 {
    let value = native!(f32, 0xB6FBFD079B8D0596, native_parameters!(p0, p1));

    value
}

pub fn set_activate_object_physics_as_soon_as_it_is_unfrozen(object: i32, toggle: bool) -> () {
    let value = native!((), 0x406137F8EF90EAF5, native_parameters!(object, toggle));

    value
}

pub fn is_any_object_near_point(x: f32, y: f32, z: f32, range: f32, p4: bool) -> bool {
    let value = native!(
        bool,
        0x397DC58FF00298D1,
        native_parameters!(x, y, z, range, p4)
    );

    value
}

pub fn is_object_near_point(objectHash: u32, x: f32, y: f32, z: f32, range: f32) -> bool {
    let value = native!(
        bool,
        0x8C90FE4B381BA60A,
        native_parameters!(objectHash, x, y, z, range)
    );

    value
}

pub fn remove_object_high_detail_model(object: i32) -> () {
    let value = native!((), 0x4A39DB43E47CF3AA, native_parameters!(object));

    value
}

pub fn break_object_fragment_child(p0: i32, p1: u32, p2: bool) -> () {
    let value = native!((), 0xE7E4C198B0185900, native_parameters!(p0, p1, p2));

    value
}

pub fn _0xe05f6aeefeb0bb02(p0: u32, p1: u32, p2: u32) -> () {
    let value = native!((), 0xE05F6AEEFEB0BB02, native_parameters!(p0, p1, p2));

    value
}

pub fn _0xf9c1681347c8bd15(object: i32) -> () {
    let value = native!((), 0xF9C1681347C8BD15, native_parameters!(object));

    value
}

pub fn track_object_visibility(object: i32) -> () {
    let value = native!((), 0xB252BC036B525623, native_parameters!(object));

    value
}

pub fn is_object_visible(object: i32) -> bool {
    let value = native!(bool, 0x8B32ACE6326A7546, native_parameters!(object));

    value
}

pub fn _0xc6033d32241f6fb5(object: i32, toggle: bool) -> () {
    let value = native!((), 0xC6033D32241F6FB5, native_parameters!(object, toggle));

    value
}

pub fn _0xeb6f1a9b5510a5d2(p0: u32, p1: bool) -> () {
    let value = native!((), 0xEB6F1A9B5510A5D2, native_parameters!(p0, p1));

    value
}

pub fn _set_unk_global_bool_related_to_damage(value: bool) -> () {
    let value = native!((), 0xABDABF4E1EDECBFA, native_parameters!(value));

    value
}

pub fn _set_create_weapon_object_light_source(object: i32, toggle: bool) -> () {
    let value = native!((), 0xBCE595371A5FBAAF, native_parameters!(object, toggle));

    value
}

pub fn get_rayfire_map_object(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    name: &std::ffi::CString,
) -> i32 {
    let value = native!(
        i32,
        0xB48FCED898292E52,
        native_parameters!(x, y, z, radius, name.as_ptr())
    );

    value
}

pub fn set_state_of_rayfire_map_object(object: i32, state: i32) -> () {
    let value = native!((), 0x5C29F698D404C5E1, native_parameters!(object, state));

    value
}

pub fn get_state_of_rayfire_map_object(object: i32) -> i32 {
    let value = native!(i32, 0x899BA936634A322E, native_parameters!(object));

    value
}

pub fn does_rayfire_map_object_exist(object: i32) -> bool {
    let value = native!(bool, 0x52AF537A0C5B8AAD, native_parameters!(object));

    value
}

pub fn get_rayfire_map_object_anim_phase(object: i32) -> f32 {
    let value = native!(f32, 0x260EE4FDBDF4DB01, native_parameters!(object));

    value
}

pub fn create_pickup(
    pickupHash: u32,
    posX: f32,
    posY: f32,
    posZ: f32,
    p4: i32,
    value: i32,
    p6: bool,
    modelHash: u32,
) -> i32 {
    let value = native!(
        i32,
        0xFBA08C503DD5FA58,
        native_parameters!(pickupHash, posX, posY, posZ, p4, value, p6, modelHash)
    );

    value
}

pub fn create_pickup_rotate(
    pickupHash: u32,
    posX: f32,
    posY: f32,
    posZ: f32,
    rotX: f32,
    rotY: f32,
    rotZ: f32,
    flag: i32,
    amount: i32,
    p9: u32,
    p10: bool,
    modelHash: u32,
) -> i32 {
    let value = native!(
        i32,
        0x891804727E0A98B7,
        native_parameters!(
            pickupHash, posX, posY, posZ, rotX, rotY, rotZ, flag, amount, p9, p10, modelHash
        )
    );

    value
}

pub fn _0x394cd08e31313c28() -> () {
    let value = native!((), 0x394CD08E31313C28, native_parameters!());

    value
}

pub fn _0x826d1ee4d1cafc78(p0: u32, p1: u32) -> () {
    let value = native!((), 0x826D1EE4D1CAFC78, native_parameters!(p0, p1));

    value
}

pub fn create_ambient_pickup(
    pickupHash: u32,
    posX: f32,
    posY: f32,
    posZ: f32,
    flags: i32,
    value: i32,
    modelHash: u32,
    p7: bool,
    p8: bool,
) -> i32 {
    let value = native!(
        i32,
        0x673966A0C0FD7171,
        native_parameters!(pickupHash, posX, posY, posZ, flags, value, modelHash, p7, p8)
    );

    value
}

pub fn _0x1e3f1b1b891a2aaa(p0: u32, p1: u32) -> () {
    let value = native!((), 0x1E3F1B1B891A2AAA, native_parameters!(p0, p1));

    value
}

pub fn create_portable_pickup(
    pickupHash: u32,
    x: f32,
    y: f32,
    z: f32,
    placeOnGround: bool,
    modelHash: u32,
) -> i32 {
    let value = native!(
        i32,
        0x2EAF1FDB2FB55698,
        native_parameters!(pickupHash, x, y, z, placeOnGround, modelHash)
    );

    value
}

pub fn _create_portable_pickup_2(
    pickupHash: u32,
    x: f32,
    y: f32,
    z: f32,
    placeOnGround: bool,
    modelHash: u32,
) -> i32 {
    let value = native!(
        i32,
        0x125494B98A21AAF7,
        native_parameters!(pickupHash, x, y, z, placeOnGround, modelHash)
    );

    value
}

pub fn attach_portable_pickup_to_ped(pickupObject: i32, ped: i32) -> () {
    let value = native!(
        (),
        0x8DC39368BDD57755,
        native_parameters!(pickupObject, ped)
    );

    value
}

pub fn detach_portable_pickup_from_ped(pickupObject: i32) -> () {
    let value = native!((), 0xCF463D1E9A0AECB1, native_parameters!(pickupObject));

    value
}

pub fn _hide_pickup(pickupObject: i32, toggle: bool) -> () {
    let value = native!(
        (),
        0x867458251D47CCB2,
        native_parameters!(pickupObject, toggle)
    );

    value
}

pub fn set_max_num_portable_pickups_carried_by_player(modelHash: u32, p1: i32) -> () {
    let value = native!((), 0x0BF3B3BD47D79C08, native_parameters!(modelHash, p1));

    value
}

pub fn set_local_player_can_collect_portable_pickups(p0: bool) -> () {
    let value = native!((), 0x78857FC65CADB909, native_parameters!(p0));

    value
}

pub fn get_safe_pickup_coords(x: f32, y: f32, z: f32, p3: f32, p4: f32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x6E16BC2503FF1FF0,
        native_parameters!(x, y, z, p3, p4)
    );

    value
}

pub fn _0xd4a7a435b3710d05(x: f32, y: f32, z: f32, radius: f32) -> () {
    let value = native!((), 0xD4A7A435B3710D05, native_parameters!(x, y, z, radius));

    value
}

pub fn _0xb7c6d80fb371659a() -> () {
    let value = native!((), 0xB7C6D80FB371659A, native_parameters!());

    value
}

pub fn get_pickup_coords(pickup: i32) -> NativeVector3 {
    let value = native!(
        NativeVector3,
        0x225B8B35C88029B3,
        native_parameters!(pickup)
    );

    value
}

pub fn _0x8dca505a5c196f05(p0: u32, p1: u32) -> () {
    let value = native!((), 0x8DCA505A5C196F05, native_parameters!(p0, p1));

    value
}

pub fn remove_all_pickups_of_type(pickupHash: u32) -> () {
    let value = native!((), 0x27F9D613092159CF, native_parameters!(pickupHash));

    value
}

pub fn has_pickup_been_collected(pickup: i32) -> bool {
    let value = native!(bool, 0x80EC48E6679313F9, native_parameters!(pickup));

    value
}

pub fn remove_pickup(pickup: i32) -> () {
    let value = native!((), 0x3288D8ACAECD2AB2, native_parameters!(pickup));

    value
}

pub fn create_money_pickups(x: f32, y: f32, z: f32, value: i32, amount: i32, model: u32) -> () {
    let value = native!(
        (),
        0x0589B5E791CE9B2B,
        native_parameters!(x, y, z, value, amount, model)
    );

    value
}

pub fn does_pickup_exist(pickup: i32) -> bool {
    let value = native!(bool, 0xAFC1CA75AD4074D1, native_parameters!(pickup));

    value
}

pub fn does_pickup_object_exist(pickupObject: i32) -> bool {
    let value = native!(bool, 0xD9EFB6DBF7DAAEA3, native_parameters!(pickupObject));

    value
}

pub fn get_pickup_object(pickup: i32) -> i32 {
    let value = native!(i32, 0x5099BC55630B25AE, native_parameters!(pickup));

    value
}

pub fn is_object_a_portable_pickup(object: i32) -> bool {
    let value = native!(bool, 0xFC481C641EBBD27D, native_parameters!(object));

    value
}

pub fn is_object_a_pickup(object: i32) -> bool {
    let value = native!(bool, 0x0378C08504160D0D, native_parameters!(object));

    value
}

pub fn does_pickup_of_type_exist_in_area(
    pickupHash: u32,
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
) -> bool {
    let value = native!(
        bool,
        0xF9C36251F6E48E33,
        native_parameters!(pickupHash, x, y, z, radius)
    );

    value
}

pub fn set_pickup_regeneration_time(pickup: i32, duration: i32) -> () {
    let value = native!((), 0x78015C9B4B3ECC9D, native_parameters!(pickup, duration));

    value
}

pub fn force_pickup_regenerate(p0: u32) -> () {
    let value = native!((), 0x758A5C1B3B1E1990, native_parameters!(p0));

    value
}

pub fn _toggle_use_pickups_for_player(player: i32, pickupHash: u32, toggle: bool) -> () {
    let value = native!(
        (),
        0x616093EC6B139DD9,
        native_parameters!(player, pickupHash, toggle)
    );

    value
}

pub fn _set_local_player_can_use_pickups_with_this_model(modelHash: u32, toggle: bool) -> () {
    let value = native!(
        (),
        0x88EAEC617CD26926,
        native_parameters!(modelHash, toggle)
    );

    value
}

pub fn _0xfdc07c58e8aab715(pickupHash: u32) -> () {
    let value = native!((), 0xFDC07C58E8AAB715, native_parameters!(pickupHash));

    value
}

pub fn set_team_pickup_object(object: i32, p1: u32, p2: bool) -> () {
    let value = native!((), 0x53E0DF1A2A3CF0CA, native_parameters!(object, p1, p2));

    value
}

pub fn prevent_collection_of_portable_pickup(object: i32, p1: bool, p2: bool) -> () {
    let value = native!((), 0x92AEFB5F6E294023, native_parameters!(object, p1, p2));

    value
}

pub fn _0x0596843b34b95ce5(p0: u32, p1: u32) -> () {
    let value = native!((), 0x0596843B34B95CE5, native_parameters!(p0, p1));

    value
}

pub fn _0xa08fe5e49bdc39dd(p0: u32, p1: f32, p2: bool) -> () {
    let value = native!((), 0xA08FE5E49BDC39DD, native_parameters!(p0, p1, p2));

    value
}

pub fn _0x62454a641b41f3c5(p0: u32) -> () {
    let value = native!((), 0x62454A641B41F3C5, native_parameters!(p0));

    value
}

pub fn _0x39a5fb7eaf150840(p0: u32, p1: u32) -> () {
    let value = native!((), 0x39A5FB7EAF150840, native_parameters!(p0, p1));

    value
}

pub fn _0xdb41d07a45a6d4b7(p0: u32) -> u32 {
    let value = native!(u32, 0xDB41D07A45A6D4B7, native_parameters!(p0));

    value
}

pub fn set_pickup_generation_range_multiplier(multiplier: f32) -> () {
    let value = native!((), 0x318516E02DE3ECE2, native_parameters!(multiplier));

    value
}

pub fn _get_pickup_generation_range_multiplier() -> f32 {
    let value = native!(f32, 0xB3ECA65C7317F174, native_parameters!());

    value
}

pub fn _0x31f924b53eaddf65(p0: bool) -> () {
    let value = native!((), 0x31F924B53EADDF65, native_parameters!(p0));

    value
}

pub fn set_pickup_uncollectable(p0: u32, p1: u32) -> () {
    let value = native!((), 0x1C1B69FAE509BA97, native_parameters!(p0, p1));

    value
}

pub fn _0x858ec9fd25de04aa(p0: u32, p1: u32) -> () {
    let value = native!((), 0x858EC9FD25DE04AA, native_parameters!(p0, p1));

    value
}

pub fn set_pickup_hidden_when_uncollectable(p0: u32, p1: u32) -> () {
    let value = native!((), 0x3ED2B83AB2E82799, native_parameters!(p0, p1));

    value
}

pub fn _0x8881c98a31117998(p0: u32, p1: u32) -> () {
    let value = native!((), 0x8881C98A31117998, native_parameters!(p0, p1));

    value
}

pub fn _0x8cff648fbd7330f1(p0: u32) -> () {
    let value = native!((), 0x8CFF648FBD7330F1, native_parameters!(p0));

    value
}

pub fn _0x46f3add1e2d5baf2(p0: u32, p1: u32) -> () {
    let value = native!((), 0x46F3ADD1E2D5BAF2, native_parameters!(p0, p1));

    value
}

pub fn _0x641f272b52e2f0f8(p0: u32, p1: u32) -> () {
    let value = native!((), 0x641F272B52E2F0F8, native_parameters!(p0, p1));

    value
}

pub fn _0x4c134b4df76025d0(pickup: i32, toggle: bool) -> () {
    let value = native!((), 0x4C134B4DF76025D0, native_parameters!(pickup, toggle));

    value
}

pub fn _0xaa059c615de9dd03(pickup: i32, toggle: bool) -> () {
    let value = native!((), 0xAA059C615DE9DD03, native_parameters!(pickup, toggle));

    value
}

pub fn _0xf92099527db8e2a7(p0: u32, p1: u32) -> () {
    let value = native!((), 0xF92099527DB8E2A7, native_parameters!(p0, p1));

    value
}

pub fn _0xa2c1f5e92afe49ed() -> () {
    let value = native!((), 0xA2C1F5E92AFE49ED, native_parameters!());

    value
}

pub fn _0x762db2d380b48d04(p0: u32) -> () {
    let value = native!((), 0x762DB2D380B48D04, native_parameters!(p0));

    value
}

pub fn render_fake_pickup_glow(x: f32, y: f32, z: f32, colorIndex: i32) -> () {
    let value = native!(
        (),
        0x3430676B11CDF21D,
        native_parameters!(x, y, z, colorIndex)
    );

    value
}

pub fn _0x7813e8b8c4ae4799(pickup: i32) -> () {
    let value = native!((), 0x7813E8B8C4AE4799, native_parameters!(pickup));

    value
}

pub fn _0xbffe53ae7e67fcdc(pickup: i32, toggle: bool) -> () {
    let value = native!((), 0xBFFE53AE7E67FCDC, native_parameters!(pickup, toggle));

    value
}

pub fn _0xd05a3241b9a86f19(entity: i32, toggle: bool) -> () {
    let value = native!((), 0xD05A3241B9A86F19, native_parameters!(entity, toggle));

    value
}

pub fn _0xb2d0bde54f0e8e5a(object: i32, toggle: bool) -> () {
    let value = native!((), 0xB2D0BDE54F0E8E5A, native_parameters!(object, toggle));

    value
}

pub fn get_weapon_type_from_pickup_type(pickupHash: u32) -> u32 {
    let value = native!(u32, 0x08F96CA6C551AD51, native_parameters!(pickupHash));

    value
}

pub fn _get_pickup_hash_from_weapon(weaponHash: u32) -> u32 {
    let value = native!(u32, 0xD6429A016084F1A5, native_parameters!(weaponHash));

    value
}

pub fn is_pickup_weapon_object_valid(object: i32) -> bool {
    let value = native!(bool, 0x11D1E53A726891FE, native_parameters!(object));

    value
}

pub fn _get_object_texture_variation(object: i32) -> i32 {
    let value = native!(i32, 0xE84EB93729C5F36A, native_parameters!(object));

    value
}

pub fn _set_object_texture_variation(object: i32, textureVariation: i32) -> () {
    let value = native!(
        (),
        0x971DA0055324D033,
        native_parameters!(object, textureVariation)
    );

    value
}

pub fn _set_texture_variation_of_closest_object_of_type(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    modelHash: u32,
    textureVariation: i32,
) -> bool {
    let value = native!(
        bool,
        0xF12E33034D887F66,
        native_parameters!(x, y, z, radius, modelHash, textureVariation)
    );

    value
}

pub fn _0x31574b1b41268673(p0: u32, p1: u32) -> () {
    let value = native!((), 0x31574B1B41268673, native_parameters!(p0, p1));

    value
}

pub fn _set_object_light_color(object: i32, p1: bool, r: i32, g: i32, b: i32) -> u32 {
    let value = native!(
        u32,
        0x5F048334B4A4E774,
        native_parameters!(object, p1, r, g, b)
    );

    value
}

pub fn _0xadf084fb8f075d06(object: i32) -> bool {
    let value = native!(bool, 0xADF084FB8F075D06, native_parameters!(object));

    value
}

pub fn _0x3b2fd68db5f8331c(object: i32, toggle: bool) -> () {
    let value = native!((), 0x3B2FD68DB5F8331C, native_parameters!(object, toggle));

    value
}

pub fn _set_object_stunt_prop_speedup(object: i32, p1: u32) -> () {
    let value = native!((), 0x96EE0EBA0163DF80, native_parameters!(object, p1));

    value
}

pub fn _set_object_stunt_prop_duration(object: i32, duration: f32) -> () {
    let value = native!((), 0xDF6CA0330F2E737B, native_parameters!(object, duration));

    value
}

pub fn _get_pickup_hash(pickupHash: u32) -> u32 {
    let value = native!(u32, 0x5EAAD83F8CFB4575, native_parameters!(pickupHash));

    value
}

pub fn set_force_object_this_frame(x: f32, y: f32, z: f32, p3: f32) -> () {
    let value = native!((), 0xF538081986E49E9D, native_parameters!(x, y, z, p3));

    value
}

pub fn _mark_object_for_deletion(object: i32) -> () {
    let value = native!((), 0xADBE4809F19F927A, native_parameters!(object));

    value
}

pub fn _0x8caab2bd3ea58bd4(p0: u32) -> () {
    let value = native!((), 0x8CAAB2BD3EA58BD4, native_parameters!(p0));

    value
}

pub fn _0x63ecf581bc70e363(p0: u32, p1: u32) -> () {
    let value = native!((), 0x63ECF581BC70E363, native_parameters!(p0, p1));

    value
}

pub fn _set_enable_arena_prop_physics(object: i32, toggle: bool, p2: i32) -> () {
    let value = native!(
        (),
        0x911024442F4898F0,
        native_parameters!(object, toggle, p2)
    );

    value
}

pub fn _set_enable_arena_prop_physics_on_ped(object: i32, toggle: bool, p2: i32, ped: i32) -> () {
    let value = native!(
        (),
        0xB20834A7DD3D8896,
        native_parameters!(object, toggle, p2, ped)
    );

    value
}

pub fn _0x734e1714d077da9a(object: i32, toggle: bool) -> () {
    let value = native!((), 0x734E1714D077DA9A, native_parameters!(object, toggle));

    value
}

pub fn _0x1a6cbb06e2d0d79d(object: i32, p1: bool) -> () {
    let value = native!((), 0x1A6CBB06E2D0D79D, native_parameters!(object, p1));

    value
}

pub fn _get_is_arena_prop_physics_disabled(object: i32, p1: u32) -> bool {
    let value = native!(bool, 0x43C677F1E1158005, native_parameters!(object, p1));

    value
}

pub fn _0x3bd770d281982db5(p0: u32, p1: u32) -> u32 {
    let value = native!(u32, 0x3BD770D281982DB5, native_parameters!(p0, p1));

    value
}

pub fn _0x1c57c94a6446492a(object: i32, toggle: bool) -> () {
    let value = native!((), 0x1C57C94A6446492A, native_parameters!(object, toggle));

    value
}

pub fn _0xb5b7742424bd4445(object: i32, toggle: bool) -> () {
    let value = native!((), 0xB5B7742424BD4445, native_parameters!(object, toggle));

    value
}
