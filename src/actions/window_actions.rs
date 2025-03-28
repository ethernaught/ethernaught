use gtk::gio::{SimpleAction, SimpleActionGroup};
use gtk::prelude::{ActionMapExt, ContainerExt, GtkWindowExt, ProxyResolverExt, StackExt, ToVariant, WidgetExt};
use gtk::{glib, ApplicationWindow, Stack};
use gtk::glib::{VariantDict, VariantTy};
use pcap::devices::Device;
use crate::pcap_ext::devices::Serialize;
use crate::views::inter::stackable::Stackable;
use crate::views::main_view::MainView;
use crate::windows::main_window::MainWindow;

pub fn register_window_actions(window: &MainWindow) {
    //window.action_group("win");
    //let actions = SimpleActionGroup::new();

    //let actions = SimpleActionGroup::new();
    //window.insert_action_group("win", Some(&actions));
    let action = SimpleAction::new("minimize", None);
    action.connect_activate({
        let window = window.window.clone();
        move |_, _| {
            window.iconify();
        }
    });
    window.window.add_action(&action);

    let action = SimpleAction::new("maximize", None);
    action.connect_activate({
        let window = window.window.clone();
        move |_, _| {
            if window.is_maximized() {
                window.unmaximize();
                return;
            }

            window.maximize();
        }
    });
    window.window.add_action(&action);
}

pub fn register_stack_actions(window: &MainWindow) {
    let action = SimpleAction::new("open", Some(&VariantTy::new("a{sv}").unwrap()));
    //let action = SimpleAction::new("open", Some(&glib::VariantTy::BYTE_STRING));//Some(&glib::VariantTy::ANY));
    action.connect_activate({
        let window = window.clone();
        move |_, param| {
            if let Some(param) = param {
                if let Some(dict) = param.get::<VariantDict>() {
                    /*
                    match dict.lookup::<String>("name").ok() {
                        Some(_) => {}
                        None => {}
                    }
                    */

                    println!("{:?}", dict.lookup::<Vec<u8>>("device"));
                }
                /*
                let device = Device::unserialize(&param.get::<Vec<u8>>().unwrap());
                window.add_view(Box::new(MainView::from_device(&window, &device)));
                */
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
