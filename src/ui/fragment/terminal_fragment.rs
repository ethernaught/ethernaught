use std::any::Any;
use gtk::{Builder, Button, Container, DrawingArea};
use gtk::prelude::{BuilderExtManual, ButtonExt, Cast, ContainerExt, PanedExt, WidgetExt, WidgetExtManual};
use pcap::packet::packet::Packet;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::fragment::inter::fragment::Fragment;

#[derive(Clone)]
pub struct TerminalFragment {
    activity: Box<dyn Activity>,
    root: Option<Container>
}

impl TerminalFragment {

    pub fn new(activity: Box<dyn Activity>) -> Self {
        Self {
            activity,
            root: None
        }
    }
}

impl Fragment for TerminalFragment {

    fn on_create(&mut self) -> &Container {
        let builder = Builder::from_resource("/com/ethernaut/rust/res/ui/gtk3/terminal_fragment.ui");

        self.root = Some(builder
            .object("terminal_layout")
            .expect("Couldn't find 'terminal_layout' in window.ui"));

        &self.root.as_ref().unwrap().upcast_ref()
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

    fn get_activity(&self) -> &Box<dyn Activity> {
        &self.activity
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Fragment> {
        Box::new(self.clone())
    }
}
