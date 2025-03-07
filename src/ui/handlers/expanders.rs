use std::cell::RefCell;
use std::net::IpAddr;
use std::rc::Rc;
use gtk::{Button, Container, Image, Label, ListBox, ListBoxRow, Orientation};
use gtk::glib::Cast;
use gtk::prelude::{ButtonExt, ContainerExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, WidgetExt};
use pcap::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use pcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use pcap::packet::layers::ethernet_frame::ip::icmp::icmp_layer::IcmpLayer;
use pcap::packet::layers::ethernet_frame::ip::icmpv6::icmpv6_layer::Icmpv6Layer;
use pcap::packet::layers::ethernet_frame::ip::ipv4_layer::Ipv4Layer;
use pcap::packet::layers::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use pcap::packet::layers::ethernet_frame::ip::tcp::tcp_layer::TcpLayer;
use pcap::packet::layers::ethernet_frame::ip::udp::dhcp::dhcp_layer::DhcpLayer;
use pcap::packet::layers::ethernet_frame::ip::udp::udp_layer::UdpLayer;
use crate::database::sqlite::Database;
use crate::layers::inter::layer_ext::LayerExt;
use crate::ui::handlers::ethernet_utils::ethernet_to_company;
use crate::ui::widgets::hex_editor::HexEditor;

pub fn create_ethernet_layer_expander(db: &Database, offset: usize, hex_editor: &HexEditor, layer: &EthernetFrame) -> Container {
    let (dropdown, list_box) = create_dropdown("Ethernet II");

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

    let hex_editor = hex_editor.clone();
    let layer = layer.clone();
    list_box.connect_row_activated(move |_, row| {
        let (x, w) = match row.index() {
            0 => {
                layer.get_selection("destination")
            }
            1 => {
                layer.get_selection("source")
            }
            2 => {
                layer.get_selection("type")
            }
            _ => unimplemented!()
        };

        hex_editor.set_selection(offset+x, w);
    });

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

pub fn create_ipv4_layer_expander(offset: usize, hex_editor: &HexEditor, layer: &Ipv4Layer) -> Container {
    let (dropdown, list_box) = create_dropdown("Internet Protocol Version 4");

    list_box.add(&create_row("Version:", layer.get_version().to_string()));
    list_box.add(&create_row("TOS:", layer.get_tos().to_string())); // SHOULD BE - Differentiated Services Field
    list_box.add(&create_row("Total Length:", layer.get_total_length().to_string()));
    list_box.add(&create_row("Identification:", format!("0x{:04X} ({})", layer.get_identification(), layer.get_identification())));
    //list_box.add(&create_row(format!("Header: ({})", layer.get_version()).as_str())); //FLAGS
    list_box.add(&create_row("Time to Live:", layer.get_ttl().to_string()));
    list_box.add(&create_row("Protocol:", format!("{:?} ({})", layer.get_protocol(), layer.get_protocol().get_code())));

    let checksum_string = if layer.validate_checksum() { "correct" } else { "incorrect" };
    list_box.add(&create_row("Header Checksum:", format!("0x{:04X} [{}]", layer.get_checksum(), checksum_string)));
    list_box.add(&create_row("Source Address:", layer.get_source_address().to_string()));
    list_box.add(&create_row("Destination Address:", layer.get_destination_address().to_string()));

    let hex_editor = hex_editor.clone();
    let layer = layer.clone();
    list_box.connect_row_activated(move |_, row| {
        let (x, w) = match row.index() {
            0 => {
                layer.get_selection("version")
            }
            1 => {
                layer.get_selection("tos")
            }
            2 => {
                layer.get_selection("total_length")
            }
            3 => {
                layer.get_selection("identification")
            }
            4 => {
                layer.get_selection("ttl")
            }
            5 => {
                layer.get_selection("protocol")
            }
            6 => {
                layer.get_selection("checksum")
            }
            7 => {
                layer.get_selection("source_address")
            }
            8 => {
                layer.get_selection("destination_address")
            }
            _ => unimplemented!()
        };

        hex_editor.set_selection(offset+x, w);
    });

    dropdown.add(&list_box);
    dropdown.upcast()
}

pub fn create_ipv6_layer_expander(offset: usize, hex_editor: &HexEditor, layer: &Ipv6Layer) -> Container {
    let (dropdown, list_box) = create_dropdown("Internet Protocol Version 6");

    list_box.add(&create_row("Version:", layer.get_version().to_string()));
    list_box.add(&create_row("Payload Length:", layer.get_payload_length().to_string()));
    list_box.add(&create_row("Next Header:", format!("{:?} ({})", layer.get_next_header(), layer.get_next_header().get_code())));
    list_box.add(&create_row("Hop Limit:", layer.get_hop_limit().to_string()));
    list_box.add(&create_row("Source Address:", layer.get_source_address().to_string()));
    list_box.add(&create_row("Destination Address:", layer.get_destination_address().to_string()));

    let hex_editor = hex_editor.clone();
    let layer = layer.clone();
    list_box.connect_row_activated(move |_, row| {
        let (x, w) = match row.index() {
            0 => {
                layer.get_selection("version")
            }
            1 => {
                layer.get_selection("payload_length")
            }
            2 => {
                layer.get_selection("next_header")
            }
            3 => {
                layer.get_selection("hop_limit")
            }
            4 => {
                layer.get_selection("source_address")
            }
            5 => {
                layer.get_selection("destination_address")
            }
            _ => unimplemented!()
        };

        hex_editor.set_selection(offset+x, w);
    });

    dropdown.add(&list_box);
    dropdown.upcast()
}

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

fn create_dropdown(title: &str) -> (Container, ListBox) {
    let dropdown = gtk::Box::new(Orientation::Vertical, 0);
    dropdown.set_widget_name("dropdown");
    dropdown.show();

    let hbox = gtk::Box::new(Orientation::Horizontal, 10);
    let icon = Image::from_resource("/com/ethernaut/rust/res/icons/ic_expand_less.svg");

    let label = Label::new(Some(title));
    label.set_xalign(0.0);

    hbox.add(&icon);
    hbox.add(&label);

    let button = Button::new();
    button.set_child(Some(&hbox));

    let list_box = ListBox::new();

    let list_box_clone = list_box.clone();
    button.connect_clicked(move |_| {
        list_box_clone.set_visible(!list_box_clone.is_visible());

        if list_box_clone.is_visible() {
            icon.set_from_resource(Some("/com/ethernaut/rust/res/icons/ic_expand_more.svg"));
            return;
        }

        icon.set_from_resource(Some("/com/ethernaut/rust/res/icons/ic_expand_less.svg"));
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
