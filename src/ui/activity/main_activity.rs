use std::any::Any;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{gdk, Builder, Button, Container, CssProvider, Image, Label, Paned, StyleContext, Widget};
use pcap::devices::Device;
use pcap::packet::packet::Packet;
use pcap::pcap::pcap::Pcap;
use pcap::utils::data_link_types::DataLinkTypes;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::context::Context;
use crate::ui::fragment::inter::fragment::Fragment;
use crate::ui::fragment::main_fragment::MainFragment;
use crate::ui::fragment::sidebar_fragment::SidebarFragment;
use crate::ui::fragment::terminal_fragment::TerminalFragment;
use crate::ui::handlers::bundle::Bundle;
use crate::ui::handlers::events::capture_event::CaptureEvent;

#[derive(Clone)]
pub struct MainActivity {
    context: Context,
    footer_selected: Rc<RefCell<String>>,
    data_link_type: DataLinkTypes,
    _type: Option<String>,
    //capture_service: Option<CaptureService>,
    //running: Arc<AtomicBool>,
    root: Option<Container>
}

impl MainActivity {

    pub fn new(context: Context) -> Self {
        Self {
            context,
            footer_selected: Rc::new(RefCell::new(String::new())),
            data_link_type: DataLinkTypes::Null,
            _type: None,
            //capture_service: None,
            //running: Arc::new(AtomicBool::new(false)),
            root: None
        }
    }

    /*
    pub fn get_capture_service(&self) -> Option<&CaptureService> {
        self.capture_service.as_ref()
    }
    */

    pub fn open_footerbar(&self, title: &str, mut fragment: Box<dyn Fragment>) {
        if let Some(pane) = self.context.get_child_by_name::<Paned>(self.root.as_ref().unwrap().upcast_ref(), "main_activity_pane") {
            match pane.child2() {
                Some(child) => {
                    pane.remove(&child);
                }
                None => {}
            }

            if self.footer_selected.borrow().as_str() != "" {
                self.context.get_child_by_name::<Widget>(self.root.as_ref().unwrap().upcast_ref(), self.footer_selected.borrow().as_str()).unwrap().style_context().remove_class("selected");
            }

            self.context.get_child_by_name::<Widget>(self.root.as_ref().unwrap().upcast_ref(), title).unwrap().style_context().add_class("selected");

            self.footer_selected.replace(title.to_string());
            let content = fragment.on_create(None);
            pane.add(content);
            pane.set_child_shrink(content, false);
        }
    }

    pub fn close_footerbar(&self) {
        if let Some(pane) = self.context.get_child_by_name::<Paned>(self.root.as_ref().unwrap().upcast_ref(), "main_activity_pane") {
            match pane.child2() {
                Some(child) => {
                    if self.footer_selected.borrow().as_str() != "" {
                        self.context.get_child_by_name::<Widget>(self.root.as_ref().unwrap().upcast_ref(), self.footer_selected.borrow().as_str()).unwrap().style_context().remove_class("selected");
                    }

                    self.footer_selected.replace(String::new());
                    pane.remove(&child);
                }
                None => {}
            }
        }
    }

    pub fn open_sidebar(&self, mut fragment: Box<dyn Fragment>) {
        if let Some(pane) = self.context.get_child_by_name::<Paned>(self.root.as_ref().unwrap().upcast_ref(), "main_activity_content_pane") {
            match pane.child2() {
                Some(child) => {
                    pane.remove(&child);
                }
                None => {}
            }

            let content = fragment.on_create(None);
            pane.add(content);
            pane.set_child_shrink(content, false);
        }
    }

    pub fn close_sidebar(&self) {
        if let Some(pane) = self.context.get_child_by_name::<Paned>(self.root.as_ref().unwrap().upcast_ref(), "main_activity_content_pane") {
            match pane.child2() {
                Some(child) => {
                    pane.remove(&child);
                }
                None => {}
            }
        }
    }
}

impl Activity for MainActivity {

    fn get_name(&self) -> String {
        "main_activity".to_string()
    }

    fn get_title(&self) -> String {
        "MainActivity".to_string()
    }

    fn on_create(&mut self, bundle: Option<Bundle>) -> &Container {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/main_activity.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/main_activity.css");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        self.root = Some(builder
            .object("main_activity_layout")
            .expect("Couldn't find 'main_activity_layout' in main_activity.ui"));


        let mut main_activity_content_pane: Paned = builder
            .object("main_activity_content_pane")
            .expect("Couldn't find 'main_activity_content_pane' in main_activity.ui");


        let mut main_activity_pane: Paned = builder
            .object("main_activity_pane")
            .expect("Couldn't find 'main_activity_pane' in main_activity.ui");

        let content = main_activity_content_pane.upcast_ref::<Container>();
        main_activity_pane.set_child_shrink(content, false);
        main_activity_pane.set_child_resize(content, true);


        let terminal_button: Button = builder
            .object("terminal_button")
            .expect("Couldn't find 'terminal_button' in window.ui");

