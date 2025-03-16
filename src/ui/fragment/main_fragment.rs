use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Builder, CellRendererText, Container, ListStore, ScrolledWindow, TreeView, TreeViewColumn};
use gtk::glib::Propagation::Proceed;
use gtk::glib::{idle_add, idle_add_local, MainContext, Type};
use gtk::glib::ControlFlow::Continue;
use pcap::pcap::pcap::Pcap;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::adapters::packet_adapter::PacketAdapter;
use crate::ui::fragment::inter::fragment::Fragment;
use crate::ui::fragment::sidebar_fragment::SidebarFragment;
use crate::ui::handlers::bundle::Bundle;

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

    fn on_create(&mut self, bundle: Option<Bundle>) -> &Container {
        let builder = Builder::from_resource("/com/ethernaut/rust/res/ui/gtk3/main_fragment.ui");

        self.root = Some(builder
            .object("content_layout")
            .expect("Couldn't find 'content_layout' in window.ui"));

        let model = ListStore::new(&[Type::U32, Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::STRING]);

        match bundle {
            Some(bundle) => {
                match bundle.get::<String>("type").unwrap().as_str() {
                    "pcap" => {
                        self.packet_adapter = Some(PacketAdapter::from_packets(&model, bundle.get::<Pcap>("pcap").unwrap().get_packets()));
                    }
                    _ => {
                        self.packet_adapter = Some(PacketAdapter::new(&model));
                    }
                }
            }
            None => {
                self.packet_adapter = Some(PacketAdapter::new(&model));
            }
        }

        let tree_view: TreeView = builder
            .object("tree_view")
            .expect("Couldn't find 'tree_view' in window.ui");

        tree_view.set_model(Some(&model));

        self.add_column(&tree_view, "No.", 0, 100);
        self.add_column(&tree_view, "Time", 1, 150);
        self.add_column(&tree_view, "Source", 2, 180);
        self.add_column(&tree_view, "Destination", 3, 180);
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


        let vadj = Rc::new(list_scroll_layout.vadjustment());
        let needs_scroll = Rc::new(RefCell::new(false));
        let user_scrolled_up = Rc::new(RefCell::new(false));

        {
            let vadj = vadj.clone();
            let user_scrolled_up = user_scrolled_up.clone();
            vadj.connect_value_changed(move |adj| {
                let is_at_bottom = (adj.upper() - adj.value() - adj.page_size()).abs() < 100.0;
                *user_scrolled_up.borrow_mut() = !is_at_bottom;
            });
        }

        model.connect_row_inserted({
            let vadj = vadj.clone();
            let needs_scroll = needs_scroll.clone();
            let user_scrolled_up = user_scrolled_up.clone();

            move |_, _, _| {
                if !*user_scrolled_up.borrow() {
                    *needs_scroll.borrow_mut() = true;
                }

                let vadj = vadj.clone();
                let needs_scroll = needs_scroll.clone();
                let user_scrolled_up = user_scrolled_up.clone();

                if *needs_scroll.borrow() && !*user_scrolled_up.borrow() {
                    *needs_scroll.borrow_mut() = false;
                    vadj.set_value(vadj.upper() - vadj.page_size());
                }
            }
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
