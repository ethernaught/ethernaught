use gtk::{gdk, Builder, Container, CssProvider, StyleContext};
use gtk::prelude::{BuilderExtManual, Cast, CssProviderExt, ImageExt, LabelExt, StyleContextExt, WidgetExt, WidgetExtManual};
use pcap::devices::Device;
use pcap::utils::data_link_types::DataLinkTypes;
use crate::views::inter::view::View;
use crate::windows::main_window::MainWindow;

pub struct MainView {
    pub root: gtk::Box
}

impl MainView {

    pub fn from_device(window: &MainWindow, device: &Device) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/main_view.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/main_view.css");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in main_view.ui");

        match device.data_link_type {
            DataLinkTypes::Null => {
                window.title_bar.root.style_context().add_class("any");
                window.title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_any.svg"));
            }
            DataLinkTypes::En10mb | DataLinkTypes::En3mb | DataLinkTypes::Sll2 => {
                window.title_bar.root.style_context().add_class("ethernet");
                window.title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_ethernet.svg"));
            }
            DataLinkTypes::Loop => {
                window.title_bar.root.style_context().add_class("lan");
                window.title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_lan.svg"));
            }
            DataLinkTypes::Raw | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                window.title_bar.root.style_context().add_class("vpn");
                window.title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_vpn.svg"));
            }
            /*
            DataLinkTypes::BluetoothHciH4 => {
                titlebar.style_context().add_class("bluetooth");
                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_bluetooth.svg"));
            }
            */
            _ => {}
        }

        window.title_bar.network_type_label.set_label(&device.name);

        window.title_bar.network_type_icon.show();
        window.title_bar.network_type_label.show();

        Self {
            root
        }
    }
}

impl View for MainView {

    fn get_name(&self) -> String {
        "main_view".to_string()
    }

    fn get_root(&self) -> &Container {
        self.root.upcast_ref()
    }

    fn on_resume(&self) {
        println!("RESUME {}", self.get_name());
    }

    fn on_pause(&self) {
        println!("PAUSE {}", self.get_name());
    }

    fn on_destroy(&self) {
        todo!()
    }
}
