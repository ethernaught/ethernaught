use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::time::Duration;
use gtk::prelude::*;
use gtk::{gdk, glib, Adjustment, Application, ApplicationWindow, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Paned, ScrolledWindow, Stack, StyleContext, TextTag, TextView, Widget};
use gtk::glib::ControlFlow::Continue;
use crate::main;
use crate::pcaps::packet_capture;
use crate::ui::application::OApplication;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::fragment::inter::fragment::Fragment;
use crate::ui::fragment::main_fragment::MainFragment;
use crate::ui::fragment::sidebar_fragment::SidebarFragment;

#[derive(Clone)]
pub struct MainActivity {
    app: OApplication,
    root: Option<Container>
}

impl MainActivity {

    pub fn new(app: OApplication) -> Self {
        Self {
            app,
            root: None
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
        let builder = Builder::from_file("res/ui/gtk3/main-activity.ui");

        let provider = CssProvider::new();
        provider.load_from_path("res/ui/gtk3/main-activity.css").expect("Failed to load CSS file.");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let mut root: Paned = builder
            .object("window_layout")
            .expect("Couldn't find 'window_layout' in main-activity.ui");



        let mut main_fragment = MainFragment::new();
        let content = main_fragment.on_create();
        root.add(content);
        root.set_child_shrink(content, false);
        root.set_child_resize(content, true);




        let mut sidebar_fragment = SidebarFragment::new();
        let sidebar = sidebar_fragment.on_create();
        root.add(sidebar);
        root.set_child_shrink(sidebar, false);






        let (tx, rx) = channel();
        let tx = Arc::new(Mutex::new(tx));





        let titlebar = self.app.get_titlebar().unwrap();

        let app_buttons = Arc::new(self.app.get_child_by_name(&titlebar, "app_buttons").unwrap());
        let stop_button = Arc::new(self.app.get_child_by_name(&app_buttons, "stop_button").unwrap());
        let start_button = self.app.get_child_by_name(&app_buttons, "start_button").unwrap();

        if let Some(start_button) = start_button.downcast_ref::<Button>() {
            let app_buttons = app_buttons.clone();
            let stop_button_clone = stop_button.clone();

            start_button.connect_clicked(move |_| {
                app_buttons.style_context().add_class("running");
                stop_button_clone.show();

                println!("Start button clicked!");
                packet_capture(tx.clone());
            });
        }

        if let Some(stop_button) = stop_button.downcast_ref::<Button>() {
            let app_buttons = app_buttons.clone();
            let stop_button_clone = stop_button.clone();

            stop_button.connect_clicked(move |_| {
                app_buttons.style_context().remove_class("running");
                stop_button_clone.hide();
                println!("Stop button clicked!");
            });
        }




        let mut i = 0;
        glib::timeout_add_local(Duration::from_millis(10), move || {
            let main_fragment = main_fragment.clone();
            match rx.try_recv() {
                Ok(packet) => {
                    i += 1;

                    main_fragment.get_packet_adapter().unwrap().add(i, &packet);
                    //let row = main_fragment.create_row(i, packet);
                    //list_box.add(&row);
                    //row.show_all();
                }
                _ => {
                }
            }
            Continue
        });

        self.root = Some(root.upcast());
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

    fn dyn_clone(&self) -> Box<dyn Activity> {
        Box::new(self.clone())
    }
}


