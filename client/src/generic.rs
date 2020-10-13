use legion::systems::Builder;
use legion::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenericFunctionComponent {
    pub function: fn(),
}

pub fn run_initial() {}

pub fn add_components(_world: &mut World) {}

pub fn add_systems(builder: &mut Builder) {
    builder.add_thread_local(run_generic_functions_system());
}

#[system(for_each)]
fn run_generic_functions(function_components: &GenericFunctionComponent) {
    (function_components.function)();
}
