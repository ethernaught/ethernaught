use gtk4::gio::SimpleActionGroup;
use rlibpcap::packet::layers::ip::tcp::tcp_layer::TcpLayer;
use crate::gtk4::views::dropdown::dropdown::{context_menu, create_row, set_selection, Dropdown};
use crate::gtk4::widgets::hex_editor::HexEditor;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::pcap_ext::layers::ip::tcp::inter::tcp_ports::TcpPorts;

pub trait TcpDropdown {

    fn from_tcp_layer(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &TcpLayer, offset: usize) -> Self;
}

impl TcpDropdown for Dropdown {

    fn from_tcp_layer(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &TcpLayer, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame").unwrap());
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        //_self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        match TcpPorts::from_code(layer.get_source_port()) {
            Ok(port) => {
                _self.list_box.append(&create_row(format!("{}:", layer.get_title("source_port").unwrap()), format!("{} ({})", port.to_string(), layer.get_value("source_port").unwrap())));
            }
            Err(_) => {
                _self.list_box.append(&create_row(format!("{}:", layer.get_title("source_port").unwrap()), layer.get_value("source_port").unwrap()));
            }
        }

        match TcpPorts::from_code(layer.get_destination_port()) {
            Ok(port) => {
                _self.list_box.append(&create_row(format!("{}:", layer.get_title("destination_port").unwrap()), format!("{} ({})", port.to_string(), layer.get_value("destination_port").unwrap())));
            }
            Err(_) => {
                _self.list_box.append(&create_row(format!("{}:", layer.get_title("destination_port").unwrap()), layer.get_value("destination_port").unwrap()));
            }
        }

        _self.list_box.append(&create_row(format!("{}:", layer.get_title("sequence_number").unwrap()), layer.get_value("sequence_number").unwrap()));
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("acknowledgment_number").unwrap()), layer.get_value("acknowledgment_number").unwrap()));
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("window_size").unwrap()), layer.get_value("window_size").unwrap()));
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("checksum").unwrap()), layer.get_value("checksum").unwrap()));
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("urgent_pointer").unwrap()), layer.get_value("urgent_pointer").unwrap()));

        _self
    }
}
