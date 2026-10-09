extern crate self as modular_rogue;

pub mod components;
pub mod entities;
pub mod events;
pub mod util;
pub mod worlds;
pub mod errors;
pub mod blueprints;
pub mod mod_registry;

/// Utility code that allows integration with e.g. mods.
/// Only covered by integration tests, not unit tests.
/// Contains a lot of unsafe operations.
pub mod integration;


pub use mod_registry::core_mod::MR_COMPONENTS;

pub use ron;
pub use linkme;
pub use serde;
pub use modular_rogue_macros::{DynamicHandlers, ModElement};
