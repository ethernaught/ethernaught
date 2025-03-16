use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::time::Duration;
use gtk::glib;
use gtk::glib::ControlFlow::Continue;

#[derive(Clone)]
pub struct Handler {
    tx: Sender<(String, Option<Box<dyn Any + Send>>)>,
    runnables: Rc<RefCell<HashMap<String, Box<dyn Fn(Option<Box<dyn Any + Send>>)>>>>
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
            loop {
                match rx.try_recv() {
                    Ok((name, bundle)) => {
                        if runnables.borrow().contains_key(&name) {
                            runnables.borrow().get(&name).unwrap()(bundle);
                        }
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

    pub fn get_sender(&self) -> Sender<(String, Option<Box<dyn Any + Send>>)> {
        self.tx.clone()
    }

    pub fn post_runnable<F>(&self, name: &str, post: F)
    where
        F: Fn(Option<Box<dyn Any + Send>>) + 'static
    {
        self.runnables.borrow_mut().insert(name.to_string(), Box::new(post));
    }

    pub fn remove_runnable(&self, name: &str) {
        self.runnables.borrow_mut().remove(name);
    }
}
