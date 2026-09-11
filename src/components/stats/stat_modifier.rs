use std::fmt::Debug;


#[typetag::serde(tag = "type")]
pub trait StatModifier : Debug {
    fn get_bonus(&self) -> i32;
    fn get_cap_bonus(&self) -> i32;
    fn is_valid(&self) -> bool;
}