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
        CappedStat {
            base: value.0, 
            cap: value.1
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
            CappedStatRepr::Full {base, cap} 
                => CappedStat {base, cap}
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
}