use std::net::IpAddr;
use gtk::{Builder, Paned, ScrolledWindow};
use gtk::gdk::RGBA;
use gtk::gio::{SimpleAction, SimpleActionGroup};
use gtk::prelude::{ActionMapExt, BuilderExtManual, ContainerExt, PanedExt};
use rlibpcap::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::{EthernetFrame, ETHERNET_FRAME_LEN};
use rlibpcap::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use rlibpcap::packet::layers::inter::layer::Layer;
use rlibpcap::packet::layers::ip::icmp::icmp_layer::IcmpLayer;
use rlibpcap::packet::layers::ip::icmpv6::icmpv6_layer::Icmpv6Layer;
use rlibpcap::packet::layers::ip::inter::ip_protocols::IpProtocols;
use rlibpcap::packet::layers::ip::inter::ip_versions::IpVersions;
use rlibpcap::packet::layers::ip::ipv4_layer::{Ipv4Layer, IPV4_HEADER_LEN};
use rlibpcap::packet::layers::ip::ipv6_layer::{Ipv6Layer, IPV6_HEADER_LEN};
use rlibpcap::packet::layers::ip::tcp::tcp_layer::TcpLayer;
use rlibpcap::packet::layers::ip::udp::dhcp::dhcp_layer::DhcpLayer;
use rlibpcap::packet::layers::ip::udp::inter::udp_payloads::UdpPayloads;
use rlibpcap::packet::layers::ip::udp::inter::udp_types::UdpTypes;
use rlibpcap::packet::layers::ip::udp::udp_layer::UdpLayer;
use rlibpcap::packet::layers::loop_frame::inter::loop_types::LoopTypes;
use rlibpcap::packet::layers::loop_frame::loop_frame::{LoopFrame, LOOP_FRAME_LENGTH};
use rlibpcap::packet::layers::raw_frame::raw_frame::RawFrame;
use rlibpcap::packet::layers::sll2_frame::sll2_frame::{Sll2Frame, SLL2_FRAME_LEN};
use rlibpcap::packet::packet::Packet;
use rlibpcap::utils::data_link_types::DataLinkTypes;
use crate::database::sqlite::Database;
use crate::get_lib_path;
use crate::views::dropdown::arp_dropdown::ArpDropdown;
use crate::views::dropdown::dropdown::Dropdown;
use crate::views::dropdown::ethernet_dropdown::EthernetDropdown;
use crate::views::dropdown::icmp_dropdown::IcmpDropdown;
use crate::views::dropdown::icmpv6_dropdown::Icmpv6Dropdown;
use crate::views::dropdown::ipv4_dropdown::Ipv4Dropdown;
use crate::views::dropdown::ipv6_dropdown::Ipv6Dropdown;
use crate::views::dropdown::sll2_dropdown::Sll2Dropdown;
use crate::views::dropdown::tcp_dropdown::TcpDropdown;
use crate::views::dropdown::udp_dropdown::UdpDropdown;
use crate::widgets::hex_editor::HexEditor;

pub struct SidebarView {
    pub root: gtk::Box,
    pub hex_editor: HexEditor
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



