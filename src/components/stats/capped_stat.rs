use serde::{Deserialize, Serialize};

use crate::components::stats::stat_modifier::StatModifier;
use std::cmp::min;


#[derive(Deserialize)]
#[serde(untagged)]
enum CappedStatRepr {
    Short((i32, i32)),
    Full {base: i32, cap: i32, modifiers: Vec<Box<dyn StatModifier>> }
}


#[derive(Serialize, Debug, Default)]
pub struct CappedStat {
    base: i32,
    cap: i32,
    modifiers: Vec<Box<dyn StatModifier>>
}


impl CappedStat {
    pub fn new() -> Self {
        CappedStat::default()
    }

    pub fn get(&self) -> i32 {
        self.modifiers.iter().fold(0, |a, b| 
            a + b.get_bonus()
        ) + self.base
    }

    pub fn get_cap(&self) -> i32 {
        self.modifiers.iter().fold(0, |a, b| 
            a + b.get_cap_bonus()
        ) + self.cap
    }

    pub fn set(&mut self, value: i32) {
        self.base = min(value, self.get_cap());
    }

    pub fn set_cap(&mut self, value: i32) {
        self.cap = value;
    }

    pub fn add_modifier(&mut self, modifier: Box<dyn StatModifier>) {
        self.modifiers.push(modifier);
    }
}


impl From<(i32, i32)> for CappedStat {
    fn from(value: (i32, i32)) -> Self {
        CappedStat {
            base: value.0, 
            cap: value.1, 
            ..Default::default()
        }
    }
}

impl<'de> Deserialize<'de> for CappedStat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let repr = CappedStatRepr::deserialize(deserializer)?;
        let stat = match repr {
            CappedStatRepr::Short(base) => CappedStat::from(base),
            CappedStatRepr::Full {base, cap, modifiers} 
                => CappedStat {base, cap, modifiers}
        };
        Ok(stat)
    }
}

#[cfg(test)]
mod tests {
    use crate::components::stats::test_util::TestModifier;

use super::*;

    #[test]
    fn get() {
        let stat = CappedStat {base: 1, cap: 1, modifiers: Vec::new()};

        assert_eq!(stat.get(), 1);
    }

    #[test]
    fn get_cap() {
        let stat = CappedStat {base: 1, cap: 1, modifiers: Vec::new()};

        assert_eq!(stat.get_cap(), 1);
    }

    #[test]
    fn set() {
        let mut stat = CappedStat {base: 1, cap: 2, modifiers: Vec::new()};
        stat.set(3);

        assert_eq!(stat.get(), 2);
        assert_eq!(stat.get_cap(), 2);
    }

    #[test]
    fn set_cap() {
        let mut stat = CappedStat::new();
        stat.set_cap(1);

        assert_eq!(stat.get(), 0);
        assert_eq!(stat.get_cap(), 1);
    }

    #[test]
    fn add_1_modifier() {
        let mut stat = CappedStat::new();
        stat.add_modifier(Box::new(TestModifier {}));

        assert_eq!(stat.get(), TestModifier::BONUS);
        assert_eq!(stat.get_cap(), TestModifier::CAP_BONUS);
    }

    #[test]
    fn add_2_modifier() {
        let mut stat = CappedStat::new();
        stat.add_modifier(Box::new(TestModifier {}));
        stat.add_modifier(Box::new(TestModifier {}));

        assert_eq!(stat.get(), TestModifier::BONUS * 2);
        assert_eq!(stat.get_cap(), TestModifier::CAP_BONUS * 2);
    }
}