use gtk::Container;
use crate::ui::fragment::inter::fragment::Fragment;

pub trait Activity {

    fn get_name(&self) -> String;

    fn get_title(&self) -> String;

    fn on_create(&mut self) -> &Container;

    fn on_resume(&self);

    fn on_pause(&self);

    fn on_destroy(&self);

    fn start_fragment(&self, fragment: Box<dyn Fragment>);

    fn dyn_clone(&self) -> Box<dyn Activity>;
}
