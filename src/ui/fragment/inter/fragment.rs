use std::any::Any;
use gtk::Container;
use crate::ui::activity::inter::activity::Activity;

pub trait Fragment {

    fn on_create(&mut self) -> &Container;

    fn on_resume(&self);

    fn on_pause(&self);

    fn on_destroy(&self);

    fn get_activity(&self) -> &Box<dyn Activity>;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn dyn_clone(&self) -> Box<dyn Fragment>;
}
