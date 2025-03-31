use std::net::IpAddr;
use gtk::{gdk, gio, Builder, Button, Container, EventBox, Image, Label, ListBox, ListBoxRow, Menu, MenuItem, Orientation};
use gtk::gdk::{Display, EventButton, EventMask};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::{ActionGroup, SimpleAction, SimpleActionGroup};
use gtk::glib::Cast;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{ActionMapExt, BuilderExtManual, ButtonExt, ContainerExt, GtkMenuExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, MenuShellExt, StyleContextExt, WidgetExt, WidgetExtManual};
use rlibpcap::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use rlibpcap::packet::layers::ip::icmp::icmp_layer::IcmpLayer;
use rlibpcap::packet::layers::ip::icmpv6::icmpv6_layer::Icmpv6Layer;
use rlibpcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
use rlibpcap::packet::layers::ip::ipv6_layer::Ipv6Layer;
use rlibpcap::packet::layers::ip::tcp::tcp_layer::TcpLayer;
use rlibpcap::packet::layers::ip::udp::dhcp::dhcp_layer::DhcpLayer;
use rlibpcap::packet::layers::ip::udp::udp_layer::UdpLayer;
use rlibpcap::packet::layers::inter::layer::Layer;
use rlibpcap::packet::layers::sll2_frame::sll2_frame::Sll2Frame;
use crate::database::sqlite::Database;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::utils::ethernet_utils::ethernet_to_company;
use crate::utils::ip_utils::ip_to_icon;
use crate::widgets::hex_editor::HexEditor;

pub fn create_ethernet_layer_expander(db: &Database, offset: usize, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &EthernetFrame) -> Container {
    let (dropdown, list_box) = create_dropdown("Ethernet II", offset, hex_editor, actions, layer);

    match ethernet_to_company(db, layer.get_destination_mac()) {
        Some(company) => {
            list_box.add(&create_row("Destination:", format!("{} ({})", company, layer.get_destination_mac().to_string())));
        }
        None => {
            list_box.add(&create_row("Destination:", format!("({})", layer.get_destination_mac().to_string())));
        }
    }

    match ethernet_to_company(db, layer.get_source_mac()) {
        Some(company) => {
            list_box.add(&create_row("Source:", format!("{} ({})", company, layer.get_source_mac().to_string())));
        }
        None => {
            list_box.add(&create_row("Source:", format!("({})", layer.get_source_mac().to_string())));
        }
    }
    list_box.add(&create_row("Type:", format!("{:?} (0x{:04X})", layer.get_type(), layer.get_type().get_code())));

    dropdown.add(&list_box);
    dropdown.upcast()
}
/*
pub fn create_sll2_layer_expander(offset: usize, hex_editor: &HexEditor, layer: &Sll2Frame) -> Container {
    let (dropdown, list_box) = create_dropdown("Linux Cooked Capture v2");

    list_box.add(&create_row("Protocol:", format!("{} (0x{:04X})", layer.get_protocol().to_string(), layer.get_protocol().get_code())));
    list_box.add(&create_row("Interface Index:", layer.get_if_index().to_string()));
    list_box.add(&create_row("Link-Layer Address Type:", format!("{} ({})", layer.get_data_link_type().to_string(), layer.get_data_link_type().get_code())));
    list_box.add(&create_row("Packet Type:", format!("{} ({})", layer.get_packet_type().to_string(), layer.get_packet_type().get_code())));
    list_box.add(&create_row("Link-Layer Address Length:", layer.get_address_length().to_string()));

    let address = layer.get_address().iter()
        .take(layer.get_address_length() as usize)
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(":");
    list_box.add(&create_row("Source:", address));

    let unused = layer.get_address().iter()
        .skip(layer.get_address_length() as usize)
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .concat();
    list_box.add(&create_row("Unused:", unused));

    dropdown.add(&list_box);
    dropdown.upcast()
}

pub fn create_arp_layer_expander(layer: &ArpExtension) -> Container {
    let (dropdown, list_box) = create_dropdown("Address Resolution Protocol");

    //SHOULD BE LIKE Ethernet (1)
    list_box.add(&create_row("Hardware Type:", format!("{} ({})", layer.get_hardware_type().to_string(), layer.get_hardware_type())));

    list_box.add(&create_row("Hardware Size:", layer.get_hardware_size().to_string()));
    list_box.add(&create_row("Protocol Size:", layer.get_hardware_size().to_string()));

    //SHOULD BE LIKE reply (2)
    list_box.add(&create_row("Opcode:", format!("{} ({})", layer.get_opcode().to_string(), layer.get_opcode().get_code())));

    list_box.add(&create_row("Sender MAC Address:", layer.get_sender_mac().to_string()));
    list_box.add(&create_row("Sender IP Address:", layer.get_sender_address().to_string()));
    list_box.add(&create_row("Target MAC Address:", layer.get_target_mac().to_string()));
    list_box.add(&create_row("Target IP Address:", layer.get_target_address().to_string()));

    dropdown.add(&list_box);

    dropdown.upcast()
}
*/
pub fn create_ipv4_layer_expander(db: &Database, offset: usize, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &Ipv4Layer) -> Container {
    let (dropdown, list_box) = create_dropdown("Internet Protocol Version 4", offset, hex_editor, actions, layer);

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
    }

    dropdown.add(&list_box);
    dropdown.upcast()
}

