use gtk4::prelude::{GtkWindowExt, WidgetExt};
use gtk4::Window;
use crate::gtk4::views::packet_playground_view::PacketPlaygroundView;

#[derive(Clone)]
pub struct PacketPlaygroundWindow {
    pub window: Window
}

impl PacketPlaygroundWindow {

    pub fn new() -> Self {
        let window = Window::new();
        window.set_title(Some("Packet Playground"));
        window.set_default_size(1200, 700);

        let view = PacketPlaygroundView::new();

        window.set_child(Some(&view.root));
        window.show();

        Self {
            window
        }
    }
}
