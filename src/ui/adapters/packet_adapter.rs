use std::sync::{Arc, Mutex};
use gtk::{Builder, Label, ListBox, ListBoxRow};
use gtk::prelude::{BuilderExtManual, ContainerExt, LabelExt, StyleContextExt, WidgetExt};
use pcap::packet::inter::interfaces::Interfaces;
use pcap::packet::layers::layer_1::ethernet_layer::EthernetLayer;
use pcap::packet::layers::layer_1::inter::types::Types;
use pcap::packet::layers::layer_2::ethernet::ipv4_layer::IPv4Layer;
use pcap::packet::layers::layer_2::ethernet::ipv6_layer::IPv6Layer;
use pcap::packet::packet::Packet;

#[derive(Clone)]
pub struct PacketAdapter {
    list_box: ListBox,
    packets: Arc<Mutex<Vec<Packet>>>
}

impl PacketAdapter {

    pub fn new(list_box: &ListBox) -> Self {
        Self {
            list_box: list_box.clone(),
            packets: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn add(&mut self, packet: Packet) {
        let builder = Builder::from_file("res/ui/packet_list_item.xml");
        let row: ListBoxRow = builder
            .object("row")
            .expect("Couldn't find 'row' in packet_list_item.xml");

        //row.style_context().add_class(&packet.get_type().to_string());

        let number_label: Label = builder
            .object("number")
            .expect("Couldn't find 'number' in packet_list_item.xml");
        number_label.set_label(format!("{}", self.packets.lock().as_ref().unwrap().len()).as_str());

        let time_label: Label = builder
            .object("time")
            .expect("Couldn't find 'time' in packet_list_item.xml");
        time_label.set_label(format!("{:.5}", packet.get_frame_time()).as_str());

        let source_label: Label = builder
            .object("source")
            .expect("Couldn't find 'source' in packet_list_item.xml");

        let destination_label: Label = builder
            .object("destination")
            .expect("Couldn't find 'destination' in packet_list_item.xml");

        let protocol_label: Label = builder
            .object("protocol")
            .expect("Couldn't find 'protocol' in packet_list_item.xml");
        //protocol_label.set_label(&packet.get_type().to_string());

        let length_label: Label = builder
            .object("length")
            .expect("Couldn't find 'length' in packet_list_item.xml");
        length_label.set_label(format!("{}", packet.len()).as_str());

        let info_label: Label = builder
            .object("info")
            .expect("Couldn't find 'info' in packet_list_item.xml");

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

        self.packets.lock().as_mut().unwrap().push(packet);

        row.show_all();

        self.list_box.add(&row);
    }

    pub fn get_packet_by_index(&self, index: usize) -> Packet {
        self.packets.lock().unwrap().get(index).unwrap().clone()
    }
}
