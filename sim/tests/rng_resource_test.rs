use bevy::prelude::World;

#[test]
fn rng_resource_exists_and_can_be_inserted() {
    let mut world = World::default();
    world.insert_resource(sim::rng::Rng::default());
    let _ = world.resource::<sim::rng::Rng>();
}
