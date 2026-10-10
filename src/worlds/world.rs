use std::{array::IntoIter, collections::{HashMap, hash_map::{Entry}}, iter::Flatten};

use crate::{entities::entity::{Entity, EntityId}, errors::{expect_messages::LOGICALLY_UNREACHABLE, world_error::WorldError}, worlds::{direction::{ALL_DIRS}, position::Position, registry::IncRegistry, zones::zone::Zone}};

#[derive(Default, Debug)]
pub struct World {
    zones: HashMap<Position, Zone>,
    entities: IncRegistry<Entity>,
    player_id: EntityId
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_entity(&mut self, item: Entity) -> Result<EntityId, WorldError> {
        Ok(self.entities.register(item)?)
    }

    pub fn unregister_entity(&mut self, id: EntityId) -> Option<Entity> {
        self.entities.unregister(id)
    }

    pub fn set_player(&mut self, id: EntityId) -> Option<()> {
        match self.entities.get(id) {
            Some(_) => {
                self.player_id = id;
                Some(())
            },
            None => None
        }
    }

    ///Atomic: will add the entity to the correct position and the correct zone,
    ///then register it, or fail and do none of the above.
    pub fn register_to_zone(&mut self, mut entity: Entity, zone_position: Position, position: Position) -> Result<EntityId, WorldError> {
        let id = self.entities.next_id();
        
        let zone = match self.get_zone_mut(zone_position) {
            Some(zone) => Ok(zone),
            None => Err(WorldError::AddToNullZone { zone_position })
        }?;

        zone.put_entity(position, id)?;
        
        entity.position = Some(position);
        entity.zone_position = Some(zone_position);

        self.entities.register_with(entity, id).expect(LOGICALLY_UNREACHABLE);

        Ok(id)
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

    pub fn load_empty_zone(&mut self, zone_position: Position, width: u32, height: u32) -> Result<(), WorldError> {
        match self.zones.entry(zone_position) {
            Entry::Occupied(_) => Err(WorldError::ZoneAddClobber { zone_position }),
            Entry::Vacant(entry) => {
                entry.insert(Zone::new(width, height));
                Ok(())
            }
        }
    }

    pub fn get_zone(&self, position: Position) -> Option<&Zone> {
        self.zones.get(&position)
    }

    pub fn get_zone_mut(&mut self, position: Position) -> Option<&mut Zone> {
        self.zones.get_mut(&position)
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

    pub fn get_active_zones(&mut self) -> impl Iterator<Item = &Zone> {
        match self.get_player_zone_position() {
            Some(zone_position) => ALL_DIRS.iter()
                .map(|dir| zone_position + dir.to_delta())
                .flat_map(|pos| self.get_zone(pos))
                .collect::<Vec<_>>().into_iter(),
            None => Vec::new().into_iter()
        }
    }

    pub fn get_active_zones_mut(&mut self) -> impl Iterator<Item = &mut Zone> {
        match self.get_player_zone_position() {
            Some(zone_position) => self.get_disjoint_zones(ALL_DIRS
                .map(|dir| zone_position + dir.to_delta()))
                .collect::<Vec<_>>().into_iter(),
            None => Vec::new().into_iter()
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

        let id = world.register_entity(entity).unwrap();

        let ent_ref = world.get_entity(id).unwrap();

        assert!(ent_ref.get_id().is_some());
    }

    #[test]
    fn register_entity_with_id() {
        let mut world = World::new();
        
        let mut ent = Entity::new();
        ent.set_id(0);
        let err = world.register_entity(ent).unwrap_err();

        println!("{err}");

    }

    #[test]
    fn get_entity_mut() {
        let mut world = World::new();
        let entity = Entity::new();

        let id = world.register_entity(entity).unwrap();

        world.get_entity_mut(id).unwrap();
    }

    #[test]
    fn unregister_entity() {
        let mut world = World::new();
        let entity = Entity::new();

        let id = world.register_entity(entity).unwrap();
        let entity = world.unregister_entity(id);

        let none = world.get_entity(id);

        assert!(entity.is_some());
        assert!(none.is_none());
    }

    #[test]
    fn set_player_none() {
        let mut world = World::new();
        
        let none = world.set_player(2);

        assert!(none.is_none());
    }

    #[test]
    fn get_player_none() {
        let world = World::new();
        
        let none = world.get_player();

        assert!(none.is_none());
    }

    #[test]
    fn get_player_mut() {
        let mut world = World::new();
        let id = world.register_entity(Entity::new()).unwrap();
        world.set_player(id);

        world.get_player_mut().unwrap();
    }

    #[test]
    fn get_player_zone() {
        let mut world = World::new();
        world.load_empty_zone((1, 2).into(), 32, 32).unwrap();

        let id = world.register_to_zone(Entity::new(), (1, 2).into(), (3, 4).into()).unwrap();
        world.set_player(id);

        let zone = world.get_player_zone().unwrap();

        assert!(zone.get_entities((3, 4).into()).unwrap().any(|p| p == id))
    }

    #[test]
    fn get_player_zone_none() {
        let world = World::new();
        
        let none = world.get_player_zone();

        assert!(none.is_none());
    }

    #[test]
    fn get_player_zone_mut() {
        let mut world = World::new();
        world.load_empty_zone((1, 2).into(), 32, 32).unwrap();

        let id = world.register_to_zone(Entity::new(), (1, 2).into(), (3, 4).into()).unwrap();
        world.set_player(id);

        let zone = world.get_player_zone_mut().unwrap();
        zone.remove_entity((3, 4).into(), id).unwrap();

        let ents: Vec<_> = zone.get_all_entities().collect();

        assert!(ents.is_empty());
    }

    #[test]
    fn get_player_zone_mut_none() {
        let mut world = World::new();
        
        let none = world.get_player_zone_mut();

        assert!(none.is_none());
    }

    #[test]
    fn get_active_zones() {
        let mut world = World::new();
        world.load_empty_zone((1, 2).into(), 1, 1).unwrap();
        world.load_empty_zone((2, 2).into(), 2, 2).unwrap();
        world.load_empty_zone((0, 3).into(), 3, 3).unwrap();

        let id = world.register_to_zone(Entity::new(), (1, 2).into(), (0, 0).into()).unwrap();
        world.set_player(id);

        let zones: Vec<_> = world.get_active_zones().collect();

        println!("{zones:?}");

        assert!(zones.len() == 3);
        assert!(zones.iter().any(|zone| zone.height == 1));
        assert!(zones.iter().any(|zone| zone.height == 2));
        assert!(zones.iter().any(|zone| zone.height == 3));
    }

    #[test]
    fn get_active_zones_none() {
        let mut world = World::new();
        
        let zones: Vec<_> = world.get_active_zones().collect();

        assert!(zones.is_empty());
    }

    #[test]
    fn get_active_zones_mut() {
        let mut world = World::new();
        world.load_empty_zone((1, 2).into(), 1, 1).unwrap();
        world.load_empty_zone((2, 2).into(), 2, 2).unwrap();
        world.load_empty_zone((0, 3).into(), 3, 3).unwrap();

        let id = world.register_to_zone(Entity::new(), (1, 2).into(), (0, 0).into()).unwrap();
        world.set_player(id);

        let zones: Vec<_> = world.get_active_zones_mut().collect();

        println!("{zones:?}");

        assert!(zones.len() == 3);
        assert!(zones.iter().any(|zone| zone.height == 1));
        assert!(zones.iter().any(|zone| zone.height == 2));
        assert!(zones.iter().any(|zone| zone.height == 3));
    }

    #[test]
    fn get_active_zones_mut_none() {
        let mut world = World::new();
        
        let zones: Vec<_> = world.get_active_zones_mut().collect();

        assert!(zones.is_empty());
    }

    #[test]
    fn register_to_zone() {
        let mut world = World::new();
        world.load_empty_zone((1, 2).into(), 32, 32).unwrap();

        let id = world.register_to_zone(Entity::new(), (1, 2).into(), (3, 4).into()).unwrap();
        world.set_player(id);

        let zone = world.get_zone((1, 2).into()).unwrap();

        assert!(zone.get_entities((3, 4).into()).unwrap().any(|p| p == id))
    }

    #[test]
    fn register_to_zone_oob() {
        let mut world = World::new();
        world.load_empty_zone((1, 2).into(), 1, 1).unwrap();

        let err = world.register_to_zone(Entity::new(), (1, 2).into(), (3, 4).into()).unwrap_err();

        println!("{err}");
    }

    #[test]
    fn register_to_null_zone() {
        let mut world = World::new();

        let err = world.register_to_zone(Entity::new(), (1, 2).into(), (3, 4).into()).unwrap_err();

        println!("{err}")
    }

    #[test]
    fn load_zone_clobber() {
        let mut world = World::new();
        
        world.load_empty_zone((0,0).into(), 32, 32).unwrap();
        let err = world.load_empty_zone((0,0).into(), 32, 32).unwrap_err();
        println!("{err}");
    }
}