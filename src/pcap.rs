use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use gtk::prelude::SocketExtManual;
use pcap::{Capture, Device};
use crate::packet::packet::Packet;
use crate::packet::inter::interfaces::Interfaces;
use crate::packet::layers::inter::layer::Layer;
use crate::packet::layers::layer_1::ethernet_layer::EthernetLayer;
use crate::packet::layers::layer_1::inter::types::Types;
use crate::packet::layers::layer_2::ethernet::inter::protocols::Protocols;
use crate::packet::layers::layer_2::ethernet::ipv4_layer;
use crate::packet::layers::layer_2::ethernet::ipv4_layer::IPv4Layer;
use crate::packet::layers::layer_3::ip::tcp_layer::TcpLayer;
use crate::packet::layers::layer_3::ip::udp_layer::UdpLayer;

pub fn packet_capture(tx: Arc<Mutex<Sender<Packet>>>) {
    thread::spawn(move || {
        let devices = Device::list().expect("Failed to get device list");

        let device = devices.into_iter().find(|d| d.name.contains("wlp7s0"))
            .expect("No suitable device found");

        println!("Listening on device: {}", device.name);

        let mut cap = Capture::from_device(device)
            .expect("Failed to open device")
            .promisc(true)
            .immediate_mode(true)
            .open()
            .expect("Failed to start capture");

        let interface = Interfaces::Ethernet;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as f64;

        while let Ok(packet) = cap.next_packet() {
            let timestamp = (get_timestamp(packet.header.ts.tv_sec as u32, packet.header.ts.tv_usec as u32)-now)/1000.0;

            let mut frame = Packet::new(interface.clone(), timestamp, packet.header.len);

            match frame.get_interface() {
                Interfaces::Ethernet => {
                    let ethernet_layer = EthernetLayer::from_bytes(packet.data).expect("Failed to parse Ethernet frame");
                    frame.add_layer(ethernet_layer.dyn_clone());
                    let mut off = ethernet_layer.len();

                    match ethernet_layer.get_type() {
                        Types::IPv4 => {
                            let ipv4_layer = IPv4Layer::from_bytes(&packet.data[off..]).expect("Failed to parse IPv4 frame");
                            frame.add_layer(ipv4_layer.dyn_clone());
                            off += ipv4_layer.len();

                            match ipv4_layer.get_protocol() {
                                Protocols::Icmp => {}
                                Protocols::Igmp => {}
                                Protocols::Tcp => {
                                    let tcp_layer = TcpLayer::from_bytes(&packet.data[off..]).expect("Failed to parse TCP frame");
                                    frame.add_layer(tcp_layer.dyn_clone());
                                    off += tcp_layer.len();
                                }
                                Protocols::Udp => {
                                    let udp_layer = UdpLayer::from_bytes(&packet.data[off..]).expect("Failed to parse UDP frame");
                                    frame.add_layer(udp_layer.dyn_clone());
                                    off += udp_layer.len();
                                }
                                Protocols::Ipv6 => {}
                                Protocols::Gre => {}
                                Protocols::Ospf => {}
                                Protocols::Sps => {}
                            }




                        }
                        Types::Arp => {}
                        Types::IPv6 => {}
                        Types::Broadcast => {}
                    }





                }
                Interfaces::WiFi => {}
                Interfaces::Bluetooth => {}
            }

            tx.lock().unwrap().send(frame).expect("Failed to send packet");

            /*
            let mut frame = Frame::new();

            let ddl_layer = EthernetLayer::from_bytes(packet.data).expect("Failed to parse Ethernet frame");
            frame.add_layer(ddl_layer.dyn_clone());

            let mut off = ddl_layer.len();
            let network_layer = match ddl_layer.get_type() {
                EthernetTypes::IPv4 => {
                    IPv4Layer::from_bytes(&packet.data[off..]).expect("Failed to parse IPv4 frame")
                }
                _ => {
                    todo!()
                }
            };

            off += network_layer.len();
            frame.add_layer(network_layer.dyn_clone());



            frame.get_layer(1).unwrap().get_type();*/


            /*
            let ethernet_frame = EthernetFrame::from_bytes(packet.data).expect("Failed to parse Ethernet frame");

            match ethernet_frame.get_type() {
                EthernetTypes::IPv4 => {
                    let ip_header = Ipv4Header::from_bytes(&packet.data[14..]).expect("Failed to parse IP header");

                    let time = get_timestamp(packet.header.ts.tv_sec as u32, packet.header.ts.tv_usec as u32)-now;

                    let packet = match ip_header.get_protocol() {
                        Protocols::Icmp => {
                            let header = IcmpHeader::from_bytes(&packet.data[34..]).expect("Failed to parse ICMP header");
                            IcmpPacket::from_bytes(ethernet_frame, ip_header, header, time, packet.len(), &packet.data[42..]).expect("Failed to parse ICMP packet").dyn_clone()
                        }
                        Protocols::Tcp => {
                            let header = TcpHeader::from_bytes(&packet.data[34..]).expect("Failed to parse TCP header");
                            TcpPacket::from_bytes(ethernet_frame, ip_header, header, time, packet.len(), &packet.data[54..]).expect("Failed to parse TCP packet").dyn_clone()
                        }
                        Protocols::Udp => {
                            let header = UdpHeader::from_bytes(&packet.data[34..]).expect("Failed to parse UDP header");

                            //LAZY DNS CHECK, WE WILL PARSE THE HEADER LATER FOR THIS
                            if header.get_source_port() == 53 || header.get_destination_port() == 53 {
                                DnsPacket::from_bytes(ethernet_frame, ip_header, header, time, packet.len(), &packet.data[42..]).expect("Failed to parse DNS packet").dyn_clone()

                            } else {
                                UdpPacket::from_bytes(ethernet_frame, ip_header, header, time, packet.len(), &packet.data[42..]).expect("Failed to parse UDP packet").dyn_clone()
                            }
                        }
                        _ => {
                            todo!()
                        }
                    };

                    tx.lock().unwrap().send(packet).expect("Failed to send packet");
                }
                EthernetTypes::Arp => {}
                EthernetTypes::IPv6 => {}
                _ => {}
            }
            */
        }
    });
}

fn get_timestamp(ts_sec: u32, ts_usec: u32) -> f64 {
    //(ts_sec as u128 * 1000) + (ts_usec as u128 / 1000)
    ts_sec as f64 * 1000.0 + ts_usec as f64 / 1000.0
}
