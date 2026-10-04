use serde::{Deserialize, Serialize};

use std::cmp::min;


#[derive(Deserialize)]
#[serde(untagged)]
enum CappedStatRepr {
    Short((i32, i32)),
    Full {base: i32, cap: i32 }
}


#[derive(Serialize, Debug, Default)]
pub struct CappedStat {
    base: i32,
    cap: i32
}


impl CappedStat {
    pub fn new() -> Self {
        CappedStat::default()
    }

    pub fn build(base: i32, cap: i32) -> Self {
        CappedStat {
            base: min(base, cap), 
            cap
        }
    }

    pub fn get(&self) -> i32 {
        self.base
    }

    pub fn get_cap(&self) -> i32 {
        self.cap
    }

    pub fn set(&mut self, value: i32) {
        self.base = min(value, self.get_cap());
    }

    pub fn set_cap(&mut self, value: i32) {
        self.cap = value;
    }
}


impl From<(i32, i32)> for CappedStat {
    fn from(value: (i32, i32)) -> Self {
        Self::build(value.0, value.1)
    }
}

impl<'de> Deserialize<'de> for CappedStat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let repr = CappedStatRepr::deserialize(deserializer)?;
        let stat = match repr {
            CappedStatRepr::Short(short) => CappedStat::from(short),
            CappedStatRepr::Full {base, cap} 
                => CappedStat::build(base, cap)
        };
        Ok(stat)
    }
}

#[cfg(test)]
mod tests {

use super::*;

    #[test]
    fn get() {
        let stat = CappedStat {base: 1, cap: 1};

        assert_eq!(stat.get(), 1);
    }

    #[test]
    fn get_cap() {
        let stat = CappedStat {base: 1, cap: 1};

        assert_eq!(stat.get_cap(), 1);
    }

    #[test]
    fn set() {
        let mut stat = CappedStat {base: 1, cap: 2};
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
    fn from() {
        let stat: CappedStat = (1, 2).into();

        assert_eq!(stat.base, 1);
        assert_eq!(stat.cap, 2);
    }
    
    #[test]
    fn from_capped() {
        let stat: CappedStat = (3, 2).into();

        assert_eq!(stat.base, 2);
        assert_eq!(stat.cap, 2);
    }
    
    #[test]
    fn serde() {
        let stat = CappedStat {base: 1, cap: 2};

        let serialized = ron::to_string(&stat).unwrap();
        let deserialized: CappedStat = ron::from_str(&serialized).unwrap();

        assert_eq!(deserialized.base, 1);
        assert_eq!(deserialized.cap, 2);
    }
    
    #[test]
    fn deserialize_short() {
        let deserialized: CappedStat = ron::from_str("(1,2)").unwrap();

        assert_eq!(deserialized.base, 1);
        assert_eq!(deserialized.cap, 2);
    }
    
    #[test]
    fn deserialize_capped() {
        let deserialized: CappedStat = ron::from_str("(3,2)").unwrap();

        assert_eq!(deserialized.base, 2);
        assert_eq!(deserialized.cap, 2);
    }
}