use std::cell::RefCell;
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use gtk::{AboutDialog, ApplicationWindow, Builder, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label, CssProvider, StyleContext, gdk, Stack, Container, TreeView, Widget, Window, gio, MenuBar, MenuItem, Menu, FileChooserDialog, ResponseType, FileChooserAction, glib, show_uri_on_window};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::gio::{resources_register, ApplicationFlags, Resource, SimpleAction};
use gtk::glib::Bytes;
use gtk::glib::ControlFlow::{Break, Continue};
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{ActionMapExt, GtkWindowExt};
use crate::ui::activity::devices_activity::DevicesActivity;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::bottombar::BottomBar;
use crate::ui::context::Context;
use crate::ui::handlers::bundle::Bundle;
use crate::ui::titlebar::TitleBar;
use crate::ui::widgets::graph::Graph;
use crate::ui::widgets::hex_editor::HexEditor;
use crate::ui::widgets::terminal::Terminal;

#[derive(Clone)]
pub struct OApplication {
    context: Context
}

impl OApplication {

    pub fn new() -> Self {
        let app = Application::new(Some("com.ethernaut.rust"), ApplicationFlags::HANDLES_OPEN);

        Self {
            context: Context::new(app)
        }
    }

    pub fn run(&self) {
        let _self = self.clone();
        self.context.get_application().connect_activate(move |app| {
            _self.on_create(app);
            _self.context.start_activity(Box::new(DevicesActivity::new(_self.context.clone())), None);
        });

        let _self = self.clone();
        self.context.get_application().connect_open(move |app, files, _hint| {
            for file in files {
                if let Some(path) = file.path() {
                    _self.on_create(app);
                    let mut bundle = Bundle::new();
                    bundle.put("type", String::from("file"));
                    bundle.put("file", path);

                    _self.context.start_activity(Box::new(MainActivity::new(_self.context.clone())), Some(bundle));
                }
            }
        });

        self.context.get_application().run();
    }

    fn on_create(&self, app: &Application) {
        Graph::static_type();
        HexEditor::static_type();
        Terminal::static_type();

        let resource_data = include_bytes!("../../res/resources.gresources");

        let resource = Resource::from_data(&Bytes::from(resource_data)).unwrap();
        resources_register(&resource);

        let builder = Builder::from_resource("/com/ethernaut/rust/res/ui/gtk3/window.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/com/ethernaut/rust/res/ui/gtk3/window.css");
        //provider.load_from_path("res/ui/gtk3/window.css").expect("Failed to load CSS file.");

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

        #[cfg(profile = "nightly")]
        window.style_context().add_class("nightly");

        #[cfg(profile = "release")]
        window.style_context().add_class("release");

        //window.set_icon_from_file("res/icons/ic_launcher.svg").expect("Failed to load icon");

        let mut titlebar = TitleBar::new(self.context.clone());
        window.set_titlebar(Some(titlebar.on_create()));

        let window_content: gtk::Box = builder
            .object("window_content")
            .expect("Failed to get the 'window_content' from window.ui");

        let stack = Stack::new();
        window_content.add(&stack);
        stack.show();

        let mut bottombar = BottomBar::new(self.clone());
        window_content.add(bottombar.on_create());

        self.init_actions(&window);

        let context = self.context.clone();
        window.connect_button_press_event(move |_, event| {
            match event.button() {
                8 => {
                    context.on_back_pressed();
                }
                9 => {
                    context.on_next_pressed();
                }
                _ => {}
            }

            Proceed
        });

        window.show();
    }

    fn init_actions(&self, window: &ApplicationWindow) {
        let action = SimpleAction::new("open", None);
        action.connect_activate({
            let context = self.context.clone();
            move |_, _| {
                if let Some(path) = open_file_selector(context.get_window().unwrap().upcast_ref()) {
                    let mut bundle = Bundle::new();
                    bundle.put("type", String::from("file"));
                    bundle.put("file", path);

                    context.start_activity(Box::new(MainActivity::new(context.clone())), Some(bundle));
                }
            }
        });
        window.add_action(&action);

        let action = SimpleAction::new("website", None);
        action.connect_activate({
            let window = window.clone();
            move |_, _| {
                if let Err(err) = show_uri_on_window(Some(&window), "https://ethernaut.com", gtk::current_event_time()) {
                    eprintln!("Failed to open link: {}", err);
                }
            }
        });
        window.add_action(&action);

        let action = SimpleAction::new("quit", None);
        action.connect_activate({
            let context = self.context.get_application();
            move |_, _| {
                context.quit();
            }
        });
        window.add_action(&action);

        let action = SimpleAction::new("show-about-dialog", None);
        action.connect_activate({
            let window = window.clone();
            move |_, _| {
                open_about_dialog(window.upcast_ref());
            }
        });
        window.add_action(&action);
    }
}

pub fn open_file_selector(parent: &Window) -> Option<PathBuf> {
    let dialog = FileChooserDialog::new(
        Some("Open File"),
        Some(parent),
        FileChooserAction::Open
    );

    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Open", ResponseType::Accept);

    if dialog.run() == ResponseType::Accept {
        dialog.close();
        return dialog.filename();
    }

    dialog.close();

    None
}

pub fn open_about_dialog(window: &Window) {
    let icon_pixbuf = Pixbuf::from_resource("/com/ethernaut/rust/res/icons/ic_launcher.svg").expect("Failed to get Pixbuf from SVG");

    let dialog = AboutDialog::builder()
        .transient_for(window)
        .modal(true)
        .program_name("Ethernaut")
        .version(format!("{}-{}", env!("PROFILE"), env!("CARGO_PKG_VERSION")).as_str())
        .authors(vec!["DrBrad"])
        .website_label("https://ethernaut.com")
        .website("https://ethernaut.com")
        .comments("")
        .copyright("Copyright (c) 2024 Ethernaut")
        .license("Copyright (c) 2024 Ethernaut\r\n\r\n\
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
}
