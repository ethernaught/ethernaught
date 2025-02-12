use std::any::Any;

pub trait Layer {

    fn get_layer_name(&self) -> &str;

    fn len(&self) -> usize;

    fn get_type(&self) -> String;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn dyn_clone(&self) -> Box<dyn Layer>;
}
