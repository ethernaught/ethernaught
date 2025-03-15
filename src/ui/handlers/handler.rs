use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::time::Duration;
use gtk::glib;
use gtk::glib::ControlFlow::Continue;
use crate::ui::handlers::bundle::Bundle;

pub type Runnable = Box<dyn Fn(Option<Bundle>)>;

#[derive(Clone)]
pub struct Handler {
    tx: Sender<(String, Option<Bundle>)>,
    runnables: Rc<RefCell<HashMap<String, Runnable>>>
}

impl Handler {

    pub fn new() -> Self {
        let (tx, rx) = channel();

        let _self = Self {
            tx,
            runnables: Rc::new(RefCell::new(HashMap::new()))
        };

        let runnables = _self.runnables.clone();
        glib::timeout_add_local(Duration::from_millis(10), move || {
            //for post in runnables.lock().unwrap().drain(..) {
            //    post();
            //}
            //CHECK POSTS...
            loop {
                match rx.try_recv() {
                    Ok((name, bundle)) => {
                        runnables.borrow().get(&name).unwrap()(bundle);
                    }
                    Err(_) => {
                        break;
                    }
                }
            }

            Continue
        });

        _self
    }

    pub fn get_sender(&self) -> Sender<(String, Option<Bundle>)> {
        self.tx.clone()
    }

    pub fn post_runnable<F>(&self, name: &str, post: F)
    where
        F: Fn(Option<Bundle>) + 'static
    {
        self.runnables.borrow_mut().insert(name.to_string(), Box::new(post));
        //register timeouts so that we can use them later for call backs...
    }
}