pub fn create_ipv6_layer_expander(db: &Database, offset: usize, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &Ipv6Layer) -> Container {
    let (dropdown, list_box) = create_dropdown(&layer.get_title("frame"), offset, hex_editor, actions, layer);

    list_box.add(&create_row(format!("{}:", layer.get_title("version")).as_str(), layer.get_value("version")));
    list_box.add(&create_row(format!("{}:", layer.get_title("payload_length")).as_str(), layer.get_value("payload_length")));
    list_box.add(&create_row(format!("{}:", layer.get_title("next_header")).as_str(), layer.get_value("next_header")));
    list_box.add(&create_row(format!("{}:", layer.get_title("hop_limit")).as_str(), layer.get_value("hop_limit")));

    match ip_to_icon(db, IpAddr::V6(layer.get_source_address())) {
        Some(icon) => {
            list_box.add(&create_row_with_icon("Source Address:", icon, layer.get_source_address().to_string()));
        }
        None => {
            list_box.add(&create_row("Source Address:", layer.get_source_address().to_string()));
        }
    }

    match ip_to_icon(db, IpAddr::V6(layer.get_destination_address())) {
        Some(icon) => {
            list_box.add(&create_row_with_icon("Destination Address:", icon, layer.get_destination_address().to_string()));
        }
        None => {
            list_box.add(&create_row("Destination Address:", layer.get_destination_address().to_string()));
        }
    }

    dropdown.add(&list_box);
    dropdown.upcast()
}
/*
pub fn create_icmp_layer_expander(layer: &IcmpLayer) -> Container {
    let (dropdown, list_box) = create_dropdown("Internet Control Message Protocol");

    //SHOULD BE LIKE 8 (Echo (ping) request)
    list_box.add(&create_row("Type:", format!("{} ({})", layer.get_type(), layer.get_type().to_string())));
    list_box.add(&create_row("Code:", layer.get_code().to_string()));

    //SHOULD BE LIKE 0x544c [correct]
    list_box.add(&create_row("Checksum:", format!("0x{:04X}", layer.get_checksum())));

    let identifier_be = layer.get_identifier().to_be();
    let identifier_le = layer.get_identifier().to_le();
    list_box.add(&create_row("Identifier (BE):", format!("{} (0x{:04X})", identifier_be, identifier_be)));
    list_box.add(&create_row("Identifier (LE):", format!("{} (0x{:04X})", identifier_le, identifier_le)));

    let sequence_number_be = layer.get_sequence_number().to_be();
    let sequence_number_le = layer.get_sequence_number().to_le();
    list_box.add(&create_row("Sequence Number (BE):", format!("{} (0x{:04X})", sequence_number_be, sequence_number_be)));
    list_box.add(&create_row("Sequence Number (LE):", format!("{} (0x{:04X})", sequence_number_le, sequence_number_le)));

    dropdown.add(&list_box);

    dropdown.upcast()
}

pub fn create_icmpv6_layer_expander(layer: &Icmpv6Layer) -> Container {
    let (dropdown, list_box) = create_dropdown("Internet Control Message Protocol Version 6");

    //SHOULD BE LIKE 8 (Echo (ping) request)
    list_box.add(&create_row("Type:", format!("{} ({})", layer.get_type(), layer.get_type().to_string())));
    list_box.add(&create_row("Code:", layer.get_code().to_string()));

    //SHOULD BE LIKE 0x544c [correct]
    list_box.add(&create_row("Checksum:", format!("0x{:04X}", layer.get_checksum())));

    let identifier_be = layer.get_identifier().to_be();
    let identifier_le = layer.get_identifier().to_le();
    list_box.add(&create_row("Identifier (BE):", format!("{} (0x{:04X})", identifier_be, identifier_be)));
    list_box.add(&create_row("Identifier (LE):", format!("{} (0x{:04X})", identifier_le, identifier_le)));

    let sequence_number_be = layer.get_sequence_number().to_be();
    let sequence_number_le = layer.get_sequence_number().to_le();
    list_box.add(&create_row("Sequence Number (BE):", format!("{} (0x{:04X})", sequence_number_be, sequence_number_be)));
    list_box.add(&create_row("Sequence Number (LE):", format!("{} (0x{:04X})", sequence_number_le, sequence_number_le)));

    dropdown.add(&list_box);

    dropdown.upcast()
}

pub fn create_udp_layer_expander(layer: &UdpLayer, source_address: IpAddr, destination_address: IpAddr) -> Container {
    let (dropdown, list_box) = create_dropdown("User Datagram Protocol");

    list_box.add(&create_row("Source Port:", layer.get_source_port().to_string()));
    list_box.add(&create_row("Destination Port:", layer.get_destination_port().to_string()));
    list_box.add(&create_row("Length:", layer.get_length().to_string()));

    let checksum_string = if layer.validate_checksum(source_address, destination_address) { "correct" } else { "incorrect" };
    list_box.add(&create_row("Checksum:", format!("0x{:04X} [{}]", layer.get_checksum(), checksum_string)));

    dropdown.add(&list_box);

    dropdown.upcast()
}

pub fn create_dhcp_layer_expander(layer: &DhcpLayer) -> Container {
    let (dropdown, list_box) = create_dropdown("Dynamic Host Configuration Protocol");

    /*
    list_box.add(&create_row("Source Port:", layer.get_source_port().to_string()));
    list_box.add(&create_row("Destination Port:", layer.get_destination_port().to_string()));
    list_box.add(&create_row("Length:", layer.get_length().to_string()));
    list_box.add(&create_row("Checksum:", format!("0x{:04X}", layer.get_checksum())));
    */
    dropdown.add(&list_box);

    dropdown.upcast()
}