        /*
        match packet.get_data_link_type() {
            DataLinkTypes::En10mb => {
                let ethernet_frame = packet.get_frame().as_any().downcast_ref::<EthernetFrame>().unwrap();
                details_layout.add(&create_ethernet_layer_expander(&db, 0, &hex_editor, &actions, &ethernet_frame));

                match ethernet_frame.get_type() {
                    EthernetTypes::Ipv4 => {
                        let ipv4_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                        create_ipv4_details(&details_layout, &db, ethernet_frame.len(), &hex_editor, &actions, &ipv4_layer);
                    }
                    EthernetTypes::Arp => {
                        let arp_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<ArpExtension>().unwrap();
                        //details_layout.add(&create_arp_layer_expander(&arp_layer));
                    }
                    EthernetTypes::Ipv6 => {
                        let ipv6_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                        create_ipv6_details(&details_layout, &db, ethernet_frame.len(), &hex_editor, &actions, &ipv6_layer);
                    }
                    EthernetTypes::Broadcast => {
                    }
                }
            }
            DataLinkTypes::Sll2 => {
                let sll2_frame = packet.get_frame().as_any().downcast_ref::<Sll2Frame>().unwrap();
                //details_layout.add(&create_sll2_layer_expander(0, &hex_editor, &sll2_frame));

                match sll2_frame.get_protocol() {
                    EthernetTypes::Ipv4 => {
                        let ipv4_layer = sll2_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                        create_ipv4_details(&details_layout, &db, sll2_frame.len(), &hex_editor, &actions, &ipv4_layer);
                    }
                    EthernetTypes::Ipv6 => {
                        let ipv6_layer = sll2_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                        create_ipv6_details(&details_layout, &db, sll2_frame.len(), &hex_editor, &actions, &ipv6_layer);
                    }
                    _ => {}
                }
            }
            DataLinkTypes::Raw => {
                let raw_frame = packet.get_frame().as_any().downcast_ref::<RawFrame>().unwrap();

                match raw_frame.get_version() {
                    IpVersions::Ipv4 => {
                        let ipv4_layer = raw_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                        create_ipv4_details(&details_layout, &db, raw_frame.len(), &hex_editor, &actions, &ipv4_layer);
                    }
                    IpVersions::Ipv6 => {
                        let ipv6_layer = raw_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                        create_ipv6_details(&details_layout, &db, raw_frame.len(), &hex_editor, &actions, &ipv6_layer);
                    }
                }
            }
            DataLinkTypes::Loop => {
                let loop_frame = packet.get_frame().as_any().downcast_ref::<LoopFrame>().unwrap();

                match loop_frame.get_type() {
                    LoopTypes::Ipv4 => {
                        let ipv4_layer = loop_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                        create_ipv4_details(&details_layout, &db, loop_frame.len(), &hex_editor, &actions, &ipv4_layer);
                    }
                    LoopTypes::Ipv6 | LoopTypes::Ipv6e2 | LoopTypes::Ipv6e3 => {
                        let ipv6_layer = loop_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                        create_ipv6_details(&details_layout, &db, loop_frame.len(), &hex_editor, &actions, &ipv6_layer);
                    }
                    _ => {}
                }
            }
            _ => {}
        };
        */



        let mut offset = 0;

