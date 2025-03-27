use gtk::{gdk, Builder, CssProvider, StyleContext};
use gtk::prelude::{BuilderExtManual, CssProviderExt};
use pcap::devices::Device;
use crate::views::devices_view::DevicesView;
use crate::views::inter::view::View;

pub struct MainView {
    pub root: gtk::Box
}

impl MainView {

    pub fn from_device(device: &Device) -> Self {
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

        Self {
            root
        }
    }
}

impl View for DevicesView {

    fn get_name(&self) -> String {
        "devices_view".to_string()
    }

    fn get_title(&self) -> String {
        "DevicesView".to_string()
    }
}
