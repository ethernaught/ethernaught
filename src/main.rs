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
use gtk::{Application, Builder, gio, CssProvider, StyleContext, gdk, ApplicationWindow, ListBox, ListBoxRow, Label, Orientation, ScrolledWindow, Image, ProgressBar, TreeView, ListStore, CellRendererText, TreeViewColumn, HeaderBar};
use gtk::gdk::{EventButton, EventMask};
use gtk::glib::Propagation;

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
        //window.set_decorated(false);



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
        window.set_title("Omniscient");
        window.connect_destroy(|_| exit(0));



        let list_box = ListBox::new();
        //for i in 0..100 {
            list_box.add(&create_row());
        //}

        let list_scroll_layout: ScrolledWindow = builder
            .object("list_scroll_layout")
            .expect("Couldn't find 'list_scroll_layout' in window.ui");

        list_scroll_layout.add(&list_box);
        list_box.show_all();




        /*
        let tree_view: TreeView = builder
            .object("hex_view")
            .expect("Couldn't find 'hex_view' in window.ui");
        let list_store = ListStore::new(&[String::static_type(), String::static_type(), String::static_type()]);
        tree_view.set_model(Some(&list_store));


        //tree_view.column(0).unwrap().style_context().add_class("offset_column");
        tree_view.column(0).unwrap().set_title("000000");

        //tree_view.column(1).unwrap().style_context().add_class("hex_column");
        tree_view.column(1).unwrap().set_title("2e f8 84 00 00 01 00 03 00 00 00 01 07 65 78 61
6d 70 6c 65 03 63 6f 6d 00 00 30 00 01 c0 0c 00
30 00 01 00 00 0e 10 00 44 01 00 03 0d c3 ef d2
96 36 59 79 f7 3b 7d 1a 58 2c 23 89 49 e3 3d 3d
17 83 9f c7 de 32 91 38 93 96 95 5b 56 0c aa d5
59 a0 e6 32 5b 06 39 66 49 85 29 31 13 6e 01 41
20 4c b5 97 c6 8a 8c 65 23 1d c1 30 4b c0 0c 00
30 00 01 00 00 0e 10 00 44 01 01 03 0d 91 72 a4
bd 65 37 bc 66 1f 4c 91 a5 de a0 5d e2 a8 62 5a
9e 5a 46 ce d8 b6 40 89 c4 3d 9d fa de ca 5e ac
1a 87 0c 39 22 02 6d c4 94 f6 c8 52 2d 96 08 1a
cf 27 d7 a8 91 15 3a 63 09 de a4 f4 b5 c0 0c 00
2e 00 01 00 00 0e 10 00 5f 00 30 0d 02 00 00 0e
10 67 b3 89 f7 67 98 06 7c 01 72 07 65 78 61 6d
70 6c 65 03 63 6f 6d 00 60 09 73 6b 8b 1a b9 18
46 27 1f 97 e3 a8 e1 59 e3 b0 a7 e4 05 04 29 99
03 56 09 0a 65 86 ea 25 c2 d7 9f 60 77 74 85 81
fb b3 65 4b ca 55 7b 72 18 47 ce 0f ad eb aa 22
1b b8 ee c3 ca fc ad bf 00 00 29 10 00 00 00 80
00 00 00");

        //tree_view.column(2).unwrap().style_context().add_class("ascii_column");
        tree_view.column(2).unwrap().set_title("HELLO");

        */

        /*
        let window_layout: gtk::Box = builder
            .object("window_layout")
            .expect("Couldn't find 'window_layout' in window.ui");*/

        /*

        let builder = Builder::from_file("res/ui/omniscient-ui-2.xml");
        let menu_layout: gtk::HeaderBar = builder
            .object("menu_layout")
            .expect("Couldn't find 'menu_layout' in window.ui");

        //window_layout.pack_end(&menu_layout, false, true, 0);

        //let header_bar = HeaderBar::new();

        menu_layout.show_all();
        window.add(&list_box);
        window.set_titlebar(Some(&menu_layout));*/

        /*
        let builder = Builder::from_file("res/ui/omniscient-ui.xml");
        let menubar: gio::Menu = builder
            .object("main_window_menu")
            .expect("Couldn't find 'main_window_menu' in omniscient-ui.xml");

        app.set_menubar(Some(&menubar));*/

        //init_actions(&app, &window);

        window.show_all();
    });

    app.run();
}








fn create_row() -> ListBoxRow {
    let builder = Builder::from_file("res/ui/list_item.xml");
    let row: ListBoxRow = builder
        .object("row")
        .expect("Couldn't find 'row' in list_item.xml");

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



