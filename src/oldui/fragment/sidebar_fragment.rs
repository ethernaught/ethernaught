use std::any::Any;
use std::cell::RefCell;
use std::net::IpAddr;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use gtk::{Builder, Button, Container, DrawingArea, Paned, ScrolledWindow};
use gtk::ffi::GtkScrolledWindow;
use gtk::gdk::{EventMask, RGBA};
use gtk::gio::{SimpleAction, SimpleActionGroup};
use gtk::glib::{clone, Propagation};
use gtk::prelude::{ActionMapExt, BuilderExtManual, ButtonExt, Cast, ContainerExt, PanedExt, SocketExtManual, WidgetExt, WidgetExtManual};
use pcap::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use pcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use pcap::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use pcap::packet::layers::ethernet_frame::ip::icmp::icmp_layer::IcmpLayer;
use pcap::packet::layers::ethernet_frame::ip::icmpv6::icmpv6_layer::Icmpv6Layer;
use pcap::packet::layers::ethernet_frame::ip::inter::ip_protocols::IpProtocols;
use pcap::packet::layers::ethernet_frame::ip::inter::ip_versions::IpVersions;
use pcap::packet::layers::ethernet_frame::ip::ipv4_layer::Ipv4Layer;
use pcap::packet::layers::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use pcap::packet::layers::ethernet_frame::ip::tcp::tcp_layer::TcpLayer;
use pcap::packet::layers::ethernet_frame::ip::udp::dhcp::dhcp_layer::DhcpLayer;
use pcap::packet::layers::ethernet_frame::ip::udp::inter::udp_payloads::UdpPayloads;
use pcap::packet::layers::ethernet_frame::ip::udp::inter::udp_types::UdpTypes;
use pcap::packet::layers::ethernet_frame::ip::udp::udp_layer::UdpLayer;
use pcap::packet::layers::inter::layer::Layer;
use pcap::packet::layers::loop_frame::inter::loop_types::LoopTypes;
use pcap::packet::layers::loop_frame::loop_frame::LoopFrame;
use pcap::packet::layers::raw_frame::raw_frame::RawFrame;
use pcap::packet::layers::sll2_frame::sll2_frame::Sll2Frame;
use pcap::packet::packet::Packet;
use pcap::utils::data_link_types::DataLinkTypes;
use crate::database::sqlite::Database;
use crate::get_lib_path;
use crate::oldui::activity::devices_activity::DevicesActivity;
use crate::oldui::activity::inter::activity::Activity;
use crate::oldui::activity::main_activity::MainActivity;
use crate::oldui::fragment::inter::fragment::Fragment;
use crate::oldui::handlers::bundle::Bundle;
use crate::oldui::handlers::expanders::{create_arp_layer_expander, create_dhcp_layer_expander, create_ethernet_layer_expander, create_icmp_layer_expander, create_icmpv6_layer_expander, create_ipv4_layer_expander, create_ipv6_layer_expander, create_sll2_layer_expander, create_tcp_layer_expander, create_udp_layer_expander};
use crate::oldui::widgets::hex_editor::HexEditor;

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

    fn on_create(&mut self, bundle: Option<Bundle>) -> &Container {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/oldui/gtk3/sidebar_fragment.oldui");

        let sidebar_content: Paned = builder
            .object("sidebar_content")
            .expect("Couldn't find 'sidebar_content' in window.oldui");

        let hex_scroll_layout: Container = builder
            .object("hex_scroll_layout")
            .expect("Couldn't find 'hex_scroll_layout' in window.oldui");

        sidebar_content.set_child_shrink(&hex_scroll_layout, false);
        sidebar_content.set_child_resize(&hex_scroll_layout, true);

        let details_scroll_layout: Container = builder
            .object("details_scroll_layout")
            .expect("Couldn't find 'details_scroll_layout' in window.oldui");

        sidebar_content.set_child_shrink(&details_scroll_layout, false);


        self.root = Some(builder
            .object("sidebar_layout")
            .expect("Couldn't find 'sidebar_layout' in window.oldui"));







        let dismiss_button: Button = builder
            .object("dismiss_button")
            .expect("Couldn't find 'dismiss_button' in window.oldui");

        dismiss_button.connect_clicked({
            let _self = self.clone();
            move |_| {
                let main_activity = _self.activity.as_any().downcast_ref::<MainActivity>().unwrap();
                main_activity.close_sidebar();
            }
        });

        let replay_button: Button = builder
            .object("replay_button")
            .expect("Couldn't find 'replay_button' in window.oldui");

        let _self = self.clone();
        replay_button.connect_clicked({
            move |_| {
                /*
                let main_activity = _self.activity.as_any().downcast_ref::<MainActivity>().unwrap();

                if let Some(capture_service) = main_activity.get_capture_service() {
                    capture_service.send(_self.packet.clone());
                }
                */
            }
        });


        let hex_editor: HexEditor = builder
            .object("hex_editor")
            .expect("Couldn't find 'hex_editor' in window.oldui");

        hex_editor.set_data(self.packet.to_bytes());
        hex_editor.set_hexpand(true);
        hex_editor.set_vexpand(true);
        hex_editor.set_line_number_color(RGBA::new(0.286, 0.306, 0.341, 1.0));
        hex_editor.set_cursor_color(RGBA::new(0.608, 0.616, 0.624, 1.0));
        hex_editor.set_selection_color(RGBA::new(0.349, 0.263, 0.431, 1.0));


        let db = Database::open(get_lib_path("database.db").to_str().unwrap()).expect("Couldn't open database.db");


        let actions = SimpleActionGroup::new();

        let action = SimpleAction::new("open-new-window", None);
        action.connect_activate({
            let context = self.activity.get_context().clone();
            move |_, _| {
                //context.create_window_from_activity(Box::new(DevicesActivity::new(context.clone())), None);
            }
        });
        actions.add_action(&action);

        let details_layout: gtk::Box = builder
            .object("details_layout")
            .expect("Couldn't find 'details_layout' in window.oldui");


        match self.packet.get_data_link_type() {
            DataLinkTypes::En10mb => {
                let ethernet_frame = self.packet.get_frame().as_any().downcast_ref::<EthernetFrame>().unwrap();
                details_layout.add(&create_ethernet_layer_expander(&db, 0, &hex_editor, &actions, &ethernet_frame));

                match ethernet_frame.get_type() {
                    EthernetTypes::Ipv4 => {
                        let ipv4_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                        create_ipv4_details(&details_layout, &hex_editor, &db, &ipv4_layer, ethernet_frame.len());
                    }
                    EthernetTypes::Arp => {
                        let arp_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<ArpExtension>().unwrap();
                        details_layout.add(&create_arp_layer_expander(&arp_layer));
                    }
                    EthernetTypes::Ipv6 => {
                        let ipv6_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                        create_ipv6_details(&details_layout, &hex_editor, &db, &ipv6_layer, ethernet_frame.len());
                    }
                    EthernetTypes::Broadcast => {
                    }
                }
            }
            DataLinkTypes::Sll2 => {
                let sll2_frame = self.packet.get_frame().as_any().downcast_ref::<Sll2Frame>().unwrap();
                details_layout.add(&create_sll2_layer_expander(0, &hex_editor, &sll2_frame));

                match sll2_frame.get_protocol() {
                    EthernetTypes::Ipv4 => {
                        let ipv4_layer = sll2_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                        create_ipv4_details(&details_layout, &hex_editor, &db, &ipv4_layer, sll2_frame.len());
                    }
                    EthernetTypes::Ipv6 => {
                        let ipv6_layer = sll2_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                        create_ipv6_details(&details_layout, &hex_editor, &db, &ipv6_layer, sll2_frame.len());
                    }
                    _ => {}
                }
            }
            DataLinkTypes::Raw => {
                let raw_frame = self.packet.get_frame().as_any().downcast_ref::<RawFrame>().unwrap();

                match raw_frame.get_version() {
                    IpVersions::Ipv4 => {
                        let ipv4_layer = raw_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                        create_ipv4_details(&details_layout, &hex_editor, &db, &ipv4_layer, raw_frame.len());
                    }
                    IpVersions::Ipv6 => {
                        let ipv6_layer = raw_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                        create_ipv6_details(&details_layout, &hex_editor, &db, &ipv6_layer, raw_frame.len());
                    }
                }
            }
            DataLinkTypes::Loop => {
                let loop_frame = self.packet.get_frame().as_any().downcast_ref::<LoopFrame>().unwrap();

                match loop_frame.get_type() {
                    LoopTypes::Ipv4 => {
                        let ipv4_layer = loop_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                        create_ipv4_details(&details_layout, &hex_editor, &db, &ipv4_layer, loop_frame.len());
                    }
                    LoopTypes::Ipv6 | LoopTypes::Ipv6e2 | LoopTypes::Ipv6e3 => {
                        let ipv6_layer = loop_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                        create_ipv6_details(&details_layout, &hex_editor, &db, &ipv6_layer, loop_frame.len());
                    }
                    _ => {}
                }
            }
            _ => {}
        };

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


fn create_ipv4_details(details_layout: &gtk::Box, hex_editor: &HexEditor, db: &Database, layer: &Ipv4Layer, offset: usize) {
    details_layout.add(&create_ipv4_layer_expander(db, offset-layer.len(), hex_editor, layer));

    match layer.get_protocol() {
        IpProtocols::HopByHop => {}
        IpProtocols::Icmp => {
            let icmp_layer = layer.get_data().unwrap().as_any().downcast_ref::<IcmpLayer>().unwrap();
            details_layout.add(&create_icmp_layer_expander(&icmp_layer));
        }
        IpProtocols::Igmp => {}
        IpProtocols::Tcp => {
            let tcp_layer = layer.get_data().unwrap().as_any().downcast_ref::<TcpLayer>().unwrap();
            details_layout.add(&create_tcp_layer_expander(&tcp_layer));
        }
        IpProtocols::Udp => {
            let udp_layer = layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();
            details_layout.add(&create_udp_layer_expander(&udp_layer, IpAddr::V4(layer.get_source_address()), IpAddr::V4(layer.get_destination_address())));

            match udp_layer.get_payload() {
                UdpPayloads::Known(_type, payload) => {
                    match _type {
                        UdpTypes::Dhcp => {
                            let dhcp_layer = payload.as_any().downcast_ref::<DhcpLayer>().unwrap();
                            details_layout.add(&create_dhcp_layer_expander(&dhcp_layer));
                        }
                        UdpTypes::Dns => {}
                        UdpTypes::Quick => {}
                        UdpTypes::Utp => {}
                        UdpTypes::BitTorrent => {}
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        IpProtocols::Ipv6 => {}
        IpProtocols::Gre => {}
        IpProtocols::Icmpv6 => {}
        IpProtocols::Ospf => {}
        IpProtocols::Sps => {}
    }
}

fn create_ipv6_details(details_layout: &gtk::Box, hex_editor: &HexEditor, db: &Database, layer: &Ipv6Layer, offset: usize) {
    details_layout.add(&create_ipv6_layer_expander(db, offset-layer.len(), hex_editor, layer));

    match layer.get_next_header() {
        IpProtocols::HopByHop => {}
        IpProtocols::Icmp => {}
        IpProtocols::Igmp => {}
        IpProtocols::Tcp => {
            let tcp_layer = layer.get_data().unwrap().as_any().downcast_ref::<TcpLayer>().unwrap();
            details_layout.add(&create_tcp_layer_expander(&tcp_layer));
        }
        IpProtocols::Udp => {
            let udp_layer = layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();
            details_layout.add(&create_udp_layer_expander(&udp_layer, IpAddr::V6(layer.get_source_address()), IpAddr::V6(layer.get_destination_address())));

            match udp_layer.get_payload() {
                UdpPayloads::Known(_type, payload) => {
                    match _type {
                        UdpTypes::Dhcp => {
                            let dhcp_layer = payload.as_any().downcast_ref::<DhcpLayer>().unwrap();
                            details_layout.add(&create_dhcp_layer_expander(&dhcp_layer));
                        }
                        UdpTypes::Dns => {}
                        UdpTypes::Quick => {}
                        UdpTypes::Utp => {}
                        UdpTypes::BitTorrent => {}
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        IpProtocols::Ipv6 => {}
        IpProtocols::Gre => {}
        IpProtocols::Icmpv6 => {
            let icmpv6_layer = layer.get_data().unwrap().as_any().downcast_ref::<Icmpv6Layer>().unwrap();
            details_layout.add(&create_icmpv6_layer_expander(&icmpv6_layer));
        }
        IpProtocols::Ospf => {}
        IpProtocols::Sps => {}
    }
}
