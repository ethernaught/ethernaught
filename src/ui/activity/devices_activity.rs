use std::any::Any;
use gtk::{gdk, Builder, Container, CssProvider, ListBox, Paned, Stack, StyleContext};
use gtk::glib::Cast;
use gtk::prelude::{BuilderExtManual, CssProviderExt, ListBoxExt, ListBoxRowExt, StackExt};
use pcap::devices::Device;
use crate::ui::application::OApplication;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::adapters::devices_adapter::DevicesAdapter;
use crate::ui::handlers::bundle::Bundle;

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

        let device_adapter = DevicesAdapter::new(&devices_list);

        let devices = Device::list().expect("Failed to get device list");
        devices.iter().for_each(|d| {
            device_adapter.add(d);
        });

        device_adapter.add_any();

        let app = self.app.clone();
        devices_list.connect_row_activated(move |_, row| {
            if row.index() < devices.len() as i32 {
                let mut bundle = Bundle::new();
                bundle.put("type", String::from("device"));
                bundle.put("device", devices[row.index() as usize].clone());
                app.start_activity(Box::new(MainActivity::new(app.clone())), Some(bundle));
                return;
            }

            let mut bundle = Bundle::new();
            bundle.put("type", String::from("device"));
            app.start_activity(Box::new(MainActivity::new(app.clone())), Some(bundle));
        });

        self.devices_adapter = Some(device_adapter);

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
