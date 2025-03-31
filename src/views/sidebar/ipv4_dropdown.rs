use std::net::IpAddr;
use gtk::{gdk, Builder, Button, Container, CssProvider, Image, Label, ListBox, Orientation, StyleContext};
use gtk::gio::SimpleActionGroup;
use gtk::glib::Cast;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{BuilderExtManual, ButtonExt, ContainerExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, WidgetExt};
use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use rlibpcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::utils::ip_utils::ip_to_icon;
use crate::widgets::hex_editor::HexEditor;

pub struct Ipv4Dropdown {
    pub root: gtk::Box,
    pub dropdown_button: Button,
    pub expander_icon: Image,
    pub label: Label
}

impl Ipv4Dropdown {

    pub fn from_layer(layer: &EthernetFrame) -> Self {

        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/layer_dropdown.ui");

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in layer_dropdown.ui");

        let dropdown_button: Button = builder
            .object("dropdown_button")
            .expect("Couldn't find 'dropdown_button' in layer_dropdown.ui");

        let expander_icon: Image = builder
            .object("expander_icon")
            .expect("Couldn't find 'expander_icon' in layer_dropdown.ui");

        let label: Label = builder
            .object("label")
            .expect("Couldn't find 'label' in layer_dropdown.ui");
        label.set_text(&layer.get_title("frame"));

        /*
        list_box.add(&create_row("Version:", layer.get_version().to_string()));
        list_box.add(&create_row("TOS:", layer.get_tos().to_string())); // SHOULD BE - Differentiated Services Field
        list_box.add(&create_row("Total Length:", layer.get_total_length().to_string()));
        list_box.add(&create_row("Identification:", format!("0x{:04X} ({})", layer.get_identification(), layer.get_identification())));
        //list_box.add(&create_row(format!("Header: ({})", layer.get_version()).as_str())); //FLAGS
        list_box.add(&create_row("Time to Live:", layer.get_ttl().to_string()));
        list_box.add(&create_row("Protocol:", format!("{:?} ({})", layer.get_protocol(), layer.get_protocol().get_code())));

        let checksum_string = if layer.validate_checksum() { "correct" } else { "incorrect" };
        list_box.add(&create_row("Header Checksum:", format!("0x{:04X} [{}]", layer.get_checksum(), checksum_string)));

        match ip_to_icon(db, IpAddr::V4(layer.get_source_address())) {
            Some(icon) => {
                list_box.add(&create_row_with_icon("Source Address:", icon, layer.get_source_address().to_string()));
            }
            None => {
                list_box.add(&create_row("Source Address:", layer.get_source_address().to_string()));
            }
        }

        match ip_to_icon(db, IpAddr::V4(layer.get_destination_address())) {
            Some(icon) => {
                list_box.add(&create_row_with_icon("Destination Address:", icon, layer.get_destination_address().to_string()));
            }
            None => {
                list_box.add(&create_row("Destination Address:", layer.get_destination_address().to_string()));
            }
        }*/

        Self {
            root,
            dropdown_button,
            expander_icon,
            label
        }
    }


    fn create_dropdown(title: &str, offset: usize, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &dyn LayerExt) -> (Container, ListBox) {
        let dropdown = gtk::Box::new(Orientation::Vertical, 0);
        dropdown.set_widget_name("dropdown");
        dropdown.show();

        let hbox = gtk::Box::new(Orientation::Horizontal, 10);
        let icon = Image::from_resource("/net/ethernaught/rust/res/icons/ic_expand_less.svg");

        let label = Label::new(Some(title));
        label.set_xalign(0.0);

        hbox.add(&icon);
        hbox.add(&label);

        let button = Button::new();
        button.set_child(Some(&hbox));

        let list_box = ListBox::new();

        /*
        list_box.connect_row_activated({
            let hex_editor = hex_editor.clone();
            let layer = layer.clone();
            move |_, row| {
                let (x, w) = layer.get_selection(layer.get_variables().get(row.index() as usize).unwrap().clone());
                hex_editor.set_selection(offset + x, w);
            }
        });

        list_box.connect_button_press_event({
            let hex_editor = hex_editor.clone();
            let layer = layer.clone();
            let actions = actions.clone();
            move |list_box, event| {
                if event.button() != 3 {
                    return Proceed;
                }

                let (_, y) = event.position();

                if let Some(row) = list_box.row_at_y(y as i32) {
                    let variable = layer.get_variables().get(row.index() as usize).unwrap().clone();

                    //create_row_context_menu(&row, event, &actions, variable, layer.as_ref());

                    let (x, w) = layer.get_selection(variable);
                    hex_editor.set_selection(offset + x, w);
                }

                Proceed
            }
        });*/

        let list_box_clone = list_box.clone();
        button.connect_clicked(move |_| {
            list_box_clone.set_visible(!list_box_clone.is_visible());

            if list_box_clone.is_visible() {
                icon.set_from_resource(Some("/net/ethernaught/rust/res/icons/ic_expand_more.svg"));
                return;
            }

            icon.set_from_resource(Some("/net/ethernaught/rust/res/icons/ic_expand_less.svg"));
        });

        dropdown.add(&button);
        button.show_all();

        (dropdown.upcast(), list_box)
    }
}
