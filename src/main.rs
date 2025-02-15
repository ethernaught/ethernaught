mod pcaps;
mod ui;

use pcap::devices::Device;
use gtk::prelude::*;
use crate::ui::application::OApplication;

//SIDEBAR SHOULD BE A FRAGMENT...

fn main() {
    let devices = Device::list().expect("Failed to get device list");
    println!("Devices: {:?}", devices);

    let app = OApplication::new();
    app.run();
}
