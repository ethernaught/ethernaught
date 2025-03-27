use gtk::{glib, Application};
use gtk::gio::SimpleAction;
use gtk::prelude::ActionMapExt;

pub fn register_app_actions(app: &Application) {
    let action = SimpleAction::new("open", Some(&glib::VariantTy::STRING));
    action.connect_activate({
        move |_, param| {
            println!("BBBBB");
        }
    });
    app.add_action(&action);
}
