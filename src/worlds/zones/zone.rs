use crate::{entities::entity::EntityId, errors::illegal_argument_error::IllegalArgumentError, worlds::{position::Position, registry::RegistryID, zones::{error_messages::{OUT_OF_BOUNDS, ZERO_SIZE_ZONE}, quadtree::QuadTree}}};

pub type ZoneID = RegistryID;

pub struct Zone {
    width: u32,
    height: u32,
    entity_tree: QuadTree
}

impl Zone {
    pub fn new(width: u32, height: u32) -> Self {
        Self::check_size_valid(width, height);

        Self {
            width,
            height,
            entity_tree: QuadTree::new(width, height)
        }
    }

    pub fn get_all_entities(&self) -> impl Iterator<Item = EntityId> {
        self.entity_tree.get_all()
    }

    pub fn get_entities(&self, pos: Position) -> impl Iterator<Item = EntityId> {
        self.check_position_in_range(pos);
        self.entity_tree.get(pos)
    }

    pub fn put_entities(&mut self, pos: Position, entity: EntityId) {
        self.check_position_in_range(pos);
        self.entity_tree.put(pos, entity)
    }

    pub fn remove_entities(&mut self, pos: Position, entity: EntityId) {
        self.check_position_in_range(pos);
        self.entity_tree.remove(pos, entity)
    }

    fn check_size_valid(width: u32, height: u32) {
        if width == 0 {
            let err = IllegalArgumentError {
                argument_name: "width",
                argument: width, 
                reason: ZERO_SIZE_ZONE
            };
            panic!("{err}");
        } else if height == 0 {
            let err = IllegalArgumentError {
                argument_name: "height",
                argument: height, 
                reason: ZERO_SIZE_ZONE
            };
            panic!("{err}");
        }
    }

    fn check_position_in_range(&self, pos: Position) {
        if pos.x >= self.width || pos.y >= self.height {
            let err = IllegalArgumentError {
                argument_name: "pos",
                argument: pos, 
                reason: OUT_OF_BOUNDS
            };
            panic!("{err}");
        } 
    }
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn zero_size_zone() {
        let _ = Zone::new(1, 0);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let zone = Zone::new(1, 1);
        let _ = zone.get_entities((1, 1).into());
    }
}