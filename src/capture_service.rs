use std::net::IpAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::SocketExtManual;
use pcap::capture::Capture;
use pcap::devices::Device;
use pcap::packet::inter::interfaces::Interfaces;
use pcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use pcap::packet::layers::ethernet_frame::inter::types::Types;
use pcap::packet::layers::ethernet_frame::ip::inter::protocols::Protocols;
use pcap::packet::layers::ethernet_frame::ip::ipv4_layer::Ipv4Layer;
use pcap::packet::layers::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use pcap::packet::layers::ethernet_frame::ip::udp::inter::udp_payloads::UdpPayloads;
use pcap::packet::layers::ethernet_frame::ip::udp::udp_layer::UdpLayer;
use pcap::packet::packet::Packet;
use crate::database::sqlite::Database;
use crate::ui::handlers::ip_utils::ip_to_code;

#[derive(Clone)]
pub struct CaptureService {
    cap: Option<Capture>,
    device: Device,
    running: Arc<AtomicBool>,
    tx: Option<Sender<(Packet, Option<String>, Option<String>)>>
}

impl CaptureService {

    pub fn new(device: &Device) -> Self {
        let cap = match Capture::from_device(&device) {
            Ok(cap) => {
                cap.set_immediate_mode(true);
                cap.open().expect("Failed to start capture");
                Some(cap)
            }
            Err(error) => {
                println!("Failed to open capture: {}", error);
                None
            }
        };

        Self {
            cap,
            device: device.clone(),
            running: Arc::new(AtomicBool::new(false)),
            tx: None
        }
    }

    pub fn send(&self, packet: Packet) {
        match self.cap.as_ref() {
            Some(cap) => {
                cap.send_packet(packet);
            }
            _ => unimplemented!()
        }
    }

    pub fn set_tx(&mut self, tx: Sender<(Packet, Option<String>, Option<String>)>) {
        self.tx = Some(tx);
    }

    pub fn start(&self) {
        if self.is_running() {
            return;
        }

        self.running.store(true, Ordering::Relaxed);
        let mut _self = self.clone();
        thread::spawn(move || {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as f64;
            let db = Database::open("database.db").unwrap();

            match _self.cap.as_mut() {
                Some(cap) => {
                    while _self.running.load(Ordering::Relaxed) {
                        match cap.next_packet() {
                            Ok(packet) => {
                                //packet.get_frame_time()-now);

                                let (source_icon, destination_icon) = match packet.get_interface() {
                                    Interfaces::Ethernet => {
                                        let ethernet_frame = packet.get_frame().as_any().downcast_ref::<EthernetFrame>().unwrap();

                                        match ethernet_frame.get_type() {
                                            Types::IPv4 => {
                                                let ipv4_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv4Layer>().unwrap();
                                                (ip_to_code(&db, IpAddr::V4(ipv4_layer.get_source_address())), ip_to_code(&db, IpAddr::V4(ipv4_layer.get_destination_address())))
                                            }
                                            Types::IPv6 => {
                                                let ipv6_layer = ethernet_frame.get_data().unwrap().as_any().downcast_ref::<Ipv6Layer>().unwrap();
                                                (ip_to_code(&db, IpAddr::V6(ipv6_layer.get_source_address())), ip_to_code(&db, IpAddr::V6(ipv6_layer.get_destination_address())))
                                            }
                                            _ => {
                                                (None, None)
                                            }
                                        }
                                    }
                                    _ => {
                                        (None, None)
                                    }
                                };

                                _self.tx.as_ref().unwrap().send((packet, source_icon, destination_icon)).expect("Failed to send packet");
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                }
                _ => unimplemented!()
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn get_device(&self) -> &Device {
        &self.device
    }
}

/*
fn get_timestamp(ts_sec: u32, ts_usec: u32) -> f64 {
    //(ts_sec as u128 * 1000) + (ts_usec as u128 / 1000)
    ts_sec as f64 * 1000.0 + ts_usec as f64 / 1000.0
}
*/
