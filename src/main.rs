use crate::entity::entity::Entity;

mod component;
mod entity;
mod event;
mod util;

fn main() {
    let ent = Entity::new();

    println!("Hello, world!");
}
