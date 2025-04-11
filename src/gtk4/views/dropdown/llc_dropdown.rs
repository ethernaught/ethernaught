use gtk4::gio::SimpleActionGroup;
use rlibpcap::packet::layers::ethernet_frame::llc::llc_extension::LlcExtension;
use crate::gtk4::views::dropdown::dropdown::{context_menu, create_row, set_selection, Dropdown};
use crate::gtk4::widgets::hex_editor::HexEditor;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;

pub trait LlcDropdown {

    fn from_llc_extension(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &LlcExtension, offset: usize) -> Self;
}

impl LlcDropdown for Dropdown {

    fn from_llc_extension(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &LlcExtension, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame").unwrap());
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        _self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        _self.list_box.append(&create_row(format!("{}:", layer.get_title("dsap").unwrap()), layer.get_value("dsap").unwrap()));
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("ssap").unwrap()), layer.get_value("ssap").unwrap()));
        _self.list_box.append(&create_row(format!("{}:", layer.get_title("control").unwrap()), layer.get_value("control").unwrap()));

        _self
    }
}
