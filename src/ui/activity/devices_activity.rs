use gtk::{gdk, Builder, Container, CssProvider, ListBox, Paned, Stack, StyleContext};
use gtk::glib::Cast;
use gtk::prelude::{BuilderExtManual, CssProviderExt, ListBoxExt, ListBoxRowExt, StackExt};
use pcap::devices::Device;
use crate::ui::application::OApplication;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::adapters::devices_adapter::DevicesAdapter;
use crate::ui::fragment::inter::fragment::Fragment;

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

    fn on_create(&mut self) -> &Container {
        let builder = Builder::from_file("res/ui/gtk3/devices-activity.ui");

        let provider = CssProvider::new();
        provider.load_from_path("res/ui/gtk3/devices-activity.css").expect("Failed to load CSS file.");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        self.root = Some(builder
            .object("devices_layout")
            .expect("Couldn't find 'devices_layout' in devices-activity.ui"));


        let list_box: ListBox = builder
            .object("list_box")
            .expect("Couldn't find 'list_box' in devices-activity.ui");
        list_box.set_selection_mode(gtk::SelectionMode::Single);

        let device_adapter = DevicesAdapter::new(&list_box);

        let devices = Device::list().expect("Failed to get device list");
        devices.iter().for_each(|d| {
            device_adapter.add(d);
        });

        let app = self.app.clone();
        list_box.connect_row_activated(move |_, row| {
            app.start_activity(Box::new(MainActivity::new(app.clone(), &devices[row.index() as usize])));
        });

        self.devices_adapter = Some(device_adapter);

        &self.root.as_ref().unwrap().upcast_ref()
    }

    fn on_resume(&self) {
        todo!()
    }

    fn on_pause(&self) {
        todo!()
    }

    fn on_destroy(&self) {
        todo!()
    }

    fn start_fragment(&self, fragment: Box<dyn Fragment>) {

    }

    fn dyn_clone(&self) -> Box<dyn Activity> {
        Box::new(self.clone())
    }
}
