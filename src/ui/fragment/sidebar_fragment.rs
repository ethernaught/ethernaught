use std::any::Any;
use std::cell::Cell;
use std::rc::Rc;
use gtk::{gdk, Builder, Button, Container, EventBox, Expander, Label, ListBox, ListBoxRow, Orientation, Paned, TextTag, TextView};
use gtk::gdk::EventMask;
use gtk::glib::{clone, Propagation};
use gtk::prelude::{BuilderExtManual, ButtonExt, Cast, ContainerExt, LabelExt, PanedExt, StyleContextExt, TextBufferExt, TextTagExt, TextTagTableExt, TextViewExt, WidgetExt, WidgetExtManual};
use pcap::packet::inter::interfaces::Interfaces;
use pcap::packet::layers::inter::layer::Layer;
use pcap::packet::layers::layer_1::ethernet_layer::EthernetLayer;
use pcap::packet::layers::layer_1::inter::types::Types;
use pcap::packet::layers::layer_2::ethernet::inter::protocols::Protocols;
use pcap::packet::layers::layer_2::ethernet::ipv4_layer::IPv4Layer;
use pcap::packet::layers::layer_2::ethernet::ipv6_layer::IPv6Layer;
use pcap::packet::layers::layer_3::ip::udp_layer::UdpLayer;
use pcap::packet::packet::Packet;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::fragment::inter::fragment::Fragment;
use crate::ui::handlers::expanders::{create_ethernet_layer_expander, create_ipv4_layer_expander, create_udp_layer_expander};

#[derive(Clone)]
pub struct SidebarFragment {
    activity: Box<dyn Activity>,
    root: Option<Container>,
    packet: Packet
}

impl SidebarFragment {

    pub fn new(activity: Box<dyn Activity>, packet: Packet) -> Self {
        Self {
            activity,
            root: None,
            packet
        }
    }
}

impl Fragment for SidebarFragment {

