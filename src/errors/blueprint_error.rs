use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlueprintError {
    #[error("{0:?}")]
    RonError(ron::Error),
    #[error("Field 'components' of EntityBlueprint should be a ron::Value::Map. Got {value:?}")]
    BadEntityBlueprintComponents {
        value: ron::Value
    }
}