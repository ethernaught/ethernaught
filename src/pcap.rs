use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use gtk::ListBox;
use gtk::prelude::{ContainerExt, SocketExtManual};
use pcap::{Capture, Device};
use crate::application::create_row;
use crate::packet::headers::ethernet_frame::EthernetFrame;
use crate::packet::inter::types::Types;
use crate::packet::headers::ip_header::IpHeader;
use crate::packet::packets::inter::packet::Packet;
use crate::packet::packets::udp_packet::UdpPacket;
//use crate::PacketType;

pub fn packet_capture(tx: Arc<Mutex<Sender<Box<dyn Packet>>>>) {
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

        while let Ok(packet) = cap.next_packet() {
            //println!("Captured packets: {:?} ({} bytes)", packets, packets.data.len());

            let ethernet_frame = EthernetFrame::from_bytes(packet.data).expect("Failed to parse Ethernet frame");
            println!("{:?}", ethernet_frame._type);

            match ethernet_frame._type {
                Types::IPv4 => {
                    let ip_header = IpHeader::from_bytes(&packet.data[14..]).expect("Failed to parse IP header");
                    //tx.send(packets.header.ts, ip_header.source_ip, ip_header.destination_ip);
                    println!("{:?} {} {}", ip_header.protocol, ip_header.source_ip.to_string(), ip_header.destination_ip.to_string());

                    let packet = UdpPacket::from_bytes(ethernet_frame, ip_header, &packet.data[14..]).expect("Failed to parse UDP packet").dyn_clone();
                    //tx.lock().unwrap().send(packet).expect("Failed to send packet");
                }
                Types::Arp => {}
                Types::IPv6 => {}
                _ => {}
            }



        }
    });
}
