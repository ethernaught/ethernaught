use std::sync::{Arc, Mutex};
use std::thread;
use gtk::ListBox;
use gtk::prelude::ContainerExt;
use pcap::{Capture, Device};
use crate::application::create_row;
use crate::PacketType;

pub fn packet_capture(list_box: Arc<Mutex<ListBox>>) {//list_box: &ListBox) {
    let list_box = list_box.clone();

    /*
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
            println!("Captured packet: {:?} ({} bytes)", packet, packet.data.len());

            if packet.data.len() > 20 { // Ensure it's at least an IPv4 header
                let protocol = packet.data[23]; // Byte 9 in IPv4 header

                match protocol {
                    0x01 => println!("Captured an ICMP Packet"),
                    0x06 => println!("Captured a TCP Packet"),
                    0x11 => println!("Captured a UDP Packet"),
                    0x2F => println!("Captured a GRE Packet"),
                    _    => println!("Captured an unknown protocol: {}", protocol),
                }

                list_box.lock().unwrap().add(&create_row(PacketType::Gre));
            }
        }
    });*/
}