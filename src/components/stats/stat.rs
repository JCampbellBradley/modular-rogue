use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(untagged)]
enum StatRepr {
    Short(i32),
    Full {base: i32}
}

pub trait StatCategory: Eq + std::hash::Hash {}

#[derive(Serialize, Debug, Default)]
pub struct Stat {
    base: i32,
}


impl Stat {
    pub fn new() -> Self {
        Stat::default()
    }

    pub fn build(base: i32) -> Self {
        Stat {
            base
        }
    }

    pub fn get(&self) -> i32 {
        self.base
    }

    pub fn set(&mut self, value: i32) {
        self.base = value;
    }
}

impl From<i32> for Stat {
    fn from(value: i32) -> Self {
        Stat{
            base: value
        }
    }
}

impl<'de> Deserialize<'de> for Stat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let repr = StatRepr::deserialize(deserializer)?;
        let stat = match repr {
            StatRepr::Short(base) => Stat::from(base),
            StatRepr::Full {base} => Stat {base}
        };
        Ok(stat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let stat = Stat::new();

        assert_eq!(stat.get(), 0);
    }

    #[test]
    fn set() {
        let mut stat = Stat::build(1);
        stat.set(3);

        assert_eq!(stat.get(), 3);
    }
    
    #[test]
    fn from() {
        let stat: Stat = 1.into();

        assert_eq!(stat.base, 1);
    }
    
    #[test]
    fn serde() {
        let stat = Stat::build(1);

        let serialized = ron::to_string(&stat).unwrap();
        let deserialized: Stat = ron::from_str(&serialized).unwrap();

        assert_eq!(deserialized.base, 1);
    }
    
    #[test]
    fn deserialize_short() {
        let deserialized: Stat = ron::from_str("1").unwrap();

        assert_eq!(deserialized.base, 1);
    }
}