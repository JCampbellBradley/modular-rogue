use crate::{entities::entity::EntityId, errors::zone_error::ZoneError, worlds::{position::Position, registry::RegistryID, zones::quadtree::QuadTree}};

pub type ZoneID = RegistryID;

#[derive(Debug)]
pub struct Zone {
    pub width: u32,
    pub height: u32,
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

    pub fn get_entities(&self, pos: Position) -> Result<impl Iterator<Item = EntityId>, ZoneError> {
        self.check_position_in_range(pos)?;
        Ok(self.entity_tree.get(pos))
    }

    pub fn put_entity(&mut self, pos: Position, entity: EntityId) -> Result<(), ZoneError> {
        self.check_position_in_range(pos)?;
        self.entity_tree.put(pos, entity);
        Ok(())
    }

    pub fn remove_entity(&mut self, pos: Position, entity: EntityId) -> Result<(), ZoneError> {
        self.check_position_in_range(pos)?;
        self.entity_tree.remove(pos, entity);
        Ok(())
    }

    fn check_position_in_range(&self, pos: Position) -> Result<(), ZoneError> {
        if pos.x >= self.width || pos.y >= self.height {
            Err(ZoneError::PositionOutOfBounds { 
                arg: pos, 
                low_bound: Position{x:0, y:0}, 
                high_bound: Position{x:self.width, y:self.height}
            })
        } else {
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
    fn out_of_bounds() {
        let zone = Zone::new(1, 1);

        assert!(zone.get_entities((1, 1).into()).is_err());
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
    fn get_entity_oob() {
        let mut zone = Zone::new(3, 3);
        let id = 1;

        let err = zone.put_entity((4, 4).into(), id).unwrap_err();
        println!("{err}")
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

    #[test]
    fn remove_entity_oob() {
        let mut zone = Zone::new(3, 3);
        let id = 1;

        let err = zone.remove_entity((4, 4).into(), id).unwrap_err();
        println!("{err}")
    }
}