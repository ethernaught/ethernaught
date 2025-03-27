use std::process::exit;
use gtk::{gdk, Application, ApplicationWindow, Builder, CssProvider, Stack, StyleContext};
use gtk::gio::{resources_register, ApplicationFlags, Resource};
use gtk::glib::Bytes;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{ApplicationExt, ApplicationExtManual, BuilderExtManual, ContainerExt, CssProviderExt, FileExt, GtkWindowExt, WidgetExt};

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
        self.app.connect_activate(move |app| {
            let resource_data = include_bytes!("../res/resources.gresources");

            let resource = Resource::from_data(&Bytes::from(resource_data)).unwrap();
            resources_register(&resource);

            let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/window.ui");

            let provider = CssProvider::new();
            provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/window.css");
            //provider.load_from_path("res/ui/gtk3/window.css").expect("Failed to load CSS file.");

            StyleContext::add_provider_for_screen(
                &gdk::Screen::default().expect("Failed to get default screen."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            let window: ApplicationWindow = builder
                .object("main_window")
                .expect("Failed to get the 'main_window' from window.ui");

            window.set_application(Some(app));
            window.connect_destroy(|_| exit(0));
            //window.set_decorated(false);
            window.set_border_width(1);

            #[cfg(profile = "nightly")]
            window.style_context().add_class("nightly");

            #[cfg(profile = "release")]
            window.style_context().add_class("release");

            //window.set_icon_from_file("res/icons/ic_launcher.svg").expect("Failed to load icon");

            //let mut titlebar = TitleBar::new(self.context.clone());
            //window.set_titlebar(Some(titlebar.on_create()));

            let window_content: gtk::Box = builder
                .object("window_content")
                .expect("Failed to get the 'window_content' from window.ui");

            //window_content.add(&create_alertbar());

            let stack = Stack::new();
            window_content.add(&stack);
            stack.show();

            //let mut bottombar = BottomBar::new(self.clone());
            //window_content.add(bottombar.on_create());

            /*
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
            */

            window.show();
        });

        /*
        self.app.connect_open(move |app, files, _hint| {
            for file in files {
                if let Some(path) = file.path() {
                }
            }
        });
        */

        self.app.run();
    }
}
