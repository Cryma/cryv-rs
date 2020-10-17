use super::{print_line, ConsoleData};
use crate::wrapped_natives::*;
use legion::systems::CommandBuffer;
use legion::*;

pub(super) fn command_veh(
    command_buffer: &mut CommandBuffer,
    console_data: &mut ConsoleData,
    arguments: &mut Vec<String>,
) {
    if arguments.len() < 1 {
        print_line(console_data, "Please specifiy a vehicle model name.");

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

    command_buffer.push((crate::cleanup::Entity { id },));

    print_line(
        console_data,
        format!("Spawned vehicle ({}) with model: {}", id, model).as_str(),
    );
}

pub(super) fn command_rmveh(
    command_buffer: &mut CommandBuffer,
    console_data: &mut ConsoleData,
    _arguments: &mut Vec<String>,
) {
    let player_ped_id = hook::natives::player::player_ped_id();
    let is_in_vehicle = hook::natives::ped::is_ped_in_any_vehicle(player_ped_id, false);

    if is_in_vehicle == false {
        print_line(console_data, "You are not in any vehicle.");

        return;
    }

    let mut vehicle_id = hook::natives::ped::get_vehicle_ped_is_in(player_ped_id, false);
    // Copy the vehicle id, as GTA5 will clear the current one, after deleting the entity
    let vehicle_id_copy = vehicle_id.clone();

    entities::delete_entity(&mut vehicle_id);

    command_buffer.exec_mut(move |world| {
        // TODO: Try to find a better solution for this
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
    });
}
