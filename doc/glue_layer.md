# Glue Layer

We want a 'glue layer' written in rust that can patch the game engine's base code with any supplied mods. We do not want users to need a rust compiler installed, so mods should be pre-compiled.

Mods should hook into the game engine by defining custom `Event`, and `Component` structs, as well as RON blueprints that can be read in at runtime.

## Core Engine

The core engine uses `crate-type = ["dylib"]` and builds with `-C prefer-dynamic`. At runtime, it should scan a mod directory, and loads its contents with `libloading`.

## Mods

Mods should use `crate-type = ["dylib"]` and build with `-C prefer-dynamic`. Their entry point should be a function `register(&mut ModContext)` that registers structs to the `ModContext`. TODO: write a macro that can automatically register structs.

Mods should declare their `id`, which must be unique, and their `version` and `dependencies`. Mods can override blueprints from their dependencies. Circular dependencies will cause a mod registration error.