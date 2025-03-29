use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use gtk::glib::once_cell::sync::Lazy;
use pcap::packet::packet::Packet;
use crate::bus::events::inter::event::Event;

pub static EVENT_BUS: Lazy<Arc<EventBus>> = Lazy::new(|| Arc::new(EventBus::new()));

type EventCallback = Box<dyn Fn(&Box<dyn Event>) + Send + Sync>;

pub struct EventBus {
    subscribers: Mutex<HashMap<String, Vec<EventCallback>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(HashMap::new()),
        }
    }

    pub fn subscribe<F>(&self, event: &str, callback: F)
    where
        F: Fn(&Box<dyn Event>) + Send + Sync + 'static,
    {
        let mut subs = self.subscribers.lock().unwrap();
        subs.entry(event.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(callback));
    }

    pub fn unsubscribe(&self, event: &str, callback_id: usize) -> bool {
        let mut subs = self.subscribers.lock().unwrap();

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

    pub fn publish(&self, event: &str, data: Box<dyn Event>) {
        if let Ok(subs) = self.subscribers.lock() {
            if let Some(callbacks) = subs.get(event) {
                for callback in callbacks {
                    callback(&data);
                }
            }
        }
    }
}
