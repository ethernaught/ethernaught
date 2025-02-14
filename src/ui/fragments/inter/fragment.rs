use gtk::Container;

pub trait Fragment {

    fn on_create(&mut self) -> &Container;

    fn on_resume(&self);

    fn on_pause(&self);

    fn on_destroy(&self);
}
