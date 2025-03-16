use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::time::Duration;
use gtk::glib;
use gtk::glib::ControlFlow::Continue;
use crate::ui::handlers::events::inter::event::Event;

#[derive(Clone)]
pub struct Handler {
    tx: Sender<Box<dyn Event>>,
    event_listeners: Rc<RefCell<HashMap<String, Box<dyn Fn(Box<dyn Event>)>>>>
}

impl Handler {

    pub fn new() -> Self {
        let (tx, rx) = channel();

        let _self = Self {
            tx,
            event_listeners: Rc::new(RefCell::new(HashMap::new()))
        };

        let event_listeners = _self.event_listeners.clone();
        glib::timeout_add_local(Duration::from_millis(10), move || {
            loop {
                match rx.try_recv() {
                    Ok(event) => {
                        if event_listeners.borrow().contains_key(&event.get_name()) {
                            event_listeners.borrow().get(&event.get_name()).unwrap()(event);
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

    pub fn get_sender(&self) -> Sender<Box<dyn Event>> {
        self.tx.clone()
    }

    pub fn register_listener<F>(&self, name: &str, post: F)
    where
        F: Fn(Box<dyn Event>) + 'static
    {
        self.event_listeners.borrow_mut().insert(name.to_string(), Box::new(post));
    }

    pub fn remove_listener(&self, name: &str) {
        self.event_listeners.borrow_mut().remove(name);
    }
}
