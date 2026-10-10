use thiserror::Error;

use crate::{errors::{inc_registry_error::IncRegistryError, zone_error::ZoneError}, worlds::position::Position};


#[derive(Error, Debug)]
pub enum WorldError {
    #[error("Tried to add an entity to zone with position `{zone_position:?}`, which does not exist.")]
    AddToNullZone {
        zone_position: Position
    },
    #[error("Tried to add a zone at `{zone_position:?}` where one was already present.")]
    ZoneAddClobber {
        zone_position: Position
    },
    #[error("The player was expected, but was not present")]
    PlayerMissing,
    #[error("Got an error when working with the entity registry:\n {0}")]
    IncrementalRegistryError(#[from] IncRegistryError),
    #[error("{0}")]
    ZoneError(#[from] ZoneError)
}