mod world;
use raylib::prelude::*;
use world::World;

fn main() {
    let mut world = World::new();
    world.add_entity(
        None,
        Some(100),
        Some(Vector2::zero()),
        Some(String::from("square")),
    );
    world.add_entity(
        None,
        Some(100),
        Some(Vector2::zero()),
        Some(String::from("square")),
    );
    world.add_entity(
        None,
        Some(250),
        Some(Vector2::zero()),
        Some(String::from("triangle")),
    );
    world.add_entity(
        None,
        Some(300),
        Some(Vector2::zero()),
        Some(String::from("triangle")),
    );
    println!("{:?}", world);

    println!("{:?}", world.get_entity_by_name(String::from("square")));
}