    fn on_create(&mut self) -> &Container {
        let builder = Builder::from_file("res/ui/gtk3/sidebar-fragment.ui");

        self.root = Some(builder
            .object("sidebar_layout")
            .expect("Couldn't find 'sidebar_layout' in window.ui"));


        let dismiss_button: Button = builder
            .object("dismiss_button")
            .expect("Couldn't find 'dismiss_button' in window.ui");

        let _self = self.clone();
        dismiss_button.connect_clicked(move |_| {
            let main_activity = _self.activity.as_any().downcast_ref::<MainActivity>().unwrap();
            main_activity.close_sidebar();
        });




        let details_layout: gtk::Box = builder
            .object("details_layout")
            .expect("Couldn't find 'details_layout' in window.ui");


        match self.packet.get_interface() {
            Interfaces::Ethernet => {
                let ethernet_layer = self.packet.get_layer(0).unwrap().as_any().downcast_ref::<EthernetLayer>().unwrap();
                details_layout.add(&create_ethernet_layer_expander(&ethernet_layer));

                match ethernet_layer.get_type() {
                    Types::IPv4 => {
                        let ipv4_layer = self.packet.get_layer(1).unwrap().as_any().downcast_ref::<IPv4Layer>().unwrap();
                        details_layout.add(&create_ipv4_layer_expander(&ipv4_layer));

                        match ipv4_layer.get_protocol() {
                            Protocols::Icmp => {}
                            Protocols::Igmp => {}
                            Protocols::Tcp => {}
                            Protocols::Udp => {
                                let udp_layer = self.packet.get_layer(2).unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();
                                details_layout.add(&create_udp_layer_expander(&udp_layer));
                            }
                            Protocols::Ipv6 => {}
                            Protocols::Gre => {}
                            Protocols::Icmpv6 => {}
                            Protocols::Ospf => {}
                            Protocols::Sps => {}
                        }
                    }
                    Types::Arp => {
                    }
                    Types::IPv6 => {
                        let ipv6_layer = self.packet.get_layer(1).unwrap().as_any().downcast_ref::<IPv6Layer>().unwrap();
                        //details_layout.add(&create_ethernet_layer_expander(&ethernet_layer));

                        match ipv6_layer.get_next_header() {
                            Protocols::Icmp => {}
                            Protocols::Igmp => {}
                            Protocols::Tcp => {}
                            Protocols::Udp => {
                                let udp_layer = self.packet.get_layer(2).unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();
                                details_layout.add(&create_udp_layer_expander(&udp_layer));
                            }
                            Protocols::Ipv6 => {}
                            Protocols::Gre => {}
                            Protocols::Icmpv6 => {}
                            Protocols::Ospf => {}
                            Protocols::Sps => {}
                        }
                    }
                    Types::Broadcast => {
                    }
                }

            }
            Interfaces::WiFi => {
                //"[WiFi] TODO".to_string()
            }
            Interfaces::Bluetooth => {
                //"[Bluetooth] TODO".to_string()
            }
        };















        println!("{:?}", self.packet);



        /*
        let hex_data = self.packet.to_bytes();


        let line_numbers: TextView = builder.object("hex_line_numbers").unwrap();
        line_numbers.set_cursor_visible(false);
        let hex_text_view: TextView = builder.object("hex_text_view").unwrap();
        hex_text_view.set_cursor_visible(false);
        let ascii_text_view: TextView = builder.object("ascii_text_view").unwrap();
        ascii_text_view.set_cursor_visible(false);


        let mut off = 0;
        let points: Vec<usize> = self.packet.get_layers().iter().map(|layer| {
            off += layer.len();
            off
        }).collect();

        println!("{:?}", points);




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







        let hex_buffer = hex_text_view.buffer().unwrap();
        let ascii_buffer = ascii_text_view.buffer().unwrap();

        ascii_text_view.set_events(EventMask::POINTER_MOTION_MASK);

        let hex_hover_tag = TextTag::builder()
            .name("hover_char")
            .background("#59436e") // Highlight with yellow background
            .build();
        hex_buffer.tag_table().unwrap().add(&hex_hover_tag);

        let ascii_hover_tag = TextTag::builder()
            .name("hover_char")
            .background("#59436e") // Highlight with yellow background
            .build();
        ascii_buffer.tag_table().unwrap().add(&ascii_hover_tag);

        let previous_char_offset = Rc::new(Cell::new(None));
        let points_clone = points.clone();

        ascii_text_view.connect_motion_notify_event({
            let previous_char_offset = previous_char_offset.clone();
            move |text_view, event| {
                let (mouse_x, mouse_y) = event.position();

                let mouse_x = mouse_x-10 as f64;
                let mouse_y = mouse_y-10 as f64;

                //let buffer = text_view.buffer().unwrap();

                if let Some(iter) = text_view.iter_at_location(mouse_x as i32, mouse_y as i32) {
                    let char_offset = iter.offset();

                    let range_start = points_clone.iter().enumerate().find(|(_, &point)| (char_offset as usize) < point).map(|(index, &point)| {
                        if index > 0 { points_clone[index - 1] } else { 0 }
                    }).unwrap_or(0) as i32;

                    let range_end = points_clone.iter().enumerate().find(|(_, &point)| (char_offset as usize) < point).map(|(_, &point)| point)
                        .unwrap_or(*points_clone.last().unwrap()) as i32;

                    if previous_char_offset.get() == Some(range_end) {
                        return Propagation::Proceed;
                    }

                    if let Some(prev_offset) = previous_char_offset.get() {
                        let prev_iter = hex_buffer.iter_at_offset(prev_offset);
                        hex_buffer.remove_tag(&hex_hover_tag, &hex_buffer.iter_at_offset(0), &prev_iter);

                        let prev_iter = ascii_buffer.iter_at_offset(prev_offset);
                        ascii_buffer.remove_tag(&ascii_hover_tag, &ascii_buffer.iter_at_offset(0), &prev_iter);
                    }

                    let start_iter = hex_buffer.iter_at_offset(range_start);
                    let end_iter = hex_buffer.iter_at_offset(range_end);

                    hex_buffer.apply_tag(&hex_hover_tag, &start_iter, &end_iter);

                    let start_iter = ascii_buffer.iter_at_offset(range_start);
                    let end_iter = ascii_buffer.iter_at_offset(range_end);
                    ascii_buffer.apply_tag(&ascii_hover_tag, &start_iter, &end_iter);

                    previous_char_offset.set(Some(range_end));
                }

                Propagation::Proceed
            }
        });
        */







        /*
        let buffer = hex_text_view.buffer().unwrap();

        hex_text_view.set_events(EventMask::POINTER_MOTION_MASK);

        let hover_tag = TextTag::builder()
            .name("hover_char")
            .background("#59436e") // Highlight with yellow background
            .build();
        buffer.tag_table().unwrap().add(&hover_tag);

        let previous_char_offset = Rc::new(Cell::new(None));
        let mut points_clone = points.clone();

        hex_text_view.connect_motion_notify_event({
            let previous_char_offset = previous_char_offset.clone();
            move |text_view, event| {
                let (mouse_x, mouse_y) = event.position();

                let mouse_x = mouse_x-10 as f64;
                let mouse_y = mouse_y-10 as f64;

                let buffer = text_view.buffer().unwrap();

                if let Some(iter) = text_view.iter_at_location(mouse_x as i32, mouse_y as i32) {
                    let char_offset = iter.offset();

                    let range_start = points_clone.iter().enumerate().find(|(_, &point)| (char_offset as usize) < point * 3)
                        .map(|(index, &point)| if index > 0 { points_clone[index - 1] * 3 } else { 0 })
                        .unwrap_or(0) as i32;

                    let range_end = points_clone.iter().enumerate().find(|(_, &point)| (char_offset as usize) < point * 3)
                        .map(|(_, &point)| point * 3)
                        .unwrap_or(*points_clone.last().unwrap() * 3) as i32;

                    if previous_char_offset.get() == Some(range_end) {
                        return Propagation::Proceed;
                    }

                    if let Some(prev_offset) = previous_char_offset.get() {
                        let prev_iter = buffer.iter_at_offset(prev_offset);
                        buffer.remove_tag(&hover_tag, &buffer.iter_at_offset(0), &prev_iter);
                    }

                    let start_iter = buffer.iter_at_offset(range_start);
                    let end_iter = buffer.iter_at_offset(range_end);

                    buffer.apply_tag(&hover_tag, &start_iter, &end_iter);

                    previous_char_offset.set(Some(range_end));
                }

                Propagation::Proceed
            }
        });
        */



        &self.root.as_ref().unwrap().upcast_ref()
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

    fn get_activity(&self) -> &Box<dyn Activity> {
        &self.activity
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Fragment> {
        Box::new(self.clone())
    }
}
