use std::sync::{Arc, Mutex};
use gtk::{Builder, Label, ListBox, ListBoxRow, ListStore};
use gtk::prelude::{BuilderExtManual, ContainerExt, GtkListStoreExt, GtkListStoreExtManual, LabelExt, StyleContextExt, ToValue, WidgetExt};
use pcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use pcap::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use pcap::packet::layers::ethernet_frame::ip::inter::ip_protocols::IpProtocols;
use pcap::packet::layers::ethernet_frame::ip::ipv4_layer::Ipv4Layer;
use pcap::packet::layers::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use pcap::packet::layers::ethernet_frame::ip::udp::inter::udp_payloads::UdpPayloads;
use pcap::packet::layers::ethernet_frame::ip::udp::udp_layer::UdpLayer;
use pcap::packet::layers::loop_frame::inter::loop_types::LoopTypes;
use pcap::packet::layers::loop_frame::loop_frame::LoopFrame;
use pcap::packet::layers::sll2_frame::sll2_frame::Sll2Frame;
use pcap::packet::packet::Packet;
use pcap::utils::data_link_types::DataLinkTypes;

#[derive(Clone)]
pub struct PacketAdapter {
    model: ListStore,
    packets: Arc<Mutex<Vec<Packet>>>
}

impl PacketAdapter {

