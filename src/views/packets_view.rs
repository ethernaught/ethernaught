use std::cell::RefCell;
use std::net::IpAddr;
use std::rc::Rc;
use gtk::{gdk, Builder, Button, CellRendererState, CellRendererText, Container, CssProvider, Entry, Image, Label, ListBox, ListStore, ScrolledWindow, StyleContext, TreeIter, TreeModel, TreeModelFilter, TreeView, TreeViewColumn, Widget};
use gtk::glib::{ObjectExt, PropertyGet, ToValue, Type};
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{AdjustmentExt, BuilderExtManual, CellLayoutExt, CellRendererExt, ContainerExt, CssProviderExt, EntryExt, GtkListStoreExt, GtkListStoreExtManual, LabelExt, ListBoxExt, ScrolledWindowExt, TreeModelExt, TreeModelFilterExt, TreeViewColumnExt, TreeViewExt, WidgetExt};
use gtk::subclass::container::Callback;
use rlibpcap::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::{EthernetFrame, ETHERNET_FRAME_LEN};
use rlibpcap::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use rlibpcap::packet::layers::ip::icmp::icmp_layer::IcmpLayer;
use rlibpcap::packet::layers::ip::inter::ip_protocols::IpProtocols;
use rlibpcap::packet::layers::ip::inter::ip_versions::IpVersions;
use rlibpcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
use rlibpcap::packet::layers::ip::ipv6_layer::Ipv6Layer;
use rlibpcap::packet::layers::ip::tcp::tcp_layer::TcpLayer;
use rlibpcap::packet::layers::ip::udp::dhcp::dhcp_layer::DhcpLayer;
use rlibpcap::packet::layers::ip::udp::inter::udp_payloads::UdpPayloads;
use rlibpcap::packet::layers::ip::udp::inter::udp_types::UdpTypes;
use rlibpcap::packet::layers::ip::udp::udp_layer::UdpLayer;
use rlibpcap::packet::layers::loop_frame::inter::loop_types::LoopTypes;
use rlibpcap::packet::layers::loop_frame::loop_frame::LoopFrame;
use rlibpcap::packet::layers::raw_frame::raw_frame::RawFrame;
use rlibpcap::packet::layers::sll2_frame::sll2_frame::Sll2Frame;
use rlibpcap::packet::packet::Packet;
use rlibpcap::pcap::pcap::Pcap;
use rlibpcap::utils::data_link_types::DataLinkTypes;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::views::dropdown::dropdown::Dropdown;

#[derive(Clone)]
pub struct PacketsView {
    pub root: gtk::Box,
    pub search: Entry,
    pub scroll_layout: ScrolledWindow,
    pub tree_view: TreeView,
    pub model: ListStore,
    pub tree_filter: TreeModelFilter,
    pub packets: Rc<RefCell<Vec<Packet>>>
}

