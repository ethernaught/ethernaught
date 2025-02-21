use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Builder, Button, Container, DrawingArea};
use gtk::gdk::EventMask;
use gtk::glib::{clone, Propagation};
use gtk::prelude::{BuilderExtManual, ButtonExt, Cast, ContainerExt, PanedExt, WidgetExt, WidgetExtManual};
use pcap::packet::inter::interfaces::Interfaces;
use pcap::packet::layers::inter::layer::Layer;
use pcap::packet::layers::layer_1::ethernet_layer::EthernetLayer;
use pcap::packet::layers::layer_1::inter::types::Types;
use pcap::packet::layers::layer_2::ethernet::inter::protocols::Protocols;
use pcap::packet::layers::layer_2::ethernet::ipv4_layer::IPv4Layer;
use pcap::packet::layers::layer_2::ethernet::ipv6_layer::IPv6Layer;
use pcap::packet::layers::layer_3::ip::tcp_layer::TcpLayer;
use pcap::packet::layers::layer_3::ip::udp_layer::UdpLayer;
use pcap::packet::packet::Packet;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::fragment::inter::fragment::Fragment;
use crate::ui::handlers::expanders::{create_ethernet_layer_expander, create_ipv4_layer_expander, create_ipv6_layer_expander, create_tcp_layer_expander, create_udp_layer_expander};
use crate::ui::widgets::hex_editor::HexEditor;

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




        let hex_content: gtk::Box = builder
            .object("hex_content")
            .expect("Couldn't find 'hex_content' in window.ui");

        let mut editor = Rc::new(RefCell::new(HexEditor::new(self.packet.to_bytes())));
        editor.borrow_mut().set_line_number_color(0.286, 0.306, 0.341);
        editor.borrow_mut().set_cursor_color(0.608, 0.616, 0.624);
        editor.borrow_mut().set_selection_color(0.349, 0.263, 0.431);
        editor.borrow_mut().set_text_color(0.608, 0.616, 0.624);

        //editor.borrow_mut().set_selection(0, 14);

        let drawing_area = DrawingArea::new();
        drawing_area.set_widget_name("hex_editor");
        let (width, height) = editor.borrow_mut().content_size();
        drawing_area.set_size_request(width, height);
        drawing_area.set_hexpand(true);
        drawing_area.set_vexpand(true);
        drawing_area.show();

        drawing_area.add_events(EventMask::POINTER_MOTION_MASK);

        let editor_clone = Rc::clone(&editor);
        let drawing_clone = drawing_area.clone();
        drawing_area.connect_motion_notify_event(move |_, event| {
            let (x, y) = event.position();
            editor_clone.borrow_mut().update_cursor(x, y);
            drawing_clone.queue_draw();
            Propagation::Proceed
        });

        let editor_clone = Rc::clone(&editor);
        drawing_area.connect_draw(clone!(@strong editor => move |_, cr| {
            editor_clone.borrow_mut().draw_hex_editor(cr);
            Propagation::Proceed
        }));

        hex_content.add(&drawing_area);


















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
                            Protocols::Tcp => {
                                let tcp_layer = self.packet.get_layer(2).unwrap().as_any().downcast_ref::<TcpLayer>().unwrap();
                                details_layout.add(&create_tcp_layer_expander(&tcp_layer));
                            }
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
                        details_layout.add(&create_ipv6_layer_expander(&ipv6_layer));

                        match ipv6_layer.get_next_header() {
                            Protocols::Icmp => {}
                            Protocols::Igmp => {}
                            Protocols::Tcp => {
                                let tcp_layer = self.packet.get_layer(2).unwrap().as_any().downcast_ref::<TcpLayer>().unwrap();
                                details_layout.add(&create_tcp_layer_expander(&tcp_layer));
                            }
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

        let mut off = 0;
        let points: Vec<usize> = self.packet.get_layers().iter().map(|layer| {
            off += layer.len();
            off
        }).collect();

        println!("{:?}", points);
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
