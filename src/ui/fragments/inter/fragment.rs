
pub trait Fragment {

    fn on_create(&self);

    fn on_resume(&self);

    fn on_pause(&self);

    fn on_destroy(&self);
}
