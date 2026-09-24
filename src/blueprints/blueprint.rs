use serde::Deserialize;

use crate::errors::blueprint_error::BlueprintError;

pub trait Blueprint<T: for<'de> Deserialize<'de>> {
    fn build(self) -> Result<T, BlueprintError>;
}