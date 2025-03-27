use std::any::Any;
use std::collections::HashMap;
use crate::oldui::handlers::events::inter::event::Event;

#[derive(Clone)]
pub struct PermissionEvent {
    prevent_default: bool,
    has_permission: bool
}

impl PermissionEvent {

    pub fn new(has_permission: bool) -> Self {
        Self {
            prevent_default: false,
            has_permission
        }
    }

    pub fn has_permission(&self) -> bool {
        self.has_permission
    }
}

impl Event for PermissionEvent {

    fn get_name(&self) -> String {
        String::from("permission_event")
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
