use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use gtk::prelude::SocketExtManual;
use pcap::capture::Capture;
use pcap::devices::Device;
use pcap::packet::packet::Packet;

pub fn packet_capture(tx: Arc<Mutex<Sender<Packet>>>, device: Device) {
    thread::spawn(move || {
        let mut cap = Capture::from_device(device).expect("Failed to open device");
        cap.set_promiscuous_mode(true);
        cap.set_immediate_mode(true);
        cap.open().expect("Failed to start capture");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as f64;

        while let Ok(packet) = cap.next_packet() {
            //packet.get_frame_time()-now);
            tx.lock().unwrap().send(packet).expect("Failed to send packet");
        }
    });
}

fn get_timestamp(ts_sec: u32, ts_usec: u32) -> f64 {
    //(ts_sec as u128 * 1000) + (ts_usec as u128 / 1000)
    ts_sec as f64 * 1000.0 + ts_usec as f64 / 1000.0
}
