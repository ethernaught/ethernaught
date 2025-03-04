use std::sync::{Arc, Mutex};
use gtk::{Builder, Label, ListBox, ListBoxRow, ListStore};
use gtk::prelude::{BuilderExtManual, ContainerExt, GtkListStoreExt, GtkListStoreExtManual, LabelExt, StyleContextExt, ToValue, WidgetExt};
use pcap::packet::inter::interfaces::Interfaces;
use pcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use pcap::packet::layers::ethernet_frame::inter::types::Types;
use pcap::packet::layers::ethernet_frame::ip::inter::protocols::Protocols;
use pcap::packet::layers::ethernet_frame::ip::ipv4_layer::Ipv4Layer;
use pcap::packet::layers::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use pcap::packet::layers::ethernet_frame::ip::udp::inter::udp_payloads::UdpPayloads;
use pcap::packet::layers::ethernet_frame::ip::udp::udp_layer::UdpLayer;
use pcap::packet::packet::Packet;

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

    pub fn add(&mut self, packet: Packet) {
        let (source, destination, protocol) = match packet.get_interface() {
            Interfaces::Ethernet => {
                let ethernet_frame = packet.get_frame().as_any().downcast_ref::<EthernetFrame>().unwrap();

                match ethernet_frame.get_type() {
                    Types::IPv4 => {
                        let ipv4_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();

                        match ipv4_layer.get_protocol() {
                            Protocols::Udp => {
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
                    Types::IPv6 => {
                        let ipv6_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();

                        match ipv6_layer.get_next_header() {
                            Protocols::Udp => {
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
                    Types::Broadcast => {
                        //source_label.set_label(&ethernet_layer.get_source().to_string());
                        //destination_label.set_label(&ethernet_layer.get_destination().to_string());
                        (ethernet_frame.get_source_mac().to_string(), ethernet_frame.get_destination_mac().to_string(), ethernet_frame.get_type().to_string())
                    }
                    _ => {
                        (ethernet_frame.get_source_mac().to_string(), ethernet_frame.get_destination_mac().to_string(), ethernet_frame.get_type().to_string())
                    }
                }
            }
            Interfaces::WiFi => {
                //"[WiFi] TODO".to_string()
                todo!()
            }
            Interfaces::Bluetooth => {
                //"[Bluetooth] TODO".to_string()
                todo!()
            }
        };

        let packet_count = 1 + self.packets.lock().as_ref().unwrap().len() as u32;
        let frame_time = packet.get_frame_time().to_string();
        let packet_length = packet.len().to_string();

        self.model.insert_with_values(None, &[
            (0, &packet_count),
            (1, &frame_time),
            (2, &source),
            (3, &destination),
            (4, &protocol),
            (5, &packet_length),
            //(6, &"TODO".to_string()),
        ]);

        self.packets.lock().as_mut().unwrap().push(packet);
    }

    pub fn get_packet_by_index(&self, index: usize) -> Packet {
        self.packets.lock().unwrap().get(index).unwrap().clone()
    }

    pub fn clear(&mut self) {
        self.model.clear();
        self.packets.lock().unwrap().clear();
    }
}
