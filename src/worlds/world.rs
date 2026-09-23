use std::{array::IntoIter, collections::HashMap, iter::Flatten};

use crate::{entities::entity::{Entity, EntityId}, events::event::Event, worlds::{position::{ALL_DIRS, Position}, registry::Registry, zones::zone::Zone}};

#[derive(Default)]
pub struct World {
    zones: HashMap<Position, Zone>,
    entities: Registry<Entity>,
    player_id: EntityId
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_entity(&mut self, item: Entity) -> EntityId {
        self.entities.register(item)
    }

    pub fn unregister_entity(&mut self, id: EntityId) -> Option<Entity> {
        self.entities.unregister(id)
    }

    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(id)
    }

    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(id)
    }

    pub fn get_player(&self) -> Option<&Entity> {
        self.get_entity(self.player_id)
    }

    pub fn get_player_mut(&mut self) -> Option<&mut Entity> {
        self.get_entity_mut(self.player_id)
    }

    pub fn get_zone(&self, position: &Position) -> Option<& Zone> {
        self.zones.get(position)
    }

    pub fn get_zone_mut(&mut self, position: &Position) -> Option<&mut Zone> {
        self.zones.get_mut(position)
    }

    pub fn get_player_zone(&self) -> Option<&Zone> {
        match self.get_player_zone_position() {
            Some(zone_position) => self.zones.get(&zone_position),
            None => None
        }
    }

    pub fn get_player_zone_mut(&mut self) -> Option<&mut Zone> {
        match self.get_player_zone_position() {
            Some(zone_position) => self.zones.get_mut(&zone_position),
            None => None
        }
    }

    pub fn get_active_zones(&mut self) -> impl Iterator<Item = &mut Zone> {
        match self.get_player_zone_position() {
            Some(zone_position) => self.get_disjoint_zones(ALL_DIRS
                .map(|dir| zone_position + dir.to_delta()))
                .collect::<Vec<_>>().into_iter(),
            None => self.get_disjoint_zones([])
                .collect::<Vec<_>>().into_iter()
        }
    }

    fn get_player_zone_position(&self) -> Option<Position> {
        match self.get_player() {
            Some(player) => player.zone_position,
            None => None
        }
    }

    fn get_disjoint_zones<const N: usize>(&mut self, positions: [Position; N]) -> Flatten<IntoIter<Option<&mut Zone>, N>> {
        self.zones.get_disjoint_mut(positions.each_ref())
            .into_iter().flatten()
    }

    pub fn handle<T: Event + 'static>(&mut self, e: &mut T) {
        for id in self.get_active_zones()
            .flat_map(|zone| zone.get_all_entities()).collect::<Vec<EntityId>>()
        {
            if let Some(entity) = self.get_entity_mut(id) {
                entity.handle(e)
            }
        }
    }
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::worlds::registry::Registerable;

use super::*;

    #[test]
    fn register_entity() {
        let mut world = World::new();
        let entity = Entity::new();

        assert!(entity.get_id().is_none());

        let id = world.register_entity(entity);

        let ent_ref = world.get_entity(id).unwrap();

        assert!(ent_ref.get_id().is_some());
    }

    #[test]
    fn unregister_entity() {
        let mut world = World::new();
        let entity = Entity::new();

        let id = world.register_entity(entity);
        let entity = world.unregister_entity(id);

        let none = world.get_entity(id);

        assert!(entity.is_some());
        assert!(none.is_none());
    }

    #[test]
    fn get_player_none() {
        let world = World::new();
        
        let none = world.get_player();

        assert!(none.is_none());
    }

    #[test]
    fn get_player_zone_none() {
        let world = World::new();
        
        let none = world.get_player_zone();

        assert!(none.is_none());
    }

    #[test]
    fn get_active_zones
    () {
        let world = World::new();
        
        let none = world.get_player_zone();

        assert!(none.is_none());
    }
}