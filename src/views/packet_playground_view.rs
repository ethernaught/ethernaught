use gtk::{gdk, Builder, CssProvider, StyleContext};
use gtk::prelude::{BuilderExtManual, CssProviderExt};

pub struct PacketPlaygroundView {
    pub root: gtk::Box
}

impl PacketPlaygroundView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/packet_playground_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/packet_playground_view.css");

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
