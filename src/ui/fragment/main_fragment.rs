use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::time::Duration;
use gtk::prelude::*;
use gtk::{glib, Builder, Button, CellRendererText, Container, Label, ListStore, ScrolledWindow, TreeView, TreeViewColumn};
use gtk::glib::ControlFlow::Continue;
use gtk::glib::Propagation::Proceed;
use gtk::glib::Type;
use pcap::devices::Device;
use crate::capture_service::CaptureService;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::adapters::packet_adapter::PacketAdapter;
use crate::ui::fragment::inter::fragment::Fragment;
use crate::ui::fragment::sidebar_fragment::SidebarFragment;

#[derive(Clone)]
pub struct MainFragment {
    activity: Box<dyn Activity>,
    root: Option<Container>,
    packet_adapter: Option<PacketAdapter>,
    capture_service: Option<CaptureService>
}

impl MainFragment {

    pub fn new(activity: Box<dyn Activity>) -> Self {
        Self {
            activity,
            root: None,
            packet_adapter: None,
            capture_service: None
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

    fn on_create(&mut self, bundle: Option<&dyn Any>) -> &Container {
        let builder = Builder::from_resource("/com/ethernaut/rust/res/ui/gtk3/main_fragment.ui");

        self.root = Some(builder
            .object("content_layout")
            .expect("Couldn't find 'content_layout' in window.ui"));



        let device = bundle.unwrap().downcast_ref::<Device>().unwrap().clone();

        let (tx, rx) = channel();
        let capture_service = CaptureService::new(&device, tx.clone());

        self.capture_service = Some(capture_service);




        let model = ListStore::new(&[Type::U32, Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::STRING]);
        self.packet_adapter = Some(PacketAdapter::new(&model));

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

                    let mut sidebar_fragment = SidebarFragment::new(_self.activity.dyn_clone(), tx.clone(), packet);
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




        let app = self.activity.get_application();



        let titlebar = app.get_titlebar().unwrap();

        app.get_child_by_name(&titlebar, "network_type_label").unwrap().downcast_ref::<Label>().unwrap().set_label(&device.get_name());



        let app_options = Rc::new(RefCell::new(app.get_child_by_name(&titlebar, "app_options").unwrap()));
        app_options.borrow().show();
        let stop_button = Rc::new(RefCell::new(app.get_child_by_name(&app_options.borrow(), "stop_button").unwrap()));
        let start_button = app.get_child_by_name(&app_options.borrow(), "start_button").unwrap();

        if let Some(start_button) = start_button.downcast_ref::<Button>() {
            let app_options = Rc::clone(&app_options);
            let stop_button = Rc::clone(&stop_button);
            //let main_fragment = Rc::clone(&main_fragment);
            let mut packet_adapter = self.packet_adapter.clone().unwrap();
            let capture_service = self.capture_service.clone().unwrap();

            start_button.connect_clicked(move |_| {
                app_options.borrow().style_context().add_class("running");
                stop_button.borrow().show();

                packet_adapter.clear();
                capture_service.start();
            });
        }

        if let Some(button) = stop_button.borrow().downcast_ref::<Button>() {
            let app_options = Rc::clone(&app_options);
            let stop_button = Rc::clone(&stop_button);
            let capture_service = self.capture_service.clone().unwrap();

            button.connect_clicked(move |_| {
                app_options.borrow().style_context().remove_class("running");
                stop_button.borrow().hide();
                capture_service.stop();
            });
        }

        let mut packet_adapter = self.packet_adapter.clone().unwrap();

        glib::timeout_add_local(Duration::from_millis(10), move || {
            loop {
                match rx.try_recv() {
                    Ok(packet) => {
                        packet_adapter.add(packet);
                    }
                    _ => {
                        break;
                    }
                }
            }
            Continue
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
