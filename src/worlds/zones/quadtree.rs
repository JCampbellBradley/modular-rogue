use std::{fmt::Debug, iter, ops::Deref};

use crate::{entities::entity::EntityId, worlds::position::{DiagonalDir, Position}};

#[derive(Debug)]
pub struct QuadTree {
    root: QuadBranch
}

impl QuadTree {
    pub fn new(width: u32, height: u32) -> Self {
        QuadTree { root: QuadBranch::new(width, height) }
    }

    pub fn get_all(&self) -> impl Iterator<Item = EntityId> {
        self.root.get_all()
    }

    pub fn get(&self, pos: Position) -> impl Iterator<Item = EntityId> {
        self.root.get(pos)
    }

    pub fn put(&mut self, pos: Position, entity: EntityId) {
        self.root.put(pos, entity)
    }

    pub fn remove(&mut self, pos: Position, entity: EntityId) {
        self.root.remove(pos, entity)
    }
}

trait QuadNode : Debug {
    fn is_empty(&self) -> bool;
    fn get(&self, pos: Position) -> Box<dyn Iterator<Item = EntityId> + '_>;
    fn get_all(&self) -> Box<dyn Iterator<Item = EntityId> + '_>;
    fn put(&mut self, pos: Position, entity: EntityId);
    fn remove(&mut self, pos: Position, entity: EntityId);
}

#[derive(Debug)]
struct QuadBranch {
    width: u32,
    height: u32,
    subnodes: [Option<Box<dyn QuadNode>>; 4],
}

impl QuadBranch {
    fn new(width: u32, height: u32) -> Self {
        Self { 
            width, 
            height, 
            subnodes: [None, None, None, None]
        }
    }

    fn left_width(&self) -> u32 {
        self.width / 2
    }

    fn right_width(&self) -> u32 {
        self.width - self.left_width()
    }

    fn bottom_height(&self) -> u32 {
        self.height / 2
    }

    fn top_height(&self) -> u32 {
        self.height - self.bottom_height()
    }

    fn get_subnode_index(&self, pos: Position) -> usize {
        match self.pos_to_orthant(pos) {
            DiagonalDir::NE => 0,
            DiagonalDir::NW => 1,
            DiagonalDir::SE => 2,
            DiagonalDir::SW => 3
        }
    }

    fn get_subnode_size(&self, pos: Position) -> (u32, u32) {
        match self.pos_to_orthant(pos) {
            DiagonalDir::NE => (self.right_width(), self.top_height()),
            DiagonalDir::NW => (self.left_width(), self.top_height()),
            DiagonalDir::SE => (self.right_width(), self.bottom_height()),
            DiagonalDir::SW => (self.left_width(), self.bottom_height())
        }
    }

    fn localize_pos(&self, pos: Position) -> Position {
        let new_zero = match self.pos_to_orthant(pos) {
            DiagonalDir::NE => (self.left_width(), self.bottom_height()).into(),
            DiagonalDir::NW => (0, self.bottom_height()).into(),
            DiagonalDir::SE => (self.left_width(), 0).into(),
            DiagonalDir::SW => (0, 0).into()
        };

        (pos - new_zero).try_into().unwrap()
    }

    fn pos_to_orthant(&self, pos: Position) -> DiagonalDir {
        let delta = pos - (self.left_width(), self.bottom_height()).into();
        DiagonalDir::orthant_from_delta(&delta)
    }

    fn get_node(&self, pos: Position) -> Option<&dyn QuadNode> {
        match &self.subnodes[self.get_subnode_index(pos)] {
            Some(node) => Some(node.deref()),
            _ => None
        }
    }

    fn get_node_mut(&mut self, pos: Position) -> Option<&mut Box<dyn QuadNode>> {
        match &mut self.subnodes[self.get_subnode_index(pos)] {
            Some(node) => Some(node),
            _ => None
        }
    }

    fn set_node(&mut self, pos: Position, node: Box<dyn QuadNode>) {
        let index = self.get_subnode_index(pos);
        self.subnodes[index] = Some(node);
    }

    fn remove_node(&mut self, pos: Position) {
        let index = self.get_subnode_index(pos);
        self.subnodes[index] = None;    
    }

    fn get_node_or_create(&mut self, pos: Position) -> Option<&mut Box<dyn QuadNode>> {
        if self.get_node(pos).is_none() {
            let (width, height) = self.get_subnode_size(pos);
            
            self.set_node(pos, if width == 1 && height == 1 {
                Box::new(QuadLeaf::new())
            } else {
                Box::new(QuadBranch::new(width, height))
            });
        }

        match &mut self.subnodes[self.get_subnode_index(pos)] {
            Some(node) => Some(node),
            _ => None
        }
    }
}

impl QuadNode for QuadBranch {    
    fn is_empty(&self) -> bool {
        self.subnodes.iter().all(|node| node.is_none())
    }

    fn get(&self, pos: Position) -> Box<dyn Iterator<Item = EntityId> + '_> {
        let new_pos = self.localize_pos(pos);
        
        if let Some(node) = self.get_node(pos) {
            node.get(new_pos)
        } else {
            Box::new(iter::empty())
        }
    }

    fn put(&mut self, pos: Position, entity: EntityId) {
        let new_pos = self.localize_pos(pos);

        if let Some(node) = self.get_node_or_create(pos) {
            node.put(new_pos, entity)
        }
    }

    fn remove(&mut self, pos: Position, entity: EntityId) {
        let new_pos = self.localize_pos(pos);

        if let Some(node) = self.get_node_mut(pos){
            node.remove(new_pos, entity);

            if node.is_empty() {
                self.remove_node(pos);
            }
        }
    }

    fn get_all(&self) -> Box<dyn Iterator<Item = EntityId> + '_> {
        Box::new(
            self.subnodes.iter().flatten()
                .flat_map(|node| node.get_all())
        )
    }
}

