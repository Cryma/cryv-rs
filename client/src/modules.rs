use legion::systems::Builder;
use legion::*;

pub trait Module {
    fn run_initial(&self) {}
    fn add_components(&self, _world: &mut World) {}
    fn add_resources(&self, _resources: &mut Resources) {}
    fn add_systems(&self, _builder: &mut Builder) {}
}
