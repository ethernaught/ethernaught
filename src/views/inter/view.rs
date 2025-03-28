use gtk::Container;

pub trait View {

    fn get_name(&self) -> String;

    fn get_root(&self) -> &Container;

    fn on_resume(&self);

    fn on_pause(&self);

    fn on_destroy(&self);
}