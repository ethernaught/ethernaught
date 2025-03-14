use gtk::{Builder, DrawingArea, Image, Label, ListBox, ListBoxRow};
use gtk::glib::{Cast, Propagation};
use gtk::prelude::{BinExt, BuilderExtManual, ContainerExt, GLAreaExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, StyleContextExt, WidgetExt};
use pcap::devices::Device;
use pcap::packet::inter::data_link_types::DataLinkTypes;

#[derive(Clone)]
pub struct DevicesAdapter {
    list_box: ListBox
}

impl DevicesAdapter {

    pub fn new(list_box: &ListBox) -> Self {
        Self {
            list_box: list_box.clone()
        }
    }

    pub fn add(&self, device: &Device) {
        let builder = Builder::from_resource("/com/ethernaut/rust/res/ui/gtk3/device_list_item.ui");
        let row: ListBoxRow = builder
            .object("row")
            .expect("Couldn't find 'row' in device_list_item.ui");


        let icon: Image = builder
            .object("icon")
            .expect("Couldn't find 'icon' in device_list_item.ui");

        match device.get_data_link_type() {
            DataLinkTypes::Ethernet => {
                row.style_context().add_class("ethernet");
                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_ethernet.svg"));
            }
            DataLinkTypes::Loopback => {
                row.style_context().add_class("lan");
                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_lan.svg"));
            }
            DataLinkTypes::Raw | DataLinkTypes::Tun | DataLinkTypes::Ipv4 | DataLinkTypes::Ipv6 => {
                row.style_context().add_class("vpn");
                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_vpn.svg"));
            }
            DataLinkTypes::BluetoothHciH4 => {
                row.style_context().add_class("bluetooth");
                icon.set_resource(Some("/com/ethernaut/rust/res/icons/ic_bluetooth.svg"));
            }
            _ => {}
        }


        let title_label: Label = builder
            .object("title")
            .expect("Couldn't find 'title' in device_list_item.ui");
        title_label.set_label(format!("{}", device.get_name()).as_str());


        let description_label: Label = builder
            .object("description")
            .expect("Couldn't find 'description' in device_list_item.ui");
        description_label.set_label(format!("{:?}", device.get_flags()).as_str());


        /*
        let row_root: gtk::Box = builder
            .object("row_root")
            .expect("Couldn't find 'row_root' in packet_list_item.ui");

        let drawing_area = DrawingArea::new();

        let values = [65.29466635230628, 53.0803544621237, 93.19658085271716, 7.642971979832735, 90.71958617195229,
            29.437472243918737, 97.11681837654415, 15.742274423083424, 36.65575615836729, 20.011880035731767,
            11.72352580214181, 23.787189048253985, 23.063004865077442, 28.03720200888149, 83.84072466832403,
            90.65897623947095, 31.719246649394496, 18.706881314764278, 89.75684987935968, 1.7371977127742522,
            71.7181185199138, 76.81933279959775, 83.3660090795961, 82.06193796708725, 57.95041987297941,
            21.28174534950269, 84.76655437939519, 6.191640230754414, 9.581959179918421, 1.3281426350473469];

        let drawing_area_clone = drawing_area.clone();
        drawing_area.connect_draw(move |_, cr| {
            cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            cr.paint().unwrap();

            cr.set_source_rgb(0.341, 0.588, 0.361); //57965C
            cr.set_line_width(1.0);

            if !values.is_empty() {
                let width = drawing_area_clone.allocated_width() as f64;
                let height = drawing_area_clone.allocated_height() as f64;
                let step = width / 30.0;

                cr.move_to(0.0, height - (values[0] * (height / 100.0)));

                for (i, &value) in values.iter().enumerate() {
                    let x = i as f64 * step;
                    let y = height - (value * (height / 100.0));
                    cr.line_to(x, y);
                }

                cr.stroke().unwrap();
            }

            Propagation::Proceed
        });

        drawing_area.set_hexpand(true);

        row_root.add(&drawing_area);
        row.show_all();
        */
        //row.show_all();

        self.list_box.add(&row);
    }
}
