use crate::{entities::entity::EntityId, errors::access_error::AccessError, worlds::{position::Position, registry::RegistryID, zones::quadtree::QuadTree}};

pub type ZoneID = RegistryID;

pub struct Zone {
    width: u32,
    height: u32,
    entity_tree: QuadTree
}

impl Zone {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            entity_tree: QuadTree::new(width, height)
        }
    }

    pub fn get_all_entities(&self) -> impl Iterator<Item = EntityId> {
        self.entity_tree.get_all()
    }

    pub fn get_entities(&self, pos: Position) -> Result<impl Iterator<Item = EntityId>, AccessError<Position, EntityId>> {
        self.check_position_in_range(pos)?;
        Ok(self.entity_tree.get(pos))
    }

    pub fn put_entity(&mut self, pos: Position, entity: EntityId) -> Result<(), AccessError<Position, EntityId>> {
        self.check_position_in_range(pos)?;
        self.entity_tree.put(pos, entity);
        Ok(())
    }

    pub fn remove_entity(&mut self, pos: Position, entity: EntityId) -> Result<(), AccessError<Position, EntityId>> {
        self.check_position_in_range(pos)?;
        self.entity_tree.remove(pos, entity);
        Ok(())
    }

    fn check_position_in_range(&self, pos: Position) -> Result<(), AccessError<Position, EntityId>> {
        if pos.x >= self.width || pos.y >= self.height {
            Err(AccessError::PositionOutOfBounds { 
                arg: pos, 
                low_bound: (0, 0).into(), 
                high_bound: (self.width, self.height).into() })
        }  else {
            Ok(())
        }
    }
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_size_zone() {
        let zone = Zone::new(0, 0);
        let _ = zone.get_all_entities();
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let zone = Zone::new(1, 1);
        let _ = zone.get_entities((1, 1).into()).expect("");
    }

    #[test]
    fn get_entity() {
        let mut zone = Zone::new(3, 3);
        let id = 1;

        zone.put_entity((1, 1).into(), id).unwrap();

        let ids: Vec<_> = zone.get_entities((1, 1).into()).unwrap().collect();

        assert_eq!(ids.len(), 1);
        assert!(ids.contains(&id));
    }

    #[test]
    fn remove_entity() {
        let mut zone = Zone::new(3, 3);
        let id = 1;

        zone.put_entity((1, 1).into(), id).unwrap();
        zone.remove_entity((1, 1).into(), id).unwrap();

        let ids: Vec<_> = zone.get_entities((1, 1).into()).unwrap().collect();

        assert_eq!(ids.len(), 0);
    }
}