use std::any::Any;
use gtk::prelude::*;
use gtk::{gdk, glib, Adjustment, Application, ApplicationWindow, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Paned, ScrolledWindow, Stack, StyleContext, TextTag, TextView, Widget};
use pcap::packet::inter::interfaces::Interfaces;
use pcap::packet::packet::Packet;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::adapters::packet_adapter::PacketAdapter;
use crate::ui::fragment::inter::fragment::Fragment;
use crate::ui::fragment::sidebar_fragment::SidebarFragment;

#[derive(Clone)]
pub struct MainFragment {
    activity: Box<dyn Activity>,
    root: Option<Container>,
    packet_adapter: Option<PacketAdapter>
}

impl MainFragment {

    pub fn new(activity: Box<dyn Activity>) -> Self {
        Self {
            activity,
            root: None,
            packet_adapter: None
        }
    }

    pub fn get_packet_adapter(&self) -> Option<PacketAdapter> {
        self.packet_adapter.clone()
    }
}

impl Fragment for MainFragment {

    fn on_create(&mut self) -> &Container {
        let builder = Builder::from_file("res/ui/gtk3/main-fragment.ui");

        self.root = Some(builder
            .object("content_layout")
            .expect("Couldn't find 'content_layout' in window.ui"));

        let hadjustment = Adjustment::new(0.0, 0.0, 1000.0, 10.0, 100.0, 100.0);
        let vadjustment = Adjustment::new(0.0, 0.0, 1000.0, 10.0, 100.0, 100.0);

        let list_header_scroll_layout: ScrolledWindow = builder
            .object("list_header_scroll_layout")
            .expect("Couldn't find 'list_header_scroll_layout' in window.ui");
        list_header_scroll_layout.set_hadjustment(Some(&hadjustment));
        list_header_scroll_layout.set_vadjustment(None::<&Adjustment>);

        let list_scroll_layout: ScrolledWindow = builder
            .object("list_scroll_layout")
            .expect("Couldn't find 'list_scroll_layout' in window.ui");

        list_scroll_layout.set_hadjustment(Some(&hadjustment));
        list_scroll_layout.set_vadjustment(Some(&vadjustment));

        let list_box: ListBox = builder
            .object("list_box")
            .expect("Couldn't find 'list_box' in window.ui");

        self.packet_adapter = Some(PacketAdapter::new(&list_box));

        let _self = self.clone();
        list_box.connect_row_activated(move |_, row| {
            let main_activity = _self.activity.as_any().downcast_ref::<MainActivity>().unwrap();

            let packet = _self.packet_adapter.as_ref().unwrap().get_packet_by_index(row.index() as usize);

            let mut sidebar_fragment = SidebarFragment::new(_self.activity.dyn_clone(), packet);
            main_activity.open_sidebar(sidebar_fragment.dyn_clone());
        });

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
