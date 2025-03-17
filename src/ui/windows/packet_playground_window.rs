use gtk::{gdk, ApplicationWindow, Builder, CssProvider, StyleContext, Window, WindowType};
use gtk::prelude::{BuilderExtManual, CssProviderExt, GtkWindowExt, WidgetExt};

pub struct PacketPlaygroundWindow {
    window: Option<Window>
}

impl PacketPlaygroundWindow {

    pub fn new() -> Self {
        Self {
            window: None
        }
    }

    pub fn on_create(&mut self) {
        let builder = Builder::from_resource("/com/ethernaut/rust/res/ui/gtk3/packet_playground_window.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/com/ethernaut/rust/res/ui/gtk3/packet_playground_window.css");
        //provider.load_from_path("res/ui/gtk3/window.css").expect("Failed to load CSS file.");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window: Window = builder
            .object("PacketPlaygroundWindow")
            .expect("Failed to get the 'PacketPlaygroundWindow' from window.ui");

        window.show();

        self.window = Some(window);
    }
}
