use crate::types::NativeVector3;

pub fn set_roads_in_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    nodeEnabled: bool,
    unknown2: bool,
) -> () {
    let value = native!(
        (),
        0xBF1A602B5BA52FEE,
        native_parameters!(x1, y1, z1, x2, y2, z2, nodeEnabled, unknown2)
    );

    value
}

pub fn set_roads_in_angled_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    width: f32,
    unknown1: bool,
    unknown2: bool,
    unknown3: bool,
) -> () {
    let value = native!(
        (),
        0x1A5AA1208AF5DB59,
        native_parameters!(x1, y1, z1, x2, y2, z2, width, unknown1, unknown2, unknown3)
    );

    value
}

pub fn set_ped_paths_in_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    unknown: bool,
    p7: u32,
) -> () {
    let value = native!(
        (),
        0x34F060F4BF92E018,
        native_parameters!(x1, y1, z1, x2, y2, z2, unknown, p7)
    );

    value
}

pub fn get_safe_coord_for_ped(
    x: f32,
    y: f32,
    z: f32,
    onGround: bool,
    outPosition: *mut NativeVector3,
    flags: i32,
) -> bool {
    let value = native!(
        bool,
        0xB61C8E878A4199CA,
        native_parameters!(x, y, z, onGround, outPosition, flags)
    );

    value
}

pub fn get_closest_vehicle_node(
    x: f32,
    y: f32,
    z: f32,
    outPosition: *mut NativeVector3,
    nodeType: i32,
    p5: f32,
    p6: f32,
) -> bool {
    let value = native!(
        bool,
        0x240A18690AE96513,
        native_parameters!(x, y, z, outPosition, nodeType, p5, p6)
    );

    value
}

pub fn get_closest_major_vehicle_node(
    x: f32,
    y: f32,
    z: f32,
    outPosition: *mut NativeVector3,
    unknown1: f32,
    unknown2: i32,
) -> bool {
    let value = native!(
        bool,
        0x2EABE3B06F58C1BE,
        native_parameters!(x, y, z, outPosition, unknown1, unknown2)
    );

    value
}

pub fn get_closest_vehicle_node_with_heading(
    x: f32,
    y: f32,
    z: f32,
    outPosition: *mut NativeVector3,
    outHeading: *mut f32,
    nodeType: i32,
    p6: f32,
    p7: i32,
) -> bool {
    let value = native!(
        bool,
        0xFF071FB798B803B0,
        native_parameters!(x, y, z, outPosition, outHeading, nodeType, p6, p7)
    );

    value
}

pub fn get_nth_closest_vehicle_node(
    x: f32,
    y: f32,
    z: f32,
    nthClosest: i32,
    outPosition: *mut NativeVector3,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
) -> bool {
    let value = native!(
        bool,
        0xE50E52416CCF948B,
        native_parameters!(
            x,
            y,
            z,
            nthClosest,
            outPosition,
            unknown1,
            unknown2,
            unknown3
        )
    );

    value
}

pub fn get_nth_closest_vehicle_node_id(
    x: f32,
    y: f32,
    z: f32,
    nth: i32,
    nodetype: i32,
    p5: f32,
    p6: f32,
) -> i32 {
    let value = native!(
        i32,
        0x22D7275A79FE8215,
        native_parameters!(x, y, z, nth, nodetype, p5, p6)
    );

    value
}

pub fn get_nth_closest_vehicle_node_with_heading(
    x: f32,
    y: f32,
    z: f32,
    nthClosest: i32,
    outPosition: *mut NativeVector3,
    outHeading: *mut f32,
    unknown1: *mut u32,
    unknown2: i32,
    unknown3: f32,
    unknown4: f32,
) -> bool {
    let value = native!(
        bool,
        0x80CA6A8B6C094CC4,
        native_parameters!(
            x,
            y,
            z,
            nthClosest,
            outPosition,
            outHeading,
            unknown1,
            unknown2,
            unknown3,
            unknown4
        )
    );

    value
}

pub fn get_nth_closest_vehicle_node_id_with_heading(
    x: f32,
    y: f32,
    z: f32,
    nthClosest: i32,
    outPosition: *mut NativeVector3,
    outHeading: *mut f32,
    p6: u32,
    p7: f32,
    p8: f32,
) -> i32 {
    let value = native!(
        i32,
        0x6448050E9C2A7207,
        native_parameters!(x, y, z, nthClosest, outPosition, outHeading, p6, p7, p8)
    );

    value
}

pub fn get_nth_closest_vehicle_node_favour_direction(
    x: f32,
    y: f32,
    z: f32,
    desiredX: f32,
    desiredY: f32,
    desiredZ: f32,
    nthClosest: i32,
    outPosition: *mut NativeVector3,
    outHeading: *mut f32,
    nodetype: i32,
    p10: f32,
    p11: u32,
) -> bool {
    let value = native!(
        bool,
        0x45905BE8654AE067,
        native_parameters!(
            x,
            y,
            z,
            desiredX,
            desiredY,
            desiredZ,
            nthClosest,
            outPosition,
            outHeading,
            nodetype,
            p10,
            p11
        )
    );

    value
}

