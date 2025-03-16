use gtk::{Window, WindowType};
use gtk::prelude::{GtkWindowExt, WidgetExt};

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
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Packet Playground");
        window.set_default_size(450, 300);

        window.show_all();

        self.window = Some(window);
    }
}
