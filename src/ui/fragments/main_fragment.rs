use std::time::Duration;
use gtk::prelude::*;
use gtk::{gdk, glib, Adjustment, Application, ApplicationWindow, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Paned, ScrolledWindow, Stack, StyleContext, TextTag, TextView};
use gtk::ffi::GtkPaned;
use gtk::gdk::EventMask;
use gtk::glib::ControlFlow::Continue;
use gtk::glib::Propagation;
use crate::packet::inter::interfaces::Interfaces;
use crate::packet::layers::layer_1::ethernet_layer::EthernetLayer;
use crate::packet::layers::layer_1::inter::types::Types;
use crate::packet::layers::layer_2::ethernet::ipv4_layer::IPv4Layer;
use crate::packet::layers::layer_2::ethernet::ipv6_layer::IPv6Layer;
use crate::packet::packet::Packet;
use crate::ui::fragments::inter::fragment::Fragment;

pub struct MainFragment {
    builder: Builder,
    root: Option<Paned>
}

impl MainFragment {

    pub fn new() -> Self {
        let builder = Builder::from_file("res/ui/gtk3/main-fragment.ui");

        Self {
            builder,
            root: None
        }
    }
}

impl Fragment for MainFragment {

    fn on_create(&mut self) -> &Paned {
        self.root = Some(self.builder
            .object("window_layout")
            .expect("Couldn't find 'window_layout' in main-fragment.ui"));

        let content_layout: gtk::Box = self.builder
            .object("content_layout")
            .expect("Couldn't find 'content_layout' in window.ui");
        self.root.as_ref().unwrap().set_child_shrink(&content_layout, false);
        self.root.as_ref().unwrap().set_child_resize(&content_layout, true);

        let sidebar_layout: gtk::Box = self.builder
            .object("sidebar_layout")
            .expect("Couldn't find 'sidebar_layout' in window.ui");
        self.root.as_ref().unwrap().set_child_shrink(&sidebar_layout, false);






        /*
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
        */







        let hadjustment = Adjustment::new(0.0, 0.0, 1000.0, 10.0, 100.0, 100.0);
        let vadjustment = Adjustment::new(0.0, 0.0, 1000.0, 10.0, 100.0, 100.0);

        let list_header_scroll_layout: ScrolledWindow = self.builder
            .object("list_header_scroll_layout")
            .expect("Couldn't find 'list_header_scroll_layout' in window.ui");
        list_header_scroll_layout.set_hadjustment(Some(&hadjustment));
        list_header_scroll_layout.set_vadjustment(None::<&Adjustment>);

        let list_scroll_layout: ScrolledWindow = self.builder
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


        let line_numbers: TextView = self.builder.object("hex_line_numbers").unwrap();
        let hex_text_view: TextView = self.builder.object("hex_text_view").unwrap();
        let ascii_text_view: TextView = self.builder.object("ascii_text_view").unwrap();

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


        // Enable Mouse Motion Events
        ascii_text_view.set_events(EventMask::POINTER_MOTION_MASK);

        // Create Hover Tag (Initially Invisible)
        let hover_tag = TextTag::builder()
            .name("hover_char")
            .background("#59436e") // Highlight with yellow background
            .build();
        buffer.tag_table().unwrap().add(&hover_tag);

        // Track Previously Hovered Character
        let previous_char_offset = std::rc::Rc::new(std::cell::Cell::new(None));

        // Connect Mouse Hover Event
        ascii_text_view.connect_motion_notify_event({
            let previous_char_offset = previous_char_offset.clone();
            move |text_view, event| {
                let (mouse_x, mouse_y) = event.position();

                let mouse_x = mouse_x-10 as f64;
                let mouse_y = mouse_y-10 as f64;

                let buffer = text_view.buffer().unwrap();

                if let Some(iter) = text_view.iter_at_location(mouse_x as i32, mouse_y as i32) {
                    let char_offset = iter.offset();

                    // If we're still hovering the same character, do nothing
                    if previous_char_offset.get() == Some(char_offset) {
                        return Propagation::Proceed;
                    }

                    // Remove the tag from the previously highlighted character
                    if let Some(prev_offset) = previous_char_offset.get() {
                        let prev_iter = buffer.iter_at_offset(prev_offset);
                        let mut next_iter = prev_iter.clone();
                        next_iter.forward_char();
                        buffer.remove_tag(&hover_tag, &prev_iter, &next_iter);
                    }

                    // Apply the tag to the new character
                    let mut next_iter = iter.clone();
                    next_iter.forward_char(); // Move one char forward
                    buffer.apply_tag(&hover_tag, &iter, &next_iter);

                    // Update the previously hovered character
                    previous_char_offset.set(Some(char_offset));
                }

                Propagation::Proceed
            }
        });





        /*

        let (tx, rx) = channel();

        let tx = Arc::new(Mutex::new(tx));

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
        });*/

        &self.root.as_ref().unwrap()
    }

