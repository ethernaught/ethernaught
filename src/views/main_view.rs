use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use gtk::{gdk, Builder, Container, CssProvider, Paned, StyleContext};
use gtk::prelude::{BuilderExtManual, Cast, ContainerExt, CssProviderExt, ImageExt, LabelExt, ListModelExtManual, PanedExt, StyleContextExt, WidgetExt, WidgetExtManual};
use pcap::devices::Device;
use pcap::pcap::pcap::Pcap;
use pcap::utils::data_link_types::DataLinkTypes;
use crate::views::inter::stackable::Stackable;
use crate::views::packets_view::PacketsView;
use crate::views::sidebar_view::SidebarView;
use crate::windows::main_window::MainWindow;

pub struct MainView {
    pub show_title_bar: Box<dyn Fn(bool)>,
    pub root: gtk::Box,
    pub activity_pane: Paned,
    pub content_pane: Paned,
    pub packets: PacketsView,
    pub sidebar: Rc<RefCell<Option<SidebarView>>>,
}

impl MainView {

    pub fn new(window: &MainWindow) -> Self {
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

        let activity_pane: Paned = builder
            .object("activity_pane")
            .expect("Couldn't find 'activity_pane' in main_view.ui");

        let content_pane: Paned = builder
            .object("content_pane")
            .expect("Couldn't find 'content_pane' in main_view.ui");
        activity_pane.set_child_shrink(content_pane.upcast_ref::<Container>(), false);
        activity_pane.set_child_resize(content_pane.upcast_ref::<Container>(), true);

        let show_title_bar = Box::new(show_title_bar(window, "Any", DataLinkTypes::Null));

        let sidebar = Rc::new(RefCell::new(None::<SidebarView>));

        let packets = PacketsView::new();
        packets.connect_select({
            let content_pane = content_pane.clone();
            let sidebar = sidebar.clone();
            move |packet| {
                if let Some(sidebar) = sidebar.borrow().as_ref() {
                    content_pane.remove(&sidebar.root);
                }

                let view = SidebarView::from_packet(packet);
                content_pane.add(&view.root);
                content_pane.set_child_shrink(&view.root, false);
                *sidebar.borrow_mut() = Some(view);
            }
        });
        content_pane.add(&packets.root);

        Self {
            show_title_bar,
            root,
            activity_pane,
            content_pane,
            packets,
            sidebar: Rc::new(RefCell::new(None))
        }
    }

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

        let activity_pane: Paned = builder
            .object("activity_pane")
            .expect("Couldn't find 'activity_pane' in main_view.ui");

        let content_pane: Paned = builder
            .object("content_pane")
            .expect("Couldn't find 'content_pane' in main_view.ui");
        activity_pane.set_child_shrink(content_pane.upcast_ref::<Container>(), false);
        activity_pane.set_child_resize(content_pane.upcast_ref::<Container>(), true);

        let show_title_bar = Box::new(show_title_bar(window, &device.name, device.data_link_type));

        let sidebar = Rc::new(RefCell::new(None::<SidebarView>));

        let packets = PacketsView::new();
        packets.connect_select({
            let content_pane = content_pane.clone();
            let sidebar = sidebar.clone();
            move |packet| {
                if let Some(sidebar) = sidebar.borrow().as_ref() {
                    content_pane.remove(&sidebar.root);
                }

                let view = SidebarView::from_packet(packet);
                content_pane.add(&view.root);
                content_pane.set_child_shrink(&view.root, false);
                *sidebar.borrow_mut() = Some(view);
            }
        });
        content_pane.add(&packets.root);

