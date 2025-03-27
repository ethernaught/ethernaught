use std::process::exit;
use gtk::{gdk, Application, ApplicationWindow, Builder, CssProvider, Stack, StyleContext, Window};
use gtk::prelude::{BuilderExtManual, ContainerExt, CssProviderExt, GtkWindowExt, StackExt, WidgetExt};
use pcap::devices::Device;
use pcap::utils::interface_flags::InterfaceFlags;
use crate::actions::window_actions::register_window_actions;
use crate::views::bottom_bar::BottomBar;
use crate::views::devices_view::DevicesView;
use crate::views::inter::view::View;
use crate::views::title_bar::TitleBar;

pub struct MainWindow {
    pub window: ApplicationWindow
}

impl MainWindow {

    pub fn new(app: &Application) -> Self {
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

        let title_bar = TitleBar::new();
        window.set_titlebar(Some(&title_bar.root));

        let window_content: gtk::Box = builder
            .object("window_content")
            .expect("Failed to get the 'window_content' from window.ui");

        //window_content.add(&create_alertbar());

        let stack: Stack = builder
            .object("stack")
            .expect("Failed to get the 'stack' from window.ui");

        stack.show();

        let bottom_bar = BottomBar::new();
        window_content.add(&bottom_bar.root);


        let mut devices = Device::list().expect("Failed to get device list");
        devices.sort_by(|a, b| {
            b.get_flags().contains(&InterfaceFlags::Running).cmp(&a.get_flags().contains(&InterfaceFlags::Running))
        });

        let view = DevicesView::new(&window, devices);
        stack.add_titled(&view.root, &view.get_name(), &view.get_title());

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

        register_window_actions(&window);

        window.show();

        Self {
            window
        }
    }
}
