use std::any::Any;
use crate::ui::handlers::bundle::Bundle;

pub trait IWindow {

    fn get_name(&self) -> String;

    fn get_title(&self) -> String;

    fn on_create(&mut self, bundle: Option<Bundle>);

    fn on_resume(&self);

    fn on_pause(&self);

    fn on_destroy(&self);

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn dyn_clone(&self) -> Box<dyn IWindow>;
}

impl Clone for Box<dyn IWindow> {

    fn clone(&self) -> Box<dyn IWindow> {
        self.dyn_clone()
    }
}
