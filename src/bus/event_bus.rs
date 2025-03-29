use std::cell::RefCell;
use std::collections::HashMap;
use gtk::glib;
use crate::bus::events::inter::event::Event;

type EventCallback = Box<dyn Fn(&Box<dyn Event>)>;

thread_local! {
    static EVENT_BUS: RefCell<HashMap<String, Vec<EventCallback>>> = RefCell::new(HashMap::new());
}

pub fn register_event<F>(event: &str, callback: F) -> usize
where
    F: Fn(&Box<dyn Event>) + 'static,
{
    EVENT_BUS.with(|subs| {
        let mut subs = subs.borrow_mut();
        subs.entry(event.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
        subs.len()
    })
}

pub fn unregister_event(event: &str, callback_id: usize) -> bool {
    EVENT_BUS.with(|subs| {
        let mut subs = subs.borrow_mut();

        if let Some(callbacks) = subs.get_mut(event) {
            if callback_id < callbacks.len() {
                callbacks.remove(callback_id);
                if callbacks.is_empty() {
                    subs.remove(event);
                }
                return true;
            }
        }
        false
    })
}

pub fn send_event(data: Box<dyn Event>) {
    glib::MainContext::default().invoke(move || {
        EVENT_BUS.with(|subs| {
            let subs = subs.borrow();
            if let Some(callbacks) = subs.get(&data.get_name()) {
                for callback in callbacks {
                    callback(&data);
                }
            }
        });
    });
}
