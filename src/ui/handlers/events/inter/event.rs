use std::any::Any;

pub trait Event: Send {

    fn get_name(&self) -> String;

    fn is_prevent_default(&self) -> bool;

    fn prevent_default(&mut self);

    fn upcast(&self) -> &dyn Event;

    fn upcast_mut(&mut self) -> &mut dyn Event;

    fn dyn_clone(&self) -> Box<dyn Event>;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
