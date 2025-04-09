use std::collections::HashMap;
use std::{io, thread};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rlibpcap::capture::Capture;
use rlibpcap::devices::Device;
use rlibpcap::utils::interface_flags::InterfaceFlags;
use crate::bus::event_bus::send_event;
use crate::bus::events::capture_event::CaptureEvent;
use crate::bus::events::permission_event::PermissionEvent;
use crate::bus::events::transmitted_event::TransmittedEvent;

pub struct Sniffer {
}

impl Sniffer {

    pub fn new() -> Self {
        Self {
        }
    }

    pub fn run(&self) {
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
                                    Err(e) => {
                                        match e.kind() {
                                            io::ErrorKind::WouldBlock => {
                                            }
                                            _ => {
                                                println!("BAD PACKET TYPE....");
                                            }
                                        }
                                    }
                                }

                                let now = SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .expect("Time went backwards")
                                    .as_millis();

                                if now-time >= 1000 {
                                    time = now;

                                    send_event(Box::new(TransmittedEvent::new(if_bytes.clone())));

                                    if_bytes.clear();
                                }

                                sleep(Duration::from_millis(10));
                            }
                        }
                        Err(_) => {
                            send_event(Box::new(PermissionEvent::new(false)));
                        }
                    }
                }
                Err(_) => {
                    send_event(Box::new(PermissionEvent::new(false)));
                }
            }
        });

        //#[cfg(target_os = "macos")]
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        {
            let mut devices = Device::list().expect("Failed to get device list");

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
                                        //send_event(Box::new(PermissionEvent::new(false)));
                                        //return;
                                    }
                                }
                            }
                            Err(_) => {
                                //send_event(Box::new(PermissionEvent::new(false)));
                                //return;
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

                                send_event(Box::new(CaptureEvent::new(device.get_index(), packet)));
                            }
                            Err(e) => {
                                match e.kind() {
                                    io::ErrorKind::WouldBlock => {
                                    }
                                    _ => {
                                        println!("BAD PACKET TYPE....");
                                    }
                                }
                            }
                        }
                    }

                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_millis();

                    if now-time >= 1000 {
                        time = now;

                        send_event(Box::new(TransmittedEvent::new(if_bytes.clone())));

                        if_bytes.clear();
                    }

                    sleep(Duration::from_millis(10));
                }
            });
        }
    }
}