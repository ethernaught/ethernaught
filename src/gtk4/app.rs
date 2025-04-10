use gtk4::{gdk, Application, CssProvider, StyleContext};
use gtk4::gio::{resources_register, ApplicationFlags, Resource};
use gtk4::glib::Bytes;
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, StyleContextExt};
use crate::gtk4::windows::main_window::MainWindow;

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
        //Graph::static_type();
        //HexEditor::static_type();
        //Terminal::static_type();
        //Overlay::static_type();

        self.app.connect_activate(move |app| {
            let resource_data = include_bytes!("../../res/resources.gresources");

            let resource = Resource::from_data(&Bytes::from(resource_data)).unwrap();
            resources_register(&resource);


            let provider = CssProvider::new();
            provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk4/theme.css");

            gtk4::style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

            //let provider = CssProvider::new();
            //provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/theme.css");

            //StyleContext::add_provider(app, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

            #[cfg(target_os = "macos")]
            {
                let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/ethernaught_ui.xml");
                let model: gio::MenuModel = builder
                    .object("main_window_menu")
                    .expect("Couldn't find 'main_window_menu' in ethernaught_ui.xml");

                app.set_menubar(Some(&model));
            }

            MainWindow::new(&app);

            //register_app_actions(&app);
        });

        /*
        self.app.connect_open(move |app, files, _hint| {
            for file in files {
                if let Some(path) = file.path() {
                    let resource_data = include_bytes!("../../res/resources.gresources");

                    let resource = Resource::from_data(&Bytes::from(resource_data)).unwrap();
                    resources_register(&resource);

                    let provider = CssProvider::new();
                    provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/theme.css");

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
        });*/

        self.app.run();
    }
}
