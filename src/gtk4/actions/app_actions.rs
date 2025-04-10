use gtk4::Application;
use gtk4::gio::SimpleAction;
use gtk4::prelude::{ActionMapExt, ApplicationExt};

pub fn register_app_actions(app: &Application) {
    let action = SimpleAction::new("packet-playground", None);
    action.connect_activate({
        move |_, _| {
            //PacketPlaygroundWindow::new();
        }
    });
    app.add_action(&action);

    let action = SimpleAction::new("quit", None);
    action.connect_activate({
        let app = app.clone();
        move |_, _| {
            app.quit();
        }
    });
    app.add_action(&action);
}
