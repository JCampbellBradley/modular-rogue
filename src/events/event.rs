pub trait Event {

}

pub trait MutEvent : Event {
    fn is_mut_event() -> bool {
        true
    }
}
pub trait ConstEvent : Event {
    fn is_mut_event() -> bool {
        false
    }
}