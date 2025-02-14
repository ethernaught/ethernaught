use std::process::exit;
use gtk::{AboutDialog, ApplicationWindow, Builder, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label, CssProvider, StyleContext, gdk, Stack, Container, TreeView, Widget, Window};
use gtk::gdk_pixbuf::PixbufLoader;
use gtk::prelude::*;
use gtk::gio::SimpleAction;
use gtk::glib::PropertyGet;
use gtk::prelude::{ActionMapExt, GtkWindowExt};
use crate::packet::packet::Packet;
use crate::packet::inter::interfaces::Interfaces;
use crate::packet::layers::layer_1::ethernet_layer::EthernetLayer;
use crate::packet::layers::layer_1::inter::types::Types;
use crate::packet::layers::layer_2::ethernet::inter::protocols::Protocols;
use crate::packet::layers::layer_2::ethernet::ipv4_layer::IPv4Layer;
use crate::packet::layers::layer_2::ethernet::ipv6_layer::IPv6Layer;
use crate::ui::fragments::devices_fragment::DevicesFragment;
use crate::ui::fragments::inter::fragment::Fragment;
use crate::ui::fragments::main_fragment::MainFragment;
//use crate::config::VERSION;

#[derive(Clone)]
pub struct OApplication {
    app: Application
}

impl OApplication {

    pub fn new() -> Self {
        let app = Application::new(Some("com.omniscient.rust"), Default::default());

        Self {
            app
        }
    }

    pub fn on_create(&self) {
        let _self = self.clone();
        self.app.connect_activate(move |app| {
            let builder = Builder::from_file("res/ui/gtk3/window.ui");

            let provider = CssProvider::new();
            provider.load_from_path("res/ui/gtk3/window.css").expect("Failed to load CSS file.");

            StyleContext::add_provider_for_screen(
                &gdk::Screen::default().expect("Failed to get default screen."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            let window: ApplicationWindow = builder
                .object("MainWindow")
                .expect("Failed to get the 'MainWindow' from window.ui");

            window.set_application(Some(app));
            window.connect_destroy(|_| exit(0));
            //window.set_decorated(false);
            window.set_border_width(1);

            window.set_titlebar(Some(&_self.init_titlebar(&window)));

            let stack = Stack::new();
            window.add(&stack);
            stack.show();

            let mut fragment = DevicesFragment::new(_self.clone());
            //let mut fragment = MainFragment::new(_self.clone());
            let name = fragment.get_name();
            let title = fragment.get_title();
            let root = fragment.on_create();
            stack.add_titled(root, &name, &title);
            //stack.set_visible_child_name(&fragment.get_name());

            _self.init_actions(&window);

            window.show();
        });

        self.app.run();
    }

    pub fn get_window(&self) -> Option<Window> {
        self.app.active_window()
    }

    pub fn get_titlebar(&self) -> Option<Widget> {
        self.app.active_window().unwrap().titlebar()
    }

    fn init_titlebar(&self, window: &ApplicationWindow) -> Widget {
        let builder = Builder::from_file("res/ui/titlebar-ui.xml");

        let titlebar: gtk::Box = builder
            .object("titlebar")
            .expect("Couldn't find 'titlebar' in titlebar-ui.xml");

        //window.set_titlebar(Some(&titlebar));
        titlebar.set_size_request(-1, 32);

        titlebar.style_context().add_class("wifi");


        let network_type_label: Label = builder
            .object("network_type_label")
            .expect("Couldn't find 'network_type_label' in titlebar-ui.xml");
        network_type_label.set_label("wlp2s0");



        let minimize_button: Button = builder
            .object("minimize_button")
            .expect("Couldn't find 'minimize_button' in titlebar-ui.xml");

        let window_clone = window.clone();
        minimize_button.connect_clicked(move |_| {
            window_clone.iconify();
        });

        let maximize_button: Button = builder
            .object("maximize_button")
            .expect("Couldn't find 'maximize_button' in titlebar-ui.xml");

        let window_clone = window.clone();
        maximize_button.connect_clicked(move |_| {
            if window_clone.is_maximized() {
                window_clone.unmaximize();
                return;
            }

            window_clone.maximize();
        });

        let close_button: Button = builder
            .object("close_button")
            .expect("Couldn't find 'close_button' in titlebar-ui.xml");

        let app_clone = self.app.clone();
        close_button.connect_clicked(move |_| {
            app_clone.quit();
        });

        titlebar.upcast()
    }

    fn init_actions(&self, window: &ApplicationWindow) {
        /*
        let action = SimpleAction::new("quit", None);
        let app_clone = app.clone();
        action.connect_activate(move |_, _| {
            app_clone.quit();
        });
        window.add_action(&action);

        let action = SimpleAction::new("show-about-dialog", None);
        let window_clone = window.clone();
        action.connect_activate(move |_, _| {
            show_about(&window_clone);
        });
        window.add_action(&action);
        */
    }

    pub fn get_child_by_name(&self, widget: &Widget, name: &str) -> Option<Widget> {
        if widget.widget_name().as_str() == name {
            return Some(widget.clone());
        }

        if let Some(container) = widget.dynamic_cast_ref::<Container>() {
            for child in container.children() {
                if let Some(found) = self.get_child_by_name(&child, name) {
                    return Some(found);
                }
            }
        }

        None
    }
}







pub fn show_about(window: &ApplicationWindow) {
    /*
    let svg_data = include_bytes!("../res/images/ic_launcher.svg");
    let loader = PixbufLoader::with_type("svg").expect("Failed to create SVG loader");
    loader.write(svg_data).expect("Failed to load SVG data");
    loader.close().expect("Failed to close SVG loader");
    let icon_pixbuf = loader.pixbuf().expect("Failed to get Pixbuf from SVG");

    let dialog = AboutDialog::builder()
        .transient_for(window)
        .modal(true)
        .program_name("Omniscient")
        .version(VERSION)
        .authors(vec!["DrBrad"])
        .website_label("https://omniscient.com")
        .website("https://omniscient.com")
        .comments("")
        .copyright("Copyright (c) 2024 Omniscient")
        .license("Copyright (c) 2024 Omniscient\r\n\r\n\
        \
        Permission is hereby granted, free of charge, to any person obtaining a copy\r\n\
        of this software and associated documentation files (the \"Software\"), to deal\r\n\
        in the Software without restriction, including without limitation the rights\r\n\
        to use, copy, modify, merge, publish, distribute, sublicense, and/or sell\r\n\
        copies of the Software, and to permit persons to whom the Software is\r\n\
        furnished to do so, subject to the following conditions:\r\n\r\n\
        \
        The above copyright notice and this permission notice shall be included in all\r\n\
        copies or substantial portions of the Software.\r\n\r\n\
        \
        THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\r\n\
        IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\r\n\
        FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\r\n\
        AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\r\n\
        LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\r\n\
        OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\r\n\
        SOFTWARE.")
        .logo(&icon_pixbuf)
        .build();

    dialog.present();
    */
}
