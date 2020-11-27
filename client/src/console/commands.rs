use std::net::SocketAddr;

use super::ConsoleData;
use crate::{
    entities::create_vehicle_with_model_name,
    entities::get_entity_transform,
    entities::{get_entity_model, EntityHandle},
    utility::ModelValidityExt,
    wrapped_natives::*,
};
use hook::natives::*;
use shared::bevy::ecs::prelude::*;
use shared::bevy_prototype_networking_laminar::{NetworkDelivery, NetworkResource};

macro_rules! console_print {
    ($resources:expr, $text:expr) => {{
        let mut console_data = $resources.get_mut::<ConsoleData>().unwrap();
        console_data.print_line($text);
    }};
}

pub(super) fn command_veh(world: &mut World, resources: &mut Resources, arguments: Vec<String>) {
    let mut arguments = arguments.clone();

    if arguments.len() < 1 {
        console_print!(resources, "Please specifiy a vehicle model name.");

        return;
    }

    let model = &arguments.pop().unwrap()[..];
    if model.is_valid_vehicle() == false {
        console_print!(
            resources,
            "The vehicle model name you specified is not valid."
        );

        return;
    }

    let color_primary = match arguments.pop() {
        Some(value) => value.parse::<i32>().unwrap_or(0),
        None => 0,
    };

    let color_secondary = match arguments.pop() {
        Some(value) => value.parse::<i32>().unwrap_or(0),
        None => 0,
    };

    let player_ped_id = hook::natives::player::player_ped_id();
    let position = hook::natives::entity::get_entity_coords(player_ped_id, true);
    let rotation = hook::natives::entity::get_entity_rotation(player_ped_id, 2);

    let (handle, model, transform, vehicle) =
        create_vehicle_with_model_name(&model, position, rotation, color_primary, color_secondary);

    hook::natives::ped::set_ped_into_vehicle(player_ped_id, handle.handle, -1);

    world.spawn((handle, model, transform, vehicle));

    console_print!(
        resources,
        format!("Spawned vehicle ({}) with model: {}", handle.handle, model).as_str()
    );
}

pub(super) fn command_rmveh(world: &mut World, resources: &mut Resources, _arguments: Vec<String>) {
    let player_ped_id = hook::natives::player::player_ped_id();
    let is_in_vehicle = hook::natives::ped::is_ped_in_any_vehicle(player_ped_id, false);

    if is_in_vehicle == false {
        console_print!(resources, "You are not in any vehicle.");

        return;
    }

    let mut vehicle_id = hook::natives::ped::get_vehicle_ped_is_in(player_ped_id, false);
    // Copy the vehicle id, as GTA5 will clear the current one, after deleting the entity
    let vehicle_id_copy = vehicle_id.clone();

    entities::delete_entity(&mut vehicle_id);

    let existing_entities = world.query::<(Entity, &EntityHandle)>();

    let mut found_entity: Option<Entity> = None;

    for (entity, entity_data) in existing_entities {
        if entity_data.handle != vehicle_id_copy {
            continue;
        }

        found_entity = Some(entity);
    }

    if let Some(x) = found_entity {
        world.despawn(x).unwrap();
    }
}

pub(super) fn command_connect(
    _world: &mut World,
    resources: &mut Resources,
    arguments: Vec<String>,
) {
    let mut network_resource = resources.get_mut::<NetworkResource>().unwrap();
    network_resource.bind("127.0.0.1:1336").unwrap();

    let server_address: SocketAddr = arguments.first().unwrap().parse().unwrap();

    let player_ped_id = player::player_ped_id();
    let model = get_entity_model(player_ped_id);
    let transform = get_entity_transform(player_ped_id);

    let message = shared::NetworkMessage::EstablishConnection(model, transform);

    match network_resource.send(
        server_address,
        &message.encode()[..],
        NetworkDelivery::ReliableOrdered(Some(1)),
    ) {
        Ok(_) => console_print!(
            resources,
            format!("Connecting to \"{:?}\"", server_address).as_str()
        ),
        Err(error) => console_print!(
            resources,
            &format!("Something went wrong while trying to connect: {}", error)
        ),
    };
}
