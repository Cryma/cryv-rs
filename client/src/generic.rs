use crate::modules::Module;
use legion::systems::Builder;
use legion::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenericFunctionComponent {
    pub function: fn(),
}

pub struct GenericModule;

impl Module for GenericModule {
    fn add_systems(&self, builder: &mut Builder) {
        builder.add_thread_local(run_generic_functions_system());
    }
}

#[system(for_each)]
fn run_generic_functions(function_components: &GenericFunctionComponent) {
    (function_components.function)();
}
