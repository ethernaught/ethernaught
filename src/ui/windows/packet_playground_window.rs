use gtk::{gdk, ApplicationWindow, Builder, CssProvider, StyleContext, Window, WindowType};
use gtk::gdk::RGBA;
use gtk::prelude::{BuilderExtManual, CssProviderExt, GtkWindowExt, WidgetExt};
use crate::ui::widgets::hex_editor::HexEditor;

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
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/packet_playground_window.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/packet_playground_window.css");
        //provider.load_from_path("res/ui/gtk3/window.css").expect("Failed to load CSS file.");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window: Window = builder
            .object("PacketPlaygroundWindow")
            .expect("Failed to get the 'PacketPlaygroundWindow' from window.ui");



        let hex_data: Vec<u8> = vec![0xe6, 0x38, 0x83, 0x2e, 0xf3, 0x2, 0xf0, 0x77, 0xc3, 0xbe, 0xd0, 0x70, 0x8, 0x0, 0x45, 0x0, 0x0, 0x48, 0x10, 0x1c, 0x0, 0x0, 0x40, 0x11, 0x3d, 0xf8, 0xa, 0x1, 0xc, 0x8f, 0xa, 0x1, 0xc, 0x1, 0x81, 0xf9, 0x0, 0x35, 0x0, 0x34, 0x2c, 0xd7, 0x39, 0xe9, 0x1, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x3, 0x73, 0x73, 0x6c, 0x7, 0x67, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x3, 0x63, 0x6f, 0x6d, 0x0, 0x0, 0x41, 0x0, 0x1, 0x0, 0x0, 0x29, 0x5, 0xc0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

        let hex_editor: HexEditor = builder
            .object("hex_editor")
            .expect("Couldn't find 'hex_editor' in window.ui");

        hex_editor.set_data(hex_data);
        hex_editor.set_hexpand(true);
        hex_editor.set_vexpand(true);
        hex_editor.set_line_number_color(RGBA::new(0.286, 0.306, 0.341, 1.0));
        hex_editor.set_cursor_color(RGBA::new(0.608, 0.616, 0.624, 1.0));
        hex_editor.set_selection_color(RGBA::new(0.349, 0.263, 0.431, 1.0));




        window.show();

        self.window = Some(window);
    }
}
