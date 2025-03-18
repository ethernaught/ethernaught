use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use gtk::{gdk, glib, Builder, Container, CssProvider, ListBox, ListBoxRow, Paned, Stack, StyleContext};
use gtk::glib::{Cast, PropertyGet, Receiver, Sender};
use gtk::glib::ControlFlow::{Break, Continue};
use gtk::prelude::{BuilderExtManual, ContainerExt, CssProviderExt, GridExt, ListBoxExt, ListBoxRowExt, StackExt};
use pcap::capture::Capture;
use pcap::devices::Device;
use pcap::interface_flags::InterfaceFlags;
use crate::qsync::task::Task;
use crate::ui::application::OApplication;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::adapters::devices_adapter::DevicesAdapter;
use crate::ui::context::Context;
use crate::ui::handlers::bundle::Bundle;
use crate::ui::handlers::events::capture_event::CaptureEvent;
use crate::ui::handlers::events::inter::event::Event;
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
            .object("devices_layout")
            .expect("Couldn't find 'devices_layout' in devices_activity.ui"));


        let devices_list: ListBox = builder
            .object("devices_list")
            .expect("Couldn't find 'devices_list' in devices_activity.ui");
        devices_list.set_selection_mode(gtk::SelectionMode::Single);


        let devices = Device::list().expect("Failed to get device list");
        let device_adapter = DevicesAdapter::from_devices(&devices_list, devices.clone());

        let context = self.context.clone();
        devices_list.connect_row_activated(move |_, row| {
            if row.index() > 0 {
                let mut bundle = Bundle::new();
                bundle.put("type", String::from("device"));
                bundle.put("device", devices[row.index() as usize - 1].clone());
                context.start_activity(Box::new(MainActivity::new(context.clone())), Some(bundle));
                return;
            }

            let mut bundle = Bundle::new();
            bundle.put("type", String::from("device"));
            context.start_activity(Box::new(MainActivity::new(context.clone())), Some(bundle));
        });





        let tx = self.context.get_handler().get_sender();

        self.context.get_task().spawn(async move {
            let mut cap = Capture::any().expect("Failed to open device");
            cap.set_immediate_mode(true);
            cap.open().expect("Failed to start capture");

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

                        let event = CaptureEvent::new(address.sll_ifindex, packet);
                        tx.send(Box::new(event)).unwrap();
                    }
                    _ => {}
                }

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis();

                if now-time >= 250 {
                    time = now;

                    let event = TransmittedEvent::new(if_bytes.clone());
                    tx.send(Box::new(event)).unwrap();

                    if_bytes.clear();
                }

                Task::delay_for(Duration::from_millis(1)).await;
            }
        });

        let device_adapter_clone = device_adapter.clone();
        self.context.get_handler().register_listener("transmitted_event", move |event| {
            let event = event.as_any().downcast_ref::<TransmittedEvent>().unwrap();

            let mut i = 0;
            device_adapter_clone.if_map.borrow().iter().for_each(|index| {
                let row = devices_list.row_at_index(i).unwrap();

                if event.if_bytes.contains_key(index) {
                    row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                        .downcast_ref::<Graph>().unwrap().add_point(event.if_bytes.get(index).unwrap().clone() as u32);

                } else {
                    row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                        .downcast_ref::<Graph>().unwrap().add_point(0);
                }

                i += 1;
            });
        });

        self.devices_adapter = Some(device_adapter);

        &self.root.as_ref().unwrap().upcast_ref()
    }

    fn on_resume(&self) {
        let devices_adapter = self.devices_adapter.as_ref().unwrap().clone();

        self.context.get_handler().register_listener("transmitted_event", move |event| {
            let event = event.as_any().downcast_ref::<TransmittedEvent>().unwrap();

            let mut i = 0;
            devices_adapter.if_map.borrow().iter().for_each(|index| {
                let row = devices_adapter.list_box.row_at_index(i).unwrap();

                if event.if_bytes.contains_key(index) {
                    row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                        .downcast_ref::<Graph>().unwrap().add_point(event.if_bytes.get(index).unwrap().clone() as u32);

                } else {
                    row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                        .downcast_ref::<Graph>().unwrap().add_point(0);
                }

                i += 1;
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

        self.context.get_handler().remove_listener("transmitted_event");
    }

    fn on_destroy(&self) {
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
