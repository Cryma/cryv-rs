use crate::generic::GenericFunctionComponent;
use hook::natives::*;
use legion::systems::Builder;
use legion::*;
use log::{debug, error};

#[derive(Copy, Clone, Debug, PartialEq)]
enum EntityCleanupType {
    Ped,
    Vehicle,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct EntityCleanupData {
    cleanup_type: EntityCleanupType,
    last_run_at: std::time::SystemTime,
}

pub fn run_initial() {
    let player_ped_id = player::player_ped_id();
    entity::set_entity_coords_no_offset(player_ped_id, 412.4, -976.71, 29.43, false, false, false);

    cam::destroy_all_cams(true);
    script::set_no_loading_screen(true);

    dlc::on_enter_mp();

    let weather_type = std::ffi::CString::new("EXTRASUNNY").unwrap();
    misc::set_weather_type_now(&weather_type);

    clock::pause_clock(true);

    for i in 0..=50 {
        misc::disable_stunt_jump_set(i);
        misc::delete_stunt_jump(i);
    }
}

pub fn add_components(world: &mut World) {
    world.extend(vec![
        (GenericFunctionComponent {
            function: hijack_frontend_menu,
        },),
        (GenericFunctionComponent {
            function: run_cleanup_tick,
        },),
    ]);

    world.extend(vec![
        (EntityCleanupData {
            cleanup_type: EntityCleanupType::Ped,
            last_run_at: std::time::SystemTime::UNIX_EPOCH,
        },),
        (EntityCleanupData {
            cleanup_type: EntityCleanupType::Vehicle,
            last_run_at: std::time::SystemTime::UNIX_EPOCH,
        },),
    ]);
}

pub fn add_systems(builder: &mut Builder) {
    builder.add_thread_local(run_entity_cleanup_system());
}

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

fn run_cleanup_tick() {
    vehicle::set_random_trains(false);
    vehicle::set_random_boats(false);
    vehicle::set_number_of_parked_vehicles(-1);
    vehicle::set_parked_vehicle_density_multiplier_this_frame(0.0);
    vehicle::set_random_vehicle_density_multiplier_this_frame(0.0);
    vehicle::set_vehicle_density_multiplier_this_frame(0.0);
    vehicle::set_far_draw_vehicles(false);
    vehicle::set_all_low_priority_vehicle_generators_active(false);
    vehicle::set_distant_cars_enabled(false);

    ped::set_create_random_cops(false);
    ped::can_create_random_ped(false);
    ped::set_ped_density_multiplier_this_frame(0.0);
    ped::set_scenario_ped_density_multiplier_this_frame(0.0, 0.0);

    let player_id = player::player_id();
    player::set_auto_give_scuba_gear_when_exit_vehicle(player_id, false);
    player::set_auto_give_parachute_when_enter_plane(player_id, false);
    player::set_player_health_recharge_multiplier(player_id, 0.0);
    player::set_player_wanted_level(player_id, 0, false);
    player::set_player_wanted_level_now(player_id, false);
    player::set_max_wanted_level(0);

    pad::disable_control_action(0, 19, true); // INPUT_CHARACTER_WHEEL
    pad::disable_control_action(0, 27, true); // INPUT_PHONE
    pad::disable_control_action(0, 28, true); // INPUT_SPECIAL_ABILITY
    pad::disable_control_action(0, 29, true); // INPUT_SPECIAL_ABILITY_SECONDARY
    pad::disable_control_action(0, 36, true); // INPUT_DUCK
    pad::disable_control_action(0, 140, true); // INPUT_MELEE_ATTACK_LIGHT
    pad::disable_control_action(0, 171, true); // INPUT_SPECIAL_ABILITY_PC
    pad::disable_control_action(0, 212, true); // INPUT_FRONTEND_SOCIAL_CLUB
    pad::disable_control_action(0, 213, true); // INPUT_FRONTEND_SOCIAL_CLUB_SECONDARY
    pad::disable_control_action(0, 243, true); // INPUT_ENTER_CHEAT_CODE
}

#[system(for_each)]
fn run_entity_cleanup(data: &mut EntityCleanupData) {
    let now = std::time::SystemTime::now();

    match now.duration_since(data.last_run_at) {
        Ok(duration) => {
            if duration.as_millis() < 500 {
                return;
            }

            data.last_run_at = now;
        }
        Err(error) => {
            error!(
                "Error while running entity cleanup for type {:?}: {}",
                data.cleanup_type, error
            );

            return;
        }
    }

    let (_, entities) = match data.cleanup_type {
        EntityCleanupType::Ped => hook::get_all_peds(),
        EntityCleanupType::Vehicle => hook::get_all_vehicles(),
    };

    let mut deleted_entities = 0;

    for mut entity in entities {
        if entity::does_entity_exist(entity) == false {
            continue;
        }

        entity::set_entity_as_no_longer_needed(&mut entity);
        entity::set_entity_as_mission_entity(entity, false, true);

        entity::delete_entity(&mut entity);

        deleted_entities += 1;
    }

    if deleted_entities != 0 {
        debug!(
            "Deleted {} entities of type {:?}",
            deleted_entities, data.cleanup_type
        );
    }
}
