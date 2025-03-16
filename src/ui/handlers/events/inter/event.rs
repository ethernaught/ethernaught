pub trait Event {

    fn get_name(&self) -> String;

    fn is_prevent_default(&self) -> bool;

    fn prevent_default(&mut self);
}
