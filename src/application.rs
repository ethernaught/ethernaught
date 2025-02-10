use gtk::{AboutDialog, ApplicationWindow, Builder, Box, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label};
use gtk::gdk_pixbuf::PixbufLoader;
use gtk::prelude::*;
use gtk::gio::SimpleAction;
use gtk::prelude::{ActionMapExt, GtkWindowExt};
use crate::PacketType;
//use crate::config::VERSION;

pub fn init_actions(app: &Application, window: &ApplicationWindow) {
    /*
    let action = SimpleAction::new("quit", None);
    let app_clone = app.clone();
    action.connect_activate(move |_, _| {
        app_clone.quit();
    });
    window.add_action(&action);

    let action = SimpleAction::new("show-about-dialog", None);
    let window_clone = window.clone();
    action.connect_activate(move |_, _| {
        show_about(&window_clone);
    });
    window.add_action(&action);
    */
}


pub fn init_titlebar(window: &ApplicationWindow, app: &Application) -> Builder {
    let builder = Builder::from_file("res/ui/titlebar-ui.xml");

    let titlebar: gtk::Box = builder
        .object("titlebar")
        .expect("Couldn't find 'titlebar' in titlebar-ui.xml");

    window.set_titlebar(Some(&titlebar));
    titlebar.set_size_request(-1, 32);


    let minimize_button: Button = builder
        .object("minimize_button")
        .expect("Couldn't find 'minimize_button' in titlebar-ui.xml");

    let window_clone = window.clone();
    minimize_button.connect_clicked(move |_| {
        window_clone.iconify();
    });

    let maximize_button: Button = builder
        .object("maximize_button")
        .expect("Couldn't find 'maximize_button' in titlebar-ui.xml");

    let window_clone = window.clone();
    maximize_button.connect_clicked(move |_| {
        if window_clone.is_maximized() {
            window_clone.unmaximize();
            return;
        }

        window_clone.maximize();
    });

    let close_button: Button = builder
        .object("close_button")
        .expect("Couldn't find 'close_button' in titlebar-ui.xml");

    let app_clone = app.clone();
    close_button.connect_clicked(move |_| {
        app_clone.quit();
    });

    builder
}


pub fn create_row(packet_type: PacketType) -> ListBoxRow {
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

pub fn show_about(window: &ApplicationWindow) {
    /*
    let svg_data = include_bytes!("../res/images/ic_launcher.svg");
    let loader = PixbufLoader::with_type("svg").expect("Failed to create SVG loader");
    loader.write(svg_data).expect("Failed to load SVG data");
    loader.close().expect("Failed to close SVG loader");
    let icon_pixbuf = loader.pixbuf().expect("Failed to get Pixbuf from SVG");

    let dialog = AboutDialog::builder()
        .transient_for(window)
        .modal(true)
        .program_name("Omniscient")
        .version(VERSION)
        .authors(vec!["DrBrad"])
        .website_label("https://omniscient.com")
        .website("https://omniscient.com")
        .comments("")
        .copyright("Copyright (c) 2024 Omniscient")
        .license("Copyright (c) 2024 Omniscient\r\n\r\n\
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
        .logo(&icon_pixbuf)
        .build();

    dialog.present();
    */
}