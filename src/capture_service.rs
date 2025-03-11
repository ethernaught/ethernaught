use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use gtk::prelude::SocketExtManual;
use pcap::capture::Capture;
use pcap::devices::Device;
use pcap::packet::packet::Packet;

#[derive(Clone)]
pub struct CaptureService {
    cap: Option<Capture>,
    device: Device,
    running: Arc<AtomicBool>,
    tx: Sender<Packet>
}

impl CaptureService {

    pub fn new(device: &Device, tx: Sender<Packet>) -> Self {
        let cap = match Capture::from_device(&device) {
            Ok(mut cap) => {
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
            tx
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

            match _self.cap.as_mut() {
                Some(cap) => {
                    while _self.running.load(Ordering::Relaxed) {
                        match cap.next_packet() {
                            Ok(packet) => {
                                //packet.get_frame_time()-now);
                                _self.tx.send(packet).expect("Failed to send packet");
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
