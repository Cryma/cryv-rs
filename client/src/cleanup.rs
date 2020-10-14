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
}

pub fn add_components(world: &mut World) {
    world.push((GenericFunctionComponent {
        function: hijack_frontend_menu,
    },));

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
