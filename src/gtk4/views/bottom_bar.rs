use std::cell::RefCell;
use std::rc::Rc;
use gtk4::{gio, ApplicationWindow, Builder, Button, HeaderBar, Image, Label, Orientation, PackType, PopoverMenuBar, Widget, WindowControls, WindowHandle};

#[derive(Clone, Default)]
pub struct PacketStatus {
    pub received: u32,
    pub dropped: u32
}

#[derive(Clone)]
pub struct BottomBar {
    pub root: gtk4::Box,
    pub status: Label,
    pub license: Label,
    packet_status: Rc<RefCell<PacketStatus>>
}

impl BottomBar {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/bottom_bar.ui");


        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in bottom_bar.ui");

        let status: Label = builder
            .object("status")
            .expect("Couldn't find 'status' in bottom_bar.ui");

        let license: Label = builder
            .object("license")
            .expect("Couldn't find 'license' in bottom_bar.ui");

        license.set_label(format!("{}-{}-{}", env!("PROFILE"), env!("CARGO_PKG_VERSION"), "gtk4").as_str());

        Self {
            root,
            status,
            license,
            packet_status: Rc::new(RefCell::new(PacketStatus::default()))
        }
    }

    pub fn reset_status(&self) {
        self.packet_status.borrow_mut().received = 0;
        self.packet_status.borrow_mut().dropped = 0;
        self.update_status();
    }

    pub fn add_packet(&self) {
        self.packet_status.borrow_mut().received += 1;
        self.update_status();
    }

    pub fn add_dropped(&self) {
        self.packet_status.borrow_mut().dropped += 1;
        self.update_status();
    }

    fn update_status(&self) {
        let packet_status = self.packet_status.borrow();
        self.status.set_label(&format!("Packets: {} • Dropped: {} ({}%)", packet_status.received, packet_status.dropped, "100"));
    }
}
