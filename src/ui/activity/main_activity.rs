use std::any::Any;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Duration;
use gtk::prelude::*;
use gtk::{gdk, glib, Builder, Button, Container, CssProvider, Label, Paned, StyleContext};
use gtk::glib::ControlFlow::Continue;
use pcap::devices::Device;
use crate::capture_service::CaptureService;
use crate::ui::application::OApplication;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::fragment::inter::fragment::Fragment;
use crate::ui::fragment::main_fragment::MainFragment;
use crate::ui::fragment::terminal_fragment::TerminalFragment;

#[derive(Clone)]
pub struct MainActivity {
    app: OApplication,
    footer_selected: Rc<RefCell<String>>,
    root: Option<Container>,
    capture_service: CaptureService
}

impl MainActivity {

    pub fn new(app: OApplication, device: &Device) -> Self {
        Self {
            app,
            root: None,
            footer_selected: Rc::new(RefCell::new(String::new())),
            capture_service: CaptureService::new(device)
        }
    }

    pub fn get_capture_service(&self) -> &CaptureService {
        &self.capture_service
    }

    pub fn open_footerbar(&self, title: &str, mut fragment: Box<dyn Fragment>) {
        if let Some(pane) = self.app.get_child_by_name(self.root.as_ref().unwrap().upcast_ref(), "window_pane").unwrap().downcast_ref::<Paned>() {
            match pane.child2() {
                Some(child) => {
                    pane.remove(&child);
                }
                None => {}
            }

            if self.footer_selected.borrow().as_str() != "" {
                self.app.get_child_by_name(self.root.as_ref().unwrap().upcast_ref(), self.footer_selected.borrow().as_str()).unwrap().style_context().remove_class("selected");
            }

            self.app.get_child_by_name(self.root.as_ref().unwrap().upcast_ref(), title).unwrap().style_context().add_class("selected");

            self.footer_selected.replace(title.to_string());
            let content = fragment.on_create();
            pane.add(content);
            pane.set_child_shrink(content, false);
        }
    }

    pub fn close_footerbar(&self) {
        if let Some(pane) = self.app.get_child_by_name(self.root.as_ref().unwrap().upcast_ref(), "window_pane").unwrap().downcast_ref::<Paned>() {
            match pane.child2() {
                Some(child) => {
                    if self.footer_selected.borrow().as_str() != "" {
                        self.app.get_child_by_name(self.root.as_ref().unwrap().upcast_ref(), self.footer_selected.borrow().as_str()).unwrap().style_context().remove_class("selected");
                    }

                    self.footer_selected.replace(String::new());
                    pane.remove(&child);
                }
                None => {}
            }
        }
    }

    pub fn open_sidebar(&self, mut fragment: Box<dyn Fragment>) {
        if let Some(pane) = self.app.get_child_by_name(self.root.as_ref().unwrap().upcast_ref(), "window_content_pane").unwrap().downcast_ref::<Paned>() {
            match pane.child2() {
                Some(child) => {
                    pane.remove(&child);
                }
                None => {}
            }

            let content = fragment.on_create();
            pane.add(content);
            pane.set_child_shrink(content, false);
        }
    }

    pub fn close_sidebar(&self) {
        if let Some(pane) = self.app.get_child_by_name(self.root.as_ref().unwrap().upcast_ref(), "window_content_pane").unwrap().downcast_ref::<Paned>() {
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

    fn on_create(&mut self) -> &Container {
        let builder = Builder::from_file("res/ui/gtk3/main_activity.ui");

        let provider = CssProvider::new();
        provider.load_from_path("res/ui/gtk3/main_activity.css").expect("Failed to load CSS file.");

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

        let mut main_fragment = MainFragment::new(self.dyn_clone());
        let content = main_fragment.on_create();
        window_content_pane.add(content);
        window_content_pane.set_child_shrink(content, false);
        window_content_pane.set_child_resize(content, true);








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






        let main_fragment = Rc::new(RefCell::new(main_fragment));


        let (tx, rx) = channel();
        self.capture_service.set_tx(tx);


        let titlebar = self.app.get_titlebar().unwrap();
        //let menu_buttons =self.app.get_child_by_name(&titlebar, "navigation_buttons").unwrap();
        //menu_buttons.show();


        self.app.get_child_by_name(&titlebar, "network_type_label").unwrap().downcast_ref::<Label>().unwrap().set_label(&self.capture_service.get_device().get_name());



        let app_options = Rc::new(RefCell::new(self.app.get_child_by_name(&titlebar, "app_options").unwrap()));
        app_options.borrow().show();
        let stop_button = Rc::new(RefCell::new(self.app.get_child_by_name(&app_options.borrow(), "stop_button").unwrap()));
        let start_button = self.app.get_child_by_name(&app_options.borrow(), "start_button").unwrap();

        if let Some(start_button) = start_button.downcast_ref::<Button>() {
            let app_options = Rc::clone(&app_options);
            let stop_button = Rc::clone(&stop_button);
            let main_fragment = Rc::clone(&main_fragment);;

            let packet_service = self.capture_service.clone();
            start_button.connect_clicked(move |_| {
                app_options.borrow().style_context().add_class("running");
                stop_button.borrow().show();

                println!("Start button clicked!");
                main_fragment.borrow().get_packet_adapter().unwrap().clear();
                packet_service.start();
            });
        }

        if let Some(button) = stop_button.borrow().downcast_ref::<Button>() {
            let app_options = Rc::clone(&app_options);
            let stop_button = Rc::clone(&stop_button);

            let packet_service = self.capture_service.clone();
            button.connect_clicked(move |_| {
                app_options.borrow().style_context().remove_class("running");
                stop_button.borrow().hide();

                println!("Stop button clicked!");
                packet_service.stop();
            });
        }









        let _self = self.clone();
        let main_fragment = Rc::clone(&main_fragment);
        glib::timeout_add_local(Duration::from_millis(10), move || {
            match rx.try_recv() {
                Ok(packet) => {
                    main_fragment.borrow().get_packet_adapter().unwrap().add(packet);
                }
                _ => {
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

    fn get_application(&self) -> &OApplication {
        &self.app
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


