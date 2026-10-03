use std::{any::{Any, TypeId}, collections::HashMap, ops::DerefMut};


use crate::{components::component::{Component, ConstDispatchFn, MutDispatchFn}, errors::access_error::AccessError, events::event::{ConstEvent, MutEvent}};


#[derive(Debug, Default)]
pub struct ComponentMap {
    component_map: HashMap<TypeId, Vec<(TypeId, ConstDispatchFn)>>,
    component_map_mut: HashMap<TypeId, Vec<(TypeId, MutDispatchFn)>>,
    components: HashMap<TypeId, Box<dyn Component>>
}

impl ComponentMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, component: Box<dyn Component>) -> Result<(), AccessError<TypeId, Box<dyn Component>>> {
        let component_type = (*component).type_id();
        if self.contains_key(component_type) {
            Err(AccessError::BadAdd { 
                item: component
            })
        } else {
            let c = self.components.entry(component_type)
                .or_insert(component);

            for (t, f) in c.get_handlers().iter() {
                self.component_map.entry(*t)
                    .or_default()
                    .push((component_type, *f));
            }


            for (t, f) in c.get_handlers_mut().iter() {
                self.component_map_mut.entry(*t)
                    .or_default()
                    .push((component_type, *f));
            }

            Ok(())
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

    pub fn components(&self) -> impl Iterator<Item=&dyn Component> {
        self.components.values().map(|b| b.as_ref())
    }

    pub fn dispatch<T : ConstEvent + 'static>(&self, e: &mut T ) {
        if let Some(vec)= self.component_map.get(&TypeId::of::<T>()) {
            for (t, f) in vec {
                let component = self.components.get(t).unwrap().as_ref();
                (f)(component, e);
            }
        }   
    }

    pub fn dispatch_mut<T : MutEvent + 'static>(&mut self, e: &mut T ) {
        if let Some(vec)= self.component_map_mut.get(&TypeId::of::<T>()) {
            for (t, f) in vec {
                let component = self.components.get_mut(t).unwrap().deref_mut();
                (f)(component, e);
            }
        }    
    }
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {

    use crate::components::test_util::{TestGetAttributesEvent, TestOneHandlesComponent, TestSetAttributesEvent, TestZeroHandlesComponent};

    use super::*;

    #[test]
    fn insert() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(TestOneHandlesComponent {attr1: 1, attr2: 2})).unwrap();
        component_map.insert(Box::new(TestZeroHandlesComponent {})).unwrap();

        assert_eq!(component_map.components().collect::<Vec<_>>().len(), 2);
        assert!(component_map.contains_key(TypeId::of::<TestOneHandlesComponent>()));
        assert!(component_map.contains_key(TypeId::of::<TestZeroHandlesComponent>()));
    }

    #[test]
    fn double_insert() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(TestZeroHandlesComponent {})).unwrap();
        assert!(component_map.insert(Box::new(TestZeroHandlesComponent {})).is_err());

        assert_eq!(component_map.components.len(), 1);
        assert!(component_map.contains_key(TypeId::of::<TestZeroHandlesComponent>()));
    }

    #[test]
    fn get() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(TestOneHandlesComponent {attr1: 1, attr2: 2})).unwrap();
        component_map.insert(Box::new(TestZeroHandlesComponent {})).unwrap();

        let comp = component_map.get::<TestOneHandlesComponent>().unwrap();

        assert!(comp.attr2 == 2);
    }

    #[test]
    fn dispatch() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(TestOneHandlesComponent {attr1: 1, attr2: 2})).unwrap();

        let mut getter = TestGetAttributesEvent::default();

        component_map.dispatch(&mut getter);

        assert_eq!(getter.attr1, 1);
    }

    #[test]
    fn dispatch_mut() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(TestOneHandlesComponent {attr1: 0, attr2: 0})).unwrap();

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
}