#[derive(Default, Debug)]
struct QuadLeaf {
    contents: Vec<EntityId>
}

impl QuadLeaf {
    pub fn new() -> Self {
        Self::default()
    }
}

impl QuadNode for QuadLeaf {
    fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    fn get(&self, _pos: Position) -> Box<dyn Iterator<Item = EntityId> + '_> {
        self.get_all()
    }

    fn put(&mut self, _pos: Position, entity: EntityId) {
        self.contents.push(entity);
    }

    fn remove(&mut self, _pos: Position, entity: EntityId) {
        self.contents.retain(|ent| *ent != entity);
    }

    fn get_all(&self) -> Box<dyn Iterator<Item = EntityId> + '_> {
        Box::new(self.contents.iter().copied())
    }
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get_unitary() {
        let mut tree = QuadTree::new(1, 1);
        tree.put((0, 0).into(), 1);
        tree.put((0, 0).into(), 2);
        tree.put((0, 0).into(), 3);
        let ents: Vec<EntityId> = tree.get((0, 0).into()).collect();
        assert_eq!(ents.len(), 3);
        assert!(ents.contains(&1));
        assert!(ents.contains(&2));
        assert!(ents.contains(&3));
    }

    #[test]
    fn put_remove_unitary() {
        let mut tree = QuadTree::new(1, 1);
        tree.put((0, 0).into(), 1);
        tree.put((0, 0).into(), 2);
        tree.put((0, 0).into(), 3);
        tree.remove((0, 0).into(), 1);
        tree.remove((0, 0).into(), 3);
        let ents: Vec<EntityId> = tree.get((0, 0).into()).collect();
        assert_eq!(ents.len(), 1);
        assert!(ents.contains(&2));
    }
    
    #[test]
    fn put_iter_unitary() {
        let mut tree = QuadTree::new(1, 1);
        tree.put((0, 0).into(), 1);
        tree.put((0, 0).into(), 2);
        tree.put((0, 0).into(), 3);
        let ents: Vec<EntityId> = tree.get_all().collect();
        assert_eq!(ents.len(), 3);
        assert!(ents.contains(&1));
        assert!(ents.contains(&2));
        assert!(ents.contains(&3));
    }

    #[test]
    fn put_get_oblong() {
        let mut tree = QuadTree::new(1, 2);
        tree.put((0, 1).into(), 1);
        tree.put((0, 1).into(), 2);
        tree.put((0, 0).into(), 3);
        let ents: Vec<EntityId> = tree.get((0, 1).into()).collect();
        assert_eq!(ents.len(), 2);
        assert!(ents.contains(&1));
        assert!(ents.contains(&2));
    }

    #[test]
    fn put_remove_oblong() {
        let mut tree = QuadTree::new(1, 2);
        tree.put((0, 1).into(), 1);
        tree.put((0, 1).into(), 2);
        tree.put((0, 0).into(), 3);
        tree.remove((0, 1).into(), 1);
        tree.remove((0, 0).into(), 3);
        let ents: Vec<EntityId> = tree.get_all().collect();
        assert_eq!(ents.len(), 1);
        assert!(ents.contains(&2));
    }

    #[test]
    fn put_iter_oblong() {
        let mut tree = QuadTree::new(1, 2);
        tree.put((0, 1).into(), 1);
        tree.put((0, 1).into(), 2);
        tree.put((0, 0).into(), 3);
        let ents: Vec<EntityId> = tree.get_all().collect();
        assert_eq!(ents.len(), 3);
        assert!(ents.contains(&1));
        assert!(ents.contains(&2));
        assert!(ents.contains(&3));
    }

    #[test]
    fn put_get_square() {
        let mut tree = QuadTree::new(2, 2);
        tree.put((0, 0).into(), 1);
        tree.put((0, 1).into(), 2);
        tree.put((1, 0).into(), 3);
        tree.put((1, 1).into(), 4);
        let ents1: Vec<EntityId> = tree.get((0, 0).into()).collect();
        let ents2: Vec<EntityId> = tree.get((0, 1).into()).collect();
        let ents3: Vec<EntityId> = tree.get((1, 0).into()).collect();
        let ents4: Vec<EntityId> = tree.get((1, 1).into()).collect();
        assert!(ents1.contains(&1));
        assert!(ents2.contains(&2));
        assert!(ents3.contains(&3));
        assert!(ents4.contains(&4));
    }

    #[test]
    fn put_get_big() {
        let mut tree = QuadTree::new(9, 9);
        tree.put((0, 0).into(), 1);
        tree.put((3, 7).into(), 2);
        tree.put((8, 8).into(), 3);
        tree.put((8, 0).into(), 4);
        let ents1: Vec<EntityId> = tree.get((0, 0).into()).collect();
        let ents2: Vec<EntityId> = tree.get((3, 7).into()).collect();
        let ents3: Vec<EntityId> = tree.get((8, 8).into()).collect();
        let ents4: Vec<EntityId> = tree.get((8, 0).into()).collect();
        assert!(ents1.contains(&1));
        assert!(ents2.contains(&2));
        assert!(ents3.contains(&3));
        assert!(ents4.contains(&4));
    }

    #[test]
    fn put_remove_big() {
        let mut tree = QuadTree::new(9, 9);
        tree.put((0, 0).into(), 1);
        tree.put((3, 7).into(), 2);
        tree.put((8, 8).into(), 3);
        tree.put((8, 0).into(), 4);
        tree.remove((0, 0).into(), 1);
        tree.remove((3, 7).into(), 2);
        tree.remove((8, 8).into(), 3);
        tree.remove((8, 0).into(), 4);
        assert!(tree.root.is_empty());
    }
}