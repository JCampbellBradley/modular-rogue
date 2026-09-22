use crate::{entities::entity::EntityId, errors::illegal_argument_error::IllegalArgumentError, worlds::{position::Position, registry::RegistryID, zones::{error_messages::{OUT_OF_BOUNDS}, quadtree::QuadTree}}};

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

    pub fn get_entities(&self, pos: Position) -> Result<impl Iterator<Item = EntityId>, IllegalArgumentError<Position>> {
        self.check_position_in_range(pos)?;
        Ok(self.entity_tree.get(pos))
    }

    pub fn put_entity(&mut self, pos: Position, entity: EntityId) -> Result<(), IllegalArgumentError<Position>> {
        self.check_position_in_range(pos)?;
        self.entity_tree.put(pos, entity);
        Ok(())
    }

    pub fn remove_entity(&mut self, pos: Position, entity: EntityId) -> Result<(), IllegalArgumentError<Position>> {
        self.check_position_in_range(pos)?;
        self.entity_tree.remove(pos, entity);
        Ok(())
    }

    fn check_position_in_range(&self, pos: Position) -> Result<(), IllegalArgumentError<Position>> {
        if pos.x >= self.width || pos.y >= self.height {
            Err(IllegalArgumentError {
                argument_name: "pos",
                argument: pos, 
                reason: OUT_OF_BOUNDS
            })
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
}