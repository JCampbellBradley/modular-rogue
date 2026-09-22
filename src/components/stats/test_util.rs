use serde::{Deserialize, Serialize};

use crate::components::stats::stat_modifier::StatModifier;

#[derive(Serialize, Deserialize, Debug)]
pub struct TestModifier {
}

impl TestModifier {
    pub const BONUS: i32 = 1; 
    pub const CAP_BONUS: i32 = 1; 
}

#[typetag::serde(name="_stat_test_test_modifier")]
impl StatModifier for TestModifier {
    fn get_bonus(&self) -> i32 {TestModifier::BONUS}
    fn get_cap_bonus(&self) -> i32 {TestModifier::CAP_BONUS}
    fn is_valid(&self) -> bool {true}
}