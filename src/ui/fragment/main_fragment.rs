use gtk::{Container, Paned};
use crate::ui::fragment::inter::fragment::Fragment;

pub struct MainFragment {
    root: Option<Paned>
}

impl MainFragment {

    pub fn new() -> Self {
        Self {
            root: None
        }
    }
}

impl Fragment for MainFragment {

    fn on_create(&mut self) -> &Container {
        todo!()
    }

    fn on_resume(&self) {
        todo!()
    }

    fn on_pause(&self) {
        todo!()
    }

    fn on_destroy(&self) {
        todo!()
    }
}