pub fn create_tcp_layer_expander(layer: &TcpLayer) -> Container {
    let (dropdown, list_box) = create_dropdown("Transmission Control Protocol");

    list_box.add(&create_row("Source Port:", layer.get_source_port().to_string()));
    list_box.add(&create_row("Destination Port:", layer.get_destination_port().to_string()));
    list_box.add(&create_row("Sequence Number:", layer.get_window_size().to_string()));
    list_box.add(&create_row("Acknowledgement Number:", layer.get_window_size().to_string()));
    //FLAGS
    list_box.add(&create_row("Window:", layer.get_window_size().to_string()));
    list_box.add(&create_row("Checksum:", format!("0x{:04X}", layer.get_checksum())));
    list_box.add(&create_row("Urgent Pointer:", layer.get_urgent_pointer().to_string()));

    dropdown.add(&list_box);

    dropdown.upcast()
}
*/
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

    list_box.connect_row_activated({
        let hex_editor = hex_editor.clone();
        let layer = layer.clone_ext();
        move |_, row| {
            let (x, w) = layer.get_selection(layer.get_fields().get(row.index() as usize).unwrap().clone());
            hex_editor.set_selection(offset + x, w);
        }
    });

    list_box.connect_button_press_event({
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

                create_row_context_menu(&row, event, &actions, variable, layer.as_ref());

                let (x, w) = layer.get_selection(variable);
                hex_editor.set_selection(offset + x, w);
            }

            Proceed
        }
    });

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

fn create_row(key: &str, value: String) -> ListBoxRow {
    let row = ListBoxRow::new();

    let hbox = gtk::Box::new(Orientation::Horizontal, 10);

    let label = Label::new(Some(key));
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

fn create_row_with_icon(key: &str, icon: Pixbuf, value: String) -> ListBoxRow {
    let row = ListBoxRow::new();

    let hbox = gtk::Box::new(Orientation::Horizontal, 10);

    let label = Label::new(Some(key));
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

fn create_row_context_menu(row: &ListBoxRow, event: &EventButton, actions: &SimpleActionGroup, variable: &str, layer: &dyn LayerExt) {
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

    menu.insert_action_group("context", Some(actions));

    menu.connect_deactivate({
        let row = row.clone();
        move |_| {
            row.style_context().remove_class("selected");
        }
    });

    menu.popup_at_pointer(Some(event));
}
