use std::net::IpAddr;
use gtk::{gdk, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Orientation, StyleContext};
use gtk::gio::SimpleActionGroup;
use gtk::glib::Cast;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{BuilderExtManual, ButtonExt, ContainerExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, TextExt, WidgetExt};
use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use rlibpcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::utils::ip_utils::ip_to_icon;
use crate::views::sidebar_view::SidebarView;
use crate::widgets::hex_editor::HexEditor;

pub struct Dropdown {
    pub root: gtk::Box,
    pub dropdown_button: Button,
    pub label: Label,
    pub list_box: ListBox
}

impl Dropdown {

    pub fn from_layer(sidebar_view: &SidebarView, layer: &EthernetFrame, offset: usize) -> Self {
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

        let list_box: ListBox = builder
            .object("list_box")
            .expect("Couldn't find 'list_box' in layer_dropdown.ui");

        dropdown_button.connect_clicked({
            let list_box = list_box.clone();
            move |_| {
                list_box.set_visible(!list_box.is_visible());

                if list_box.is_visible() {
                    expander_icon.set_from_resource(Some("/net/ethernaught/rust/res/icons/ic_expand_more.svg"));
                    return;
                }

                expander_icon.set_from_resource(Some("/net/ethernaught/rust/res/icons/ic_expand_less.svg"));
            }
        });

        list_box.connect_row_activated({
            let hex_editor = sidebar_view.hex_editor.clone();
            let layer = layer.clone_ext();
            move |_, row| {
                let (x, w) = layer.get_selection(layer.get_variables().get(row.index() as usize).unwrap().clone());
                hex_editor.set_selection(offset + x, w);
            }
        });

        list_box.connect_button_press_event({
            let hex_editor = sidebar_view.hex_editor.clone();
            let layer = layer.clone_ext();
            //let actions = actions.clone();
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
        });

        for variable in layer.get_variables() {
            let row = create_row(layer.get_title(variable), layer.get_value(variable));
            list_box.add(&row);
        }

        Self {
            root,
            dropdown_button,
            label,
            list_box
        }
    }
}

fn create_row(key: String, value: String) -> ListBoxRow {
    let row = ListBoxRow::new();

    let hbox = gtk::Box::new(Orientation::Horizontal, 10);

    let label = Label::new(Some(key.as_str()));
    label.set_widget_name("key");
    label.set_xalign(0.0);
    hbox.add(&label);

    let label = Label::new(Some(value.as_str()));
    label.set_widget_name("value");
    label.set_xalign(0.0);
    hbox.add(&label);

    row.add(&hbox);
    row.show_all();

    row
}
