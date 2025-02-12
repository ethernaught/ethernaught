mod application;
mod pcap;
mod packet;
mod interface;

use std::process::exit;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, RecvError};
use std::thread;
use std::time::Duration;
use ::pcap::{Capture, Device};
use gtk::prelude::*;
use gtk::{Application, Builder, gio, CssProvider, StyleContext, gdk, ApplicationWindow, ListBox, ListBoxRow, Label, Orientation, ScrolledWindow, Image, ProgressBar, TreeView, ListStore, CellRendererText, TreeViewColumn, HeaderBar, Toolbar, Button, glib, StackSwitcher, Stack, Paned, TextView, TextBuffer};
use gtk::gdk::{EventButton, EventMask};
use gtk::gio::spawn_blocking;
use gtk::glib::ControlFlow::Continue;
use gtk::glib::{idle_add, Propagation};
use gtk::glib::UnicodeBreakType::Contingent;
use crate::application::{init_titlebar, create_row, init_actions};
use crate::pcap::packet_capture;

//let (tx, rx) = channel();
/*
thread_local!(
    static GLOBAL RefCell<Option<(UiModel, mpsc::Receiver<String>)>> = RefCell::new(None);
);
*/

fn main() {
    let app = Application::new(Some("com.omniscient.rust"), Default::default());

    app.connect_activate(|app| {
        let builder = Builder::from_file("res/ui/gtk3/window.ui");

        let provider = CssProvider::new();
        provider.load_from_path("res/ui/gtk3/style.css").expect("Failed to load CSS file.");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window: ApplicationWindow = builder
            .object("MainWindow")
            .expect("Failed to get the 'MainWindow' from window.ui");

        window.set_application(Some(app));
        window.connect_destroy(|_| exit(0));
        //window.set_decorated(false);
        window.set_border_width(1);

        let titlebar_builder = init_titlebar(&window, app);








        /*
        let svg_data = include_bytes!("../res/ic_launcher.svg");
        let loader = PixbufLoader::with_type("svg").expect("Failed to create SVG loader");
        loader.write(svg_data).expect("Failed to load SVG data");
        loader.close().expect("Failed to close SVG loader");
        let icon_pixbuf = loader.pixbuf().expect("Failed to get Pixbuf from SVG");

        window.set_icon(Some(&icon_pixbuf));
        */
        //window.set_icon_from_file("res/images/ic_launcher.svg").expect("Failed to load icon");

        //let window = Window::new(WindowType::Toplevel);
        //window.set_title("Omniscient");



        let stack = Stack::new();
        window.add(&stack);
        stack.show();

        //let switcher = StackSwitcher::new();
        //switcher.set_stack(Some(&stack));
        //window.add(&switcher);


        //stack.set_visible_child_name("interface_layout");


        let builder = Builder::from_file("res/ui/gtk3/application-fragment.ui");
        let window_layout: Paned = builder
            .object("window_layout")
            .expect("Couldn't find 'window_layout' in application-fragment.ui");

        stack.add_titled(&window_layout, "application_fragment", "Application");
        stack.set_visible_child_name("application_fragment");

        //window.add(&window_layout);

        let hex_view: TextView = builder
            .object("hex_view")
            .expect("Couldn't find 'hex_view' in application-fragment.ui");

        let buffer = TextBuffer::new(None::<&gtk::TextTagTable>);
        hex_view.set_buffer(Some(&buffer));

        let hex_data = vec![
            0x00, 0x00, 0x84, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x03, 0x32, 0x32, 0x36,
            0x01, 0x30, 0x03, 0x31, 0x36, 0x38, 0x03, 0x31, 0x39, 0x32, 0x07, 0x69, 0x6e, 0x2d, 0x61, 0x64,
            0x64, 0x72, 0x04, 0x61, 0x72, 0x70, 0x61, 0x00, 0x00, 0x0c, 0x80, 0x01, 0x00, 0x00, 0x00, 0x78,
            0x00, 0x0f, 0x07, 0x41, 0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x05, 0x6c, 0x6f, 0x63, 0x61, 0x6c,
            0x00, 0xc0, 0x0c, 0x00, 0x2f, 0x80, 0x01, 0x00, 0x00, 0x00, 0x78, 0x00, 0x06, 0xc0, 0x0c, 0x00,
            0x02, 0x00, 0x08
        ];

        let hex_data = hex_data.chunks(16)
            .map(|chunk| chunk.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n");

        buffer.set_text(&hex_data);







        let list_box = ListBox::new();

        let list_scroll_layout: ScrolledWindow = builder
            .object("list_scroll_layout")
            .expect("Couldn't find 'list_scroll_layout' in window.ui");

        list_scroll_layout.add(&list_box);
        list_box.show_all();



        let (tx, rx) = channel();

        let tx = Arc::new(Mutex::new(tx));


        let titlebar_app_options: gtk::Box = titlebar_builder
            .object("titlebar_app_options")
            .expect("Couldn't find 'titlebar_app_options' in titlebar-ui.xml");

        let start_button: Button = titlebar_builder
            .object("start_button")
            .expect("Couldn't find 'start_button' in titlebar-ui.xml");

        let start_icon: Image = titlebar_builder
            .object("start_icon")
            .expect("Couldn't find 'start_icon' in titlebar-ui.xml");

        let stop_button: Button = titlebar_builder
            .object("stop_button")
            .expect("Couldn't find 'stop_button' in titlebar-ui.xml");








        start_button.connect_clicked(move |_| {
            titlebar_app_options.style_context().add_class("running");
            start_icon.set_from_file(Some("res/images/ic_restart.svg"));
            stop_button.show();

            println!("Start button clicked!");
            packet_capture(tx.clone());
        });


        let mut i = 0;

        glib::timeout_add_local(Duration::from_millis(10), move || {
            match rx.try_recv() {
                Ok(packet) => {
                    i += 1;

                    let row = create_row(i, packet);
                    list_box.add(&row);
                    row.show_all();
                }
                _ => {
                }
            }
            Continue
        });






        init_actions(&app, &window);





        window.show();

    });

    app.run();
}
