use std::any::Any;
use gtk::Container;
use crate::ui::application::OApplication;
use crate::ui::handlers::bundle::Bundle;

pub trait Activity {

    fn get_name(&self) -> String;

    fn get_title(&self) -> String;

    fn on_create(&mut self, bundle: Option<Bundle>) -> &Container;

    fn on_resume(&self);

    fn on_pause(&self);

    fn on_destroy(&self);

    fn get_application(&self) -> &OApplication;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn dyn_clone(&self) -> Box<dyn Activity>;
}

impl Clone for Box<dyn Activity> {

    fn clone(&self) -> Box<dyn Activity> {
        self.dyn_clone()
    }
}
