use std::net::IpAddr;
use gtk4::gio::SimpleActionGroup;
use rlibpcap::packet::layers::ip::udp::udp_layer::UdpLayer;
use crate::gtk4::views::dropdown::dropdown::{context_menu, create_row, set_selection, Dropdown};
use crate::gtk4::widgets::hex_editor::HexEditor;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::pcap_ext::layers::ip::udp::inter::udp_ports::UdpPorts;

pub trait UdpDropdown {

    fn from_udp_layer(source_address: IpAddr, destination_address: IpAddr, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &UdpLayer, offset: usize) -> Self;
}

impl UdpDropdown for Dropdown {

    fn from_udp_layer(source_address: IpAddr, destination_address: IpAddr, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &UdpLayer, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame").unwrap());
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        //_self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        match UdpPorts::from_code(layer.get_source_port()) {
            Ok(port) => {
                _self.list_box.append(&create_row(format!("{}:", layer.get_title("source_port").unwrap()), format!("{} ({})", port.to_string(), layer.get_value("source_port").unwrap())));
            }
            Err(_) => {
                _self.list_box.append(&create_row(format!("{}:", layer.get_title("source_port").unwrap()), layer.get_value("source_port").unwrap()));
            }
        }

        match UdpPorts::from_code(layer.get_destination_port()) {
            Ok(port) => {
                _self.list_box.append(&create_row(format!("{}:", layer.get_title("destination_port").unwrap()), format!("{} ({})", port.to_string(), layer.get_value("destination_port").unwrap())));
            }
            Err(_) => {
                _self.list_box.append(&create_row(format!("{}:", layer.get_title("destination_port").unwrap()), layer.get_value("destination_port").unwrap()));
            }
        }

        _self.list_box.append(&create_row(format!("{}:", layer.get_title("length").unwrap()), layer.get_value("length").unwrap()));

        let checksum_string = if layer.validate_checksum(source_address, destination_address) { "correct" } else { "incorrect" };
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("checksum").unwrap()), format!("{} [{}]", layer.get_value("checksum").unwrap(), checksum_string)));

        _self
    }
}
