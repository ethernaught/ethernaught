use std::any::Any;
use gtk::{Expander, Label, ListBox, ListBoxRow};
use gtk::prelude::{ContainerExt, LabelExt, WidgetExt};
use pcap::packet::layers::layer_1::ethernet_layer::EthernetLayer;
use pcap::packet::layers::layer_2::ethernet::ipv4_layer::IPv4Layer;
use pcap::packet::layers::layer_3::ip::udp_layer::UdpLayer;

pub fn create_ethernet_layer_expander(layer: &EthernetLayer) -> Expander {
    let expander = Expander::new(Some("Ethernet II"));

    let list_box = ListBox::new();

    list_box.add(&create_row(format!("Destination: ({})", layer.get_destination().to_string()).as_str()));
    list_box.add(&create_row(format!("Source: ({})", layer.get_source().to_string()).as_str()));
    list_box.add(&create_row(format!("Type: {:?} (0x{:04x})", layer.get_type(), layer.get_type().get_code()).as_str()));

    expander.add(&list_box);
    expander.show_all();

    expander
}

pub fn create_ipv4_layer_expander(layer: &IPv4Layer) -> Expander {
    let expander = Expander::new(Some("Internet Protocol Version 4"));

    let list_box = ListBox::new();

    list_box.add(&create_row(format!("Version: {}", layer.get_version()).as_str()));
    list_box.add(&create_row(format!("TOS: {}", layer.get_tos()).as_str())); // SHOULD BE - Differentiated Services Field
    list_box.add(&create_row(format!("Total Length: {}", layer.get_total_length()).as_str()));
    list_box.add(&create_row(format!("Identification: 0x{:04x} ({})", layer.get_identification(), layer.get_identification()).as_str()));
    //list_box.add(&create_row(format!("Header: ({})", layer.get_version()).as_str())); //FLAGS
    list_box.add(&create_row(format!("Time to Live: {}", layer.get_ttl()).as_str()));
    list_box.add(&create_row(format!("Protocol: {:?} ({})", layer.get_protocol(), layer.get_protocol().get_code()).as_str()));

    list_box.add(&create_row(format!("Header Checksum: 0x{:04x}", layer.get_checksum()).as_str()));
    list_box.add(&create_row(format!("Source Address: {}", layer.get_source_ip().to_string()).as_str()));
    list_box.add(&create_row(format!("Destination Address: {}", layer.get_destination_ip().to_string()).as_str()));

    expander.add(&list_box);
    expander.show_all();

    expander
}

pub fn create_udp_layer_expander(layer: &UdpLayer) -> Expander {
    let expander = Expander::new(Some("User Datagram Protocol"));

    let list_box = ListBox::new();

    list_box.add(&create_row(format!("Source Port: {}", layer.get_source_port()).as_str()));
    list_box.add(&create_row(format!("Destination Port: {}", layer.get_destination_port()).as_str()));
    list_box.add(&create_row(format!("Length: {}", layer.get_length()).as_str()));
    list_box.add(&create_row(format!("Checksum: 0x{:04x}", layer.get_checksum()).as_str()));

    expander.add(&list_box);
    expander.show_all();

    expander
}

fn create_row(title: &str) -> ListBoxRow {
    let row = ListBoxRow::new();

    let label = Label::new(Some(title));
    label.set_xalign(0.0);
    row.add(&label);

    row
}
