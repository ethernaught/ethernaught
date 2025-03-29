use std::net::IpAddr;
use gtk::{Builder, Paned, ScrolledWindow};
use gtk::gdk::RGBA;
use gtk::gio::{SimpleAction, SimpleActionGroup};
use gtk::prelude::{ActionMapExt, BuilderExtManual, ContainerExt, PanedExt};
use pcap::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use pcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use pcap::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use pcap::packet::layers::inter::layer::Layer;
use pcap::packet::layers::ip::icmp::icmp_layer::IcmpLayer;
use pcap::packet::layers::ip::icmpv6::icmpv6_layer::Icmpv6Layer;
use pcap::packet::layers::ip::inter::ip_protocols::IpProtocols;
use pcap::packet::layers::ip::inter::ip_versions::IpVersions;
use pcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
use pcap::packet::layers::ip::ipv6_layer::Ipv6Layer;
use pcap::packet::layers::ip::tcp::tcp_layer::TcpLayer;
use pcap::packet::layers::ip::udp::dhcp::dhcp_layer::DhcpLayer;
use pcap::packet::layers::ip::udp::inter::udp_payloads::UdpPayloads;
use pcap::packet::layers::ip::udp::inter::udp_types::UdpTypes;
use pcap::packet::layers::ip::udp::udp_layer::UdpLayer;
use pcap::packet::layers::loop_frame::inter::loop_types::LoopTypes;
use pcap::packet::layers::loop_frame::loop_frame::LoopFrame;
use pcap::packet::layers::raw_frame::raw_frame::RawFrame;
use pcap::packet::layers::sll2_frame::sll2_frame::Sll2Frame;
use pcap::packet::packet::Packet;
use pcap::utils::data_link_types::DataLinkTypes;
use crate::database::sqlite::Database;
use crate::get_lib_path;
use crate::views::utils::sidebar_expanders::{create_arp_layer_expander, create_dhcp_layer_expander, create_ethernet_layer_expander, create_icmp_layer_expander, create_icmpv6_layer_expander, create_ipv4_layer_expander, create_ipv6_layer_expander, create_sll2_layer_expander, create_tcp_layer_expander, create_udp_layer_expander};
use crate::widgets::hex_editor::HexEditor;

#[derive(Clone)]
pub struct SidebarView {
    pub root: gtk::Box,
    pub hex_editor: HexEditor,

}

impl SidebarView {

    pub fn from_packet(packet: &Packet) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/sidebar_view.ui");

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in sidebar_view.ui");


        let sidebar_content: Paned = builder
            .object("sidebar_content")
            .expect("Couldn't find 'sidebar_content' in sidebar_view.ui");

        let hex_scroll_layout: ScrolledWindow = builder
            .object("hex_scroll_layout")
            .expect("Couldn't find 'hex_scroll_layout' in sidebar_view.ui");

        sidebar_content.set_child_shrink(&hex_scroll_layout, false);
        sidebar_content.set_child_resize(&hex_scroll_layout, true);

        let details_scroll_layout: ScrolledWindow = builder
            .object("details_scroll_layout")
            .expect("Couldn't find 'details_scroll_layout' in sidebar_view.ui");

        sidebar_content.set_child_shrink(&details_scroll_layout, false);


        let hex_editor: HexEditor = builder
            .object("hex_editor")
            .expect("Couldn't find 'hex_editor' in sidebar_view.ui");

        hex_editor.set_data(packet.to_bytes());
        hex_editor.set_line_number_color(RGBA::new(0.286, 0.306, 0.341, 1.0));
        hex_editor.set_cursor_color(RGBA::new(0.608, 0.616, 0.624, 1.0));
        hex_editor.set_selection_color(RGBA::new(0.349, 0.263, 0.431, 1.0));






        let db = Database::open(get_lib_path("database.db").to_str().unwrap()).expect("Couldn't open database.db");


        let actions = SimpleActionGroup::new();

        let action = SimpleAction::new("open-new-window", None);
        action.connect_activate({
            move |_, _| {
                //context.create_window_from_activity(Box::new(DevicesActivity::new(context.clone())), None);
            }
        });
        actions.add_action(&action);

        let details_layout: gtk::Box = builder
            .object("details_layout")
            .expect("Couldn't find 'details_layout' in sidebar_view.ui");


        match packet.get_data_link_type() {
            DataLinkTypes::En10mb => {
                let ethernet_frame = packet.get_frame().as_any().downcast_ref::<EthernetFrame>().unwrap();
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
                let sll2_frame = packet.get_frame().as_any().downcast_ref::<Sll2Frame>().unwrap();
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
                let raw_frame = packet.get_frame().as_any().downcast_ref::<RawFrame>().unwrap();

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
                let loop_frame = packet.get_frame().as_any().downcast_ref::<LoopFrame>().unwrap();

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

        


        Self {
            root,
            hex_editor
        }
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
