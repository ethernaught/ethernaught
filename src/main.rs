use pcap::{Capture, Device};

//fn main() {
    /*
    let devices = Device::list().expect("Failed to get device list");

    let device = devices.into_iter().find(|d| d.name.contains("wlp2s0"))
        .expect("No suitable device found");

    println!("Listening on device: {}", device.name);

    let mut cap = Capture::from_device(device)
        .expect("Failed to open device")
        .promisc(true)
        .immediate_mode(true)
        .open()
        .expect("Failed to start capture");

    while let Ok(packet) = cap.next_packet() {
        println!("Captured packet: {:?} ({} bytes)", packet, packet.data.len());
    }
    */
//}

mod application;

use std::process::exit;
use gtk::prelude::*;
use gtk::{Application, Builder, gio, CssProvider, StyleContext, gdk, ApplicationWindow, ListBox, ListBoxRow, Label, Orientation, ScrolledWindow, Image, ProgressBar, TreeView, ListStore, CellRendererText, TreeViewColumn, HeaderBar, Toolbar, Button};
use gtk::gdk::{EventButton, EventMask};
use gtk::glib::Propagation;
use crate::application::init_actions;

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

        /*
        let titlebar: gtk::Box = builder
            .object("titlebar")
            .expect("Couldn't find 'titlebar' in window.ui");

        window.set_titlebar(Some(&titlebar));
        //window.set_hide_titlebar_when_maximized(false);*/


        let titlebar_builder = Builder::from_file("res/ui/titlebar-ui.xml");

        let titlebar: gtk::Box = titlebar_builder
            .object("titlebar")
            .expect("Couldn't find 'titlebar' in window.ui");

        window.set_titlebar(Some(&titlebar));
        titlebar.set_size_request(-1, 32);


        let minimize_button: Button = titlebar_builder
            .object("minimize_button")
            .expect("Couldn't find 'minimize_button' in window.ui");

        let window_clone = window.clone();
        minimize_button.connect_clicked(move |_| {
            window_clone.iconify();
        });

        let maximize_button: Button = titlebar_builder
            .object("maximize_button")
            .expect("Couldn't find 'maximize_button' in window.ui");

        let window_clone = window.clone();
        maximize_button.connect_clicked(move |_| {
            if window_clone.is_maximized() {
                window_clone.unmaximize();
                return;
            }

            window_clone.maximize();
        });

        let close_button: Button = titlebar_builder
            .object("close_button")
            .expect("Couldn't find 'close_button' in window.ui");

        let app_clone = app.clone();
        close_button.connect_clicked(move |_| {
            app_clone.quit();
        });


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













        let list_box = ListBox::new();
        //for i in 0..100 {
            //list_box.add(&create_row());
        //}
        list_box.add(&create_row(PacketType::Tcp));
        list_box.add(&create_row(PacketType::Udp));
        list_box.add(&create_row(PacketType::Icmp));
        list_box.add(&create_row(PacketType::Gre));

        let list_scroll_layout: ScrolledWindow = builder
            .object("list_scroll_layout")
            .expect("Couldn't find 'list_scroll_layout' in window.ui");

        list_scroll_layout.add(&list_box);
        list_box.show_all();

        /*
        let builder = Builder::from_file("res/ui/omniscient-ui.xml");
        let menubar: gio::Menu = builder
            .object("main_window_menu")
            .expect("Couldn't find 'main_window_menu' in omniscient-ui.xml");

        app.set_menubar(Some(&menubar));
        */

        init_actions(&app, &window);

        window.show_all();
    });

    app.run();
}






pub enum PacketType {
    Tcp,
    Udp,
    Icmp,
    Gre
}


fn create_row(packet_type: PacketType) -> ListBoxRow {
    let builder = Builder::from_file("res/ui/list_item.xml");
    let row: ListBoxRow = builder
        .object("row")
        .expect("Couldn't find 'row' in list_item.xml");

    match packet_type {
        PacketType::Tcp => {
            row.style_context().add_class("tcp");
        }
        PacketType::Udp => {
            row.style_context().add_class("udp");
        }
        PacketType::Icmp => {
            row.style_context().add_class("icmp");
        }
        PacketType::Gre => {
            row.style_context().add_class("gre");
        }
    }

    let number: Label = builder
        .object("number")
        .expect("Couldn't find 'number' in list_item.xml");
    number.set_label("216");

    let time: Label = builder
        .object("time")
        .expect("Couldn't find 'time' in list_item.xml");
    time.set_label("1.617305868");

    let source: Label = builder
        .object("source")
        .expect("Couldn't find 'source' in list_item.xml");
    source.set_label("192.168.0.1");

    let destination: Label = builder
        .object("destination")
        .expect("Couldn't find 'destination' in list_item.xml");
    destination.set_label("192.168.0.1");

    let protocol: Label = builder
        .object("protocol")
        .expect("Couldn't find 'protocol' in list_item.xml");
    protocol.set_label("DNS");

    let length: Label = builder
        .object("length")
        .expect("Couldn't find 'length' in list_item.xml");
    length.set_label("105");

    let info: Label = builder
        .object("info")
        .expect("Couldn't find 'info' in list_item.xml");
    info.set_label("Standard query response 0x39bc A");

    row
}



