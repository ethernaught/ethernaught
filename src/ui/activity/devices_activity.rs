use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use gtk::{gdk, glib, Builder, Container, CssProvider, ListBox, ListBoxRow, Paned, Stack, StyleContext};
use gtk::glib::{Cast, PropertyGet, Receiver, Sender};
use gtk::glib::ControlFlow::{Break, Continue};
use gtk::prelude::{BuilderExtManual, ContainerExt, CssProviderExt, GridExt, ListBoxExt, ListBoxRowExt, StackExt, StyleContextExt, WidgetExt};
use pcap::capture::Capture;
use pcap::devices::Device;
use pcap::utils::interface_flags::InterfaceFlags;
use crate::ui::application::OApplication;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::adapters::devices_adapter::DevicesAdapter;
use crate::ui::context::Context;
use crate::ui::handlers::bundle::Bundle;
use crate::ui::handlers::events::capture_event::CaptureEvent;
use crate::ui::handlers::events::inter::event::Event;
use crate::ui::handlers::events::permission_event::PermissionEvent;
use crate::ui::handlers::events::transmitted_event::TransmittedEvent;
use crate::ui::widgets::graph::Graph;

#[derive(Clone)]
pub struct DevicesActivity {
    context: Context,
    root: Option<Container>,
    devices_adapter: Option<DevicesAdapter>
}

impl DevicesActivity {

    pub fn new(context: Context) -> Self {
        Self {
            context,
            root: None,
            devices_adapter: None
        }
    }
}

impl Activity for DevicesActivity {

    fn get_name(&self) -> String {
        "devices_activity".to_string()
    }

    fn get_title(&self) -> String {
        "DevicesActivity".to_string()
    }

    fn on_create(&mut self, bundle: Option<Bundle>) -> &Container {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/devices_activity.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/devices_activity.css");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        self.root = Some(builder
            .object("devices_activity_layout")
            .expect("Couldn't find 'devices_activity_layout' in devices_activity.ui"));


        let devices_list: ListBox = builder
            .object("devices_list")
            .expect("Couldn't find 'devices_list' in devices_activity.ui");
        devices_list.set_selection_mode(gtk::SelectionMode::Single);


        let mut devices = Device::list().expect("Failed to get device list");
        devices.sort_by(|a, b| {
            b.get_flags().contains(&InterfaceFlags::Running).cmp(&a.get_flags().contains(&InterfaceFlags::Running))
        });
        let device_adapter = DevicesAdapter::from_devices(&devices_list, devices.clone());

        let context = self.context.clone();
        let devices_clone = devices.clone();
        devices_list.connect_row_activated(move |_, row| {
            if row.index() > 0 {
                let mut bundle = Bundle::new();
                bundle.put("type", String::from("device"));
                bundle.put("device", devices_clone[row.index() as usize - 1].clone());
                context.start_activity(Box::new(MainActivity::new(context.clone())), Some(bundle));
                return;
            }

            let mut bundle = Bundle::new();
            bundle.put("type", String::from("device"));
            context.start_activity(Box::new(MainActivity::new(context.clone())), Some(bundle));
        });


        let tx = self.context.get_event_handler().get_sender();

        #[cfg(target_os = "linux")]
        thread::spawn(move || {
            match Capture::any() {
                Ok(cap) => {
                    cap.set_immediate_mode(true).unwrap();

                    match cap.open() {
                        Ok(_) => {
                            let mut if_bytes = HashMap::new();

                            let mut time = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards")
                                .as_millis();

                            loop {
                                match cap.try_recv() {
                                    Ok((address, packet)) => {
                                        *if_bytes.entry(-1).or_insert(0) += packet.len();
                                        *if_bytes.entry(address.sll_ifindex).or_insert(0) += packet.len();

                                        tx.send(Box::new(CaptureEvent::new(address.sll_ifindex, packet))).unwrap();
                                    }
                                    _ => {}
                                }

                                let now = SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .expect("Time went backwards")
                                    .as_millis();

                                if now-time >= 1000 {
                                    time = now;

                                    tx.send(Box::new(TransmittedEvent::new(if_bytes.clone()))).unwrap();

                                    if_bytes.clear();
                                }

                                sleep(Duration::from_millis(10));
                            }
                        }
                        Err(_) => {
                            tx.send(Box::new(PermissionEvent::new(false))).unwrap();
                        }
                    }
                }
                Err(_) => {
                    tx.send(Box::new(PermissionEvent::new(false))).unwrap();
                }
            }
        });

