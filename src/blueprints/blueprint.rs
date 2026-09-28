use crate::glue::bake::Bake;

pub trait Blueprint<T> : Bake<T> {
}