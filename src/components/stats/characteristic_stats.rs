use std::fmt::Debug;

use modular_rogue_macros::DynamicHandlers;
use serde::{Deserialize, Serialize};

use crate::{components::{component::{Component, HandlesConst, HandlesMut}, stats::stat::{Stat, StatCategory}}, events::{stat_change_event::StatChangeEvent, stat_collect_event::{StatCollectEvent, StatCollected}}};


#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Characteristic {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma
}

impl StatCategory for Characteristic {}

pub const CHARACTERISTICS: [Characteristic; 6] = [
    Characteristic::Strength, 
    Characteristic::Dexterity, 
    Characteristic::Constitution, 
    Characteristic::Intelligence, 
    Characteristic::Wisdom, 
    Characteristic::Charisma
];

#[derive(Serialize, Deserialize, Default, DynamicHandlers, Debug)]
#[handles(StatCollectEvent<Characteristic>, mut StatChangeEvent<Characteristic>)]
#[serde(default)]
pub struct CharacteristicStats {
    strength: Stat,
    dexterity: Stat,
    constitution: Stat,
    intelligence: Stat,
    wisdom: Stat,
    charisma: Stat
}

impl CharacteristicStats {
    pub fn new() -> Self {
        Self::default()
    }

    fn get(&self, stat_type: Characteristic) -> &Stat {
        match stat_type {
            Characteristic::Strength => &self.strength,
            Characteristic::Dexterity => &self.dexterity,
            Characteristic::Constitution => &self.constitution,
            Characteristic::Intelligence => &self.intelligence,
            Characteristic::Wisdom => &self.wisdom,
            Characteristic::Charisma => &self.charisma,
        }
    }

    fn get_mut(&mut self, stat_type: Characteristic) -> &mut Stat {
        match stat_type {
            Characteristic::Strength => &mut self.strength,
            Characteristic::Dexterity => &mut self.dexterity,
            Characteristic::Constitution => &mut self.constitution,
            Characteristic::Intelligence => &mut self.intelligence,
            Characteristic::Wisdom => &mut self.wisdom,
            Characteristic::Charisma => &mut self.charisma,
        }
    }
}

#[typetag::serde(name="characteristic_stats")]
impl Component for CharacteristicStats {}

impl HandlesMut<StatChangeEvent<Characteristic>> for CharacteristicStats {
    fn handle_mut(&mut self, e: &mut StatChangeEvent<Characteristic>) {
        for (&stat_type, v) in e.impulses.iter() {
            let stat = self.get_mut(stat_type);

            if let Some(new_base) = v.new_base {
                stat.set(new_base);
            }
        }
    }
}

impl HandlesConst<StatCollectEvent<Characteristic>> for CharacteristicStats {
    fn handle(&self, e: &mut StatCollectEvent<Characteristic>) {
        for &stat_type in e.wanted.iter() {
            let stat = self.get(stat_type);

            let collected = StatCollected {
                base: stat.get(),
                cap: stat.get(),
            };

            e.stats.insert(stat_type, Some(collected));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::events::stat_collect_event::StatCollectEvent;

    use super::*;

    #[test]
    fn change_stat() {
        let mut stats = CharacteristicStats::new();

        for (i, &stat_type) in CHARACTERISTICS.iter().enumerate() {
            let val = i.try_into().unwrap();
            let mut change = StatChangeEvent::new()
                .change_base(stat_type, val);

            stats.handle_mut(&mut change);

            let mut collect = StatCollectEvent::new()
                .want(stat_type);

            stats.handle(&mut collect);

            let intelligence = collect.get(stat_type).unwrap();

            assert_eq!(intelligence.base, val);
        }
    }

    #[test]
    fn unwanted_stat() {
        let stats: CharacteristicStats = CharacteristicStats::new();
        let mut collect = StatCollectEvent::new();

        stats.handle(&mut collect);

        assert!(collect.get(Characteristic::Intelligence).is_none())
    }
}