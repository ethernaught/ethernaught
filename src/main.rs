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
use gtk::{Application, Builder, gio, CssProvider, StyleContext, gdk, ApplicationWindow, ListBox, ListBoxRow, Label, Orientation, ScrolledWindow, Image, ProgressBar, TreeView, ListStore, CellRendererText, TreeViewColumn, HeaderBar, Toolbar, Button, glib, StackSwitcher, Stack, Paned, TextView, TextBuffer, Adjustment, Grid, TextTag};
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

        let content_layout: gtk::Box = builder
            .object("content_layout")
            .expect("Couldn't find 'content_layout' in window.ui");
        window_layout.set_child_shrink(&content_layout, false);
        window_layout.set_child_resize(&content_layout, true);

        let sidebar_layout: gtk::Box = builder
            .object("sidebar_layout")
            .expect("Couldn't find 'sidebar_layout' in window.ui");
        window_layout.set_child_shrink(&sidebar_layout, false);











        let hadjustment = Adjustment::new(0.0, 0.0, 1000.0, 10.0, 100.0, 100.0);
        let vadjustment = Adjustment::new(0.0, 0.0, 1000.0, 10.0, 100.0, 100.0);

        let list_header_scroll_layout: ScrolledWindow = builder
            .object("list_header_scroll_layout")
            .expect("Couldn't find 'list_header_scroll_layout' in window.ui");
        list_header_scroll_layout.set_hadjustment(Some(&hadjustment));
        list_header_scroll_layout.set_vadjustment(None::<&Adjustment>);

        let list_scroll_layout: ScrolledWindow = builder
            .object("list_scroll_layout")
            .expect("Couldn't find 'list_scroll_layout' in window.ui");

        list_scroll_layout.set_hadjustment(Some(&hadjustment));
        list_scroll_layout.set_vadjustment(Some(&vadjustment));

        let list_box = ListBox::new();
        list_scroll_layout.add(&list_box);
        list_box.show_all();









        let hex_data = vec![
            0x3c, 0x52, 0xa1, 0x12, 0xa4, 0x50, 0x1c, 0xce,
            0x51, 0x34, 0x00, 0x9f, 0x08, 0x00, 0x45, 0x00,
            0x00, 0x3f, 0xe5, 0xa4, 0x40, 0x00, 0x40, 0x11,
            0x79, 0x9d, 0xc0, 0xa8, 0x00, 0x81, 0x34, 0x60,
            0xe5, 0xe2, 0xb8, 0x04, 0x01, 0xbb, 0x00, 0x2b,
            0xd1, 0xd5, 0x44, 0x39, 0x34, 0x60, 0xe5, 0xe2,
            0x52, 0x60, 0x99, 0x2f, 0x68, 0xd0, 0x12, 0x6d,
            0xe8, 0x37, 0x2b, 0x67, 0x1a, 0xa5, 0x2b, 0x0c,
            0xf6, 0x3b, 0x5e, 0xfa, 0x74, 0x80, 0xbc, 0x29,
            0xd8, 0x37, 0xbf, 0xe3, 0x5c,
        ];


        let line_numbers: TextView = builder.object("hex_line_numbers").unwrap();
        let hex_text_view: TextView = builder.object("hex_text_view").unwrap();
        let ascii_text_view: TextView = builder.object("ascii_text_view").unwrap();

        let line_numbers_string = hex_data.chunks(16)
            .enumerate()
            .map(|(i, _)| format!("{:08X}", i * 16))  // Format line numbers in hex
            .collect::<Vec<_>>()
            .join("\n");
        line_numbers.buffer().unwrap().set_text(&line_numbers_string);

        let hex_string = hex_data.chunks(16)
            .map(|chunk| chunk.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n");
        hex_text_view.buffer().unwrap().set_text(&hex_string);







        /*
        let hex_ascii_string = hex_data.chunks(16)
            .map(|chunk| {
                let hex_part = chunk.iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ");

                let ascii_part = chunk.iter()
                    .map(|&b| {
                        // Check if byte is a printable ASCII character (0x20 to 0x7E)
                        if (b >= 0x20 && b <= 0x7E) {
                            char::from_u32(b as u32).unwrap_or('.')
                        } else {
                            '.' // Non-printable characters replaced with '.'
                        }
                    })
                    .collect::<String>();

                format!("{: <47} {}", hex_part, ascii_part)
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Insert Text
        ascii_text_view.buffer().unwrap().set_text(&ascii_string);
        */


        let ascii_string = hex_data.chunks(16)
            .map(|chunk| {
                chunk.iter()
                    .map(|&b| {
                        // Check if byte is a printable ASCII character (0x20 to 0x7E)
                        if (b >= 0x20 && b <= 0x7E) {
                            // Convert byte to char using `char::from_u32()`
                            char::from_u32(b as u32).unwrap_or('.') // Fall back to '.' if invalid
                        } else {
                            '.' // Non-printable characters replaced with '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        ascii_text_view.buffer().unwrap().set_text(&ascii_string);

        // Create Tags
        let buffer = ascii_text_view.buffer().unwrap();
        let tag_table = buffer.tag_table().unwrap();

        /*
        let layer_1 = TextTag::builder().name("layer_1").background("#3f5222").build();
        let layer_2 = TextTag::builder().name("layer_2").background("#1c314a").build();
        let layer_3 = TextTag::builder().name("layer_3").background("#070c1f").build();
        tag_table.add(&layer_1);
        tag_table.add(&layer_2);
        tag_table.add(&layer_3);

        let start_iter = buffer.start_iter();
        let mut end_iter = start_iter.clone();
        end_iter.forward_chars(14);
        buffer.apply_tag(&layer_1, &start_iter, &end_iter);

        let start_iter = end_iter;
        //start_iter.forward_chars(14);
        let mut end_iter = start_iter.clone();
        end_iter.forward_chars(20);
        buffer.apply_tag(&layer_2, &start_iter, &end_iter);

        let start_iter = end_iter;
        let mut end_iter = start_iter.clone();
        end_iter.forward_chars(8);
        buffer.apply_tag(&layer_3, &start_iter, &end_iter);
        */



        /*
        //let hover_tag = layer_1;
        let hover_start = buffer.iter_at_offset(0); // "Special"
        let mut hover_end = buffer.iter_at_offset(14); // "words"

        ascii_text_view.connect_motion_notify_event(move |text_view, event| {
            let (mouse_x, mouse_y) = event.position();

            // Get text offset from coordinates
            let buffer = text_view.buffer().unwrap();
            if let Some(iter) = text_view.iter_at_location(mouse_x as i32, mouse_y as i32) {
                // Remove previous highlight
                buffer.remove_all_tags(&hover_start, &hover_end);

                // Check if the mouse is inside the word range
                if iter.offset() >= hover_start.offset() && iter.offset() <= hover_end.offset() {

                    let layer_1 = TextTag::builder().name("layer_1").background("#3f5222").build();
                    buffer.apply_tag(&layer_1, &hover_start, &hover_end);
                }
            }

            Propagation::Proceed
        });
        */

























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
