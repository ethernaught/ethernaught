mod pcap;
mod packet;
mod ui;

use gtk::prelude::*;
use crate::ui::application::OApplication;

fn main() {
    let app = OApplication::new();
    app.on_create();
}
