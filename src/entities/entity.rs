use std::any::TypeId;


use crate::{components::{component::Component, component_map::ComponentMap}, errors::entity_error::EntityError, events::event::{ConstEvent, MutEvent}, worlds::{position::Position, registry::{Registerable, RegistryID}}};

pub type EntityId = RegistryID;

#[derive(Debug)]
pub struct Entity {
    id: Option<EntityId>,
    component_map: ComponentMap,
    pub zone_position: Option<Position>,
    pub position: Option<Position>
}

impl Entity{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn require<T: Component + Default>(&mut self) -> &mut T {
        let _ = self.component_map.insert(Box::new(T::default()));
        self.component_map.get_mut::<T>().unwrap()
    }

    pub fn generate<T: Component + Default>(&mut self) -> Result<&mut dyn Component, EntityError> {
        self.attach(Box::new(T::default()))
    }

    pub fn attach(&mut self, component: Box<dyn Component>) -> Result<&mut dyn Component, EntityError> {
        self.component_map.insert(component)
    }

    pub fn handle<T: ConstEvent + 'static>(&self, e: &mut T) {
        self.component_map.dispatch::<T>(e);
    }

    pub fn handle_mut<T: MutEvent + 'static>(&mut self, e: &mut T) {
        self.component_map.dispatch_mut::<T>(e);
    }

    pub fn has_component(&self, t: TypeId) -> bool {
        self.component_map.contains_key(t)
    }

    pub fn get_component<T: Component>(&self) -> Option<&T> {
        self.component_map.get::<T>()
    }

    pub fn components(&self) -> impl Iterator<Item=&dyn Component> {
        self.component_map.components()
    }
    
    pub fn num_components(&self) -> usize {
        self.component_map.num_components()
    }
}

impl Registerable for Entity {
    fn get_id(&self) -> Option<RegistryID> {
        self.id
    }

    fn set_id(&mut self, id: RegistryID) {
        self.id = Some(id);
    }

    fn remove_id(&mut self) {
        self.id = None;
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity { 
            id: None, 
            component_map: ComponentMap::new(), 
            zone_position: None, 
            position: None 
        }
    }
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {

    use crate::components::test_util::{TestGetAttributesEvent, TestStatefulComponent, TestSetAttributesEvent, TestStatelessComponent};

use super::*;

    #[test]
    fn generate() {
        let mut ent = Entity::new();
        ent.generate::<TestStatefulComponent>().unwrap();
        ent.generate::<TestStatelessComponent>().unwrap();

        assert_eq!(ent.num_components(), 2);
        assert!(ent.has_component(TypeId::of::<TestStatefulComponent>()));
        assert!(ent.has_component(TypeId::of::<TestStatelessComponent>()));
    }

    #[test]
    fn generate_and_use() {
        let mut ent = Entity::new();
        let comp = ent.generate::<TestStatefulComponent>().unwrap();

        assert!((*comp).type_id() == TypeId::of::<TestStatefulComponent>())
    }

    #[test]
    fn generate_double() {
        let mut ent = Entity::new();
        ent.generate::<TestStatefulComponent>().unwrap();
        let err = ent.generate::<TestStatefulComponent>();

        assert!(err.is_err());

        assert_eq!(ent.num_components(), 1);
        assert!(ent.has_component(TypeId::of::<TestStatefulComponent>()));
    }
    
    #[test]
    fn attach() {
        let mut ent = Entity::new();
        ent.attach(Box::new(TestStatefulComponent::default())).unwrap();
        ent.attach(Box::new(TestStatelessComponent::default())).unwrap();

        assert_eq!(ent.num_components(), 2);
        assert!(ent.has_component(TypeId::of::<TestStatefulComponent>()));
        assert!(ent.has_component(TypeId::of::<TestStatelessComponent>()));
    }
    
    #[test]
    fn attach_and_use() {
        let mut ent = Entity::new();
        let comp = ent.attach(Box::new(TestStatefulComponent::default())).unwrap();
       
        assert!((*comp).type_id() == TypeId::of::<TestStatefulComponent>())
    }

    #[test]
    fn attach_double() {
        let mut ent = Entity::new();
        ent.attach(Box::new(TestStatefulComponent::default())).unwrap();
        let err = ent.attach(Box::new(TestStatefulComponent::default()));

        assert!(err.is_err());

        assert_eq!(ent.num_components(), 1);
        assert!(ent.has_component(TypeId::of::<TestStatefulComponent>()));
    }
    
    #[test]
    fn require() {
        let mut ent = Entity::new();
        ent.require::<TestStatelessComponent>();
        ent.require::<TestStatefulComponent>();

        assert_eq!(ent.num_components(), 2);
        assert!(ent.has_component(TypeId::of::<TestStatefulComponent>()));
        assert!(ent.has_component(TypeId::of::<TestStatelessComponent>()));
    }
    
    #[test]
    fn require_and_use() {
        let mut ent = Entity::new();
        let comp = ent.require::<TestStatefulComponent>();

        assert_eq!(comp.attr1, 0);
    }

    #[test]
    fn require_double() {
        let mut ent = Entity::new();
        ent.require::<TestStatelessComponent>();
        ent.require::<TestStatelessComponent>();

        assert_eq!(ent.num_components(), 1);
        assert!(ent.has_component(TypeId::of::<TestStatelessComponent>()));
    }

    #[test]
    fn get() {
        let mut ent = Entity::new();
        ent.attach(Box::new(TestStatefulComponent {attr1: 1, attr2: 2})).unwrap();
        ent.attach(Box::new(TestStatelessComponent {})).unwrap();

        let comp = ent.get_component::<TestStatefulComponent>().unwrap();

        assert!(comp.attr2 == 2);
    }

    #[test]
    fn dispatch() {
        let mut ent = Entity::new();
        ent.attach(Box::new(TestStatefulComponent {attr1: 1, attr2: 2})).unwrap();

        let mut getter = TestGetAttributesEvent::default();

        ent.handle(&mut getter);

        assert_eq!(getter.attr1, 1);
    }

    #[test]
    fn dispatch_mut() {
        let mut ent = Entity::new();
        ent.attach(Box::new(TestStatefulComponent {attr1: 0, attr2: 0})).unwrap();

        let mut setter = TestSetAttributesEvent {attr1: 3, attr2:0};

        ent.handle_mut(&mut setter);

        assert_eq!(setter.attr1, 3);
    }

    #[test]
    fn components(){
        let mut ent = Entity::new();
        ent.require::<TestStatelessComponent>();
        ent.require::<TestStatefulComponent>();

        let ids: Vec<_> = ent.components()
            .map(|comp| comp.type_id())
            .collect();

        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&TypeId::of::<TestStatelessComponent>()));
        assert!(ids.contains(&TypeId::of::<TestStatefulComponent>()));
    }
}