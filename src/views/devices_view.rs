use gtk::{gdk, Builder, Container, CssProvider, StyleContext};
use gtk::prelude::{BuilderExtManual, CssProviderExt, WidgetExt};
use crate::views::inter::view::View;

pub struct DevicesView {
    pub root: Container
}

impl DevicesView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/devices_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/devices_view.css");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let root: Container = builder
            .object("devices_activity_layout")
            .expect("Couldn't find 'devices_activity_layout' in devices_view.ui");


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
