use gtk::gio::{SimpleAction, SimpleActionGroup};
use gtk::prelude::{ActionMapExt, WidgetExt};
use gtk::{glib, ApplicationWindow};
use pcap::devices::Device;

pub fn register_window_actions(window: &ApplicationWindow) {
    //window.action_group("win");
    //let actions = SimpleActionGroup::new();

    //let actions = SimpleActionGroup::new();
    let action = SimpleAction::new("open", None);
    action.connect_activate({
        move |_, param| {
            println!("A  {:?}", param);
            if let Some(param) = param {
                //if let Ok(view_name) = param.str() {
                    println!("Switching to view: {:?}", param);
                //}
            }
        }
    });
    window.add_action(&action);
    //window.insert_action_group("win", Some(&actions));
}
