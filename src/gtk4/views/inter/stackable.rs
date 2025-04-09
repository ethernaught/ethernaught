use gtk4::Widget;

pub trait Stackable {

    fn get_name(&self) -> String;

    fn get_root(&self) -> &Widget;

    fn on_create(&self);

    fn on_resume(&self);

    fn on_pause(&self);

    fn on_destroy(&self);
}