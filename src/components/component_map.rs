use std::{any::{Any, TypeId}, collections::{HashMap, hash_map::Entry}, ops::DerefMut};


use crate::{components::component::{Component, ConstDispatchFn, MutDispatchFn, Priority}, errors::entity_error::EntityError, events::event::{ConstEvent, MutEvent}};


#[derive(Debug, Default)]
pub struct ComponentMap {
    component_map: HashMap<TypeId, Vec<(Priority, TypeId,  ConstDispatchFn)>>,
    component_map_mut: HashMap<TypeId, Vec<(Priority, TypeId, MutDispatchFn)>>,
    components: HashMap<TypeId, Box<dyn Component>>
}

impl ComponentMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, component: Box<dyn Component>) -> Result<&mut dyn Component, EntityError> {
        let component_type = (*component).type_id();
    
        match self.components.entry(component_type) {
            Entry::Occupied(entry) => {
                Err(EntityError::AttachClobber { 
                    attached: format!("{component:?}"),
                    present: format!("{entry:?}")
                })
            },
            Entry::Vacant(entry) => {
                for (t, f) in component.get_handlers().iter() {
                    let handlers = self.component_map.entry(*t)
                        .or_default();
                        
                    handlers.push((component.get_priority(), component_type, *f));
                    handlers.sort_by(|a, b|
                        b.0.cmp(&a.0)
                    );
                }

                for (t, f) in component.get_handlers_mut().iter() {
                    let handlers = self.component_map_mut.entry(*t)
                        .or_default();
                        
                    handlers.push((component.get_priority(), component_type, *f));
                    handlers.sort_by(|a, b|
                        b.0.cmp(&a.0)
                    );
                }
                
                Ok(entry.insert(component).as_mut())
            }
        }
    }

    pub fn contains_key(&self, t: TypeId) -> bool {
        self.components.contains_key(&t)
    }

    pub fn get<T: Component>(&self) -> Option<&T> {
        self.components.get(&TypeId::of::<T>())
            .map(|comp| comp.as_ref())
            .map(|comp| (comp as &dyn Any).downcast_ref::<T>())
            .flatten()
    }

    pub fn get_mut<T: Component>(&mut self) -> Option<&mut T> {
        self.components.get_mut(&TypeId::of::<T>())
            .map(|comp| comp.as_mut())
            .map(|comp| (comp as &mut dyn Any).downcast_mut::<T>())
            .flatten()
    }

    pub fn dispatch<T : ConstEvent + 'static>(&self, e: &mut T ) {
        if let Some(vec)= self.component_map.get(&TypeId::of::<T>()) {
            for (_, t, f) in vec {
                let component = self.components.get(t).unwrap().as_ref();
                (f)(component, e);
            }
        }   
    }

    pub fn dispatch_mut<T : MutEvent + 'static>(&mut self, e: &mut T ) {
        if let Some(vec)= self.component_map_mut.get(&TypeId::of::<T>()) {
            for (_, t, f) in vec {
                let component = self.components.get_mut(t).unwrap().deref_mut();
                (f)(component, e);
            }
        }    
    }

    pub fn components(&self) -> impl Iterator<Item=&dyn Component> {
        self.components.values().map(|b| b.as_ref())
    }
    
    pub fn num_components(&self) -> usize {
        self.components().collect::<Vec<_>>().len()
    }
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {

    use crate::components::test_util::{TestCounterEvent, TestGetAttributesEvent, TestStatefulComponent, TestPriorityEvent, TestSetAttributesEvent, TestStatelessComponent};

    use super::*;

    #[test]
    fn insert() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(TestStatefulComponent {attr1: 1, attr2: 2})).unwrap();
        component_map.insert(Box::new(TestStatelessComponent {})).unwrap();

        assert_eq!(component_map.num_components(), 2);
        assert!(component_map.contains_key(TypeId::of::<TestStatefulComponent>()));
        assert!(component_map.contains_key(TypeId::of::<TestStatelessComponent>()));
    }

    #[test]
    fn get() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(TestStatefulComponent {attr1: 1, attr2: 2})).unwrap();
        component_map.insert(Box::new(TestStatelessComponent {})).unwrap();

        let comp = component_map.get::<TestStatefulComponent>().unwrap();

        assert!(comp.attr2 == 2);
    }

    #[test]
    fn dispatch() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(TestStatefulComponent {attr1: 1, attr2: 2})).unwrap();

        let mut getter = TestGetAttributesEvent::default();

        component_map.dispatch(&mut getter);

        assert_eq!(getter.attr1, 1);
    }

    #[test]
    fn dispatch_mut() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(TestStatefulComponent {attr1: 0, attr2: 0})).unwrap();

        let mut setter = TestSetAttributesEvent {attr1: 3, attr2:0};

        component_map.dispatch_mut(&mut setter);

        assert_eq!(setter.attr1, 3);
    }

    #[test]
    fn dispatch_noop() {
        let component_map = ComponentMap::new();

        let mut getter = TestGetAttributesEvent::default();

        component_map.dispatch(&mut getter);
    }

    #[test]
    fn dispatch_mut_noop() {
        let mut component_map = ComponentMap::new();

        let mut setter = TestSetAttributesEvent {attr1: 3, attr2:0};

        component_map.dispatch_mut(&mut setter);
    }

    #[test]
    fn double_insert_bug() {
        let mut component_map = ComponentMap::new();

        component_map.insert(Box::new(TestStatelessComponent {})).unwrap();
        let _ = component_map.insert(Box::new(TestStatelessComponent {}));

        let mut e = TestCounterEvent::default();

        component_map.dispatch(&mut e);

        assert_eq!(e.counter, 1);
    }

    #[test]
    fn priority1() {
        let mut component_map = ComponentMap::new();

        component_map.insert(Box::new(TestStatelessComponent::default())).unwrap();
        component_map.insert(Box::new(TestStatefulComponent::default())).unwrap();

        let mut e = TestPriorityEvent::default();

        component_map.dispatch(&mut e);

        assert_eq!(e.last_handler_id.unwrap(), TypeId::of::<TestStatefulComponent>());
    }

    #[test]
    fn priority2() {
        let mut component_map = ComponentMap::new();

        component_map.insert(Box::new(TestStatefulComponent::default())).unwrap();
        component_map.insert(Box::new(TestStatelessComponent::default())).unwrap();

        let mut e = TestPriorityEvent::default();

        component_map.dispatch(&mut e);

        assert_eq!(e.last_handler_id.unwrap(), TypeId::of::<TestStatefulComponent>());
    }
}