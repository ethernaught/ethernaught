mod capture_service;
mod ui;
mod layers;
mod database;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::{env, thread};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::process::{exit, Command};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use pcap::devices::Device;
use gtk::prelude::*;
use pcap::capture::Capture;
use crate::database::sqlite::Database;
use crate::ui::application::OApplication;

//SIDEBAR SHOULD BE A FRAGMENT...
//export GTK_DEBUG=interactive

pub static VERSION: &str = "0.1.0";

//glib-compile-resources res/gresources.xml --target=res/resources.gresources

fn main() {
    let target_double = format!("{}-{}", env::consts::ARCH, env::consts::OS);
    println!("{}", target_double);


    //if cfg!(debug_assertions) {

    if !is_root() {
        println!("Requesting root access...");
        println!("{:?}", env::current_exe().unwrap());
        /*
        let status = Command::new("pkexec")
            .arg(env::current_exe().unwrap()) // Relaunch itself with root
            .status()
            .expect("Failed to execute pkexec");

        exit(status.code().unwrap_or(1)); // Exit with the new process status
        */
    }

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

fn is_root() -> bool {
    match env::var("USER") {
        Ok(user) => user == "root",
        Err(_) => false,
    }
}
