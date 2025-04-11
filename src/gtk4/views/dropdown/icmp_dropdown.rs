use gtk4::gio::SimpleActionGroup;
use rlibpcap::packet::layers::ip::icmp::icmp_layer::IcmpLayer;
use crate::gtk4::views::dropdown::dropdown::{context_menu, create_row, set_selection, Dropdown};
use crate::gtk4::widgets::hex_editor::HexEditor;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;

pub trait IcmpDropdown {

    fn from_icmp_layer(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &IcmpLayer, offset: usize) -> Self;
}

impl IcmpDropdown for Dropdown {

    fn from_icmp_layer(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &IcmpLayer, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame").unwrap());
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        _self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        _self.list_box.append(&create_row(format!("{}:", layer.get_title("type").unwrap()), layer.get_value("type").unwrap()));
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("code").unwrap()), layer.get_value("code").unwrap()));
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("checksum").unwrap()), layer.get_value("checksum").unwrap()));
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("identifier_be").unwrap()), layer.get_value("identifier_be").unwrap()));
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("identifier_le").unwrap()), layer.get_value("identifier_le").unwrap()));
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("sequence_number_be").unwrap()), layer.get_value("sequence_number_be").unwrap()));
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("sequence_number_le").unwrap()), layer.get_value("sequence_number_le").unwrap()));

        _self
    }
}
