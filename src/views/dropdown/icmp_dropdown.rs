use std::net::IpAddr;
use gtk::{gdk, glib, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Orientation, StyleContext};
use gtk::gio::SimpleActionGroup;
use gtk::glib::Cast;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{BuilderExtManual, ButtonExt, ContainerExt, EditableExt, GestureSingleExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, TextExt, WidgetExt};
use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use rlibpcap::packet::layers::ip::icmp::icmp_layer::IcmpLayer;
use rlibpcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
use rlibpcap::packet::layers::ip::udp::udp_layer::UdpLayer;
use crate::database::sqlite::Database;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::utils::ethernet_utils::ethernet_to_company;
use crate::utils::ip_utils::ip_to_icon;
use crate::views::dropdown::dropdown::{context_menu, create_row, create_row_with_icon, set_selection, Dropdown};
use crate::views::sidebar_view::SidebarView;
use crate::widgets::hex_editor::HexEditor;

pub trait IcmpDropdown {

    fn from_icmp_layer(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &IcmpLayer, offset: usize) -> Self;
}

impl IcmpDropdown for Dropdown {

    fn from_icmp_layer(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &IcmpLayer, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame"));
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        _self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        _self.list_box.add(&create_row(format!("{}:", layer.get_title("type")), layer.get_value("type")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("code")), layer.get_value("code")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("checksum")), layer.get_value("checksum")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("identifier_be")), layer.get_value("identifier_be")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("identifier_le")), layer.get_value("identifier_le")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("sequence_number_be")), layer.get_value("sequence_number_be")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("sequence_number_le")), layer.get_value("sequence_number_le")));

        _self
    }
}
