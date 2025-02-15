mod pcaps;
mod ui;

use pcap::devices::Device;
use gtk::prelude::*;
use crate::ui::application::OApplication;

//SIDEBAR SHOULD BE A FRAGMENT...

fn main() {
    let app = OApplication::new();
    app.run();
}
