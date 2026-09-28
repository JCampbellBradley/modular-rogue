
use std::{error::Error, path::PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModError {
    #[error("Mod with path {path} has mismatching build ID. Expected {expected}, found {found}. Please rebuild the mod against the desired version of the engine.")]
    BuildMismatch {
        path: PathBuf,
        expected: String,
        found: String
    },
    #[error("Mods have unresolved or circular dependencies: {ids:?}")]
    UnresolvedDependencies {
        ids: Vec<String>
    },
    #[error("Registration error for mod with id `{id}`: {source}")]
    Registration {
        id: String,
        source: Box<dyn Error>
    },
    #[error("Mod `{id}` panicked during registration.")]
    Panicked {
        id: String
    },
    #[error("Multiple mods were registered with the same ID: {id}")]
    DuplicateModID {
        id: String
    },
    #[error("A mod was registered with the reserved ID: {id}")]
    ReservedModID {
        id: String
    }
}