    fn on_resume(&self) {
        todo!()
    }

    fn on_pause(&self) {
        todo!()
    }

    fn on_destroy(&self) {
        todo!()
    }
}


pub fn create_row(number: u32, packet: Packet) -> ListBoxRow {
    let builder = Builder::from_file("res/ui/list_item.xml");
    let row: ListBoxRow = builder
        .object("row")
        .expect("Couldn't find 'row' in list_item.xml");

    //row.style_context().add_class(&packet.get_type().to_string());



    let number_label: Label = builder
        .object("number")
        .expect("Couldn't find 'number' in list_item.xml");
    number_label.set_label(format!("{}", number).as_str());

    let time_label: Label = builder
        .object("time")
        .expect("Couldn't find 'time' in list_item.xml");
    time_label.set_label(format!("{:.5}", packet.get_frame_time()).as_str());

    let source_label: Label = builder
        .object("source")
        .expect("Couldn't find 'source' in list_item.xml");

    let destination_label: Label = builder
        .object("destination")
        .expect("Couldn't find 'destination' in list_item.xml");

    let protocol_label: Label = builder
        .object("protocol")
        .expect("Couldn't find 'protocol' in list_item.xml");
    //protocol_label.set_label(&packet.get_type().to_string());

    let length_label: Label = builder
        .object("length")
        .expect("Couldn't find 'length' in list_item.xml");
    length_label.set_label(format!("{}", packet.len()).as_str());

    let info_label: Label = builder
        .object("info")
        .expect("Couldn't find 'info' in list_item.xml");

    let protocol = match packet.get_interface() {
        Interfaces::Ethernet => {
            let ethernet_layer = packet.get_layer(0).unwrap().as_any().downcast_ref::<EthernetLayer>().unwrap();

            match ethernet_layer.get_type() {
                Types::IPv4 => {
                    let ipv4_layer = packet.get_layer(1).unwrap().as_any().downcast_ref::<IPv4Layer>().unwrap();

                    source_label.set_label(&ipv4_layer.get_source_ip().to_string());
                    destination_label.set_label(&ipv4_layer.get_destination_ip().to_string());

                    ipv4_layer.get_protocol().to_string()
                }
                Types::Arp => {
                    source_label.set_label(&ethernet_layer.get_source().to_string());
                    destination_label.set_label(&ethernet_layer.get_destination().to_string());
                    ethernet_layer.get_type().to_string()
                }
                Types::IPv6 => {
                    let ipv6_layer = packet.get_layer(1).unwrap().as_any().downcast_ref::<IPv6Layer>().unwrap();

                    source_label.set_label(&ipv6_layer.get_source_ip().to_string());
                    destination_label.set_label(&ipv6_layer.get_destination_ip().to_string());

                    ipv6_layer.get_next_header().to_string()
                }
                Types::Broadcast => {
                    source_label.set_label(&ethernet_layer.get_source().to_string());
                    destination_label.set_label(&ethernet_layer.get_destination().to_string());
                    ethernet_layer.get_type().to_string()
                }
                _ => {
                    ethernet_layer.get_type().to_string()
                }
            }

        }
        Interfaces::WiFi => {
            "[WiFi] TODO".to_string()
        }
        Interfaces::Bluetooth => {
            "[Bluetooth] TODO".to_string()
        }
    };

    row.style_context().add_class(&protocol);
    protocol_label.set_label(&protocol);


    row
}
