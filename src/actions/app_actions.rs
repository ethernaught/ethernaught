use gtk::Application;
use gtk::gio::SimpleAction;
use gtk::prelude::{ActionMapExt, ApplicationExt};

pub fn register_app_actions(app: &Application) {
    let action = SimpleAction::new("quit", None);
    action.connect_activate({
        let app = app.clone();
        move |_, _| {
            app.quit();
        }
    });
    app.add_action(&action);
}
