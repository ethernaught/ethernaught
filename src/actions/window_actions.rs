use gtk::gio::{SimpleAction, SimpleActionGroup};
use gtk::prelude::{ActionMapExt, WidgetExt};
use gtk::{glib, ApplicationWindow};

pub fn register_window_actions(window: &ApplicationWindow) {
    //window.action_group("win");
    //let actions = SimpleActionGroup::new();

    //let actions = SimpleActionGroup::new();
    let action = SimpleAction::new("open", Some(&glib::VariantTy::STRING));
    action.connect_activate({
        move |_, param| {
            println!("A");
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