impl PacketsView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/packets_view.ui");

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in packet_view.ui");

        let search: Entry = builder
            .object("search")
            .expect("Couldn't find 'search' in packet_view.ui");

        let scroll_layout: ScrolledWindow = builder
            .object("list_scroll_layout")
            .expect("Couldn't find 'list_scroll_layout' in packet_view.ui");

        let tree_view: TreeView = builder
            .object("tree_view")
            .expect("Couldn't find 'tree_view' in packet_view.ui");
        let model = ListStore::new(&[Type::U32, Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::STRING]);
        let tree_filter = TreeModelFilter::new(&model, None);

        tree_view.set_model(Some(&tree_filter));

        init_column(&tree_view, "No.", 0, 100);
        init_column(&tree_view, "Time", 1, 150);
        init_column(&tree_view, "Source", 2, 180);
        init_column(&tree_view, "Destination", 3, 180);
        init_column(&tree_view, "Protocol", 4, 80);
        init_column(&tree_view, "Length", 5, 80);
        init_column(&tree_view, "Info", 6, 80);

        let vadj = Rc::new(scroll_layout.vadjustment());
        let needs_scroll = Rc::new(RefCell::new(false));
        let user_scrolled_up = Rc::new(RefCell::new(false));

        {
            let vadj = vadj.clone();
            let user_scrolled_up = user_scrolled_up.clone();
            vadj.connect_value_changed(move |adj| {
                let is_at_bottom = (adj.upper() - adj.value() - adj.page_size()).abs() < 100.0;
                *user_scrolled_up.borrow_mut() = !is_at_bottom;
            });
        }

        model.connect_row_inserted({
            let vadj = vadj.clone();
            let needs_scroll = needs_scroll.clone();
            let user_scrolled_up = user_scrolled_up.clone();

            move |_, _, _| {
                if !*user_scrolled_up.borrow() {
                    *needs_scroll.borrow_mut() = true;
                }

                let vadj = vadj.clone();
                let needs_scroll = needs_scroll.clone();
                let user_scrolled_up = user_scrolled_up.clone();

                if *needs_scroll.borrow() && !*user_scrolled_up.borrow() {
                    *needs_scroll.borrow_mut() = false;
                    vadj.set_value(vadj.upper() - vadj.page_size());
                }
            }
        });

        Self {
            root,
            search,
            scroll_layout,
            tree_view,
            model,
            tree_filter,
            packets: Rc::new(RefCell::new(Vec::new()))
        }
    }

    pub fn from_pcap(pcap: Pcap) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/packets_view.ui");

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in packet_view.ui");

        let search: Entry = builder
            .object("search")
            .expect("Couldn't find 'search' in packet_view.ui");

        let scroll_layout: ScrolledWindow = builder
            .object("list_scroll_layout")
            .expect("Couldn't find 'list_scroll_layout' in packet_view.ui");

        let tree_view: TreeView = builder
            .object("tree_view")
            .expect("Couldn't find 'tree_view' in packet_view.ui");
        let model = ListStore::new(&[Type::U32, Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::STRING, Type::STRING]);
        let tree_filter = TreeModelFilter::new(&model, None);

        tree_view.set_model(Some(&tree_filter));

        init_column(&tree_view, "No.", 0, 100);
        init_column(&tree_view, "Time", 1, 150);
        init_column(&tree_view, "Source", 2, 180);
        init_column(&tree_view, "Destination", 3, 180);
        init_column(&tree_view, "Protocol", 4, 80);
        init_column(&tree_view, "Length", 5, 80);
        init_column(&tree_view, "Info", 6, 80);

        let vadj = Rc::new(scroll_layout.vadjustment());
        let needs_scroll = Rc::new(RefCell::new(false));
        let user_scrolled_up = Rc::new(RefCell::new(false));

        {
            let vadj = vadj.clone();
            let user_scrolled_up = user_scrolled_up.clone();
            vadj.connect_value_changed(move |adj| {
                let is_at_bottom = (adj.upper() - adj.value() - adj.page_size()).abs() < 100.0;
                *user_scrolled_up.borrow_mut() = !is_at_bottom;
            });
        }

        model.connect_row_inserted({
            let vadj = vadj.clone();
            let needs_scroll = needs_scroll.clone();
            let user_scrolled_up = user_scrolled_up.clone();

            move |_, _, _| {
                if !*user_scrolled_up.borrow() {
                    *needs_scroll.borrow_mut() = true;
                }

                let vadj = vadj.clone();
                let needs_scroll = needs_scroll.clone();
                let user_scrolled_up = user_scrolled_up.clone();

                if *needs_scroll.borrow() && !*user_scrolled_up.borrow() {
                    *needs_scroll.borrow_mut() = false;
                    vadj.set_value(vadj.upper() - vadj.page_size());
                }
            }
        });

        for (i, packet) in pcap.get_packets().iter().enumerate() {
            let (frame_time,
                source,
                destination,
                protocol,
                packet_length) = Self::get_model_values(&packet);

            model.insert_with_values(None, &[
                (0, &(i as u32)),
                (1, &frame_time),
                (2, &source),
                (3, &destination),
                (4, &protocol),
                (5, &packet_length),
                //(6, &"TODO".to_string()),
            ]);
            //self.add_to_model(p, i as i32 + 1);
        }


        let query = Rc::new(RefCell::new(String::new()));
        let packets = Rc::new(RefCell::new(pcap.get_packets()));
        tree_filter.set_visible_func(filter(&query, &packets));

        search.connect_activate({
            let query = query.clone();
            let tree_filter = tree_filter.clone();
            move |entry| {
                let text = entry.text();

                *query.borrow_mut() = text.to_string();

                tree_filter.refilter();

            }
        });


        Self {
            root,
            search,
            scroll_layout,
            tree_view,
            model,
            tree_filter,
            packets//: Rc::new(RefCell::new(pcap.get_packets()))
        }
    }

    pub fn add(&self, packet: Packet) {
        let (frame_time,
            source,
            destination,
            protocol,
            packet_length) = Self::get_model_values(&packet);

        let packet_total = self.packets.borrow().len() as u32;

        self.model.insert_with_values(None, &[
            (0, &packet_total),
            (1, &frame_time),
            (2, &source),
            (3, &destination),
            (4, &protocol),
            (5, &packet_length),
            //(6, &"TODO".to_string()),
        ]);
        self.packets.borrow_mut().push(packet);
    }

    fn get_model_values(packet: &Packet) -> (String, String, String, String, String) {
        let (source, destination, protocol) = match packet.get_data_link_type() {
            DataLinkTypes::En10mb => {
                let ethernet_frame = packet.get_frame().as_any().downcast_ref::<EthernetFrame>().unwrap();

                match ethernet_frame.get_type() {
                    EthernetTypes::Ipv4 => {
                        get_data_from_ipv4_frame(ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap())
                    }
                    EthernetTypes::Ipv6 => {
                        get_data_from_ipv6_frame(ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap())
                    }
                    EthernetTypes::Broadcast => {
                        //source_label.set_label(&ethernet_layer.get_source().to_string());
                        //destination_label.set_label(&ethernet_layer.get_destination().to_string());
                        (ethernet_frame.get_source_mac().to_string(), ethernet_frame.get_destination_mac().to_string(), ethernet_frame.get_type().to_string())
                    }
                    _ => {
                        (ethernet_frame.get_source_mac().to_string(), ethernet_frame.get_destination_mac().to_string(), ethernet_frame.get_type().to_string())
                    }
                }
            }
            DataLinkTypes::Sll2 => {
                let sll2_frame = packet.get_frame().as_any().downcast_ref::<Sll2Frame>().unwrap();

                match sll2_frame.get_protocol() {
                    EthernetTypes::Ipv4 => {
                        get_data_from_ipv4_frame(sll2_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap())
                    }
                    EthernetTypes::Ipv6 => {
                        get_data_from_ipv6_frame(sll2_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap())
                    }
                    _ => {
                        unimplemented!()
                    }
                }
            }
            DataLinkTypes::Raw => {
                let raw_frame = packet.get_frame().as_any().downcast_ref::<RawFrame>().unwrap();

                match raw_frame.get_version() {
                    IpVersions::Ipv4 => {
                        get_data_from_ipv4_frame(raw_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap())
                    }
                    IpVersions::Ipv6 => {
                        get_data_from_ipv6_frame(raw_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap())
                    }
                    _ => {
                        unimplemented!()
                    }
                }
            }
            DataLinkTypes::Loop => {
                let loop_frame = packet.get_frame().as_any().downcast_ref::<LoopFrame>().unwrap();

                match loop_frame.get_type() {
                    LoopTypes::Ipv4 => {
                        get_data_from_ipv4_frame(loop_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap())
                    }
                    LoopTypes::Ipv6 | LoopTypes::Ipv6e2 | LoopTypes::Ipv6e3 => {
                        get_data_from_ipv6_frame(loop_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap())
                    }
                    _ => {
                        unimplemented!()
                    }
                }
            }
            _ => {
                //"[WiFi] TODO".to_string()
                todo!()
            }
        };

        let frame_time = packet.get_frame_time().to_string();
        let packet_length = packet.len().to_string();

        (frame_time, source, destination, protocol, packet_length)
    }

    pub fn clear(&self) {
        self.model.clear();
        self.packets.borrow_mut().clear();
    }

    pub fn connect_select<F>(&self, callback: F)
    where
        F: Fn(&Packet) + 'static
    {
        self.tree_view.connect_button_press_event({
            let packets = self.packets.clone();
            move |tree_view, event| {
                if event.button() == 1 {
                    let (x, y) = event.position();

                    let path = tree_view.path_at_pos(x as i32, y as i32);

                    if let Some((Some(path), _column, _x, _y)) = path {
                        let model = tree_view.model().unwrap();
                        let index = model.value(&model.iter(&path).unwrap(), 0).get::<u32>().unwrap();

                        callback(packets.borrow().get(index as usize).unwrap());
                    }
                }

                Proceed
            }
        });
    }
}

