use gtk::{gdk, gio, ApplicationWindow, Builder, Container, CssProvider, Label, ListBox, StyleContext, Window};
use gtk::glib::{Cast, Variant, VariantDict};
use gtk::prelude::{ActionGroupExt, BuilderExtManual, ContainerExt, CssProviderExt, ListBoxExt, ListBoxRowExt, WidgetExt};
use pcap::devices::Device;
use crate::pcap_ext::devices::Serialize;
use crate::views::device_list_item::DeviceListItem;
use crate::views::inter::stackable::Stackable;

pub struct DevicesView {
    pub root: gtk::Box,
    pub devices_list: ListBox
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
                if row.index() > 0 {
                    //let mut bundle = Bundle::new();
                    //bundle.put("type", String::from("device"));
                    //bundle.put("device", devices_clone[row.index() as usize - 1].clone());
                    //context.start_activity(Box::new(MainActivity::new(context.clone())), Some(bundle));
                    //let view = MainView::from_device(&devices[row.index() as usize - 1]);

                    let variant = Variant::from(&devices[row.index() as usize - 1].serialize().as_slice());

                    window.activate_action("open", Some(&variant));

                    return;
                }

                //let mut bundle = Bundle::new();
                //bundle.put("type", String::from("device"));
                //context.start_activity(Box::new(MainActivity::new(context.clone())), Some(bundle));
            }
        });

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

impl Stackable for DevicesView {

    fn get_name(&self) -> String {
        "devices_view".to_string()
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
