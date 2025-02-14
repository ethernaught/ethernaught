use gtk::Paned;

pub trait Fragment {

    fn on_create(&mut self) -> &Paned;

    fn on_resume(&self);

    fn on_pause(&self);

    fn on_destroy(&self);
}
