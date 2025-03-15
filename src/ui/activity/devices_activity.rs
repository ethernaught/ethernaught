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
use crate::ui::application::OApplication;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::adapters::devices_adapter::DevicesAdapter;
use crate::ui::context::Context;
use crate::ui::handlers::bundle::Bundle;
use crate::ui::handlers::handler::Runnable;
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
        let builder = Builder::from_resource("/com/ethernaut/rust/res/ui/gtk3/devices_activity.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/com/ethernaut/rust/res/ui/gtk3/devices_activity.css");

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
        device_adapter.add_any();

        let mut index_map = Vec::new();
        devices.iter().for_each(|device| {
            index_map.push(device.get_index());
        });

        let context = self.context.clone();
        devices_list.connect_row_activated(move |_, row| {
            if row.index() < devices.len() as i32 {
                let mut bundle = Bundle::new();
                bundle.put("type", String::from("device"));
                bundle.put("device", devices[row.index() as usize].clone());
                context.start_activity(Box::new(MainActivity::new(context.clone())), Some(bundle));
                return;
            }

            let mut bundle = Bundle::new();
            bundle.put("type", String::from("device"));
            context.start_activity(Box::new(MainActivity::new(context.clone())), Some(bundle));
        });

        self.devices_adapter = Some(device_adapter);





        let tx = self.context.get_handler().get_sender();
        thread::spawn(move || {
            let mut cap = Capture::any().expect("Failed to open device");
            cap.set_immediate_mode(true);
            cap.open().expect("Failed to start capture");

            let mut index_bytes = HashMap::new();

            let mut time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            loop {
                match cap.try_recv() {
                    Ok((address, packet)) => {
                        *index_bytes.entry(address.sll_ifindex).or_insert(0) += packet.len();
                    }
                    _ => {}
                }

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis();

                if now-time >= 1000 {
                    time = now;

                    let mut bundle = Bundle::new();
                    bundle.put("index_bytes", index_bytes.clone());

                    tx.send((String::from("device_activity"), Some(bundle))).unwrap();

                    index_bytes.clear();
                }
                thread::sleep(Duration::from_millis(1));
            }
        });

        self.context.get_handler().post_runnable("device_activity", move |bundle| {
            match bundle {
                Some(bundle) => {
                    match bundle.get::<HashMap<i32, usize>>("index_bytes") {
                        Some(index_bytes) => {
                            let mut i = 0;
                            index_map.iter().for_each(|index| {
                                if index_bytes.contains_key(index) {
                                    let row = devices_list.row_at_index(i).unwrap();
                                    row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                                        .downcast_ref::<Graph>().unwrap().add_point(index_bytes.get(index).unwrap().clone() as u32);
                                }
                                i += 1;
                            });

                        }
                        None => {}
                    }
                }
                None => {}
            }
        });

        &self.root.as_ref().unwrap().upcast_ref()
    }

    fn on_resume(&self) {
    }

    fn on_pause(&self) {
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
