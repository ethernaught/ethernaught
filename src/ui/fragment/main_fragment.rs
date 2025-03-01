use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{gdk, glib, Adjustment, Application, ApplicationWindow, Builder, Button, CellRendererText, Container, CssProvider, Image, Label, ListBox, ListBoxRow, ListStore, Paned, ScrolledWindow, Stack, StyleContext, TextTag, TextView, TreePath, TreeView, TreeViewColumn, Widget};
use gtk::glib::Propagation::Proceed;
use gtk::glib::Type;
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

    fn add_column(&self, tree: &TreeView, title: &str, col_id: i32, min_width: i32) {
        let renderer = CellRendererText::new();
        let column = TreeViewColumn::new();
        column.set_min_width(min_width);
        column.set_title(title);
        CellLayoutExt::pack_start(&column, &renderer, true);
        CellLayoutExt::add_attribute(&column, &renderer, "text", col_id);

        CellLayoutExt::set_cell_data_func(&column, &renderer, Some(Box::new(move |_, cell, model, iter| {
            let protocol: String = model.value(iter, 4).get().unwrap_or_default();

            let color = match protocol.as_str() {
                "ARP" => {
                    "#05211b"
                }
                "Broadcast" => {
                    "#000000"
                }
                "TCP" => {
                    "#1e0926"
                }
                "UDP" => {
                    "#070c1f"
                }
                "ICMP" => {
                    "#260d07"
                }
                "GRE" => {
                    "#122407"
                }
                _ => {
                    "#1e1f22"
                }
            };

            cell.set_property("cell-background", &color);
        })));

        tree.append_column(&column);
    }
}

impl Fragment for MainFragment {

    fn on_create(&mut self) -> &Container {
        let builder = Builder::from_file("res/ui/gtk3/main-fragment.ui");

        self.root = Some(builder
            .object("content_layout")
            .expect("Couldn't find 'content_layout' in window.ui"));

        let model = ListStore::new(&[Type::U32, Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::STRING]);
        self.packet_adapter = Some(PacketAdapter::new(&model));

        let tree_view: TreeView = builder
            .object("tree_view")
            .expect("Couldn't find 'tree_view' in window.ui");

        tree_view.set_model(Some(&model));

        self.add_column(&tree_view, "No.", 0, 100);
        self.add_column(&tree_view, "Time", 1, 150);
        self.add_column(&tree_view, "Source", 2, 150);
        self.add_column(&tree_view, "Destination", 3, 150);
        self.add_column(&tree_view, "Protocol", 4, 80);
        self.add_column(&tree_view, "Length", 5, 80);
        self.add_column(&tree_view, "Info", 6, 80);

        let _self = self.clone();

        tree_view.connect_button_press_event(move |tree_view, event| {
            if event.button() == 1 {
                let (x, y) = event.position();

                let path = tree_view.path_at_pos(x as i32, y as i32);

                if let Some((Some(path), _column, _x, _y)) = path {
                    let model = tree_view.model().unwrap();
                    let id: u32 = model.value(&model.iter(&path).unwrap(), 0).get::<u32>().unwrap()-1;

                    let main_activity = _self.activity.as_any().downcast_ref::<MainActivity>().unwrap();

                    let packet = _self.packet_adapter.as_ref().unwrap().get_packet_by_index(id as usize);

                    let mut sidebar_fragment = SidebarFragment::new(_self.activity.dyn_clone(), packet);
                    main_activity.open_sidebar(sidebar_fragment.dyn_clone());
                }
            }

            Proceed
        });


        let list_scroll_layout: ScrolledWindow = builder
            .object("list_scroll_layout")
            .expect("Couldn't find 'list_scroll_layout' in window.ui");

        let adj_ref = Rc::new(RefCell::new(list_scroll_layout.vadjustment()));
        let adj_ref_clone = adj_ref.clone();

        let is_scrolled_to_bottom = move || {
            let adj = adj_ref.borrow();
            (adj.upper() - adj.value() - adj.page_size()).abs() < 100.0
        };

        model.connect_row_inserted(move |_, _, _| {
            if is_scrolled_to_bottom() {
                let adj = adj_ref_clone.borrow();
                adj.set_value(adj.upper() - adj.page_size());
            }
        });


        //TEMPORARY

        /*
        let hex_data: Vec<u8> = vec![0xe6, 0x38, 0x83, 0x2e, 0xf3, 0x2, 0xf0, 0x77, 0xc3, 0xbe, 0xd0, 0x70, 0x8, 0x0, 0x45, 0x0, 0x0, 0x48, 0x10, 0x1c, 0x0, 0x0, 0x40, 0x11, 0x3d, 0xf8, 0xa, 0x1, 0xc, 0x8f, 0xa, 0x1, 0xc, 0x1, 0x81, 0xf9, 0x0, 0x35, 0x0, 0x34, 0x2c, 0xd7, 0x39, 0xe9, 0x1, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x73, 0x73, 0x6c, 0x7, 0x67, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x3, 0x63, 0x6f, 0x6d, 0x0, 0x0, 0x41, 0x0, 0x1, 0x0, 0x0, 0x29, 0x5, 0xc0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

        let packet = decode_packet(Interfaces::Ethernet, &hex_data);
        let main_activity = self.activity.as_any().downcast_ref::<MainActivity>().unwrap();
        let mut sidebar_fragment = SidebarFragment::new(self.activity.dyn_clone(), packet);
        main_activity.open_sidebar(sidebar_fragment.dyn_clone());
        */




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
