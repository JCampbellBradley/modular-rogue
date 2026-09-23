use std::{any::TypeId, collections::HashMap, error::Error, marker::PhantomData, ops::DerefMut};

use serde::{Deserialize, Serialize, de::Visitor, ser::SerializeSeq};

use crate::{components::component::{Component, DispatchFn}, errors::insert_clobber_error::InsertClobberError, events::event::Event};


#[derive(Debug, Default)]
pub struct ComponentMap {
    component_map: HashMap<TypeId, Vec<(TypeId, DispatchFn)>>,
    components: HashMap<TypeId, Box<dyn Component>>
}

impl ComponentMap {
    pub fn new() -> Self {
        ComponentMap {
            component_map: HashMap::new(),
            components: HashMap::new()
        }
    }

    pub fn insert(&mut self, component: Box<dyn Component>) -> Result<(), Box<dyn Error>> {
        let component_type = (*component).type_id();
        if self.contains_key(component_type) {
            Err(Box::new(InsertClobberError {
                argument: component
            }))
        } else {
            let c = self.components.entry(component_type)
                .or_insert(component);

            for (t, f) in c.get_handlers().iter() {
                self.component_map.entry(*t)
                    .or_default()
                    .push((component_type, *f))
            }
            Ok(())
        }
    }

    pub fn contains_key(&self, t: TypeId) -> bool {
        self.components.contains_key(&t)
    }

    pub fn dispatch<T : Event + 'static>(&mut self, e: &mut T ) {
        for (t, f) in self.component_map.entry(TypeId::of::<T>())
            .or_default()
        {
            let component = self.components.get_mut(t).unwrap().deref_mut();
            (f)(component, e);
        }
            
    }
}

impl Serialize for ComponentMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let mut components = serializer.serialize_seq(Some(self.components.len()))?;
        for v in self.components.values() {
            components.serialize_element(v)?;
        }
        components.end()
    }
}

struct VecVisitor<T> {
    marker: PhantomData<fn() -> Vec<T>>
}

impl<T> VecVisitor<T> {
    fn new() -> Self {
        VecVisitor {
            marker: PhantomData
        }
    }
}

impl<'de, T> Visitor<'de> for VecVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a vec of `Component`s")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut vec = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(el) = seq.next_element()? {
            vec.push(el);
        }

        Ok(vec)
    }
}

impl<'de> Deserialize<'de> for ComponentMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let vec = deserializer.deserialize_seq(VecVisitor::new());

        let mut component_map = ComponentMap::new();

        for el in vec.unwrap_or_default() {
            let _ = component_map.insert(el);
        }

        Ok(component_map)
    }
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {

    use crate::components::test_util::{GetAttributesEvent, TestOneHandlesComponent, TestZeroHandlesComponent};

    use super::*;

    #[test]
    fn insert() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(TestOneHandlesComponent {attr1: 1, attr2: 2})).expect("");
        component_map.insert(Box::new(TestZeroHandlesComponent {})).expect("");

        assert_eq!(component_map.components.len(), 2);
        assert!(component_map.contains_key(TypeId::of::<TestOneHandlesComponent>()));
        assert!(component_map.contains_key(TypeId::of::<TestZeroHandlesComponent>()));
    }

    #[test]
    fn double_insert() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(TestZeroHandlesComponent {})).expect("");
        assert!(component_map.insert(Box::new(TestZeroHandlesComponent {})).is_err());

        assert_eq!(component_map.components.len(), 1);
        assert!(component_map.contains_key(TypeId::of::<TestZeroHandlesComponent>()));
    }

    #[test]
    fn serialize() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(TestOneHandlesComponent {attr1: 1, attr2: 2})).expect("");
        component_map.insert(Box::new(TestZeroHandlesComponent {})).expect("");

        let serialized = serde_json::to_string(&component_map).unwrap();
        
        let mut deserialized: ComponentMap = serde_json::from_str(&serialized).unwrap();

        let mut e = GetAttributesEvent::default();

        deserialized.dispatch(&mut e);

        assert_eq!(deserialized.components.len(), 2);
        assert!(deserialized.contains_key(TypeId::of::<TestOneHandlesComponent>()));
        assert!(deserialized
            .contains_key(TypeId::of::<TestZeroHandlesComponent>()));

        assert_eq!(e.attr1, 1);
        assert_eq!(e.attr2, 2);
    }
}