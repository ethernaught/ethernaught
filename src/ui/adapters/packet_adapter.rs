use std::sync::{Arc, Mutex};
use gtk::{Builder, Label, ListBox, ListBoxRow, ListStore};
use gtk::prelude::{BuilderExtManual, ContainerExt, GtkListStoreExt, GtkListStoreExtManual, LabelExt, StyleContextExt, WidgetExt};
use pcap::packet::inter::interfaces::Interfaces;
use pcap::packet::layers::layer_1::ethernet_layer::EthernetLayer;
use pcap::packet::layers::layer_1::inter::types::Types;
use pcap::packet::layers::layer_2::ethernet::ipv4_layer::IPv4Layer;
use pcap::packet::layers::layer_2::ethernet::ipv6_layer::IPv6Layer;
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
                let ethernet_layer = packet.get_layer(0).unwrap().as_any().downcast_ref::<EthernetLayer>().unwrap();

                match ethernet_layer.get_type() {
                    Types::IPv4 => {
                        let ipv4_layer = packet.get_layer(1).unwrap().as_any().downcast_ref::<IPv4Layer>().unwrap();

                        //source.set_label(&ipv4_layer.get_source_ip().to_string());
                        //destination_label.set_label(&ipv4_layer.get_destination_ip().to_string());

                        (ipv4_layer.get_source_ip().to_string(), ipv4_layer.get_destination_ip().to_string(), ipv4_layer.get_protocol().to_string())
                    }
                    Types::IPv6 => {
                        let ipv6_layer = packet.get_layer(1).unwrap().as_any().downcast_ref::<IPv6Layer>().unwrap();

                        //source_label.set_label(&ipv6_layer.get_source_ip().to_string());
                        //destination_label.set_label(&ipv6_layer.get_destination_ip().to_string());

                        (ipv6_layer.get_source_ip().to_string(), ipv6_layer.get_destination_ip().to_string(), ipv6_layer.get_next_header().to_string())
                    }
                    Types::Broadcast => {
                        //source_label.set_label(&ethernet_layer.get_source().to_string());
                        //destination_label.set_label(&ethernet_layer.get_destination().to_string());
                        (ethernet_layer.get_source().to_string(), ethernet_layer.get_destination().to_string(), ethernet_layer.get_type().to_string())
                    }
                    _ => {
                        (ethernet_layer.get_source().to_string(), ethernet_layer.get_destination().to_string(), ethernet_layer.get_type().to_string())
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

        self.model.insert_with_values(
            None,
            &[
                (0, &(1+self.packets.lock().as_ref().unwrap().len() as u32)),
                (1, &packet.get_frame_time().to_string()),
                (2, &source),
                (3, &destination),
                (4, &protocol),
                (5, &packet.len()),
                (6, &"TODO".to_string())
            ],
        );

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
