use std::collections::{HashMap};

use crate::{entities::entity::{Entity, EntityId}, errors::error_messages::PLAYER_NOT_PRESENT, events::event::Event, worlds::{position::{ALL_DIRS, Position}, registry::Registry, zones::zone::Zone}};

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

    pub fn get_player(&self) -> &Entity {
        self.get_entity(self.player_id).expect(PLAYER_NOT_PRESENT)
    }

    pub fn get_player_mut(&mut self) -> &mut Entity {
        self.get_entity_mut(self.player_id).expect(PLAYER_NOT_PRESENT)
    }

    pub fn get_zone(&self, position: &Position) -> Option<& Zone> {
        self.zones.get(position)
    }

    pub fn get_zone_mut(&mut self, position: &Position) -> Option<&mut Zone> {
        self.zones.get_mut(position)
    }

    pub fn get_player_zone(&self) -> Option<&Zone> {
        let zone_position = &self.get_player().zone_position.clone();
        self.zones.get(zone_position)
    }

    pub fn get_player_zone_mut(&mut self) -> Option<&mut Zone> {
        let zone_position = &self.get_player_mut().zone_position.clone();
        self.zones.get_mut(zone_position)
    }

    pub fn get_active_zones(&mut self) -> impl Iterator<Item = &mut Zone> {
        let positions = ALL_DIRS
            .map(|dir| self.get_player().zone_position + dir.to_delta());
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