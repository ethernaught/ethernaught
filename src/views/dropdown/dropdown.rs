use std::net::IpAddr;
use gtk::{gdk, gio, glib, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Menu, Orientation, StyleContext};
use gtk::gdk::{Display, EventButton};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::{ActionGroup, SimpleAction, SimpleActionGroup};
use gtk::glib::Cast;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{ActionMapExt, BuilderExtManual, ButtonExt, ContainerExt, EditableExt, GestureSingleExt, GtkMenuExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, MenuShellExt, StyleContextExt, TextExt, WidgetExt};
use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use rlibpcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
use crate::database::sqlite::Database;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::utils::ethernet_utils::ethernet_to_company;
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

    pub fn new(title: &str) -> Self {
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
        label.set_label(title);

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

        Self {
            root,
            dropdown_button,
            label,
            list_box
        }
    }
}

pub fn set_selection(hex_editor: &HexEditor, layer: &dyn LayerExt, offset: usize) -> impl Fn(&ListBox, &ListBoxRow) + 'static {
    let hex_editor = hex_editor.clone();
    let layer = layer.clone_ext();
    move |_, row| {
        let (x, w) = layer.get_selection(layer.get_fields().get(row.index() as usize).unwrap().clone());
        hex_editor.set_selection(offset + x, w);
    }
}

pub fn context_menu(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &dyn LayerExt, offset: usize) -> impl Fn(&ListBox, &EventButton) -> glib::Propagation + 'static {
    let hex_editor = hex_editor.clone();
    let layer = layer.clone_ext();
    let actions = actions.clone();
    move |list_box, event| {
        if event.button() != 3 {
            return Proceed;
        }

        let (_, y) = event.position();

        if let Some(row) = list_box.row_at_y(y as i32) {
            let variable = layer.get_fields().get(row.index() as usize).unwrap().clone();

            row.style_context().add_class("selected");

            let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/sidebar_context_menu.xml");

            let model: gio::MenuModel = builder
                .object("context_menu")
                .expect("Couldn't find 'context_menu' in sidebar_context_menu.xml");

            let menu = Menu::from_model(&model);

            let action = SimpleAction::new("copy-field-name", None);
            action.connect_activate({
                let value = layer.get_field_name(variable);
                move |_, _| {
                    let display = Display::default().expect("No display available");
                    let clipboard = gtk::Clipboard::default(&display).expect("Failed to get clipboard");
                    clipboard.set_text(&value);
                }
            });
            actions.add_action(&action);

            let action = SimpleAction::new("copy-value", None);
            action.connect_activate({
                let value = layer.get_value(variable);
                move |_, _| {
                    let display = Display::default().expect("No display available");
                    let clipboard = gtk::Clipboard::default(&display).expect("Failed to get clipboard");
                    clipboard.set_text(&value);
                }
            });
            actions.add_action(&action);

            let action = SimpleAction::new("copy-description", None);
            action.connect_activate({
                let value = layer.get_description(variable);
                move |_, _| {
                    let display = Display::default().expect("No display available");
                    let clipboard = gtk::Clipboard::default(&display).expect("Failed to get clipboard");
                    clipboard.set_text(&value);
                }
            });
            actions.add_action(&action);

            let action = SimpleAction::new("copy-byte-array", None);
            action.connect_activate({
                let value = format!("let buf = [{}];", layer.get_value_as_bytes(variable)
                    .chunks(16)
                    .map(|chunk| {
                        chunk
                            .iter()
                            .map(|byte| format!("0x{:02x}", byte))
                            .collect::<Vec<String>>()
                            .join(", ")
                    })
                    .collect::<Vec<String>>()
                    .join(",\n"));
                move |_, _| {
                    let display = Display::default().expect("No display available");
                    let clipboard = gtk::Clipboard::default(&display).expect("Failed to get clipboard");
                    clipboard.set_text(&value);
                }
            });
            actions.add_action(&action);

            let action = SimpleAction::new("copy-hex", None);
            action.connect_activate({
                let value = layer.get_value_as_bytes(variable).chunks(16)
                    .enumerate()
                    .map(|(i, chunk)| {
                        let line_number = format!("{:08X}", i * 16);
                        let hex_values = chunk.iter()
                            .map(|b| format!("{:02X}", b))
                            .collect::<Vec<_>>()
                            .join(" ");
                        format!("{} {}", line_number, hex_values)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                move |_, _| {
                    let display = Display::default().expect("No display available");
                    let clipboard = gtk::Clipboard::default(&display).expect("Failed to get clipboard");
                    clipboard.set_text(&value);
                }
            });
            actions.add_action(&action);

            let action = SimpleAction::new("copy-ascii", None);
            action.connect_activate({
                let value = layer.get_value_as_bytes(variable).chunks(16)
                    .enumerate()
                    .map(|(i, chunk)| {
                        let line_number = format!("{:08X}", i * 16);
                        let ascii_string = chunk.iter()
                            .map(|&b| {
                                if b.is_ascii() && !b.is_ascii_control() {
                                    b as char
                                } else {
                                    '.'
                                }
                            }).collect::<String>();
                        format!("{}  {}", line_number, ascii_string)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                move |_, _| {
                    let display = Display::default().expect("No display available");
                    let clipboard = gtk::Clipboard::default(&display).expect("Failed to get clipboard");
                    clipboard.set_text(&value);
                }
            });
            actions.add_action(&action);

            let action = SimpleAction::new("copy-binary", None);
            action.connect_activate({
                let value = layer.get_value_as_bytes(variable)
                    .iter()
                    .map(|byte| format!("{:08b}", byte))
                    .collect::<Vec<_>>()
                    .join(" ");
                move |_, _| {
                    let display = Display::default().expect("No display available");
                    let clipboard = gtk::Clipboard::default(&display).expect("Failed to get clipboard");
                    clipboard.set_text(&value);
                }
            });
            actions.add_action(&action);

            menu.insert_action_group("context", Some(&actions));

            menu.connect_deactivate({
                let row = row.clone();
                move |_| {
                    row.style_context().remove_class("selected");
                }
            });

            menu.popup_at_pointer(Some(event));

            let (x, w) = layer.get_selection(variable);
            hex_editor.set_selection(offset + x, w);
        }

        Proceed
    }
}

pub fn create_row(key: String, value: String) -> ListBoxRow {
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

pub fn create_row_with_icon(key: String, icon: Pixbuf, value: String) -> ListBoxRow {
    let row = ListBoxRow::new();

    let hbox = gtk::Box::new(Orientation::Horizontal, 10);

    let label = Label::new(Some(&key));
    label.set_widget_name("key");
    label.set_xalign(0.0);
    hbox.add(&label);

    let image = Image::from_pixbuf(Some(&icon));
    image.set_size_request(24, 24);
    hbox.add(&image);

    let label = Label::new(Some(value.as_str()));
    label.set_widget_name("value");
    label.set_xalign(0.0);
    hbox.add(&label);

    row.add(&hbox);
    row.show_all();

    row
}
