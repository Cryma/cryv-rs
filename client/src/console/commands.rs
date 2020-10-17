use super::{print_line, ConsoleData};
use crate::wrapped_natives::*;
use legion::systems::CommandBuffer;

pub(super) fn command_veh(
    command_buffer: &mut CommandBuffer,
    console_data: &mut ConsoleData,
    arguments: Vec<String>,
) {
    // TODO: Improve argument system or at least validate them
    let mut arguments = arguments.clone();
    arguments.reverse();

    let model = arguments.pop().unwrap();

    let position =
        hook::natives::entity::get_entity_coords(hook::natives::player::player_ped_id(), true);

    let id = vehicles::create_vehicle_with_model_name(
        &model,
        (position.x, position.y, position.z),
        (0.0, 0.0, 0.0),
    );

    command_buffer.push((crate::cleanup::Entity { id },));

    print_line(
        console_data,
        format!("Spawned vehicle ({}) with model: {}", id, model).as_str(),
    );
}
