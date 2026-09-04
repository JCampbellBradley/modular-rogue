use crate::event::event::Event;



#[typetag::serde(tag = "type")]
pub trait Component {
    fn handle(&mut self, _e: &mut Box<dyn Event>) {

    }

    fn get_priority(&self) -> i64 {
        0
    }
}
