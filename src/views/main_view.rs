use gtk::{gdk, Builder, CssProvider, StyleContext};
use gtk::prelude::{BuilderExtManual, CssProviderExt};
use pcap::devices::Device;
use crate::views::inter::view::View;
use crate::windows::main_window::MainWindow;

pub struct MainView {
    pub root: gtk::Box
}

impl MainView {

    pub fn from_device(window: &MainWindow, device: &Device) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/main_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/main_view.css");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in main_view.ui");

        println!("{:?}", window.title_bar.root);

        Self {
            root
        }
    }
}

impl View for MainView {

    fn get_name(&self) -> String {
        "main_view".to_string()
    }

    fn get_title(&self) -> String {
        "MainView".to_string()
    }
}
