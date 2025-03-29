use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use gtk::glib::once_cell::sync::Lazy;
use crate::bus::events::inter::event::Event;

type EventCallback = Box<dyn Fn(&Box<dyn Event>) + Send + Sync>;

static EVENT_BUS: Lazy<Arc<Mutex<HashMap<String, Vec<EventCallback>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

pub fn register_event<F>(event: &str, callback: F)
where
    F: Fn(&Box<dyn Event>) + Send + Sync + 'static,
{
    let mut subs = EVENT_BUS.lock().unwrap();
    subs.entry(event.to_string())
        .or_insert_with(Vec::new)
        .push(Box::new(callback));
}

pub fn unregister_event(event: &str, callback_id: usize) -> bool {
    let mut subs = EVENT_BUS.lock().unwrap();

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
}

pub fn send_event(event: &str, data: Box<dyn Event>) {
    if let Ok(subs) = EVENT_BUS.lock() {
        if let Some(callbacks) = subs.get(event) {
            for callback in callbacks {
                callback(&data);
            }
        }
    }
}
