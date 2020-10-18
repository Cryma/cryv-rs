use super::ConsoleModule;
use crate::wrapped_natives::*;
use legion::*;

pub(super) fn command_veh(
    world: &mut World,
    console: &mut ConsoleModule,
    arguments: &mut Vec<String>,
) {
    if arguments.len() < 1 {
        console.print_line("Please specifiy a vehicle model name.");

        return;
    }

    let model = arguments.pop().unwrap();

    let player_ped_id = hook::natives::player::player_ped_id();
    let position = hook::natives::entity::get_entity_coords(player_ped_id, true);
    let rotation = hook::natives::entity::get_entity_rotation(player_ped_id, 2);

    let id = vehicles::create_vehicle_with_model_name(
        &model,
        (position.x, position.y, position.z),
        (0.0, 0.0, rotation.z),
    );

    hook::natives::ped::set_ped_into_vehicle(player_ped_id, id, -1);

    world.push((crate::cleanup::Entity { id },));

    console.print_line(format!("Spawned vehicle ({}) with model: {}", id, model).as_str());
}

pub(super) fn command_rmveh(
    world: &mut World,
    console: &mut ConsoleModule,
    _arguments: &mut Vec<String>,
) {
    let player_ped_id = hook::natives::player::player_ped_id();
    let is_in_vehicle = hook::natives::ped::is_ped_in_any_vehicle(player_ped_id, false);

    if is_in_vehicle == false {
        console.print_line("You are not in any vehicle.");

        return;
    }

    let mut vehicle_id = hook::natives::ped::get_vehicle_ped_is_in(player_ped_id, false);
    // Copy the vehicle id, as GTA5 will clear the current one, after deleting the entity
    let vehicle_id_copy = vehicle_id.clone();

    entities::delete_entity(&mut vehicle_id);

    let mut vehicle_query = Read::<crate::cleanup::Entity>::query();
    let entities = vehicle_query.iter_chunks(world);

    let mut found_entity: Option<Entity> = None;

    for chunk in entities {
        for (entity, entity_data) in chunk.into_iter_entities() {
            if entity_data.id != vehicle_id_copy {
                continue;
            }

            found_entity = Some(entity);

            break;
        }
    }

    if let Some(x) = found_entity {
        world.remove(x);
    }
}
