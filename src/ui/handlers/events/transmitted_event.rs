use std::collections::HashMap;
use crate::ui::handlers::events::inter::event::Event;

pub struct TransmittedEvent {
    prevent_default: bool,
    if_bytes: HashMap<i32, usize>
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
        String::from("capture_event")
    }

    fn is_prevent_default(&self) -> bool {
        todo!()
    }

    fn prevent_default(&mut self) {
        todo!()
    }
}
