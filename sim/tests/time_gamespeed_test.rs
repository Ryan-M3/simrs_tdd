use bevy::prelude::World;

#[test]
fn gamespeed_is_a_resource_and_defaults_to_one() {
    let mut world = World::default();
    world.insert_resource(sim::time::GameSpeed::default());
    let gs = world.resource::<sim::time::GameSpeed>();
    assert_eq!(gs.0, 1.0);
}
