use std::any::Any;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::time::Duration;
use gtk::prelude::*;
use gtk::{gdk, glib, Builder, Button, Container, CssProvider, Image, Label, Paned, StyleContext, Widget};
use gtk::glib::ControlFlow::{Break, Continue};
use pcap::devices::Device;
use pcap::packet::inter::data_link_types::DataLinkTypes;
use pcap::pcap::pcap::Pcap;
use crate::capture_service::CaptureService;
use crate::ui::application::OApplication;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::context::Context;
use crate::ui::fragment::inter::fragment::Fragment;
use crate::ui::fragment::main_fragment::MainFragment;
use crate::ui::fragment::terminal_fragment::TerminalFragment;
use crate::ui::handlers::bundle::Bundle;

#[derive(Clone)]
pub struct MainActivity {
    context: Context,
    footer_selected: Rc<RefCell<String>>,
    //capture_service: Option<CaptureService>,
    data_link_type: DataLinkTypes,
    running: Arc<AtomicBool>,
    root: Option<Container>
}

impl MainActivity {

    pub fn new(context: Context) -> Self {
        Self {
            context,
            footer_selected: Rc::new(RefCell::new(String::new())),
            //capture_service: None,
            running: Arc::new(AtomicBool::new(false)),
            data_link_type: DataLinkTypes::Null,
            root: None
        }
    }

    /*
    pub fn get_capture_service(&self) -> Option<&CaptureService> {
        self.capture_service.as_ref()
    }
    */

