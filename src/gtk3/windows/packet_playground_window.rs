use gtk::{Window, WindowType};
use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt};
use crate::gtk3::views::packet_playground_view::PacketPlaygroundView;

#[derive(Clone)]
pub struct PacketPlaygroundWindow {
    pub window: Window
}

impl PacketPlaygroundWindow {

    pub fn new() -> Self {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Packet Playground");
        window.set_default_size(1200, 700);

        let view = PacketPlaygroundView::new();

        window.add(&view.root);
        window.show();

        Self {
            window
        }
    }
}
