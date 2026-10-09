use modular_rogue::{DynamicHandlers, ModElement, components::component::{Component}, serde::{Deserialize, Serialize}};

#[derive(Serialize, Deserialize, Default, DynamicHandlers, Debug, ModElement)]
#[handles()]
#[serde(default, crate = "modular_rogue::serde")]
pub struct TestComponent {
}

impl Component for TestComponent {
    
}
