use std::cell::RefCell;
use std::collections::HashMap;
use gtk::glib;
use crate::bus::events::inter::event::Event;
use crate::utils::random;

type EventCallback = Box<dyn Fn(&Box<dyn Event>)>;

thread_local! {
    static EVENT_BUS: RefCell<HashMap<String, HashMap<u32, EventCallback>>> = RefCell::new(HashMap::new());
}

pub fn register_event<F>(event: &str, callback: F) -> u32
where
    F: Fn(&Box<dyn Event>) + 'static,
{
    let callback_id = random::gen::<u32>(); // Generate random u32 ID

    EVENT_BUS.with(|subs| {
        let mut subs = subs.borrow_mut();
        subs.entry(event.to_string())
            .or_insert_with(HashMap::new)
            .insert(callback_id, Box::new(callback));
    });

    callback_id
}

pub fn unregister_event(event: &str, callback_id: u32) -> bool {
    EVENT_BUS.with(|subs| {
        let mut subs = subs.borrow_mut();
        if let Some(callbacks) = subs.get_mut(event) {
            if callbacks.remove(&callback_id).is_some() {
                // Remove the event if no callbacks remain
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
                for callback in callbacks.values() {
                    callback(&data);
                }
            }
        });
    });
}
