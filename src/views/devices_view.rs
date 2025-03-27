use gtk::{gdk, Builder, Container, CssProvider, ListBox, StyleContext};
use gtk::prelude::{BuilderExtManual, ContainerExt, CssProviderExt, ListBoxExt, WidgetExt};
use pcap::devices::Device;
use pcap::utils::interface_flags::InterfaceFlags;
use crate::views::device_list_item::DeviceListItem;
use crate::views::inter::view::View;

pub struct DevicesView {
    pub root: gtk::Box,
    pub devices_list: ListBox
}

impl DevicesView {

    pub fn new(devices: Vec<Device>) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/devices_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/devices_view.css");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in devices_view.ui");


        let devices_list: ListBox = builder
            .object("devices_list")
            .expect("Couldn't find 'devices_list' in devices_activity.oldui");
        devices_list.set_selection_mode(gtk::SelectionMode::Single);



        devices.iter().for_each(|d| {
            let device_item = DeviceListItem::new(d);
            devices_list.add(&device_item.root);
        });




        Self {
            root,
            devices_list
        }
    }
}

impl View for DevicesView {

    fn get_name(&self) -> String {
        "devices_view".to_string()
    }

    fn get_title(&self) -> String {
        "DevicesView".to_string()
    }
}
