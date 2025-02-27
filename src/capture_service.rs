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
    device: Device,
    running: Arc<AtomicBool>,
    tx: Option<Sender<Packet>>
}

impl CaptureService {

    pub fn new(device: &Device) -> Self {
        Self {
            device: device.clone(),
            running: Arc::new(AtomicBool::new(false)),
            tx: None
        }
    }

    pub fn set_tx(&mut self, tx: Sender<Packet>) {
        self.tx = Some(tx);
    }

    pub fn start(&self) {
        if self.is_running() {
            return;
        }

        self.running.store(true, Ordering::Relaxed);
        let mut _self = self.clone();
        thread::spawn(move || {
            let mut cap = Capture::from_device(&_self.device).expect("Failed to open device");
            //cap.set_promiscuous_mode(true).expect("Failed to set promiscuous mode");
            cap.set_immediate_mode(true);
            cap.open().expect("Failed to start capture");

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as f64;

            while _self.running.load(Ordering::Relaxed) {
                match cap.next_packet() {
                    Ok(packet) => {
                        //packet.get_frame_time()-now);
                        _self.tx.as_mut().unwrap().send(packet).expect("Failed to send packet");
                    }
                    _ => {
                        break;
                    }
                }
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