fn init_column(tree: &TreeView, title: &str, col_id: i32, min_width: i32) {
    let renderer = CellRendererText::new();
    let column = TreeViewColumn::new();
    column.set_min_width(min_width);
    column.set_title(title);
    CellLayoutExt::pack_start(&column, &renderer, true);
    CellLayoutExt::add_attribute(&column, &renderer, "text", col_id);

    CellLayoutExt::set_cell_data_func(&column, &renderer, Some(Box::new(move |_, cell, model, iter| {
        let protocol: String = model.value(iter, 4).get().unwrap_or_default();

        let color = match protocol.as_str() {
            "ARP" => "#05211b",
            "Broadcast" => "#000000",
            "TCP" => "#1e0926",
            "UDP" => "#070c1f",
            "ICMP" => "#260d07",
            "GRE" => "#122407",
            _ => "#1e1f22",
        };

        cell.set_property("cell-background", &color);
        cell.set_property("cell-background-set", &true); // Let GTK handle selection color
    })));

    tree.append_column(&column);
}

fn get_data_from_ipv4_frame(layer: &Ipv4Layer) -> (String, String, String) {
    match layer.get_protocol() {
        IpProtocols::Udp => {
            let udp_layer = layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();

            match udp_layer.get_payload() {
                UdpPayloads::Known(_type, _) => {
                    (layer.get_source_address().to_string(), layer.get_destination_address().to_string(), udp_layer.get_type().to_string())
                }
                _ => {
                    (layer.get_source_address().to_string(), layer.get_destination_address().to_string(), layer.get_protocol().to_string())
                }
            }
        }
        _ => {
            (layer.get_source_address().to_string(), layer.get_destination_address().to_string(), layer.get_protocol().to_string())
        }
    }
}

