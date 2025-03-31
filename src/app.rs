use std::process::exit;
use gtk::{gdk, Application, ApplicationWindow, Builder, CssProvider, Stack, StyleContext};
use gtk::gio::{resources_register, ApplicationFlags, Resource};
use gtk::glib::{Bytes, StaticType};
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{ApplicationExt, ApplicationExtManual, BuilderExtManual, ContainerExt, CssProviderExt, FileExt, GtkWindowExt, StackExt, WidgetExt};
use rlibpcap::devices::Device;
use rlibpcap::utils::interface_flags::InterfaceFlags;
use crate::actions::app_actions::register_app_actions;
use crate::views::bottom_bar::BottomBar;
use crate::views::devices_view::DevicesView;
use crate::views::inter::stackable::Stackable;
use crate::widgets::graph::Graph;
use crate::widgets::hex_editor::HexEditor;
use crate::widgets::terminal::Terminal;
use crate::widgets::view_stack::ViewStack;
use crate::windows::main_window::MainWindow;

pub struct App {
    app: Application
}

impl App {

    pub fn new() -> Self {
        let app = Application::new(Some("net.ethernaught.rust"), ApplicationFlags::HANDLES_OPEN);

        Self {
            app
        }
    }

    pub fn run(&self) {
        Graph::static_type();
        HexEditor::static_type();
        Terminal::static_type();
        ViewStack::static_type();

        self.app.connect_activate(move |app| {
            let resource_data = include_bytes!("../res/resources.gresources");

            let resource = Resource::from_data(&Bytes::from(resource_data)).unwrap();
            resources_register(&resource);

            MainWindow::new(&app);

            register_app_actions(&app);
        });

        self.app.connect_open(move |app, files, _hint| {
            for file in files {
                if let Some(path) = file.path() {
                    let resource_data = include_bytes!("../res/resources.gresources");

                    let resource = Resource::from_data(&Bytes::from(resource_data)).unwrap();
                    resources_register(&resource);

                    MainWindow::from_file(&app, &path);

                    register_app_actions(&app);
                }
            }
        });

        self.app.run();
    }
}
