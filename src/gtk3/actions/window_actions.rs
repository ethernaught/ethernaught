use std::env;
use std::path::{Path, PathBuf};
use gtk::gio::{SimpleAction};
use gtk::prelude::{ActionMapExt, Cast, ContainerExt, FileChooserExt, GtkWindowExt, NativeDialogExt, ProxyResolverExt, StackExt, StyleContextExt, WidgetExt};
use gtk::glib::{PropertyGet, VariantDict, VariantTy};
use gtk::{AboutDialog, FileChooserAction, FileChooserNative, FileFilter, ResponseType, Window};
use gtk::gdk_pixbuf::Pixbuf;
use rlibpcap::devices::Device;
use crate::pcap_ext::devices::Serialize;
use crate::gtk3::views::main_view::MainView;
use crate::gtk3::windows::main_window::MainWindow;

pub fn register_window_actions(window: &MainWindow) {
    let action = SimpleAction::new("open", None);
    action.connect_activate({
        let window = window.clone();
        move |_, _| {
            let dialog = FileChooserNative::new(Some("Open File"), Some(&window.window), FileChooserAction::Open, Some("Open"), Some("Cancel"));

            let default_path = if let Ok(sudo_user) = env::var("SUDO_USER") {
                let user_home = format!("/home/{}", sudo_user);
                Path::new(&user_home).to_path_buf()
            } else {
                env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/"))
            };
            dialog.set_current_folder(&default_path);

            /*
            let filter = FileFilter::new();

            filter.add_mime_type("application/vnd.tcpdump.pcap");
            filter.add_mime_type("application/x-pcapng");
            filter.add_mime_type("application/x-snoop");
            filter.add_mime_type("application/x-iptrace");
            filter.add_mime_type("application/x-lanalyzer");
            filter.add_mime_type("application/x-nettl");
            filter.add_mime_type("application/x-radcom");
            filter.add_mime_type("application/x-etherpeek");
            filter.add_mime_type("application/x-visualnetworks");
            filter.add_mime_type("application/x-netinstobserver");
            filter.add_mime_type("application/x-5view");
            filter.add_mime_type("application/x-tektronix-rf5");
            filter.add_mime_type("application/x-micropross-mplog");
            filter.add_mime_type("application/x-apple-packetlogger");
            filter.add_mime_type("application/x-endace-erf");
            filter.add_mime_type("application/ipfix");
            filter.add_mime_type("application/x-ixia-vwr");
            filter.set_name(Some("Pcap and Dump files"));
            dialog.add_filter(filter);
            */

            dialog.connect_response({
                let window = window.clone();
                move |dialog, response| {
                    if response == ResponseType::Accept {
                        if let Some(path) = dialog.filename() {
                            println!("Selected file: {:?}", path);
                            let view = Box::new(MainView::from_pcap(&window, &path));
                            window.add_view(view);
                        }
                    }
                }
            });

            dialog.show();
        }
    });
    window.window.add_action(&action);

    let action = SimpleAction::new("show-about-dialog", None);
    action.connect_activate({
        let window = window.window.clone();
        move |_, _| {
            open_about_dialog(&window.upcast_ref());
        }
    });
    window.window.add_action(&action);

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
            //window.unfullscreen();
            //window.fullscreen();
        }
    });
    window.window.add_action(&action);
}

pub fn register_stack_actions(window: &MainWindow) {
    let action = SimpleAction::new("view", Some(&VariantTy::new("a{sv}").unwrap()));
    //let action = SimpleAction::new("open", Some(&glib::VariantTy::BYTE_STRING));//Some(&glib::VariantTy::ANY));
    action.connect_activate({
        let window = window.clone();
        move |_, param| {
            if let Some(param) = param {
                if let Some(dict) = param.get::<VariantDict>() {
                    if let Some(name) = dict.lookup::<String>("name").ok().unwrap() {
                        let view = match name.as_str() {
                            "main_view" => {
                                if let Some(_type) = dict.lookup::<String>("type").ok().unwrap() {
                                    match _type.as_str() {
                                        "device" => {
                                            let device = Device::unserialize(&dict.lookup::<Vec<u8>>("device").ok().unwrap().unwrap());
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
        let title_bar = window.title_bar.clone();
        move |_, _| {
            if let Some(current) = stack.visible_child() {
                let children = stack.children();
                if let Some(pos) = children.iter().position(|child| child == &current) {
                    if pos > 0 {
                        stack.set_visible_child(&children[pos - 1]);
                        title_bar.back.style_context().remove_class("active");
                        title_bar.next.style_context().add_class("active");
                    }
                }
            }
        }
    });
    window.window.add_action(&action);

    let action = SimpleAction::new("next", None);
    action.connect_activate({
        let stack = window.stack.clone();
        let title_bar = window.title_bar.clone();
        move |_, _| {
            if let Some(current) = stack.visible_child() {
                let children = stack.children();
                if let Some(pos) = children.iter().position(|child| child == &current) {
                    if pos < children.len() - 1 {
                        stack.set_visible_child(&children[pos + 1]);
                        title_bar.next.style_context().remove_class("active");
                        title_bar.back.style_context().add_class("active");
                    }
                }
            }
        }
    });
    window.window.add_action(&action);
}

pub fn open_about_dialog(window: &Window) {
    let icon_pixbuf = Pixbuf::from_resource("/net/ethernaught/rust/res/icons/ic_launcher.svg").expect("Failed to get Pixbuf from SVG");

    let dialog = AboutDialog::builder()
        .transient_for(window)
        .modal(true)
        .program_name("Ethernaught")
        .version(format!("{}-{}-{}", env!("PROFILE"), env!("CARGO_PKG_VERSION"), "gtk4").as_str())
        .authors(vec!["DrBrad"])
        .website_label("https://ethernaught.net")
        .website("https://ethernaught.net")
        .comments("")
        .copyright("Copyright (c) 2024 Ethernaught")
        .license("Copyright (c) 2024 Ethernaught")
        .logo(&icon_pixbuf)
        .build();

    dialog.present();
}

