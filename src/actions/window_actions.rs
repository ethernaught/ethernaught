use gtk::gio::{SimpleAction, SimpleActionGroup};
use gtk::prelude::{ActionMapExt, StackExt, WidgetExt};
use gtk::{glib, ApplicationWindow, Stack};
use pcap::devices::Device;
use crate::pcap_ext::devices::Serialize;
use crate::views::inter::view::View;
use crate::views::main_view::MainView;

pub fn register_window_actions(window: &ApplicationWindow) {
    //window.action_group("win");
    //let actions = SimpleActionGroup::new();

    //let actions = SimpleActionGroup::new();
    //window.insert_action_group("win", Some(&actions));
}

pub fn register_stack_actions(window: &ApplicationWindow, stack: &Stack) {
    let action = SimpleAction::new("open", Some(&glib::VariantTy::BYTE_STRING));//Some(&glib::VariantTy::ANY));
    action.connect_activate({
        let stack = stack.clone();
        move |_, param| {
            if let Some(param) = param {
                let device = Device::unserialize(&param.get::<Vec<u8>>().unwrap());

                let view = MainView::from_device(&device);

                let name = view.get_name();
                stack.add_titled(&view.root, &name, &view.get_title());
                stack.set_visible_child_name(&name);
            }
        }
    });
    window.add_action(&action);
}
