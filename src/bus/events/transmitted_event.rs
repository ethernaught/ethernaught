use std::any::Any;
use std::collections::HashMap;
use crate::bus::events::inter::event::Event;

#[derive(Debug, Clone)]
pub struct TransmittedEvent {
    prevent_default: bool,
    pub(crate) if_bytes: HashMap<i32, usize>
}

impl TransmittedEvent {

    pub fn new(if_bytes: HashMap<i32, usize>) -> Self {
        Self {
            prevent_default: false,
            if_bytes
        }
    }
}

impl Event for TransmittedEvent {

    fn get_name(&self) -> String {
        String::from("transmitted_event")
    }

    fn is_prevent_default(&self) -> bool {
        todo!()
    }

    fn prevent_default(&mut self) {
        todo!()
    }

    fn upcast(&self) -> &dyn Event {
        self
    }

    fn upcast_mut(&mut self) -> &mut dyn Event {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Event> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
