use std::cell::RefCell;
use gtk::{gdk, gio, ApplicationWindow, Builder, Container, CssProvider, Label, ListBox, StyleContext, Window};
use gtk::glib::{Cast, Variant, VariantDict};
use gtk::prelude::{ActionGroupExt, BuilderExtManual, ContainerExt, CssProviderExt, ListBoxExt, ListBoxRowExt, WidgetExt};
use pcap::devices::Device;
use pcap::utils::interface_flags::InterfaceFlags;
use crate::bus::event_bus::{pause_event, register_event, resume_event, unregister_event};
use crate::bus::events::inter::event::Event;
use crate::bus::events::transmitted_event::TransmittedEvent;
use crate::pcap_ext::devices::Serialize;
use crate::views::device_list_item::DeviceListItem;
use crate::views::inter::stackable::Stackable;

pub struct DevicesView {
    pub root: gtk::Box,
    pub devices_list: ListBox,
    pub event_listener: RefCell<u32>
}

impl DevicesView {

    pub fn new(window: &ApplicationWindow, devices: Vec<Device>) -> Self {
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
            .expect("Couldn't find 'devices_list' in devices_activity.ui");
        devices_list.set_selection_mode(gtk::SelectionMode::Single);



        devices_list.connect_row_activated({
            let window = window.clone();
            let devices = devices.clone();
            move |_, row| {
                let mut dict = VariantDict::new(None);
                dict.insert_value("name", &Variant::from("main_view"));

                if row.index() > 0 {
                    dict.insert_value("type", &Variant::from("device"));
                    dict.insert_value("device", &Variant::from(&devices[row.index() as usize - 1].serialize().as_slice()));
                    let params = dict.end();

                    window.activate_action("view", Some(&params));
                    return;
                }

                dict.insert_value("type", &Variant::from("any"));
                let params = dict.end();

                window.activate_action("view", Some(&params));
            }
        });

        let mut device_views = Vec::new();
        let mut if_map = Vec::new();

        let device_item = DeviceListItem::new();
        devices_list.add(&device_item.root);
        if_map.push((0, -1));
        device_views.push(device_item);

        let mut i = 1;
        devices.iter().for_each(|device| {
            if device.flags.contains(&InterfaceFlags::Running) {
                if_map.push((i, device.index));
            }
            i += 1;

            let device_item = DeviceListItem::from_device(device);
            devices_list.add(&device_item.root);
            device_views.push(device_item);
        });

        let event_listener = RefCell::new(register_event("transmitted_event", move |event| {
            let event = event.as_any().downcast_ref::<TransmittedEvent>().unwrap();

            if_map.iter().for_each(|(pos, index)| {
                if event.if_bytes.contains_key(index) {
                    device_views.get(*pos).unwrap().graph.add_point(event.if_bytes.get(index).unwrap().clone() as u32);
                } else {
                    device_views.get(*pos).unwrap().graph.add_point(0);
                }
            });
        }, false));

        Self {
            root,
            devices_list,
            event_listener
        }
    }
}

impl Stackable for DevicesView {

    fn get_name(&self) -> String {
        "devices_view".to_string()
    }

    fn get_root(&self) -> &Container {
        self.root.upcast_ref()
    }

    fn on_create(&self) {
    }

    fn on_resume(&self) {
        resume_event("transmitted_event", self.event_listener.borrow().clone());
    }

    fn on_pause(&self) {
        pause_event("transmitted_event", self.event_listener.borrow().clone());
    }

    fn on_destroy(&self) {
        unregister_event("transmitted_event", self.event_listener.borrow().clone());
    }
}
