use gtk::gio::{SimpleAction, SimpleActionGroup};
use gtk::prelude::{ActionMapExt, ContainerExt, StackExt, WidgetExt};
use gtk::{glib, ApplicationWindow, Stack};
use pcap::devices::Device;
use crate::pcap_ext::devices::Serialize;
use crate::views::inter::view::View;
use crate::views::main_view::MainView;
use crate::windows::main_window::MainWindow;

pub fn register_window_actions(window: &MainWindow) {
    //window.action_group("win");
    //let actions = SimpleActionGroup::new();

    //let actions = SimpleActionGroup::new();
    //window.insert_action_group("win", Some(&actions));
}

pub fn register_stack_actions(window: &MainWindow) {
    let action = SimpleAction::new("open", Some(&glib::VariantTy::BYTE_STRING));//Some(&glib::VariantTy::ANY));
    action.connect_activate({
        let window = window.clone();
        move |_, param| {
            if let Some(param) = param {
                let device = Device::unserialize(&param.get::<Vec<u8>>().unwrap());

                let view = MainView::from_device(&window, &device);

                let name = view.get_name();
                window.stack.add_titled(&view.root, &name, &view.get_title());
                window.stack.set_visible_child_name(&name);
            }
        }
    });
    window.window.add_action(&action);

    let action = SimpleAction::new("back", None);
    action.connect_activate({
        let stack = window.stack.clone();
        move |_, _| {
            let children = stack.children();
            if let Some(current) = stack.visible_child() {
                if let Some(pos) = children.iter().position(|child| child == &current) {
                    if pos > 0 {
                        stack.set_visible_child(&children[pos - 1]);
                    }
                }
            }
        }
    });
    window.window.add_action(&action);

    let action = SimpleAction::new("next", None);
    action.connect_activate({
        let stack = window.stack.clone();
        move |_, _| {
            let children = stack.children();
            if let Some(current) = stack.visible_child() {
                if let Some(pos) = children.iter().position(|child| child == &current) {
                    if pos < children.len() - 1 {
                        stack.set_visible_child(&children[pos + 1]);
                    }
                }
            }
        }
    });
    window.window.add_action(&action);
}