    pub fn open_footerbar(&self, title: &str, mut fragment: Box<dyn Fragment>) {
        if let Some(pane) = self.context.get_child_by_name::<Paned>(self.root.as_ref().unwrap().upcast_ref(), "window_pane") {
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
        if let Some(pane) = self.context.get_child_by_name::<Paned>(self.root.as_ref().unwrap().upcast_ref(), "window_pane") {
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
        if let Some(pane) = self.context.get_child_by_name::<Paned>(self.root.as_ref().unwrap().upcast_ref(), "window_content_pane") {
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
        if let Some(pane) = self.context.get_child_by_name::<Paned>(self.root.as_ref().unwrap().upcast_ref(), "window_content_pane") {
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
        let builder = Builder::from_resource("/com/ethernaut/rust/res/ui/gtk3/main_activity.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/com/ethernaut/rust/res/ui/gtk3/main_activity.css");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        self.root = Some(builder
            .object("window_layout")
            .expect("Couldn't find 'window_layout' in main_activity.ui"));


        let mut window_content_pane: Paned = builder
            .object("window_content_pane")
            .expect("Couldn't find 'window_content_pane' in main_activity.ui");


        let mut window_pane: Paned = builder
            .object("window_pane")
            .expect("Couldn't find 'window_pane' in main_activity.ui");

        let content = window_content_pane.upcast_ref::<Container>();
        window_pane.set_child_shrink(content, false);
        window_pane.set_child_resize(content, true);


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
                match bundle.get::<String>("type").unwrap().as_str() {
                    "device" => {
                        let titlebar = self.context.get_titlebar().unwrap();
                        let network_type_label = self.context.get_child_by_name::<Label>(&titlebar, "network_type_label").unwrap();

                        /*
                        if let Some(device) = bundle.get::<Device>("device") {
                            self.data_link_type = device.get_data_link_type();

                            self.capture_service = Some(CaptureService::from_device(&device));
                            network_type_label.set_label(&device.get_name());

                        } else {
                            self.data_link_type = DataLinkTypes::Null;

                            self.capture_service = Some(CaptureService::any());
                            network_type_label.set_label("Any");
                        }
                        */

                        //let (tx, rx) = channel();
                        //self.capture_service.as_mut().unwrap().set_tx(tx);

                        network_type_label.show();

                        let icon = self.context.get_child_by_name::<Image>(&titlebar, "network_type_icon").unwrap();

                        match self.data_link_type {
                            DataLinkTypes::Null => {
                                titlebar.style_context().add_class("any");
                                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_any.svg"));
                            }
                            DataLinkTypes::Ethernet => {
                                titlebar.style_context().add_class("ethernet");
                                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_ethernet.svg"));
                            }
                            DataLinkTypes::Loopback => {
                                titlebar.style_context().add_class("lan");
                                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_lan.svg"));
                            }
                            DataLinkTypes::Raw | DataLinkTypes::Tun | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                                titlebar.style_context().add_class("vpn");
                                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_vpn.svg"));
                            }
                            DataLinkTypes::BluetoothHciH4 => {
                                titlebar.style_context().add_class("bluetooth");
                                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_bluetooth.svg"));
                            }
                            _ => {}
                        }

                        icon.show();


                        let mut main_fragment = MainFragment::new(self.dyn_clone());
                        let content = main_fragment.on_create(None);
                        window_content_pane.add(content);
                        window_content_pane.set_child_shrink(content, false);
                        window_content_pane.set_child_resize(content, true);

                        let main_fragment = Rc::new(RefCell::new(main_fragment));

                        let app_options = Rc::new(RefCell::new(self.context.get_child_by_name::<Widget>(&titlebar, "app_options").unwrap()));
                        app_options.borrow().show();

                        let stop_button = Rc::new(RefCell::new(self.context.get_child_by_name::<Widget>(&app_options.borrow(), "stop_button").unwrap()));
                        let start_button = self.context.get_child_by_name::<Widget>(&app_options.borrow(), "start_button").unwrap();

                        /*
                        if let Some(start_button) = start_button.downcast_ref::<Button>() {
                            let app_options = Rc::clone(&app_options);
                            let stop_button = Rc::clone(&stop_button);
                            let main_fragment = Rc::clone(&main_fragment);
                            let capture_service = self.capture_service.clone().unwrap();

                            start_button.connect_clicked(move |_| {
                                app_options.borrow().style_context().add_class("running");
                                stop_button.borrow().show();

                                main_fragment.borrow().get_packet_adapter().unwrap().clear();
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
                        */

                        //let main_fragment = Rc::clone(&main_fragment);
                        //self.running.store(true, Ordering::Relaxed);
                        //let running = Arc::clone(&self.running);





                        /*
                        glib::timeout_add_local(Duration::from_millis(10), move || {
                            while running.load(Ordering::Relaxed) {
                                match rx.try_recv() {
                                    Ok(packet) => {
                                        main_fragment.borrow().get_packet_adapter().unwrap().add(packet);
                                    }
                                    _ => {
                                        break;
                                    }
                                }
                            }

                            if !running.load(Ordering::Relaxed) {
                                return Break;
                            }

                            Continue
                        });
                        */






                    }
                    "file" => {
                        let pcap = Pcap::from_file(bundle.get::<PathBuf>("file").unwrap().to_str().unwrap()).expect("Couldn't parse pcap");

                        self.data_link_type = pcap.get_data_link_type();

                        let titlebar = self.context.get_titlebar().unwrap();

                        let icon = self.context.get_child_by_name::<Image>(&titlebar, "network_type_icon").unwrap();

                        match self.data_link_type {
                            DataLinkTypes::Ethernet => {
                                titlebar.style_context().add_class("ethernet");
                                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_ethernet.svg"));
                            }
                            DataLinkTypes::Loopback => {
                                titlebar.style_context().add_class("lan");
                                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_lan.svg"));
                            }
                            DataLinkTypes::Raw | DataLinkTypes::Tun | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                                titlebar.style_context().add_class("vpn");
                                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_vpn.svg"));
                            }
                            DataLinkTypes::BluetoothHciH4 => {
                                titlebar.style_context().add_class("bluetooth");
                                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_bluetooth.svg"));
                            }
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
                        window_content_pane.add(content);
                        window_content_pane.set_child_shrink(content, false);
                        window_content_pane.set_child_resize(content, true);
                    }
                    _ => {}
                }
            }
            None => {}
        }

        &self.root.as_ref().unwrap().upcast_ref()
    }

    fn on_resume(&self) {
        let titlebar = self.context.get_titlebar().unwrap();

        match self.data_link_type {
            DataLinkTypes::Null => {
                titlebar.style_context().add_class("any");
            }
            DataLinkTypes::Ethernet => {
                titlebar.style_context().add_class("ethernet");
            }
            DataLinkTypes::Loopback => {
                titlebar.style_context().add_class("lan");
            }
            DataLinkTypes::Raw | DataLinkTypes::Tun | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                titlebar.style_context().add_class("vpn");
            }
            DataLinkTypes::BluetoothHciH4 => {
                titlebar.style_context().add_class("bluetooth");
            }
            _ => {}
        }

        self.context.get_child_by_name::<Image>(&titlebar, "network_type_icon").unwrap().show();
        self.context.get_child_by_name::<Label>(&titlebar, "network_type_label").unwrap().show();

        /*
        //ONLY IF DEVICE TYPE...
        if let Some(_) = self.capture_service.as_ref() {
            self.context.get_child_by_name::<Widget>(&titlebar, "app_options").unwrap().show();
        }
        */
    }

    fn on_pause(&self) {
        let titlebar = self.context.get_titlebar().unwrap();

        match self.data_link_type {
            DataLinkTypes::Null => {
                titlebar.style_context().remove_class("any");
            }
            DataLinkTypes::Ethernet => {
                titlebar.style_context().remove_class("ethernet");
            }
            DataLinkTypes::Loopback => {
                titlebar.style_context().remove_class("lan");
            }
            DataLinkTypes::Raw | DataLinkTypes::Tun | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                titlebar.style_context().remove_class("vpn");
            }
            DataLinkTypes::BluetoothHciH4 => {
                titlebar.style_context().remove_class("bluetooth");
            }
            _ => {}
        }

        self.context.get_child_by_name::<Image>(&titlebar, "network_type_icon").unwrap().hide();
        self.context.get_child_by_name::<Label>(&titlebar, "network_type_label").unwrap().hide();

        /*
        if let Some(capture_service) = self.capture_service.as_ref() {
            capture_service.stop();

            let app_options = self.context.get_child_by_name::<Widget>(&titlebar, "app_options").unwrap();
            app_options.style_context().remove_class("running");
            self.context.get_child_by_name::<Widget>(&app_options, "stop_button").unwrap().hide();
        }
        */
    }

    fn on_destroy(&self) {
        self.running.store(false, Ordering::Relaxed);
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