pub fn get_vehicle_node_properties(
    x: f32,
    y: f32,
    z: f32,
    density: *mut i32,
    flags: *mut i32,
) -> bool {
    let value = native!(
        bool,
        0x0568566ACBB5DEDC,
        native_parameters!(x, y, z, density, flags)
    );

    value
}

pub fn is_vehicle_node_id_valid(vehicleNodeId: i32) -> bool {
    let value = native!(bool, 0x1EAF30FCFBF5AF74, native_parameters!(vehicleNodeId));

    value
}

pub fn get_vehicle_node_position(nodeId: i32, outPosition: *mut NativeVector3) -> () {
    let value = native!(
        (),
        0x703123E5E7D429C2,
        native_parameters!(nodeId, outPosition)
    );

    value
}

pub fn get_vehicle_node_is_gps_allowed(nodeID: i32) -> bool {
    let value = native!(bool, 0xA2AE5C478B96E3B6, native_parameters!(nodeID));

    value
}

pub fn get_vehicle_node_is_switched_off(nodeID: i32) -> bool {
    let value = native!(bool, 0x4F5070AA58F69279, native_parameters!(nodeID));

    value
}

pub fn get_closest_road(
    x: f32,
    y: f32,
    z: f32,
    p3: f32,
    p4: i32,
    p5: *mut NativeVector3,
    p6: *mut NativeVector3,
    p7: *mut u32,
    p8: *mut u32,
    p9: *mut f32,
    p10: bool,
) -> u32 {
    let value = native!(
        u32,
        0x132F52BBA570FE92,
        native_parameters!(x, y, z, p3, p4, p5, p6, p7, p8, p9, p10)
    );

    value
}

pub fn _set_all_paths_cache_boundingstruct(toggle: bool) -> () {
    let value = native!((), 0x228E5C6AD4D74BFD, native_parameters!(toggle));

    value
}

pub fn _set_ai_global_path_nodes_type(type_esc: i32) -> () {
    let value = native!((), 0xF74B1FFA4A15FBEA, native_parameters!(type_esc));

    value
}

pub fn are_nodes_loaded_for_area(x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
    let value = native!(bool, 0xF7B79A50B905A30D, native_parameters!(x1, y1, x2, y2));

    value
}

pub fn _request_paths_prefer_accurate_boundingstruct(x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
    let value = native!(bool, 0x07FB139B592FA687, native_parameters!(x1, y1, x2, y2));

    value
}

