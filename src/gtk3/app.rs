use gtk::{gdk, gio, Application, ApplicationWindow, Builder, CssProvider, Settings, Stack, StyleContext};
use gtk::gio::{resources_register, ApplicationFlags, Resource};
use gtk::glib::{Bytes, StaticType};
use gtk::prelude::{ApplicationExt, ApplicationExtManual, BuilderExtManual, ContainerExt, CssProviderExt, FileExt, GtkApplicationExt, GtkWindowExt, ObjectExt, StackExt, WidgetExt};
use crate::gtk3::actions::app_actions::register_app_actions;
use crate::gtk3::widgets::graph::Graph;
use crate::gtk3::widgets::hex_editor::HexEditor;
use crate::gtk3::widgets::terminal::Terminal;
use crate::gtk3::widgets::overlay::Overlay;
use crate::gtk3::windows::main_window::MainWindow;

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
        Overlay::static_type();

        self.app.connect_activate(move |app| {
            if let Some(settings) = Settings::default() {
                settings.set_property("gtk-application-prefer-dark-theme", &true);
            }

            let resource_data = include_bytes!("../../res/resources.gresources");

            let resource = Resource::from_data(&Bytes::from(resource_data)).unwrap();
            resources_register(&resource);

            let provider = CssProvider::new();
            provider.load_from_resource("/net/ethernaught/rust/res/ui/theme.css");

            StyleContext::add_provider_for_screen(
                &gdk::Screen::default().expect("Failed to get default screen."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
            );

            #[cfg(target_os = "macos")]
            {
                let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/ethernaught_ui.xml");
                let model: gio::MenuModel = builder
                    .object("main_window_menu")
                    .expect("Couldn't find 'main_window_menu' in ethernaught_ui.xml");

                app.set_menubar(Some(&model));
            }

            MainWindow::new(&app);

            register_app_actions(&app);
        });

        self.app.connect_open(move |app, files, _hint| {
            if let Some(settings) = Settings::default() {
                settings.set_property("gtk-application-prefer-dark-theme", &true);
            }

            for file in files {
                if let Some(path) = file.path() {
                    let resource_data = include_bytes!("../../res/resources.gresources");

                    let resource = Resource::from_data(&Bytes::from(resource_data)).unwrap();
                    resources_register(&resource);

                    let provider = CssProvider::new();
                    provider.load_from_resource("/net/ethernaught/rust/res/ui/theme.css");

                    StyleContext::add_provider_for_screen(
                        &gdk::Screen::default().expect("Failed to get default screen."),
                        &provider,
                        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
                    );

                    #[cfg(target_os = "macos")]
                    {
                        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/ethernaught_ui.xml");
                        let model: gio::MenuModel = builder
                            .object("main_window_menu")
                            .expect("Couldn't find 'main_window_menu' in ethernaught_ui.xml");

                        app.set_menubar(Some(&model));
                    }

                    MainWindow::from_file(&app, &path);

                    register_app_actions(&app);
                }
            }
        });

        self.app.run();
    }
}
