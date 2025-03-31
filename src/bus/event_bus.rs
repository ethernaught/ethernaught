use std::cell::RefCell;
use std::collections::HashMap;
use gtk::glib;
use crate::bus::events::inter::event::Event;
use crate::utils::random;

type EventCallback = Box<dyn Fn(&Box<dyn Event>) -> EventPropagation>;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum EventPropagation {
    Continue,
    Stop
}

struct CallbackState {
    callback: EventCallback,
    paused: bool,
}

thread_local! {
    static EVENT_BUS: RefCell<HashMap<String, HashMap<u32, CallbackState>>> = RefCell::new(HashMap::new());
}

pub fn register_event<F>(event: &str, callback: F, paused: bool) -> u32
where
    F: Fn(&Box<dyn Event>) -> EventPropagation + 'static,
{
    let callback_id = random::gen::<u32>();

    EVENT_BUS.with(|subs| {
        let mut subs = subs.borrow_mut();
        subs.entry(event.to_string())
            .or_insert_with(HashMap::new)
            .insert(
                callback_id,
                CallbackState {
                    callback: Box::new(callback),
                    paused
                }
            );
    });

    callback_id
}

pub fn unregister_event(event: &str, callback_id: u32) -> bool {
    EVENT_BUS.with(|subs| {
        let mut subs = subs.borrow_mut();
        if let Some(callbacks) = subs.get_mut(event) {
            if callbacks.remove(&callback_id).is_some() {
                if callbacks.is_empty() {
                    subs.remove(event);
                }
                return true;
            }
        }
        false
    })
}

pub fn pause_event(event: &str, callback_id: u32) -> bool {
    EVENT_BUS.with(|subs| {
        let mut subs = subs.borrow_mut();
        if let Some(callbacks) = subs.get_mut(event) {
            if let Some(callback_state) = callbacks.get_mut(&callback_id) {
                callback_state.paused = true;
                return true;
            }
        }
        false
    })
}

pub fn resume_event(event: &str, callback_id: u32) -> bool {
    EVENT_BUS.with(|subs| {
        let mut subs = subs.borrow_mut();
        if let Some(callbacks) = subs.get_mut(event) {
            if let Some(callback_state) = callbacks.get_mut(&callback_id) {
                callback_state.paused = false;
                return true;
            }
        }
        false
    })
}

pub fn send_event(data: Box<dyn Event>) {
    glib::MainContext::default().invoke(move || {
        EVENT_BUS.with(|subs| {
            let mut subs = subs.borrow_mut();
            if let Some(callbacks) = subs.get_mut(&data.get_name()) {
                let mut keys_to_remove = Vec::new();

                for (key, callback_state) in callbacks.iter() {
                    if !callback_state.paused {
                        if !(callback_state.callback)(&data).eq(&EventPropagation::Continue) {
                            keys_to_remove.push(key.clone());
                        }
                    }
                }

                for key in keys_to_remove {
                    callbacks.remove(&key);
                }
            }
        });
    });
}
