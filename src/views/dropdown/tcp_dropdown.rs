use std::net::IpAddr;
use gtk::{gdk, glib, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Orientation, StyleContext};
use gtk::gio::SimpleActionGroup;
use gtk::glib::Cast;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{BuilderExtManual, ButtonExt, ContainerExt, EditableExt, GestureSingleExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, TextExt, WidgetExt};
use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use rlibpcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
use rlibpcap::packet::layers::ip::tcp::tcp_layer::TcpLayer;
use rlibpcap::packet::layers::ip::udp::udp_layer::UdpLayer;
use crate::database::sqlite::Database;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::pcap_ext::layers::ip::tcp::inter::tcp_ports::TcpPorts;
use crate::utils::ethernet_utils::ethernet_to_company;
use crate::utils::ip_utils::ip_to_icon;
use crate::views::dropdown::dropdown::{context_menu, create_row, create_row_with_icon, set_selection, Dropdown};
use crate::views::sidebar_view::SidebarView;
use crate::widgets::hex_editor::HexEditor;

pub trait TcpDropdown {

    fn from_tcp_layer(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &TcpLayer, offset: usize) -> Self;
}

impl TcpDropdown for Dropdown {

    fn from_tcp_layer(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &TcpLayer, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame").unwrap());
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        _self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        match TcpPorts::from_code(layer.get_source_port()) {
            Ok(port) => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("source_port").unwrap()), format!("{} ({})", port.to_string(), layer.get_source_port())));
            }
            Err(_) => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("source_port").unwrap()), layer.get_source_port().to_string()));
            }
        }

        match TcpPorts::from_code(layer.get_destination_port()) {
            Ok(port) => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("source_port").unwrap()), format!("{} ({})", port.to_string(), layer.get_destination_port())));
            }
            Err(_) => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("destination_port").unwrap()), layer.get_destination_port().to_string()));
            }
        }

        _self.list_box.add(&create_row(format!("{}:", layer.get_title("sequence_number").unwrap()), layer.get_value("sequence_number").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("acknowledgment_number").unwrap()), layer.get_value("acknowledgment_number").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("window_size").unwrap()), layer.get_value("window_size").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("checksum").unwrap()), layer.get_value("checksum").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("urgent_pointer").unwrap()), layer.get_value("urgent_pointer").unwrap()));

        _self
    }
}
