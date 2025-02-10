use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use gtk::ListBox;
use gtk::prelude::ContainerExt;
use pcap::{Capture, Device};
use crate::application::create_row;
use crate::PacketType;

pub fn packet_capture(tx: Arc<Mutex<Sender<PacketType>>>) {

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
                    0x01 => {
                        tx.lock().unwrap().send(PacketType::Icmp).unwrap();
                    },
                    0x06 => {
                        tx.lock().unwrap().send(PacketType::Tcp).unwrap();
                    },
                    0x11 => {
                        tx.lock().unwrap().send(PacketType::Udp).unwrap();
                    },
                    0x2F => {
                        tx.lock().unwrap().send(PacketType::Gre).unwrap();
                    },
                    _    => println!("Captured an unknown protocol: {}", protocol),
                }

                ;//.add(&create_row(PacketType::Gre));
            }
        }
    });
}