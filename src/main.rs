use modular_rogue::glue::{core_mod::mr_register, load_mods::{load_mods, mods_dir}, mod_registry::ModRegistry};



fn main() {
    let mut registry = ModRegistry::new();
    mr_register(&mut registry.build_context("core")).unwrap();
    let registry = load_mods(&mods_dir(), registry).unwrap();

    println!("Successfully loaded mods.");

    let tags: Vec<_> = registry.get_tags().cloned().collect();
    println!("Tags: {}", tags.join(","));
    // build the world, load blueprints, run the game loop...
}