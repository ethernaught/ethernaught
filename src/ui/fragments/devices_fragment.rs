use gtk::{Builder, Stack};
use gtk::prelude::{BuilderExtManual, StackExt};

pub fn init_devices_fragment(stack: &Stack) {
    let builder = Builder::from_file("res/ui/gtk3/devices-fragment.ui");
    let interface_layout: gtk::Box = builder
        .object("interface_layout")
        .expect("Couldn't find 'selection_layout' in devices-fragment.ui");
    stack.add_titled(&interface_layout, "interface_layout", "Selection");
}
