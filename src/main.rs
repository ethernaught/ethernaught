mod pcap;
mod packet;
mod ui;

use gtk::prelude::*;
use crate::ui::application::OApplication;

//SIDEBAR SHOULD BE A FRAGMENT...

fn main() {
    let app = OApplication::new();
    app.on_create();
}
