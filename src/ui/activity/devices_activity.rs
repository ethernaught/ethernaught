use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use gtk::{gdk, glib, Builder, Container, CssProvider, ListBox, ListBoxRow, Paned, Stack, StyleContext};
use gtk::glib::Cast;
use gtk::glib::ControlFlow::{Break, Continue};
use gtk::prelude::{BuilderExtManual, ContainerExt, CssProviderExt, ListBoxExt, ListBoxRowExt, StackExt};
use pcap::capture::Capture;
use pcap::devices::Device;
use pcap::interface_flags::InterfaceFlags;
use crate::ui::application::OApplication;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::adapters::devices_adapter::DevicesAdapter;
use crate::ui::handlers::bundle::Bundle;
use crate::ui::widgets::graph::Graph;

#[derive(Clone)]
pub struct DevicesActivity {
    app: OApplication,
    root: Option<Container>,
    devices_adapter: Option<DevicesAdapter>
}

impl DevicesActivity {

    pub fn new(app: OApplication) -> Self {
        Self {
            app,
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
        let devices = Rc::new(RefCell::new(devices));

        let app = self.app.clone();
        let devices_clone = Rc::clone(&devices);
        devices_list.connect_row_activated(move |_, row| {
            if row.index() < devices_clone.borrow().len() as i32 {
                let mut bundle = Bundle::new();
                bundle.put("type", String::from("device"));
                bundle.put("device", devices_clone.borrow()[row.index() as usize].clone());
                app.start_activity(Box::new(MainActivity::new(app.clone())), Some(bundle));
                return;
            }

            let mut bundle = Bundle::new();
            bundle.put("type", String::from("device"));
            app.start_activity(Box::new(MainActivity::new(app.clone())), Some(bundle));
        });

        self.devices_adapter = Some(device_adapter);



        let (tx, rx) = channel();

        thread::spawn(move || {
            let mut cap = Capture::any().expect("Failed to open device");
            cap.set_immediate_mode(true);
            cap.open().expect("Failed to start capture");

            loop {
                match cap.next_packet() {
                    Ok((address, packet)) => {
                        tx.send((address.sll_ifindex, packet.len())).unwrap();
                    }
                    _ => {
                        break;
                    }
                }
            }
        });

        let app = self.app.clone();
        glib::timeout_add_local(Duration::from_millis(1000), move || {
            let mut buf = HashMap::new();
            devices.borrow().iter().for_each(|d| {
                if d.get_flags().contains(&InterfaceFlags::Running) {
                    buf.insert(d.get_index(), 0);
                }
            });

            let any = devices.borrow().len() as i32;
            buf.insert(any, 0);

            loop {
                match rx.try_recv() {
                    Ok((index, len)) => {
                        *buf.get_mut(&index).unwrap() += len;
                        *buf.get_mut(&any).unwrap() += len;
                        //*buf.entry(index).or_insert(0) += len;
                    }
                    _ => {
                        break;
                    }
                }
            }

            println!("{:?}", buf);

            //println!("{:?}", row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap());

            //let row_root = app.get_child_by_name::<gtk::Box>(row.upcast_ref(), "row_root").unwrap();

            //let graph = app.get_child_by_name::<Graph>(row_root.upcast_ref(), "graph").unwrap();
            //graph.add_point(buf.get(&index).unwrap().clone() as u32);


            let mut i = 0;
            devices.borrow().iter().for_each(|d| {
                if d.get_flags().contains(&InterfaceFlags::Running) {
                    let row = devices_list.row_at_index(i).unwrap();
                    row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                        .downcast_ref::<Graph>().unwrap().add_point(buf.get(&d.get_index()).unwrap().clone() as u32);
                }

                /*
                if buf.contains_key(&d.get_index()) {
                    let row = devices_list.row_at_index(i).unwrap();
                    row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                        .downcast_ref::<Graph>().unwrap().add_point(buf.get(&d.get_index()).unwrap().clone() as u32);
                }*/
                i += 1;
            });

            let row = devices_list.row_at_index(any).unwrap();
            row.children().get(0).unwrap().downcast_ref::<gtk::Box>().unwrap().children().get(1).unwrap()
                .downcast_ref::<Graph>().unwrap().add_point(buf.get(&any).unwrap().clone() as u32);

            Continue
        });








        &self.root.as_ref().unwrap().upcast_ref()
    }

    fn on_resume(&self) {
    }

    fn on_pause(&self) {
    }

    fn on_destroy(&self) {
    }

    fn get_application(&self) -> &OApplication {
        &self.app
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