    pub fn new(model: &ListStore) -> Self {
        Self {
            model: model.clone(),
            packets: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn from_packets(model: &ListStore, packets: Vec<Packet>) -> Self {
        let mut i = 0;
        packets.iter().for_each(|p| {
            i += 1;
            Self::add_model(model, p, i);
        });

        Self {
            model: model.clone(),
            packets: Arc::new(Mutex::new(packets))
        }
    }

    pub fn add(&self, packet: Packet) {
        let packet_count = 1 + self.packets.lock().as_ref().unwrap().len() as u32;
        Self::add_model(&self.model, &packet, packet_count);
        self.packets.lock().as_mut().unwrap().push(packet);
    }

    fn add_model(model: &ListStore, packet: &Packet, packet_count: u32) {
        let (source, destination, protocol) = match packet.get_data_link_type() {
            DataLinkTypes::En10mb => {
                let ethernet_frame = packet.get_frame().as_any().downcast_ref::<EthernetFrame>().unwrap();

                match ethernet_frame.get_type() {
                    EthernetTypes::Ipv4 => {
                        let ipv4_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();

                        match ipv4_layer.get_protocol() {
                            IpProtocols::Udp => {
                                let udp_layer = ipv4_layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();

                                match udp_layer.get_payload() {
                                    UdpPayloads::Known(_type, _) => {
                                        (ipv4_layer.get_source_address().to_string(), ipv4_layer.get_destination_address().to_string(), udp_layer.get_type().to_string())
                                    }
                                    _ => {
                                        (ipv4_layer.get_source_address().to_string(), ipv4_layer.get_destination_address().to_string(), ipv4_layer.get_protocol().to_string())
                                    }
                                }
                            }
                            _ => {
                                (ipv4_layer.get_source_address().to_string(), ipv4_layer.get_destination_address().to_string(), ipv4_layer.get_protocol().to_string())
                            }
                        }
                    }
                    EthernetTypes::Ipv6 => {
                        let ipv6_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();

                        match ipv6_layer.get_next_header() {
                            IpProtocols::Udp => {
                                let udp_layer = ipv6_layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();

                                match udp_layer.get_payload() {
                                    UdpPayloads::Known(_type, _) => {
                                        (ipv6_layer.get_source_address().to_string(), ipv6_layer.get_destination_address().to_string(), udp_layer.get_type().to_string())
                                    }
                                    _ => {
                                        (ipv6_layer.get_source_address().to_string(), ipv6_layer.get_destination_address().to_string(), ipv6_layer.get_next_header().to_string())
                                    }
                                }
                            }
                            _ => {
                                (ipv6_layer.get_source_address().to_string(), ipv6_layer.get_destination_address().to_string(), ipv6_layer.get_next_header().to_string())
                            }
                        }
                    }
                    EthernetTypes::Broadcast => {
                        //source_label.set_label(&ethernet_layer.get_source().to_string());
                        //destination_label.set_label(&ethernet_layer.get_destination().to_string());
                        (ethernet_frame.get_source_mac().to_string(), ethernet_frame.get_destination_mac().to_string(), ethernet_frame.get_type().to_string())
                    }
                    _ => {
                        (ethernet_frame.get_source_mac().to_string(), ethernet_frame.get_destination_mac().to_string(), ethernet_frame.get_type().to_string())
                    }
                }
            }
            DataLinkTypes::Sll2 => {
                let sll2_frame = packet.get_frame().as_any().downcast_ref::<Sll2Frame>().unwrap();

                match sll2_frame.get_protocol() {
                    EthernetTypes::Ipv4 => {
                        let ipv4_layer = sll2_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();

                        match ipv4_layer.get_protocol() {
                            IpProtocols::Udp => {
                                let udp_layer = ipv4_layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();

                                match udp_layer.get_payload() {
                                    UdpPayloads::Known(_type, _) => {
                                        (ipv4_layer.get_source_address().to_string(), ipv4_layer.get_destination_address().to_string(), udp_layer.get_type().to_string())
                                    }
                                    _ => {
                                        (ipv4_layer.get_source_address().to_string(), ipv4_layer.get_destination_address().to_string(), ipv4_layer.get_protocol().to_string())
                                    }
                                }
                            }
                            _ => {
                                (ipv4_layer.get_source_address().to_string(), ipv4_layer.get_destination_address().to_string(), ipv4_layer.get_protocol().to_string())
                            }
                        }
                    }
                    EthernetTypes::Ipv6 => {
                        let ipv6_layer = sll2_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();

                        match ipv6_layer.get_next_header() {
                            IpProtocols::Udp => {
                                let udp_layer = ipv6_layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();

                                match udp_layer.get_payload() {
                                    UdpPayloads::Known(_type, _) => {
                                        (ipv6_layer.get_source_address().to_string(), ipv6_layer.get_destination_address().to_string(), udp_layer.get_type().to_string())
                                    }
                                    _ => {
                                        (ipv6_layer.get_source_address().to_string(), ipv6_layer.get_destination_address().to_string(), ipv6_layer.get_next_header().to_string())
                                    }
                                }
                            }
                            _ => {
                                (ipv6_layer.get_source_address().to_string(), ipv6_layer.get_destination_address().to_string(), ipv6_layer.get_next_header().to_string())
                            }
                        }
                    }
                    _ => {
                        unimplemented!()
                    }
                }
            }
            DataLinkTypes::Loop => {
                let loop_frame = packet.get_frame().as_any().downcast_ref::<LoopFrame>().unwrap();

                match loop_frame.get_type() {
                    LoopTypes::Ipv4 => {
                        let ipv4_layer = loop_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();

                        match ipv4_layer.get_protocol() {
                            IpProtocols::Udp => {
                                let udp_layer = ipv4_layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();

                                match udp_layer.get_payload() {
                                    UdpPayloads::Known(_type, _) => {
                                        (ipv4_layer.get_source_address().to_string(), ipv4_layer.get_destination_address().to_string(), udp_layer.get_type().to_string())
                                    }
                                    _ => {
                                        (ipv4_layer.get_source_address().to_string(), ipv4_layer.get_destination_address().to_string(), ipv4_layer.get_protocol().to_string())
                                    }
                                }
                            }
                            _ => {
                                (ipv4_layer.get_source_address().to_string(), ipv4_layer.get_destination_address().to_string(), ipv4_layer.get_protocol().to_string())
                            }
                        }
                    }
                    LoopTypes::Ipv6 | LoopTypes::Ipv6e2 | LoopTypes::Ipv6e3 => {
                        let ipv6_layer = loop_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();

                        match ipv6_layer.get_next_header() {
                            IpProtocols::Udp => {
                                let udp_layer = ipv6_layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();

                                match udp_layer.get_payload() {
                                    UdpPayloads::Known(_type, _) => {
                                        (ipv6_layer.get_source_address().to_string(), ipv6_layer.get_destination_address().to_string(), udp_layer.get_type().to_string())
                                    }
                                    _ => {
                                        (ipv6_layer.get_source_address().to_string(), ipv6_layer.get_destination_address().to_string(), ipv6_layer.get_next_header().to_string())
                                    }
                                }
                            }
                            _ => {
                                (ipv6_layer.get_source_address().to_string(), ipv6_layer.get_destination_address().to_string(), ipv6_layer.get_next_header().to_string())
                            }
                        }
                    }
                    _ => {
                        unimplemented!()
                    }
                }


            }
            _ => {
                //"[WiFi] TODO".to_string()
                todo!()
            }
        };

        let frame_time = packet.get_frame_time().to_string();
        let packet_length = packet.len().to_string();

        model.insert_with_values(None, &[
            (0, &packet_count),
            (1, &frame_time),
            (2, &source),
            (3, &destination),
            (4, &protocol),
            (5, &packet_length),
            //(6, &"TODO".to_string()),
        ]);
    }

    pub fn get_packet_by_index(&self, index: usize) -> Packet {
        self.packets.lock().unwrap().get(index).unwrap().clone()
    }

    pub fn clear(&self) {
        self.model.clear();
        self.packets.lock().unwrap().clear();
    }
}
