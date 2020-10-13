use crate::generic::GenericFunctionComponent;
use hook::natives::*;
use legion::systems::Builder;
use legion::*;

pub fn run_initial() {
    let player_ped_id = player::player_ped_id();
    entity::set_entity_coords_no_offset(player_ped_id, 412.4, -976.71, 29.43, false, false, false);
}

pub fn add_components(world: &mut World) {
    let _entity = world.extend(vec![
        (GenericFunctionComponent {
            function: hijack_frontend_menu,
        },),
        (GenericFunctionComponent {
            function: cleanup_peds,
        },),
        (GenericFunctionComponent {
            function: cleanup_vehicles,
        },),
    ]);
}

pub fn add_systems(_builder: &mut Builder) {}

fn hijack_frontend_menu() {
    pad::disable_control_action(0, 199, true);
    pad::disable_control_action(0, 200, true);

    if pad::is_disabled_control_just_released(0, 199)
        || pad::is_disabled_control_just_released(0, 200)
    {
        hud::activate_frontend_menu(
            misc::get_hash_key(&std::ffi::CString::new("FE_MENU_VERSION_SP_PAUSE").unwrap()),
            false,
            -1,
        );
    }
}

fn cleanup_peds() {
    let (_, peds) = hook::get_all_peds();

    for mut ped in peds {
        if entity::does_entity_exist(ped) == false {
            continue;
        }

        entity::set_ped_as_no_longer_needed(&mut ped);
        entity::set_entity_as_mission_entity(ped, false, true);

        entity::delete_entity(&mut ped);
    }
}

fn cleanup_vehicles() {
    let (_, vehicles) = hook::get_all_vehicles();

    for mut vehicle in vehicles {
        if entity::does_entity_exist(vehicle) == false {
            continue;
        }

        entity::set_vehicle_as_no_longer_needed(&mut vehicle);
        entity::set_entity_as_mission_entity(vehicle, false, true);

        entity::delete_entity(&mut vehicle);
    }
}
