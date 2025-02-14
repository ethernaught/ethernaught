use gtk::{Builder, Label, ListBox, ListBoxRow};
use gtk::prelude::{BuilderExtManual, ContainerExt, LabelExt, StyleContextExt, WidgetExt};
use crate::packet::inter::interfaces::Interfaces;
use crate::packet::layers::layer_1::ethernet_layer::EthernetLayer;
use crate::packet::layers::layer_1::inter::types::Types;
use crate::packet::layers::layer_2::ethernet::ipv4_layer::IPv4Layer;
use crate::packet::layers::layer_2::ethernet::ipv6_layer::IPv6Layer;
use crate::packet::packet::Packet;

#[derive(Clone)]
pub struct PacketAdapter {
    list_box: ListBox
}

impl PacketAdapter {

    pub fn new(list_box: &ListBox) -> Self {
        Self {
            list_box: list_box.clone()
        }
    }

    pub fn add_item(&self, number: u32, packet: Packet) {
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

        row.show_all();

        self.list_box.add(&row);
    }
}
