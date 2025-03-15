use std::sync::{Arc, Mutex};
use std::time::Duration;
use gtk::glib;
use gtk::glib::ControlFlow::Continue;

pub type Runnable = Box<dyn FnOnce() + Send>;

#[derive(Clone)]
pub struct Handler {
    runnables: Arc<Mutex<Vec<Runnable>>>
}

impl Handler {

    pub fn new() -> Self {
        let _self = Self {
            runnables: Arc::new(Mutex::new(Vec::new()))
        };

        let runnables = _self.runnables.clone();
        glib::timeout_add_local(Duration::from_millis(10), move || {
            for post in runnables.lock().unwrap().drain(..) {
                post();
            }
            //CHECK POSTS...

            Continue
        });

        _self
    }

    pub fn post_runnable(&self, post: Runnable) {
        self.runnables.lock().unwrap().push(post);
        //register timeouts so that we can use them later for call backs...
    }
}
