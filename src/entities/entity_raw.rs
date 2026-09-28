
use serde::{Deserialize, Serialize};

use crate::worlds::position::Position;

#[derive(Serialize, Deserialize)]
pub struct EntityRaw {
    components: Vec<(String, ron::Value)>,
    zone_position: Option<Position>,
    position: Option<Position>,
}