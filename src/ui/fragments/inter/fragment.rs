use gtk::Container;

pub trait Fragment {

    fn get_name(&self) -> String;

    fn get_title(&self) -> String;

    fn on_create(&mut self) -> &Container;

    fn on_resume(&self);

    fn on_pause(&self);

    fn on_destroy(&self);
}
