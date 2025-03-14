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

    pub fn add(&self, device: &Device) {
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

        if device.get_flags().contains(&InterfaceFlags::Running) {
            let graph: Graph = builder
                .object("graph")
                .expect("Couldn't find 'graph' in device_list_item.ui");

            /*
            let values = vec![65, 53, 93, 7, 90,
                29, 97, 15, 36, 20,
                11, 23, 23, 28, 83,
                90, 31, 18, 89, 1,
                71, 76, 83, 82, 57,
                21, 84, 6, 9, 1];
            graph.set_points(values);
            */

        } else {
            row.style_context().add_class("down");
        }

        self.list_box.add(&row);
    }
}