fn get_data_from_ipv6_frame(layer: &Ipv6Layer) -> (String, String, String) {
    match layer.get_next_header() {
        IpProtocols::Udp => {
            let udp_layer = layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();

            match udp_layer.get_payload() {
                UdpPayloads::Known(_type, _) => {
                    (layer.get_source_address().to_string(), layer.get_destination_address().to_string(), udp_layer.get_type().to_string())
                }
                _ => {
                    (layer.get_source_address().to_string(), layer.get_destination_address().to_string(), layer.get_next_header().to_string())
                }
            }
        }
        _ => {
            (layer.get_source_address().to_string(), layer.get_destination_address().to_string(), layer.get_next_header().to_string())
        }
    }
}

fn filter(query: &Rc<RefCell<String>>, packets: &Rc<RefCell<Vec<Packet>>>) -> impl Fn(&TreeModel, &TreeIter) -> bool + 'static {
    let query = query.clone();
    let packets = packets.clone();

    move |model, iter| {
        let index: u32 = model.value(iter, 0).get().unwrap_or_default();
        println!("{}  {}", query.borrow(), index);

        if let Some(packet) = packets.borrow().get(index as usize) {
            match packet.get_data_link_type() {
                DataLinkTypes::En10mb => {
                    let ethernet_frame = packet.get_frame().as_any().downcast_ref::<EthernetFrame>().unwrap();
                    if ethernet_frame.get_field_name("frame").eq(&*query.borrow()) {
                        return true;
                    }

                    if query_layer(&*query.borrow(), ethernet_frame) {
                        return true;
                    }

                    match ethernet_frame.get_type() {
                        EthernetTypes::Ipv4 => {
                            let ipv4_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                            if ipv4_layer.get_field_name("frame").eq(&*query.borrow()) {
                                return true;
                            }

                            if query_layer(&*query.borrow(), ipv4_layer) {
                                return true;
                            }



                            match ipv4_layer.get_protocol() {
                                IpProtocols::HopByHop => {}
                                IpProtocols::Icmp => {
                                    let icmp_layer = ipv4_layer.get_data().unwrap().as_any().downcast_ref::<IcmpLayer>().unwrap();
                                    if icmp_layer.get_field_name("frame").eq(&*query.borrow()) {
                                        return true;
                                    }

                                    if query_layer(&*query.borrow(), icmp_layer) {
                                        return true;
                                    }
                                }
                                IpProtocols::Igmp => {}
                                IpProtocols::Tcp => {
                                    let tcp_layer = ipv4_layer.get_data().unwrap().as_any().downcast_ref::<TcpLayer>().unwrap();
                                    if tcp_layer.get_field_name("frame").eq(&*query.borrow()) {
                                        return true;
                                    }

                                    if query_layer(&*query.borrow(), tcp_layer) {
                                        return true;
                                    }
                                }
                                IpProtocols::Udp => {
                                    let udp_layer = ipv4_layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();
                                    if udp_layer.get_field_name("frame").eq(&*query.borrow()) {
                                        return true;
                                    }

                                    if query_layer(&*query.borrow(), udp_layer) {
                                        return true;
                                    }
                                }
                                IpProtocols::Ipv6 => {}
                                IpProtocols::Gre => {}
                                IpProtocols::Icmpv6 => {}
                                IpProtocols::Ospf => {}
                                IpProtocols::Sps => {}
                            }

                        }
                        EthernetTypes::Arp => {
                            let arp_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<ArpExtension>().unwrap();
                            if arp_layer.get_field_name("frame").eq(&*query.borrow()) {
                                return true;
                            }

                            if query_layer(&*query.borrow(), arp_layer) {
                                return true;
                            }
                        }
                        EthernetTypes::Ipv6 => {
                            let ipv6_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                            if ipv6_layer.get_field_name("frame").eq(&*query.borrow()) {
                                return true;
                            }

                            if query_layer(&*query.borrow(), ipv6_layer) {
                                return true;
                            }
                        }
                        EthernetTypes::Broadcast => {}
                    }
                }
                DataLinkTypes::Sll2 => {
                    let sll2_frame = packet.get_frame().as_any().downcast_ref::<Sll2Frame>().unwrap();
                    if sll2_frame.get_field_name("frame").eq(&*query.borrow()) {
                        return true;
                    }

                    if query_layer(&*query.borrow(), sll2_frame) {
                        return true;
                    }

                    match sll2_frame.get_protocol() {
                        EthernetTypes::Ipv4 => {
                            let ipv4_layer = sll2_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                            if ipv4_layer.get_field_name("frame").eq(&*query.borrow()) {
                                return true;
                            }

                            if query_layer(&*query.borrow(), ipv4_layer) {
                                return true;
                            }

                            match ipv4_layer.get_protocol() {
                                IpProtocols::HopByHop => {}
                                IpProtocols::Icmp => {
                                    let icmp_layer = ipv4_layer.get_data().unwrap().as_any().downcast_ref::<IcmpLayer>().unwrap();
                                    if icmp_layer.get_field_name("frame").eq(&*query.borrow()) {
                                        return true;
                                    }

                                    if query_layer(&*query.borrow(), icmp_layer) {
                                        return true;
                                    }
                                }
                                IpProtocols::Igmp => {}
                                IpProtocols::Tcp => {
                                    let tcp_layer = ipv4_layer.get_data().unwrap().as_any().downcast_ref::<TcpLayer>().unwrap();
                                    if tcp_layer.get_field_name("frame").eq(&*query.borrow()) {
                                        return true;
                                    }

                                    if query_layer(&*query.borrow(), tcp_layer) {
                                        return true;
                                    }
                                }
                                IpProtocols::Udp => {
                                    let udp_layer = ipv4_layer.get_data().unwrap().as_any().downcast_ref::<UdpLayer>().unwrap();
                                    if udp_layer.get_field_name("frame").eq(&*query.borrow()) {
                                        return true;
                                    }

                                    if query_layer(&*query.borrow(), udp_layer) {
                                        return true;
                                    }
                                }
                                IpProtocols::Ipv6 => {}
                                IpProtocols::Gre => {}
                                IpProtocols::Icmpv6 => {}
                                IpProtocols::Ospf => {}
                                IpProtocols::Sps => {}
                            }
                        }
                        EthernetTypes::Arp => {
                            let arp_layer = sll2_frame.get_data().unwrap().as_any().downcast_ref::<ArpExtension>().unwrap();
                            if arp_layer.get_field_name("frame").eq(&*query.borrow()) {
                                return true;
                            }

                            if query_layer(&*query.borrow(), arp_layer) {
                                return true;
                            }
                        }
                        EthernetTypes::Ipv6 => {
                            let ipv6_layer = sll2_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                            if ipv6_layer.get_field_name("frame").eq(&*query.borrow()) {
                                return true;
                            }

                            if query_layer(&*query.borrow(), ipv6_layer) {
                                return true;
                            }
                        }
                        EthernetTypes::Broadcast => {}
                    }
                }
                _ => {}
            }
        }

        false
    }
}

fn query_layer(query: &str, layer: &dyn LayerExt) -> bool {
    if layer.get_fields().contains(&query) {
        return true;
    }

    false
}





