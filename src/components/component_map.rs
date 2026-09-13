use std::{any::{TypeId}, collections::HashMap, marker::PhantomData, ops::DerefMut};

use serde::{Deserialize, Serialize, de::Visitor, ser::SerializeSeq};

use crate::{components::component::{Component, DispatchFn}, events::event::Event};


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

    pub fn insert(&mut self, component: Box<dyn Component>) {
        let component_type = (*component).type_id();
        self.components.insert(component_type, component);
        let c = self.components.get_mut(&component_type).unwrap();

        for (t, f) in c.get_handlers().iter() {
            self.component_map.entry(*t)
                .or_default()
                .push((component_type, *f))
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
            component_map.insert(el);
        }

        Ok(component_map)
    }
}

//-----------------------------------------------TESTS-------------------------------------------------------

#[cfg(test)]
mod tests {
    use modular_rogue_macros::DynamicHandlers;

    use crate::{components::component::Handles};

    use super::*;


    #[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default)]
    #[handles()]
    struct ZeroHandlesComponent {}

    #[typetag::serde(name="_component_map_test_zero_handles_component")]
    impl Component for ZeroHandlesComponent {}

    #[derive(Default)]
    struct GetAttributesEvent {
        attr1: i32,
        attr2: i32
    }
    impl Event for GetAttributesEvent {}

    #[derive(Serialize, Deserialize, DynamicHandlers, Debug, Default)]
    #[handles(GetAttributesEvent)]
    struct OneHandlesComponent {
        attr1: i32,
        attr2: i32,
    }

    #[typetag::serde(name="_component_map_test_one_handles_component")]
    impl Component for OneHandlesComponent {}

    impl Handles<GetAttributesEvent> for OneHandlesComponent {
        fn handle(&mut self, e: &mut GetAttributesEvent) {
            e.attr1 = self.attr1;
            e.attr2 = self.attr2;
        }
    }

    #[test]
    fn insert() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(OneHandlesComponent {attr1: 1, attr2: 2}));
        component_map.insert(Box::new(ZeroHandlesComponent {}));

        assert_eq!(component_map.components.len(), 2);
        assert!(component_map.contains_key(TypeId::of::<OneHandlesComponent>()));
        assert!(component_map.contains_key(TypeId::of::<ZeroHandlesComponent>()));
    }

    #[test]
    fn serialize() {
        let mut component_map = ComponentMap::new();
        component_map.insert(Box::new(OneHandlesComponent {attr1: 1, attr2: 2}));
        component_map.insert(Box::new(ZeroHandlesComponent {}));

        let serialized = serde_json::to_string(&component_map).unwrap();
        
        let mut deserialized: ComponentMap = serde_json::from_str(&serialized).unwrap();

        let mut e = GetAttributesEvent::default();

        deserialized.dispatch(&mut e);

        assert_eq!(deserialized.components.len(), 2);
        assert!(deserialized.contains_key(TypeId::of::<OneHandlesComponent>()));
        assert!(deserialized
            .contains_key(TypeId::of::<ZeroHandlesComponent>()));

        assert_eq!(e.attr1, 1);
        assert_eq!(e.attr2, 2);
    }
}