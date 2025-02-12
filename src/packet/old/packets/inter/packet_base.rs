use std::any::Any;
use crate::packet::headers::ethernet_frame::EthernetFrame;
use crate::packet::inter::types::Types;

pub trait PacketBase: Send {

    fn get_ethernet_frame(&self) -> &EthernetFrame;

    fn get_type(&self) -> Types;

    fn get_data(&self) -> Vec<u8>;

    fn len(&self) -> usize;

    fn get_frame_time(&self) -> u128;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn upcast(&self) -> &dyn PacketBase;

    fn upcast_mut(&mut self) -> &mut dyn PacketBase;

    fn dyn_clone(&self) -> Box<dyn PacketBase>;
}
