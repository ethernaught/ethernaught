mod pcaps;
mod ui;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use pcap::devices::Device;
use gtk::prelude::*;
use pcap::capture::Capture;
use crate::ui::application::OApplication;

//SIDEBAR SHOULD BE A FRAGMENT...

fn main() {

    /*
    let devices = Device::list().expect("Failed to get device list");
    let device = devices.into_iter().find(|d| d.get_name().contains("wlp7s0")).unwrap();

    let usage_data = Arc::new(Mutex::new(vec![0u64; 30]));

    let usage_data_clone = Arc::clone(&usage_data);

    thread::spawn(move || {
        let mut cap = Capture::from_device(device).expect("Failed to open device");
        cap.set_promiscuous_mode(true);
        cap.set_immediate_mode(true);
        cap.open().expect("Failed to start capture");

        let mut last_update = SystemTime::now();
        let mut byte_count = 0u64;

        loop {
        //while let Ok(packet) = cap.next_packet() {
            let packet = cap.next_packet().unwrap();
            byte_count += packet.len() as u64;

            let now = SystemTime::now();
            if now.duration_since(last_update).unwrap().as_secs() >= 1 {
                let mut usage = usage_data_clone.lock().unwrap();
                if usage.len() >= 30 {
                    usage.remove(0);
                }
                usage.push(byte_count);

                println!("Last second usage: {} bytes", byte_count);

                byte_count = 0;
                last_update = now;
            }
        }
    });

    let usage_data_clone2 = Arc::clone(&usage_data);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));

            let usage = usage_data_clone2.lock().unwrap();
            println!("Last 30 seconds usage: {:?}", usage);
        }
    });

    loop {
        thread::sleep(Duration::from_secs(1));
    }
    */


    let app = OApplication::new();
    app.run();
}
