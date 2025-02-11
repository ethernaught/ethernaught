use std::any::Any;
use crate::packet::headers::ethernet_frame::EthernetFrame;

pub trait Packet {

    fn get_ethernet_frame(&self) -> &EthernetFrame;

    fn get_data(&self) -> Vec<u8>;

    fn len(&self) -> usize;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn upcast(&self) -> &dyn Packet;

    fn upcast_mut(&mut self) -> &mut dyn Packet;

    fn dyn_clone(&self) -> Box<dyn Packet>;
}
