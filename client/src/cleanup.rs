use hook::natives::*;
use legion::systems::Builder;
use legion::*;

#[derive(Copy, Clone, Debug, PartialEq)]
struct FunctionComponent {
    function: fn(),
}

pub fn run_initial_cleanup() {
    let player_ped_id = player::player_ped_id();
    entity::set_entity_coords_no_offset(player_ped_id, 412.4, -976.71, 29.43, false, false, false);
}

pub fn add_cleanup_components(world: &mut World) {
    let _entity = world.push((FunctionComponent {
        function: hijack_frontend_menu,
    },));
}

pub fn add_cleanup_systems(builder: &mut Builder) {
    builder.add_thread_local(run_cleanup_systems_system());
}

#[system(for_each)]
fn run_cleanup_systems(function_components: &FunctionComponent) {
    (function_components.function)();
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