        match packet.get_data_link_type() {
            DataLinkTypes::En10mb => {
                let ethernet_frame = packet.get_frame().as_any().downcast_ref::<EthernetFrame>().unwrap();
                details_layout.add(&Dropdown::from_ethernet_frame(&db, &hex_editor, &actions, ethernet_frame, offset).root);
                offset += ETHERNET_FRAME_LEN;

                match ethernet_frame.get_type() {
                    EthernetTypes::Ipv4 => {
                        let ipv4_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                        create_ipv4_details(&details_layout, &db, &hex_editor, &actions, &ipv4_layer, offset);
                    }
                    EthernetTypes::Arp => {
                        let arp_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<ArpExtension>().unwrap();
                        details_layout.add(&Dropdown::from_arp_extension(&db, &hex_editor, &actions, arp_layer, offset).root);
                    }
                    EthernetTypes::Ipv6 => {
                        let ipv6_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                        create_ipv6_details(&details_layout, &db, &hex_editor, &actions, &ipv6_layer, offset);
                    }
                    EthernetTypes::Broadcast => {
                    }
                }
            }
            DataLinkTypes::Sll2 => {
                let sll2_frame = packet.get_frame().as_any().downcast_ref::<Sll2Frame>().unwrap();
                details_layout.add(&Dropdown::from_sll2_frame(&hex_editor, &actions, sll2_frame, offset).root);
                offset += SLL2_FRAME_LEN;

                match sll2_frame.get_protocol() {
                    EthernetTypes::Ipv4 => {
                        let ipv4_layer = sll2_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                        create_ipv4_details(&details_layout, &db, &hex_editor, &actions, &ipv4_layer, offset);
                    }
                    EthernetTypes::Ipv6 => {
                        let ipv6_layer = sll2_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                        create_ipv6_details(&details_layout, &db, &hex_editor, &actions, &ipv6_layer, offset);
                    }
                    _ => {}
                }
            }
            DataLinkTypes::Raw => {
                let raw_frame = packet.get_frame().as_any().downcast_ref::<RawFrame>().unwrap();

                match raw_frame.get_version() {
                    IpVersions::Ipv4 => {
                        let ipv4_layer = raw_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                        create_ipv4_details(&details_layout, &db, &hex_editor, &actions, &ipv4_layer, offset);
                    }
                    IpVersions::Ipv6 => {
                        let ipv6_layer = raw_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                        create_ipv6_details(&details_layout, &db, &hex_editor, &actions, &ipv6_layer, offset);
                    }
                }
            }
            DataLinkTypes::Loop => {
                let loop_frame = packet.get_frame().as_any().downcast_ref::<LoopFrame>().unwrap();
                offset += LOOP_FRAME_LENGTH;

                match loop_frame.get_type() {
                    LoopTypes::Ipv4 => {
                        let ipv4_layer = loop_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                        create_ipv4_details(&details_layout, &db, &hex_editor, &actions, &ipv4_layer, offset);
                    }
                    LoopTypes::Ipv6 | LoopTypes::Ipv6e2 | LoopTypes::Ipv6e3 => {
                        let ipv6_layer = loop_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                        create_ipv6_details(&details_layout, &db, &hex_editor, &actions, &ipv6_layer, offset);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Self {
            root,
            hex_editor
        }
    }
}


fn create_ipv4_details(details_layout: &gtk::Box, db: &Database, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &Ipv4Layer, offset: usize) {
    details_layout.add(&Dropdown::from_ipv4_layer(db, hex_editor, actions, layer, offset).root);
    let mut offset = offset + IPV4_HEADER_LEN;

    match layer.get_protocol() {
        IpProtocols::HopByHop => {}
        IpProtocols::Icmp => {
            let icmp_layer = layer.get_data().unwrap().as_any().downcast_ref::<IcmpLayer>().unwrap();
            details_layout.add(&Dropdown::from_icmp_layer(hex_editor, actions, icmp_layer, offset).root);
        }
        IpProtocols::Igmp => {}
        IpProtocols::Tcp => {
            let tcp_layer = layer.get_data().unwrap().as_any().downcast_ref::<TcpLayer>().unwrap();
            details_layout.add(&Dropdown::from_tcp_layer(hex_editor, actions, tcp_layer, offset).root);
        }
        IpProtocols::Udp => {
            let udp_layer = layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();
            details_layout.add(&Dropdown::from_udp_layer(IpAddr::V4(layer.get_source_address()), IpAddr::V4(layer.get_destination_address()), hex_editor, actions, udp_layer, offset).root);

            match udp_layer.get_payload() {
                UdpPayloads::Known(_type, payload) => {
                    match _type {
                        UdpTypes::Dhcp => {
                            let dhcp_layer = payload.as_any().downcast_ref::<DhcpLayer>().unwrap();
                            //details_layout.add(&create_dhcp_layer_expander(&dhcp_layer));
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

fn create_ipv6_details(details_layout: &gtk::Box, db: &Database, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &Ipv6Layer, offset: usize) {
    details_layout.add(&Dropdown::from_ipv6_layer(db, hex_editor, actions, layer, offset).root);
    let mut offset = offset + IPV6_HEADER_LEN;

    match layer.get_next_header() {
        IpProtocols::HopByHop => {}
        IpProtocols::Icmp => {}
        IpProtocols::Igmp => {}
        IpProtocols::Tcp => {
            let tcp_layer = layer.get_data().unwrap().as_any().downcast_ref::<TcpLayer>().unwrap();
            details_layout.add(&Dropdown::from_tcp_layer(hex_editor, actions, tcp_layer, offset).root);
        }
        IpProtocols::Udp => {
            let udp_layer = layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();
            //details_layout.add(&create_udp_layer_expander(&udp_layer, IpAddr::V6(layer.get_source_address()), IpAddr::V6(layer.get_destination_address())));
            details_layout.add(&Dropdown::from_udp_layer(IpAddr::V6(layer.get_source_address()), IpAddr::V6(layer.get_destination_address()), hex_editor, actions, udp_layer, offset).root);

            match udp_layer.get_payload() {
                UdpPayloads::Known(_type, payload) => {
                    match _type {
                        UdpTypes::Dhcp => {
                            let dhcp_layer = payload.as_any().downcast_ref::<DhcpLayer>().unwrap();
                            //details_layout.add(&create_dhcp_layer_expander(&dhcp_layer));
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
            details_layout.add(&Dropdown::from_icmpv6_layer(hex_editor, actions, icmpv6_layer, offset).root);
        }
        IpProtocols::Ospf => {}
        IpProtocols::Sps => {}
    }
}