        let _self = self.clone();
        terminal_button.connect_clicked(move |_| {
            if _self.footer_selected.borrow().eq("terminal_button") {
                _self.close_footerbar();
                return;
            }
            _self.open_footerbar("terminal_button", TerminalFragment::new(_self.dyn_clone()).dyn_clone());
        });

        match bundle {
            Some(bundle) => {
                self._type = Some(bundle.get::<String>("type").unwrap().to_string());

                match bundle.get::<String>("type").unwrap().as_str() {
                    "device" => {
                        let titlebar = self.context.get_titlebar().unwrap();
                        let network_type_label = self.context.get_child_by_name::<Label>(&titlebar, "network_type_label").unwrap();

                        let if_index = if let Some(device) = bundle.get::<Device>("device") {
                            self.data_link_type = device.get_data_link_type();
                            //self.capture_service = Some(CaptureService::from_device(&self.context, &device));
                            network_type_label.set_label(&device.get_name());

                            device.get_index()

                        } else {
                            self.data_link_type = DataLinkTypes::Null;
                            //self.capture_service = Some(CaptureService::any(&self.context));
                            network_type_label.set_label("Any");

                            -1
                        };

                        //let (tx, rx) = channel();
                        //self.capture_service.as_mut().unwrap().set_tx(tx);

                        network_type_label.show();

                        let icon = self.context.get_child_by_name::<Image>(&titlebar, "network_type_icon").unwrap();

                        match self.data_link_type {
                            DataLinkTypes::Null => {
                                titlebar.style_context().add_class("any");
                                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_any.svg"));
                            }
                            DataLinkTypes::En10mb | DataLinkTypes::En3mb | DataLinkTypes::Sll2 => {
                                titlebar.style_context().add_class("ethernet");
                                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_ethernet.svg"));
                            }
                            DataLinkTypes::Loop => {
                                titlebar.style_context().add_class("lan");
                                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_lan.svg"));
                            }
                            DataLinkTypes::Raw | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                                titlebar.style_context().add_class("vpn");
                                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_vpn.svg"));
                            }
                            /*
                            DataLinkTypes::BluetoothHciH4 => {
                                titlebar.style_context().add_class("bluetooth");
                                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_bluetooth.svg"));
                            }
                            */
                            _ => {}
                        }

                        icon.show();

                        let mut main_fragment = MainFragment::new(self.dyn_clone());
                        let content = main_fragment.on_create(None);
                        main_activity_content_pane.add(content);
                        main_activity_content_pane.set_child_shrink(content, false);
                        main_activity_content_pane.set_child_resize(content, true);

                        let main_fragment = Rc::new(RefCell::new(main_fragment));

                        let app_options = Rc::new(RefCell::new(self.context.get_child_by_name::<Widget>(&titlebar, "app_options").unwrap()));
                        app_options.borrow().show();

                        let stop_button = self.context.get_child_by_name::<Button>(&app_options.borrow(), "stop_button").unwrap();
                        let start_button = self.context.get_child_by_name::<Button>(&app_options.borrow(), "start_button").unwrap();


                        {
                            let app_options = Rc::clone(&app_options);
                            let handler = self.context.get_event_handler().clone();

                            stop_button.connect_clicked(move |button| {
                                app_options.borrow().style_context().remove_class("running");
                                button.hide();

                                handler.remove_listener("capture_event");
                            });
                        }

                        {
                            let app_options = Rc::clone(&app_options);
                            let main_fragment = Rc::clone(&main_fragment);
                            let handler = self.context.get_event_handler().clone();

                            start_button.connect_clicked(move |_| {
                                app_options.borrow().style_context().add_class("running");
                                stop_button.show();

                                main_fragment.borrow().get_packet_adapter().unwrap().clear();

                                let main_fragment = Rc::clone(&main_fragment);
                                handler.register_listener("capture_event", move |event| {
                                    let event = event.as_any().downcast_ref::<CaptureEvent>().unwrap();

                                    if if_index == -1 || event.get_if_index() == if_index {
                                        main_fragment.borrow().get_packet_adapter().unwrap().add(event.get_packet().clone());
                                    }
                                });
                            });
                        }
                    }
                    "file" => {
                        let pcap = Pcap::from_file(bundle.get::<PathBuf>("file").unwrap().to_str().unwrap()).expect("Couldn't parse pcap");

                        self.data_link_type = pcap.get_data_link_type();

                        let titlebar = self.context.get_titlebar().unwrap();

                        let icon = self.context.get_child_by_name::<Image>(&titlebar, "network_type_icon").unwrap();

                        match self.data_link_type {
                            DataLinkTypes::En10mb | DataLinkTypes::En3mb | DataLinkTypes::Sll2 => {
                                titlebar.style_context().add_class("ethernet");
                                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_ethernet.svg"));
                            }
                            DataLinkTypes::Loop => {
                                titlebar.style_context().add_class("lan");
                                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_lan.svg"));
                            }
                            DataLinkTypes::Raw | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                                titlebar.style_context().add_class("vpn");
                                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_vpn.svg"));
                            }
                            /*
                            DataLinkTypes::BluetoothHciH4 => {
                                titlebar.style_context().add_class("bluetooth");
                                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_bluetooth.svg"));
                            }
                            */
                            _ => {}
                        }

                        icon.show();

                        let network_type_label = self.context.get_child_by_name::<Label>(&titlebar, "network_type_label").unwrap();
                        network_type_label.set_label(bundle.get::<PathBuf>("file").unwrap().file_name().unwrap().to_str().unwrap());
                        network_type_label.show();


                        let mut bundle = Bundle::new();
                        bundle.put("type", String::from("pcap"));
                        bundle.put("pcap", pcap);

                        let mut main_fragment = MainFragment::new(self.dyn_clone());
                        let content = main_fragment.on_create(Some(bundle));
                        main_activity_content_pane.add(content);
                        main_activity_content_pane.set_child_shrink(content, false);
                        main_activity_content_pane.set_child_resize(content, true);
                    }
                    _ => {}
                }
            }
            None => {}
        }




        //TEMPORARY
        let hex_data: Vec<u8> = vec![0xe6, 0x38, 0x83, 0x2e, 0xf3, 0x2, 0xf0, 0x77, 0xc3, 0xbe, 0xd0, 0x70, 0x8, 0x0, 0x45, 0x0, 0x0, 0x48, 0x10, 0x1c, 0x0, 0x0, 0x40, 0x11, 0x3d, 0xf8, 0xa, 0x1, 0xc, 0x8f, 0xa, 0x1, 0xc, 0x1, 0x81, 0xf9, 0x0, 0x35, 0x0, 0x34, 0x2c, 0xd7, 0x39, 0xe9, 0x1, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x73, 0x73, 0x6c, 0x7, 0x67, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x3, 0x63, 0x6f, 0x6d, 0x0, 0x0, 0x41, 0x0, 0x1, 0x0, 0x0, 0x29, 0x5, 0xc0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

        let packet = Packet::new(DataLinkTypes::En10mb, 0, &hex_data);
        let main_activity = self.as_any().downcast_ref::<MainActivity>().unwrap();
        let mut sidebar_fragment = SidebarFragment::new(self.dyn_clone(), packet);
        main_activity.open_sidebar(sidebar_fragment.dyn_clone());




        &self.root.as_ref().unwrap().upcast_ref()
    }

    fn on_resume(&self) {
        let titlebar = self.context.get_titlebar().unwrap();

        match self.data_link_type {
            DataLinkTypes::Null => {
                titlebar.style_context().add_class("any");
            }
            DataLinkTypes::En10mb | DataLinkTypes::En3mb | DataLinkTypes::Sll2 => {
                titlebar.style_context().add_class("ethernet");
            }
            DataLinkTypes::Loop => {
                titlebar.style_context().add_class("lan");
            }
            DataLinkTypes::Raw | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                titlebar.style_context().add_class("vpn");
            }
            /*
            DataLinkTypes::BluetoothHciH4 => {
                titlebar.style_context().add_class("bluetooth");
            }
            */
            _ => {}
        }

        self.context.get_child_by_name::<Image>(&titlebar, "network_type_icon").unwrap().show();
        self.context.get_child_by_name::<Label>(&titlebar, "network_type_label").unwrap().show();

        if let Some(_type) = self._type.as_ref() {
            if _type == "device" {
                self.context.get_child_by_name::<Widget>(&titlebar, "app_options").unwrap().show();
            }
        }
    }

    fn on_pause(&self) {
        let titlebar = self.context.get_titlebar().unwrap();

        match self.data_link_type {
            DataLinkTypes::Null => {
                titlebar.style_context().remove_class("any");
            }
            DataLinkTypes::En10mb | DataLinkTypes::En3mb | DataLinkTypes::Sll2 => {
                titlebar.style_context().remove_class("ethernet");
            }
            DataLinkTypes::Loop => {
                titlebar.style_context().remove_class("lan");
            }
            DataLinkTypes::Raw | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                titlebar.style_context().remove_class("vpn");
            }
            /*
            DataLinkTypes::BluetoothHciH4 => {
                titlebar.style_context().remove_class("bluetooth");
            }
            */
            _ => {}
        }

        self.context.get_child_by_name::<Image>(&titlebar, "network_type_icon").unwrap().hide();
        self.context.get_child_by_name::<Label>(&titlebar, "network_type_label").unwrap().hide();

        if let Some(_type) = self._type.as_ref() {
            if _type == "device" {
                self.context.get_event_handler().remove_listener("capture_event");

                let app_options = self.context.get_child_by_name::<Widget>(&titlebar, "app_options").unwrap();
                app_options.style_context().remove_class("running");
                app_options.hide();
                self.context.get_child_by_name::<Widget>(&app_options, "stop_button").unwrap().hide();
            }
        }
    }

    fn on_destroy(&self) {
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Activity> {
        Box::new(self.clone())
    }
}


