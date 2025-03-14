use gtk::{Builder, Image, Label, ListBox, ListBoxRow};
use gtk::prelude::{BuilderExtManual, ContainerExt, ImageExt, LabelExt, StyleContextExt, WidgetExt};
use pcap::devices::Device;
use pcap::interface_flags::InterfaceFlags;
use pcap::packet::inter::data_link_types::DataLinkTypes;
use crate::ui::widgets::graph::Graph;

#[derive(Clone)]
pub struct DevicesAdapter {
    list_box: ListBox
}

impl DevicesAdapter {

    pub fn new(list_box: &ListBox) -> Self {
        Self {
            list_box: list_box.clone()
        }
    }

    pub fn from_devices(list_box: &ListBox, devices: Vec<Device>) -> Self {
        devices.iter().for_each(|d| {
            Self::add(list_box, d);
        });

        Self {
            list_box: list_box.clone()
        }
    }

    pub fn add(list_box: &ListBox, device: &Device) {
        let builder = Builder::from_resource("/com/ethernaut/rust/res/ui/gtk3/device_list_item.ui");
        let row: ListBoxRow = builder
            .object("row")
            .expect("Couldn't find 'row' in device_list_item.ui");

        let icon: Image = builder
            .object("icon")
            .expect("Couldn't find 'icon' in device_list_item.ui");

        match device.get_data_link_type() {
            DataLinkTypes::Ethernet => {
                row.style_context().add_class("ethernet");
                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_ethernet.svg"));
            }
            DataLinkTypes::Loopback => {
                row.style_context().add_class("lan");
                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_lan.svg"));
            }
            DataLinkTypes::Raw | DataLinkTypes::Tun | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                row.style_context().add_class("vpn");
                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_vpn.svg"));
            }
            DataLinkTypes::BluetoothHciH4 => {
                row.style_context().add_class("bluetooth");
                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_bluetooth.svg"));
            }
            _ => {}
        }

        let title_label: Label = builder
            .object("title")
            .expect("Couldn't find 'title' in device_list_item.ui");
        title_label.set_label(format!("{}", device.get_name()).as_str());

        let description_label: Label = builder
            .object("description")
            .expect("Couldn't find 'description' in device_list_item.ui");
        description_label.set_label(format!("{:?}", device.get_flags()).as_str());

        if !device.get_flags().contains(&InterfaceFlags::Running) {
            row.style_context().add_class("down");
        }

        list_box.add(&row);
    }

    pub fn add_any(&self) {
        let builder = Builder::from_resource("/com/ethernaut/rust/res/ui/gtk3/device_list_item.ui");
        let row: ListBoxRow = builder
            .object("row")
            .expect("Couldn't find 'row' in device_list_item.ui");

        let icon: Image = builder
            .object("icon")
            .expect("Couldn't find 'icon' in device_list_item.ui");

        row.style_context().add_class("any");
        icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_any.svg"));


        let title_label: Label = builder
            .object("title")
            .expect("Couldn't find 'title' in device_list_item.ui");
        title_label.set_label("Any");

        let description_label: Label = builder
            .object("description")
            .expect("Couldn't find 'description' in device_list_item.ui");
        description_label.set_label("[Promiscuous]");

        self.list_box.add(&row);
    }
}
