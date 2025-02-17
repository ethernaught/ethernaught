use std::any::Any;
use gtk::prelude::*;
use gtk::{gdk, glib, Adjustment, Application, ApplicationWindow, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Paned, ScrolledWindow, Stack, StyleContext, TextTag, TextView, Widget};
use pcap::packet::inter::interfaces::Interfaces;
use pcap::packet::packet::{decode_packet, Packet};
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




        //TEMPORARY

        let hex_data: Vec<u8> = vec![0xf0, 0x77, 0xc3, 0xbe, 0xd0, 0x70, 0xe6, 0x38, 0x83, 0x2e, 0xf3, 0x2, 0x8, 0x0, 0x45, 0x0, 0x0, 0xe1, 0x3b, 0x6c, 0x40, 0x0, 0x35, 0x11, 0x58, 0xc4, 0xa1, 0x61, 0xf8, 0xea, 0xa, 0x1, 0xc, 0x8f, 0x4, 0xaa, 0x8e, 0xfd, 0x0, 0xcd, 0x9c, 0x63, 0x30, 0x51, 0xfd, 0xd7, 0xa0, 0x88, 0x9d, 0x90, 0x8c, 0x8f, 0xd, 0x59, 0x8c, 0x6b, 0x9a, 0x69, 0x35, 0xf0, 0xe1, 0xc9, 0x66, 0x62, 0x5a, 0xb0, 0xb3, 0x39, 0xbd, 0x63, 0xfc, 0x2, 0xaf, 0x73, 0x54, 0xde, 0xa5, 0xa5, 0xd, 0x7, 0xd0, 0xa, 0xe5, 0xec, 0xf8, 0x72, 0x7c, 0x15, 0x3f, 0xec, 0x82, 0x25, 0x72, 0xd8, 0x61, 0xac, 0x8, 0xa4, 0x5c, 0xcb, 0xe, 0x32, 0x5, 0x52, 0x88, 0xe9, 0xad, 0x81, 0x4f, 0x87, 0xbf, 0xde, 0x2f, 0x72, 0x1, 0x8, 0x54, 0xf2, 0xdf, 0x10, 0xda, 0x42, 0x9a, 0x4a, 0x5d, 0x83, 0x89, 0xbe, 0xac, 0x9a, 0xfd, 0xd7, 0x1b, 0xdf, 0x24, 0xd8, 0x39, 0x32, 0x4a, 0xa3, 0xfa, 0x64, 0xb, 0x8b, 0xfd, 0x1a, 0x95, 0x2f, 0xc5, 0x7e, 0xe7, 0x8b, 0xef, 0x8, 0x33, 0x8e, 0x36, 0x39, 0xd4, 0x44, 0xa8, 0x73, 0x11, 0x48, 0x4a, 0x3a, 0xb0, 0x4e, 0x65, 0x87, 0x38, 0xd0, 0xdb, 0xef, 0xe2, 0x8, 0x63, 0x43, 0xb8, 0x81, 0xd6, 0x69, 0xe2, 0x54, 0x9e, 0xaa, 0x6f, 0x53, 0xb2, 0x7a, 0x75, 0x9c, 0x97, 0x9f, 0xfb, 0x94, 0x90, 0xb2, 0xb8, 0xbd, 0x82, 0x16, 0x30, 0xda, 0x51, 0x6e, 0xaf, 0xc7, 0xd9, 0xe2, 0x16, 0x71, 0xad, 0x55, 0x10, 0x9e, 0x66, 0x3a, 0xc8, 0xda, 0xf2, 0x74, 0x9f, 0x3b, 0xd, 0xe6, 0x9e, 0x6d, 0xec, 0xe4, 0x3e, 0xac, 0xcc, 0x9a, 0x57, 0x7d, 0xfd, 0x79, 0xbe];
        
        let packet = decode_packet(Interfaces::Ethernet, &hex_data);
        let main_activity = self.activity.as_any().downcast_ref::<MainActivity>().unwrap();
        let mut sidebar_fragment = SidebarFragment::new(self.activity.dyn_clone(), packet);
        main_activity.open_sidebar(sidebar_fragment.dyn_clone());






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
