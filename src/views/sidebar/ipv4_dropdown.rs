use std::net::IpAddr;
use gtk::{gdk, glib, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Orientation, StyleContext};
use gtk::gio::SimpleActionGroup;
use gtk::glib::Cast;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{BuilderExtManual, ButtonExt, ContainerExt, EditableExt, GestureSingleExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, TextExt, WidgetExt};
use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use rlibpcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
use crate::database::sqlite::Database;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::utils::ethernet_utils::ethernet_to_company;
use crate::utils::ip_utils::ip_to_icon;
use crate::views::sidebar::dropdown::{context_menu, create_row, set_selection, Dropdown};
use crate::views::sidebar_view::SidebarView;
use crate::widgets::hex_editor::HexEditor;

pub struct Ipv4Dropdown {
    pub root: gtk::Box,
    pub dropdown_button: Button,
    pub label: Label,
    pub list_box: ListBox
}

impl Ipv4Dropdown for Dropdown {

    fn from_ipv4_frame(db: &Database, sidebar_view: &SidebarView, actions: &SimpleActionGroup, layer: &Ipv4Layer, offset: usize) -> Self {
        let _self = Self::default();
        _self.label.set_text(&layer.get_title("frame"));
        _self.list_box.connect_row_activated(set_selection(&sidebar_view.hex_editor, layer, offset));
        _self.list_box.connect_button_press_event(context_menu(&sidebar_view.hex_editor, actions, layer, offset));

        _self
    }
}

