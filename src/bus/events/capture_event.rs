use std::any::Any;
use pcap::packet::packet::Packet;
use crate::bus::events::inter::event::Event;

#[derive(Debug, Clone)]
pub struct CaptureEvent {
    prevent_default: bool,
    if_index: i32,
    packet: Packet
}

impl CaptureEvent {

    pub fn new(if_index: i32, packet: Packet) -> Self {
        Self {
            prevent_default: false,
            if_index,
            packet
        }
    }

    pub fn get_if_index(&self) -> i32 {
        self.if_index
    }

    pub fn get_packet(&self) -> &Packet {
        &self.packet
    }
}

impl Event for CaptureEvent {

    fn get_name(&self) -> String {
        String::from("capture_event")
    }

    fn is_prevent_default(&self) -> bool {
        todo!()
    }

    fn prevent_default(&mut self) {
        todo!()
    }

    fn upcast(&self) -> &dyn Event {
        self
    }

    fn upcast_mut(&mut self) -> &mut dyn Event {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Event> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
