mod pcap_ext;
mod database;
mod app;
mod actions;
mod views;
mod widgets;
mod windows;
mod utils;
mod bus;
mod sniffer;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::{env, thread};
use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use gtk::glib;
use gtk::glib::once_cell::sync::Lazy;
use rlibpcap::devices::Device;
use gtk::prelude::*;
use rlibpcap::capture::Capture;
use rlibpcap::utils::interface_flags::InterfaceFlags;
use crate::app::App;
use crate::bus::events::permission_event::PermissionEvent;
use crate::bus::event_bus::{register_event, send_event};
use crate::bus::events::capture_event::CaptureEvent;
use crate::bus::events::transmitted_event::TransmittedEvent;
use crate::database::sqlite::Database;
use crate::pcap_ext::packet_query::PacketQuery;
//SIDEBAR SHOULD BE A FRAGMENT...
//export GTK_DEBUG=interactive

//glib-compile-resources res/linux.gresources.xml --target=res/resources.gresources

/*
rustup install nightly
rustup override set nightly
*/

//re-implement dropdown replay...
//save fragments so we can clear adapter
//redo dropdown - doesnt look quite right on drop downs...

//FIND A BETTER METHOD WITHIN PCAP LIB TO SEND ADDRESS DETAILS AS IT WOND BE THE SAME STRUCT PER OS
// - not to mention we will need the data when saving, so might want to add to the packet - MAINLY FOR PROMISCUOUS MODE...

//DPI the padding for hex editor

//MacOS Font goes to /Library/fonts

//PCAP - SLL2 we need to be using SLL2 types not DataLinkTypes...

//Setting to change programming language of choice...

// - fix time in list

//look into switching from CSS for handling all icons...

fn main() {
    /*
    register_event("capture_event", |event| {
        let event = event.as_any().downcast_ref::<CaptureEvent>().unwrap();
        println!("{:?}", event.get_packet());
    });
    */
    //unsafe { env::set_var("GTK_THEME", "Adwaita:dark") };

    let app = App::new();
    app.run();

    println!("{:?}", PacketQuery::from("ethernet.source=asd & ipv4.destination=asdasd & tcp"));


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