        Self {
            show_title_bar,
            root,
            activity_pane,
            content_pane,
            packets,
            sidebar
        }
    }

    pub fn from_pcap(window: &MainWindow, path: &PathBuf) -> Self {
        let pcap = Pcap::from_file(path.to_str().unwrap()).expect("Couldn't parse pcap");

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

        let activity_pane: Paned = builder
            .object("activity_pane")
            .expect("Couldn't find 'activity_pane' in main_view.ui");

        let content_pane: Paned = builder
            .object("content_pane")
            .expect("Couldn't find 'content_pane' in main_view.ui");
        activity_pane.set_child_shrink(content_pane.upcast_ref::<Container>(), false);
        activity_pane.set_child_resize(content_pane.upcast_ref::<Container>(), true);

        let show_title_bar = Box::new(show_title_bar(window, path.file_name().unwrap().to_str().unwrap(), pcap.get_data_link_type()));

        let sidebar = Rc::new(RefCell::new(None::<SidebarView>));

        let mut packets = PacketsView::from_pcap(pcap);
        packets.connect_select({
            let content_pane = content_pane.clone();
            let sidebar = sidebar.clone();
            move |packet| {
                if let Some(sidebar) = sidebar.borrow().as_ref() {
                    content_pane.remove(&sidebar.root);
                }

                let view = SidebarView::from_packet(packet);
                content_pane.add(&view.root);
                content_pane.set_child_shrink(&view.root, false);
                *sidebar.borrow_mut() = Some(view);
            }
        });
        content_pane.add(&packets.root);

        Self {
            show_title_bar,
            root,
            activity_pane,
            content_pane,
            packets,
            sidebar
        }
    }

    pub fn open_terminal(&mut self) {

    }

    pub fn close_terminal(&mut self) {

    }
}

impl Stackable for MainView {

    fn get_name(&self) -> String {
        "main_view".to_string()
    }

    fn get_root(&self) -> &Container {
        self.root.upcast_ref()
    }

    fn on_create(&self) {
        (self.show_title_bar)(true);
    }

    fn on_resume(&self) {
        (self.show_title_bar)(true);
    }

    fn on_pause(&self) {
        (self.show_title_bar)(false);
    }

    fn on_destroy(&self) {
    }
}

fn show_title_bar(window: &MainWindow, name: &str, data_link_type: DataLinkTypes) -> impl Fn(bool) {
    let title_bar = window.title_bar.clone();
    let name = name.to_string();
    move |shown| {
        if shown {
            match data_link_type {
                DataLinkTypes::Null => {
                    title_bar.root.style_context().add_class("any");
                    title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_any.svg"));
                }
                DataLinkTypes::En10mb | DataLinkTypes::En3mb | DataLinkTypes::Sll2 => {
                    title_bar.root.style_context().add_class("ethernet");
                    title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_ethernet.svg"));
                }
                DataLinkTypes::Loop => {
                    title_bar.root.style_context().add_class("lan");
                    title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_lan.svg"));
                }
                DataLinkTypes::Raw | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                    title_bar.root.style_context().add_class("vpn");
                    title_bar.network_type_icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_vpn.svg"));
                }
                /*
                DataLinkTypes::BluetoothHciH4 => {
                    titlebar.style_context().add_class("bluetooth");
                    icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_bluetooth.svg"));
                }
                */
                _ => {}
            }

            title_bar.network_type_label.set_label(&name);

            title_bar.network_type_icon.show();
            title_bar.network_type_label.show();
            return;
        }

        match data_link_type {
            DataLinkTypes::Null => {
                title_bar.root.style_context().remove_class("any");
            }
            DataLinkTypes::En10mb | DataLinkTypes::En3mb | DataLinkTypes::Sll2 => {
                title_bar.root.style_context().remove_class("ethernet");
            }
            DataLinkTypes::Loop => {
                title_bar.root.style_context().remove_class("lan");
            }
            DataLinkTypes::Raw | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                title_bar.root.style_context().remove_class("vpn");
            }
            /*
            DataLinkTypes::BluetoothHciH4 => {
                titlebar.style_context().remove_class("bluetooth");
            }
            */
            _ => {}
        }

        title_bar.network_type_icon.hide();
        title_bar.network_type_label.hide();
    }
}
