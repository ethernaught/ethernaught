mod pcap_ext;
mod database;
mod app;
mod actions;
mod views;
mod widgets;
mod windows;
mod utils;
mod bus;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::{env, thread};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use gtk::glib::once_cell::sync::Lazy;
use pcap::devices::Device;
use gtk::prelude::*;
use pcap::capture::Capture;
use crate::app::App;
use crate::bus::events::permission_event::PermissionEvent;
use crate::bus::event_bus::{register_event, send_event};
use crate::bus::events::capture_event::CaptureEvent;
use crate::database::sqlite::Database;

//SIDEBAR SHOULD BE A FRAGMENT...
//export GTK_DEBUG=interactive

//glib-compile-resources res/gresources.xml --target=res/resources.gresources

/*
rustup install nightly
rustup override set nightly
*/

//re-implement sidebar replay...
//save fragments so we can clear adapter
//redo sidebar - doesnt look quite right on drop downs...

//FIND A BETTER METHOD WITHIN PCAP LIB TO SEND ADDRESS DETAILS AS IT WOND BE THE SAME STRUCT PER OS
// - not to mention we will need the data when saving, so might want to add to the packet - MAINLY FOR PROMISCUOUS MODE...

//DPI the padding for hex editor

//MacOS Font goes to /Library/fonts

fn main() {
    register_event("capture_event", |event| {
        let event = event.as_any().downcast_ref::<CaptureEvent>().unwrap();
        println!("{:?}", event.get_packet());
    });

    #[cfg(target_os = "linux")]
    thread::spawn(move || {
        match Capture::any() {
            Ok(cap) => {
                cap.set_immediate_mode(true).unwrap();

                match cap.open() {
                    Ok(_) => {
                        let mut if_bytes = HashMap::new();

                        let mut time = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards")
                            .as_millis();

                        loop {
                            match cap.try_recv() {
                                Ok((address, packet)) => {
                                    *if_bytes.entry(-1).or_insert(0) += packet.len();
                                    *if_bytes.entry(address.sll_ifindex).or_insert(0) += packet.len();

                                    send_event(Box::new(CaptureEvent::new(address.sll_ifindex, packet)));
                                }
                                _ => {}
                            }

                            let now = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards")
                                .as_millis();

                            if now-time >= 1000 {
                                time = now;

                                //tx.send(Box::new(TransmittedEvent::new(if_bytes.clone()))).unwrap();

                                if_bytes.clear();
                            }

                            sleep(Duration::from_millis(10));
                        }
                    }
                    Err(_) => {
                        //tx.send(Box::new(PermissionEvent::new(false))).unwrap();
                    }
                }
            }
            Err(_) => {
                //tx.send(Box::new(PermissionEvent::new(false))).unwrap();
            }
        }
    });

    #[cfg(target_os = "macos")]
    thread::spawn(move || {
        let mut captures = Vec::new();
        devices.iter().for_each(|device| {
            if device.get_flags().contains(&InterfaceFlags::Running) {
                match Capture::from_device(device) {
                    Ok(cap) => {
                        cap.set_immediate_mode(true);
                        match cap.open() {
                            Ok(_) => {
                                captures.push(cap);
                            }
                            Err(_) => {
                                tx.send(Box::new(PermissionEvent::new(false))).unwrap();
                                return;
                            }
                        }
                    }
                    Err(_) => {
                        tx.send(Box::new(PermissionEvent::new(false))).unwrap();
                        return;
                    }
                }
            }
        });

        if captures.is_empty() {
            return;
        }

        let mut if_bytes = HashMap::new();

        let mut time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        loop {
            for cap in &captures {
                match cap.try_recv() {
                    Ok((address, packet)) => {
                        let device = cap.get_device().unwrap();
                        *if_bytes.entry(-1).or_insert(0) += packet.len();
                        *if_bytes.entry(device.get_index()).or_insert(0) += packet.len();
                        //*if_bytes.entry(address.sll_ifindex).or_insert(0) += packet.len();

                        tx.send(Box::new(CaptureEvent::new(device.get_index(), packet))).unwrap();
                    }
                    _ => {}
                }
            }

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            if now-time >= 1000 {
                time = now;

                let event = TransmittedEvent::new(if_bytes.clone());
                tx.send(Box::new(event)).unwrap();

                if_bytes.clear();
            }

            sleep(Duration::from_millis(10));
        }
    });

    let app = App::new();
    app.run();


    /*
    if !cfg!(debug_assertions) {
        if !is_root() {
            println!("{:?}", env::current_exe().unwrap());
            let display = env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string());
            let xauthority = env::var("XAUTHORITY").unwrap_or_else(|_| "/run/user/1000/gdm/Xauthority".to_string());

            let args: Vec<String> = env::args().skip(1).collect();

            let mut command = Command::new("pkexec");
            command.arg("env")
                .arg(format!("DISPLAY={}", display))
                .arg(format!("XAUTHORITY={}", xauthority))
                .arg(env::current_exe().unwrap());

            for arg in args {
                command.arg(arg);
            }

            exit(command.status().expect("Failed to execute pkexec").code().unwrap_or(1));
        }
    }
    */
}

//CAN WE CHANGE THIS TO A VARIABLE SET ON BUILD...?
pub fn get_lib_path(file_name: &str) -> PathBuf {
    if cfg!(debug_assertions) {
        return PathBuf::from(file_name);
    }

    PathBuf::from(format!("/usr/var/lib/ethernaught/{}", file_name))
}

fn is_root() -> bool {
    match env::var("USER") {
        Ok(user) => user == "root",
        Err(_) => false,
    }
}
