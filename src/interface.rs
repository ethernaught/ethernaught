use gtk::{Builder, Stack};
use gtk::prelude::{BuilderExtManual, StackExt};

pub fn init_interface(stack: &Stack) {
    let builder = Builder::from_file("res/ui/gtk3/interface-fragment.ui");
    let interface_layout: gtk::Box = builder
        .object("interface_layout")
        .expect("Couldn't find 'selection_layout' in interface-fragment.ui");
    stack.add_titled(&interface_layout, "interface_layout", "Selection");
}

