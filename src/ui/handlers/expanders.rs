use std::any::Any;
use std::fmt::format;
use gtk::{Button, Container, Expander, Image, Label, ListBox, ListBoxRow, Orientation};
use gtk::glib::Cast;
use gtk::prelude::{BinExt, BoxExt, ButtonExt, ContainerExt, ExpanderExt, ImageExt, LabelExt, WidgetExt};
use pcap::packet::layers::layer_1::ethernet_layer::EthernetLayer;
use pcap::packet::layers::layer_2::ethernet::ipv4_layer::IPv4Layer;
use pcap::packet::layers::layer_2::ethernet::ipv6_layer::IPv6Layer;
use pcap::packet::layers::layer_3::ip::tcp_layer::TcpLayer;
use pcap::packet::layers::layer_3::ip::udp_layer::UdpLayer;

pub fn create_ethernet_layer_expander(layer: &EthernetLayer) -> Container {
    let (dropdown, list_box) = create_dropdown("Ethernet II");

    list_box.add(&create_row("Destination:", format!("({})", layer.get_destination().to_string())));
    list_box.add(&create_row("Source:", format!("({})", layer.get_source().to_string())));
    list_box.add(&create_row("Type:", format!("{:?} (0x{:04x})", layer.get_type(), layer.get_type().get_code())));

    dropdown.add(&list_box);

    dropdown.upcast()
}

pub fn create_ipv4_layer_expander(layer: &IPv4Layer) -> Container {
    let (dropdown, list_box) = create_dropdown("Internet Protocol Version 4");

    list_box.add(&create_row("Version:", layer.get_version().to_string()));
    list_box.add(&create_row("TOS:", layer.get_tos().to_string())); // SHOULD BE - Differentiated Services Field
    list_box.add(&create_row("Total Length:", layer.get_total_length().to_string()));
    list_box.add(&create_row("Identification:", format!("0x{:04x} ({})", layer.get_identification(), layer.get_identification())));
    //list_box.add(&create_row(format!("Header: ({})", layer.get_version()).as_str())); //FLAGS
    list_box.add(&create_row("Time to Live:", layer.get_ttl().to_string()));
    list_box.add(&create_row("Protocol:", format!("{:?} ({})", layer.get_protocol(), layer.get_protocol().get_code())));

    list_box.add(&create_row("Header Checksum:", format!("0x{:04x}", layer.get_checksum())));
    list_box.add(&create_row("Source Address:", layer.get_source_ip().to_string()));
    list_box.add(&create_row("Destination Address:", layer.get_destination_ip().to_string()));

    dropdown.add(&list_box);

    dropdown.upcast()
}

pub fn create_ipv6_layer_expander(layer: &IPv6Layer) -> Container {
    let (dropdown, list_box) = create_dropdown("Internet Protocol Version 6");

    list_box.add(&create_row("Version:", layer.get_version().to_string()));
    list_box.add(&create_row("Payload Length:", layer.get_payload_length().to_string()));
    list_box.add(&create_row("Next Header:", format!("{:?} ({})", layer.get_next_header(), layer.get_next_header().get_code())));
    list_box.add(&create_row("Hop Limit:", layer.get_hop_limit().to_string()));
    list_box.add(&create_row("Source Address:", layer.get_source_ip().to_string()));
    list_box.add(&create_row("Destination Address:", layer.get_destination_ip().to_string()));

    dropdown.add(&list_box);

    dropdown.upcast()
}

pub fn create_udp_layer_expander(layer: &UdpLayer) -> Container {
    let (dropdown, list_box) = create_dropdown("User Datagram Protocol");

    list_box.add(&create_row("Source Port:", layer.get_source_port().to_string()));
    list_box.add(&create_row("Destination Port:", layer.get_destination_port().to_string()));
    list_box.add(&create_row("Length:", layer.get_length().to_string()));
    list_box.add(&create_row("Checksum:", format!("0x{:04x}", layer.get_checksum())));

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
    list_box.add(&create_row("Checksum:", format!("0x{:04x}", layer.get_checksum())));
    list_box.add(&create_row("Urgent Pointer:", layer.get_urgent_pointer().to_string()));

    dropdown.add(&list_box);

    dropdown.upcast()
}

fn create_dropdown(title: &str) -> (Container, ListBox) {
    let dropdown = gtk::Box::new(Orientation::Vertical, 0);
    dropdown.show();
    dropdown.set_widget_name("dropdown");

    let hbox = gtk::Box::new(Orientation::Horizontal, 10);
    let icon = Image::from_file("res/images/ic_expand_less.svg");

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
            icon.set_from_file(Some("res/images/ic_expand_more.svg"));
            return;
        }

        icon.set_from_file(Some("res/images/ic_expand_less.svg"));
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
