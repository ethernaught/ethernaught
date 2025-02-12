use crate::packet::inter::interfaces::Interfaces;
use crate::packet::layers::inter::layer::Layer;

pub struct Packet {
    layers: Vec<Box<dyn Layer>>,
    frame_time: u32,
    length: usize,
    interface: Interfaces
}

impl Packet {

    pub fn new(interface: Interfaces) -> Self {
        Self {
            layers: Vec::new(),
            frame_time: 0,
            length: 0,
            interface
        }
    }

    pub fn add_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    pub fn get_layer(&self, index: usize) -> Option<&Box<dyn Layer>> {
        self.layers.get(index)
    }

    pub fn get_layers(&self) -> &Vec<Box<dyn Layer>> {
        &self.layers
    }

    pub fn get_total_layer(&self) -> usize {
        self.layers.len()
    }

    pub fn get_interface(&self) -> &Interfaces {
        &self.interface
    }

    pub fn len(&self) -> usize {
        self.length
    }
}
