use gtk::gio::{SimpleAction, SimpleActionGroup};
use gtk::prelude::{ActionMapExt, ContainerExt, GtkWindowExt, ProxyResolverExt, StackExt, ToVariant, WidgetExt};
use gtk::glib::{PropertyGet, VariantDict, VariantTy};
use pcap::devices::Device;
use crate::pcap_ext::devices::Serialize;
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

    let action = SimpleAction::new("menu", None);
    action.connect_activate({
        let title_bar = window.title_bar.clone();
        move |_, _| {
            title_bar.open_menubar();
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
                    if let Some(name) = dict.lookup::<String>("name").ok().unwrap() {
                        match window.stack.child_by_name(&name) {
                            Some(child) => {
                                let pos = window.stack.child_position(&child) as usize;

                                let children = window.stack.children();
                                for i in (pos..children.len()).rev() {
                                    let name = window.stack.child_name(&children[i]).unwrap().to_string();
                                    window.stack.remove(&children[i]);
                                    window.views.borrow_mut().remove(&name);
                                }
                            }
                            None => {
                                let children = window.stack.children();
                                if let Some(current) = window.stack.visible_child() {
                                    if let Some(pos) = children.iter().position(|child| child == &current) {
                                        for i in (pos + 1..children.len()).rev() {
                                            let name = window.stack.child_name(&children[i]).unwrap().to_string();
                                            window.stack.remove(&children[i]);
                                            window.views.borrow_mut().remove(&name);
                                        }
                                    }
                                }
                            }
                        }

                        let view = match name.as_str() {
                            "main_view" => {
                                if let Some(_type) = dict.lookup::<String>("type").ok().unwrap() {
                                    match _type.as_str() {
                                        "device" => {
                                            let device =  Device::unserialize(&dict.lookup::<Vec<u8>>("device").ok().unwrap().unwrap());
                                            Box::new(MainView::from_device(&window, &device))
                                        }
                                        "any" => {
                                            Box::new(MainView::new(&window))
                                        }
                                        _ => unimplemented!()
                                    }
                                } else {
                                    unimplemented!()
                                }
                            }
                            _ => unimplemented!()
                        };

                        window.add_view(view);
                    }
                }
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
