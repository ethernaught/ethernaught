use gtk::{AboutDialog, ApplicationWindow, Builder, Box, Image, Application, TreeViewColumn, CellRendererText};
use gtk::gdk_pixbuf::PixbufLoader;
use gtk::prelude::*;
use gtk::gio::SimpleAction;
use gtk::prelude::{ActionMapExt, GtkWindowExt};
//use crate::config::VERSION;

pub fn init_styles(builder: &Builder) {
    //let bla_bla: CellRendererText = builder.object("bla_bla").unwrap();
    //bla_bla.set_widget_name("bla_bla");

    let header_layout: gtk::Box = builder
        .object("header_layout")
        .expect("Couldn't find 'header_layout' in window.ui");
    header_layout.set_widget_name("header_layout");


    /*
    let hex_column: TreeViewColumn = builder
        .object("hex_column")
        .expect("Couldn't find 'hex_column' in window.ui");
    hex_column.set_title("hex_column");


    let ascii_column: TreeViewColumn = builder
        .object("ascii_column")
        .expect("Couldn't find 'ascii_column' in window.ui");
    ascii_column.set_title("ascii_column");*/
}

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