pub fn set_roads_back_to_original(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: u32,
) -> () {
    let value = native!(
        (),
        0x1EE7063B80FFC77C,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn set_roads_back_to_original_in_angled_area(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    width: f32,
    p7: u32,
) -> () {
    let value = native!(
        (),
        0x0027501B9F3B407E,
        native_parameters!(x1, y1, z1, x2, y2, z2, width, p7)
    );

    value
}

pub fn set_ambient_ped_range_multiplier_this_frame(multiplier: f32) -> () {
    let value = native!((), 0x0B919E1FB47CC4E0, native_parameters!(multiplier));

    value
}

pub fn _0xaa76052dda9bfc3e(p0: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32, p6: u32) -> () {
    let value = native!(
        (),
        0xAA76052DDA9BFC3E,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn set_ped_paths_back_to_original(
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
        0xE04B48F2CC926253,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn get_random_vehicle_node(
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    p4: bool,
    p5: bool,
    p6: bool,
    outPosition: *mut NativeVector3,
    nodeId: *mut i32,
) -> bool {
    let value = native!(
        bool,
        0x93E0DB8440B73A7D,
        native_parameters!(x, y, z, radius, p4, p5, p6, outPosition, nodeId)
    );

    value
}

pub fn get_street_name_at_coord(
    x: f32,
    y: f32,
    z: f32,
    streetName: *mut u32,
    crossingRoad: *mut u32,
) -> () {
    let value = native!(
        (),
        0x2EB41072B4C1E4C0,
        native_parameters!(x, y, z, streetName, crossingRoad)
    );

    value
}

pub fn generate_directions_to_coord(
    x: f32,
    y: f32,
    z: f32,
    p3: bool,
    direction: *mut i32,
    p5: *mut f32,
    distToNxJunction: *mut f32,
) -> i32 {
    let value = native!(
        i32,
        0xF90125F1F79ECDF8,
        native_parameters!(x, y, z, p3, direction, p5, distToNxJunction)
    );

    value
}

pub fn set_ignore_no_gps_flag(toggle: bool) -> () {
    let value = native!((), 0x72751156E7678833, native_parameters!(toggle));

    value
}

pub fn _set_ignore_secondary_route_nodes(toggle: bool) -> () {
    let value = native!((), 0x1FC289A0C3FF470F, native_parameters!(toggle));

    value
}

pub fn set_gps_disabled_zone(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z3: f32) -> () {
    let value = native!(
        (),
        0xDC20483CD3DD5201,
        native_parameters!(x1, y1, z1, x2, y2, z3)
    );

    value
}

pub fn get_gps_blip_route_length() -> i32 {
    let value = native!(i32, 0xBBB45C3CF5C8AA85, native_parameters!());

    value
}

pub fn _0xf3162836c28f9da5(p0: u32, p1: u32, p2: u32, p3: u32) -> u32 {
    let value = native!(u32, 0xF3162836C28F9DA5, native_parameters!(p0, p1, p2, p3));

    value
}

pub fn get_gps_blip_route_found() -> bool {
    let value = native!(bool, 0x869DAACBBE9FA006, native_parameters!());

    value
}

pub fn _get_road_side_point_with_heading(
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    outPosition: *mut NativeVector3,
) -> bool {
    let value = native!(
        bool,
        0xA0F8A7517A273C05,
        native_parameters!(x, y, z, heading, outPosition)
    );

    value
}

pub fn _get_point_on_road_side(
    x: f32,
    y: f32,
    z: f32,
    p3: i32,
    outPosition: *mut NativeVector3,
) -> bool {
    let value = native!(
        bool,
        0x16F46FB18C8009E4,
        native_parameters!(x, y, z, p3, outPosition)
    );

    value
}

pub fn is_point_on_road(x: f32, y: f32, z: f32, vehicle: i32) -> bool {
    let value = native!(
        bool,
        0x125BF4ABFC536B09,
        native_parameters!(x, y, z, vehicle)
    );

    value
}

pub fn get_next_gps_disabled_zone_index() -> i32 {
    let value = native!(i32, 0xD3A6A0EF48823A8C, native_parameters!());

    value
}

pub fn set_gps_disabled_zone_at_index(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
    index: i32,
) -> () {
    let value = native!(
        (),
        0xD0BC1C6FB18EE154,
        native_parameters!(x1, y1, z1, x2, y2, z2, index)
    );

    value
}

pub fn clear_gps_disabled_zone_at_index(index: i32) -> () {
    let value = native!((), 0x2801D0012266DF07, native_parameters!(index));

    value
}

pub fn add_navmesh_required_region(x: f32, y: f32, radius: f32) -> () {
    let value = native!((), 0x387EAD7EE42F6685, native_parameters!(x, y, radius));

    value
}

pub fn remove_navmesh_required_regions() -> () {
    let value = native!((), 0x916F0A3CDEC3445E, native_parameters!());

    value
}

pub fn _is_navmesh_required_region_owned_by_any_thread() -> bool {
    let value = native!(bool, 0x705A844002B39DC0, native_parameters!());

    value
}

pub fn disable_navmesh_in_area(
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
        0x4C8872D8CDBE1B8B,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6)
    );

    value
}

pub fn are_all_navmesh_regions_loaded() -> bool {
    let value = native!(bool, 0x8415D95B194A3AEA, native_parameters!());

    value
}

pub fn is_navmesh_loaded_in_area(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
    let value = native!(
        bool,
        0xF813C7E63F9062A5,
        native_parameters!(x1, y1, z1, x2, y2, z2)
    );

    value
}

pub fn _0x01708e8dd3ff8c65(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32) -> u32 {
    let value = native!(
        u32,
        0x01708E8DD3FF8C65,
        native_parameters!(p0, p1, p2, p3, p4, p5)
    );

    value
}

pub fn add_navmesh_blocking_object(
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: bool,
    p8: u32,
) -> u32 {
    let value = native!(
        u32,
        0xFCD5C8E06E502F5A,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn update_navmesh_blocking_object(
    p0: u32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
    p8: u32,
) -> () {
    let value = native!(
        (),
        0x109E99373F290687,
        native_parameters!(p0, p1, p2, p3, p4, p5, p6, p7, p8)
    );

    value
}

pub fn remove_navmesh_blocking_object(p0: u32) -> () {
    let value = native!((), 0x46399A7895957C0E, native_parameters!(p0));

    value
}

pub fn does_navmesh_blocking_object_exist(p0: u32) -> bool {
    let value = native!(bool, 0x0EAEB0DB4B132399, native_parameters!(p0));

    value
}

pub fn _get_heightmap_top_z_for_position(x: f32, y: f32) -> f32 {
    let value = native!(f32, 0x29C24BFBED8AB8FB, native_parameters!(x, y));

    value
}

pub fn _get_heightmap_top_z_for_area(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let value = native!(f32, 0x8ABE8608576D9CE3, native_parameters!(x1, y1, x2, y2));

    value
}

pub fn _get_heightmap_bottom_z_for_position(x: f32, y: f32) -> f32 {
    let value = native!(f32, 0x336511A34F2E5185, native_parameters!(x, y));

    value
}

pub fn _get_heightmap_bottom_z_for_area(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let value = native!(f32, 0x3599D741C9AC6310, native_parameters!(x1, y1, x2, y2));

    value
}

pub fn calculate_travel_distance_between_points(
    x1: f32,
    y1: f32,
    z1: f32,
    x2: f32,
    y2: f32,
    z2: f32,
) -> f32 {
    let value = native!(
        f32,
        0xADD95C7005C4A197,
        native_parameters!(x1, y1, z1, x2, y2, z2)
    );

    value
}
