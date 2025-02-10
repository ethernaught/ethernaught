use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use gtk::ListBox;
use gtk::prelude::ContainerExt;
use pcap::{Capture, Device};
use crate::application::create_row;
use crate::packet::ethernet_frame::EthernetFrame;
use crate::packet::inter::types::Types;
use crate::packet::ip_header::IpHeader;
//use crate::PacketType;

pub fn packet_capture(tx: Arc<Mutex<Sender<()>>>) {

    thread::spawn(move || {
        let devices = Device::list().expect("Failed to get device list");

        let device = devices.into_iter().find(|d| d.name.contains("wlp2s0"))
            .expect("No suitable device found");

        println!("Listening on device: {}", device.name);

        let mut cap = Capture::from_device(device)
            .expect("Failed to open device")
            .promisc(true)
            .immediate_mode(true)
            .open()
            .expect("Failed to start capture");

        while let Ok(packet) = cap.next_packet() {
            //println!("Captured packet: {:?} ({} bytes)", packet, packet.data.len());

            let ethernet_frame = EthernetFrame::from_bytes(packet.data).expect("Failed to parse Ethernet frame");
            println!("{:?}", ethernet_frame._type);

            match ethernet_frame._type {
                Types::IPv4 => {
                    let ip_header = IpHeader::from_bytes(&packet.data[14..]).expect("Failed to parse IP header");
                    println!("{:?} {} {}", ip_header.protocol, ip_header.source_ip.to_string(), ip_header.destination_ip.to_string());
                }
                Types::Arp => {}
                Types::IPv6 => {}
            }



        }
    });
}
