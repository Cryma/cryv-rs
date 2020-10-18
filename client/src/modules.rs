use legion::systems::Builder;
use legion::*;

pub trait Module {
    fn run_initial(&mut self) {}
    fn run_on_tick(&mut self, _world: &mut World, _resources: &mut Resources) {}
    fn add_components(&mut self, _world: &mut World) {}
    fn add_resources(&mut self, _resources: &mut Resources) {}
    fn add_systems(&mut self, _builder: &mut Builder) {}
}
