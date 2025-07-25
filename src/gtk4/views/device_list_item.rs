use gtk4::{Builder, Image, Label, ListBoxRow};
use gtk4::prelude::{StyleContextExt, WidgetExt};
use rlibpcap::devices::Device;
use rlibpcap::utils::data_link_types::DataLinkTypes;
use rlibpcap::utils::interface_flags::InterfaceFlags;
use crate::gtk4::widgets::graph::Graph;

#[derive(Clone)]
pub struct DeviceListItem {
    pub root: ListBoxRow,
    pub icon: Image,
    pub title: Label,
    pub description: Label,
    pub graph: Graph
}

impl DeviceListItem {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/device_list_item.ui");
        let root: ListBoxRow = builder
            .object("root")
            .expect("Couldn't find 'root' in device_list_item.ui");

        let icon: Image = builder
            .object("icon")
            .expect("Couldn't find 'icon' in device_list_item.ui");

        root.style_context().add_class("any");
        icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_any.svg"));

        let title: Label = builder
            .object("title")
            .expect("Couldn't find 'title' in device_list_item.ui");
        title.set_label("Any");

        let description: Label = builder
            .object("description")
            .expect("Couldn't find 'description' in device_list_item.ui");
        description.set_label("[Promiscuous]");

        let graph: Graph = builder
            .object("graph")
            .expect("Couldn't find 'graph' in device_list_item.ui");

        Self {
            root,
            icon,
            title,
            description,
            graph
        }
    }

    pub fn from_device(device: &Device) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/device_list_item.ui");
        let root: ListBoxRow = builder
            .object("root")
            .expect("Couldn't find 'root' in device_list_item.ui");

        let icon: Image = builder
            .object("icon")
            .expect("Couldn't find 'icon' in device_list_item.ui");

        match device.get_data_link_type() {
            DataLinkTypes::En10mb | DataLinkTypes::En3mb => {
                root.style_context().add_class("ethernet");
                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_ethernet.svg"));
            }
            DataLinkTypes::Loop => {
                root.style_context().add_class("lan");
                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_lan.svg"));
            }
            DataLinkTypes::Raw | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                root.style_context().add_class("vpn");
                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_vpn.svg"));
            }
            DataLinkTypes::Ieee802_11 => {
                root.style_context().add_class("wifi");
                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_wifi.svg"));
            }
            /*
            DataLinkTypes::BluetoothHciH4 => {
                row.style_context().add_class("bluetooth");
                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_bluetooth.svg"));
            }
            */
            _ => {}
        }

        let title: Label = builder
            .object("title")
            .expect("Couldn't find 'title' in device_list_item.ui");
        title.set_label(format!("{}", device.get_name()).as_str());

        let description: Label = builder
            .object("description")
            .expect("Couldn't find 'description' in device_list_item.ui");
        description.set_label(format!("{:?}", device.get_flags()).as_str());

        if !device.get_flags().contains(&InterfaceFlags::Running) {
            root.style_context().add_class("down");
        }

        let graph: Graph = builder
            .object("graph")
            .expect("Couldn't find 'graph' in device_list_item.ui");

        Self {
            root,
            icon,
            title,
            description,
            graph
        }
    }
}
