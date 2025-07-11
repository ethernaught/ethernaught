use std::env;
use std::path::{Path, PathBuf};
use gtk4::gio::{File, SimpleAction};
use gtk4::glib::{VariantDict, VariantTy};
use gtk4::prelude::{ActionMapExt, Cast, FileChooserExt, FileChooserExtManual, FileExt, GtkWindowExt, ListModelExt, NativeDialogExt, StyleContextExt, WidgetExt};
use gtk4::{AboutDialog, FileChooserAction, FileChooserNative, ResponseType, StackPage, Window};
use gtk4::gdk::Texture;
use rlibpcap::devices::Device;
use crate::gtk4::views::main_view::MainView;
use crate::gtk4::windows::main_window::MainWindow;
use crate::pcap_ext::devices::Serialize;

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

            dialog.set_current_folder(Some(&File::for_path(default_path))).unwrap();
            //dialog.set_current_folder(Some(&default_path));

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
            dialog.add_filter(&filter);*/

            dialog.connect_response({
                let window = window.clone();
                move |dialog, response| {
                    if response == ResponseType::Accept {
                        if let Some(file) = dialog.file() {
                            if let Some(path) = file.path() {
                                let view = Box::new(MainView::from_pcap(&window, &path));
                                window.add_view(view);
                            }
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
            window.minimize();
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
                let pages = stack.pages();
                for i in 0..pages.n_items() {
                    let page = pages.item(i).expect("Failed to get page")
                        .downcast::<StackPage>()
                        .expect("Item is not a StackPage");

                    if current.eq(&page.child()) && i > 0 {
                        stack.set_visible_child(&pages.item(i - 1).expect("Failed to get page")
                            .downcast::<StackPage>()
                            .expect("Item is not a StackPage").child());
                        title_bar.back.style_context().remove_class("active");
                        title_bar.next.style_context().add_class("active");
                        break;
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
                let pages = stack.pages();
                for i in 0..pages.n_items() {
                    let page = pages.item(i).expect("Failed to get page")
                        .downcast::<StackPage>()
                        .expect("Item is not a StackPage");

                    if current.eq(&page.child()) && i < pages.n_items() - 1 {
                        stack.set_visible_child(&pages.item(i + 1).expect("Failed to get page")
                            .downcast::<StackPage>()
                            .expect("Item is not a StackPage").child());
                        title_bar.next.style_context().remove_class("active");
                        title_bar.back.style_context().add_class("active");
                        break;
                    }
                }
            }
        }
    });
    window.window.add_action(&action);
}

pub fn open_about_dialog(window: &Window) {
    let icon_paintable = Texture::from_resource("/net/ethernaught/rust/res/icons/ic_launcher.svg");

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
        .logo(&icon_paintable)
        .build();

    dialog.present();
}

