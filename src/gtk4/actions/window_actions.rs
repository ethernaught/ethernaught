use std::env;
use std::path::{Path, PathBuf};
use gtk4::gio::SimpleAction;
use gtk4::glib::{VariantDict, VariantTy};
use gtk4::prelude::{ActionMapExt, Cast, DialogExt, DialogExtManual, FileChooserExt, FileChooserExtManual, FileExt, GtkWindowExt, ListModelExt, RecentManagerExt, StyleContextExt, WidgetExt};
use gtk4::{AboutDialog, FileChooserAction, FileChooserDialog, FileFilter, ResponseType, StackPage, Window};
use crate::gtk4::views::main_view::MainView;
use crate::gtk4::windows::main_window::MainWindow;

pub fn register_window_actions(window: &MainWindow) {
    let action = SimpleAction::new("open", None);
    action.connect_activate({
        let window = window.clone();
        move |_, _| {
            if let Some(path) = open_file_selector(window.window.upcast_ref()) {
                println!("{:?}", path);
                //let view = Box::new(MainView::from_pcap(&window, &path));
                //window.add_view(view);
            }
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
                                            //let device = Device::unserialize(&dict.lookup::<Vec<u8>>("device").ok().unwrap().unwrap());
                                            //Box::new(MainView::from_device(&window, &device))
                                            Box::new(MainView::new(&window))
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

pub fn open_file_selector(parent: &Window) -> Option<PathBuf> {
    let dialog = FileChooserDialog::new(Some("Open File"), Some(parent), FileChooserAction::Open, &[]);

    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Open", ResponseType::Accept);

    /*
    if let Some(default_path) = env::var("HOME").ok() {
        let default_path = Path::new(&default_path);
        dialog.set_current_folder(default_path);
    }*/
    let default_path = if let Ok(sudo_user) = env::var("SUDO_USER") {
        let user_home = format!("/home/{}", sudo_user);
        Path::new(&user_home).to_path_buf()
    } else {
        env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/"))
    };
    //dialog.set_current_folder(Some(&default_path));

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
    dialog.add_filter(&filter);

    //let s = dialog.run_future().await;
    //s.as_mut().
    //if dialog.run() == ResponseType::Accept {
    //    dialog.close();
    //    return dialog.file()?.path();
    //}

    dialog.close();

    None
}

pub fn open_about_dialog(window: &Window) {
    //let icon_pixbuf = Pixbuf::from_resource("/net/ethernaught/rust/res/icons/ic_launcher.svg").expect("Failed to get Pixbuf from SVG");

    let dialog = AboutDialog::builder()
        .transient_for(window)
        .modal(true)
        .program_name("Ethernaught")
        .version(format!("{}-{}", env!("PROFILE"), env!("CARGO_PKG_VERSION")).as_str())
        .authors(vec!["DrBrad"])
        .website_label("https://ethernaught.net")
        .website("https://ethernaught.net")
        .comments("")
        .copyright("Copyright (c) 2024 Ethernaught")
        .license("Copyright (c) 2024 Ethernaught\r\n\r\n\
        \
        Permission is hereby granted, free of charge, to any person obtaining a copy\r\n\
        of this software and associated documentation files (the \"Software\"), to deal\r\n\
        in the Software without restriction, including without limitation the rights\r\n\
        to use, copy, modify, merge, publish, distribute, sublicense, and/or sell\r\n\
        copies of the Software, and to permit persons to whom the Software is\r\n\
        furnished to do so, subject to the following conditions:\r\n\r\n\
        \
        The above copyright notice and this permission notice shall be included in all\r\n\
        copies or substantial portions of the Software.\r\n\r\n\
        \
        THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\r\n\
        IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\r\n\
        FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\r\n\
        AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\r\n\
        LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\r\n\
        OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\r\n\
        SOFTWARE.")
        //.logo(&icon_pixbuf)
        .build();

    dialog.present();
}

