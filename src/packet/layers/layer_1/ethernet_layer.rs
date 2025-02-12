use std::any::Any;
use crate::packet::layers::inter::layer::Layer;
use crate::packet::layers::layer_1::inter::types::Types;

#[derive(Clone)]
pub struct EthernetLayer {
    destination: [u8; 6],
    source: [u8; 6],
    _type: Types
}

impl EthernetLayer {

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 14 {
            return None;
        }

        Some(Self {
            destination: [buf[0], buf[1], buf[2], buf[3], buf[4], buf[5]],
            source: [buf[6], buf[7], buf[8], buf[9], buf[10], buf[11]],
            _type: Types::get_type_from_code(u16::from_be_bytes([buf[12], buf[13]])).unwrap()
        })
    }

    pub fn get_destination(&self) -> [u8; 6] {
        self.destination
    }

    pub fn get_source(&self) -> [u8; 6] {
        self.source
    }

    pub fn get_type(&self) -> Types {
        self._type
    }
}

impl Layer for EthernetLayer {

    fn len(&self) -> usize {
        14
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Layer> {
        Box::new(self.clone())
    }
}