        #[cfg(target_os = "macos")]
        thread::spawn(move || {
            let mut captures = Vec::new();
            devices.iter().for_each(|device| {
                if device.get_flags().contains(&InterfaceFlags::Running) {
                    match Capture::from_device(device) {
                        Ok(cap) => {
                            cap.set_immediate_mode(true);
                            match cap.open() {
                                Ok(_) => {
                                    captures.push(cap);
                                }
                                Err(_) => {
                                    tx.send(Box::new(PermissionEvent::new(false))).unwrap();
                                    return;
                                }
                            }
                        }
                        Err(_) => {
                            tx.send(Box::new(PermissionEvent::new(false))).unwrap();
                            return;
                        }
                    }
                }
            });

            if captures.is_empty() {
                return;
            }

            let mut if_bytes = HashMap::new();

            let mut time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            loop {
                for cap in &captures {
                    match cap.try_recv() {
                        Ok((address, packet)) => {
                            let device = cap.get_device().unwrap();
                            *if_bytes.entry(-1).or_insert(0) += packet.len();
                            *if_bytes.entry(device.get_index()).or_insert(0) += packet.len();
                            //*if_bytes.entry(address.sll_ifindex).or_insert(0) += packet.len();

                            tx.send(Box::new(CaptureEvent::new(device.get_index(), packet))).unwrap();
                        }
                        _ => {}
                    }
                }

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis();

                if now-time >= 1000 {
                    time = now;

                    let event = TransmittedEvent::new(if_bytes.clone());
                    tx.send(Box::new(event)).unwrap();

                    if_bytes.clear();
                }

                sleep(Duration::from_millis(10));
            }
        });

        self.context.get_event_handler().register_listener("permission_event", {
            let context = self.context.clone();
            let device_adapter = device_adapter.clone();
            let devices_list = devices_list.clone();
            move |event| {
                let event = event.as_any().downcast_ref::<PermissionEvent>().unwrap();

                if event.has_permission() {
                    return;
                }

                context.alert("You don't have permission to read network interfaces.");

                device_adapter.if_map.borrow().iter().for_each(|(pos, _)| {
                    devices_list.row_at_index(*pos).unwrap().style_context().add_class("error");
                });
            }
        });

        self.context.get_event_handler().register_listener("transmitted_event", {
            let device_adapter = device_adapter.clone();
            let devices_list = devices_list.clone();
            move |event| {
                let event = event.as_any().downcast_ref::<TransmittedEvent>().unwrap();

                device_adapter.if_map.borrow().iter().for_each(|(pos, index)| {
                    let row = devices_list.row_at_index(*pos).unwrap();

                    if event.if_bytes.contains_key(index) {
                        row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                            .downcast_ref::<Graph>().unwrap().add_point(event.if_bytes.get(index).unwrap().clone() as u32);
                    } else {
                        row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                            .downcast_ref::<Graph>().unwrap().add_point(0);
                    }
                });
            }
        });

        self.devices_adapter = Some(device_adapter);

        &self.root.as_ref().unwrap().upcast_ref()
    }

    fn on_resume(&self) {
        let devices_adapter = self.devices_adapter.as_ref().unwrap().clone();

        self.context.get_event_handler().register_listener("transmitted_event", move |event| {
            let event = event.as_any().downcast_ref::<TransmittedEvent>().unwrap();

            devices_adapter.if_map.borrow().iter().for_each(|(pos, index)| {
                let row = devices_adapter.list_box.row_at_index(*pos).unwrap();

                if event.if_bytes.contains_key(index) {
                    row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                        .downcast_ref::<Graph>().unwrap().add_point(event.if_bytes.get(index).unwrap().clone() as u32);

                } else {
                    row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                        .downcast_ref::<Graph>().unwrap().add_point(0);
                }
            });
        });
    }

    fn on_pause(&self) {
        let children = self.devices_adapter.as_ref().unwrap().list_box.children();

        for row in children {
            let row = row.downcast::<ListBoxRow>().unwrap();
            row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                .downcast_ref::<Graph>().unwrap().clear_points();
        }

        self.context.get_event_handler().remove_listener("transmitted_event");
    }

    fn on_destroy(&self) {
    }

    fn get_context(&self) -> &Context {
        &self.context
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Activity> {
        Box::new(self.clone())
    }
}
