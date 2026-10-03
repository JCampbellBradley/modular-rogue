use thiserror::Error;


#[derive(Error, Debug)]
pub enum EntityError {
    #[error("Component `{attached}` was attached to an Entity which already had a component of the same type: `{present}`.")]
    AttachClobber {
        attached: String,
        present: String
    }
}