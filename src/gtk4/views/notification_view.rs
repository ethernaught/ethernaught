use gtk4::{Builder, Button, Image, Label};
use gtk4::prelude::ButtonExt;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum NotificationTypes {
    Info,
    Warning,
    Error
}

#[derive(Clone)]
pub struct NotificationView {
    pub root: gtk4::Box,
    pub title: Label,
    pub description: Label
}

impl NotificationView {

    pub fn new(_type: NotificationTypes, title: &str, description: &str) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/notification_view.ui");

        let root: gtk4::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in notification_view.ui");

        let icon: Image = builder
            .object("icon")
            .expect("Couldn't find 'icon' in notification_view.ui");
        match _type {
            NotificationTypes::Info => {
                icon.set_resource(Some("/net/ethernaught/rust/res/icons/if_notification_info.svg"));
            }
            NotificationTypes::Warning => {
                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_notification_warning.svg"));
            }
            NotificationTypes::Error => {
                icon.set_resource(Some("/net/ethernaught/rust/res/icons/ic_notification_error.svg"));
            }
        }

        let title_label: Label = builder
            .object("title")
            .expect("Couldn't find 'title' in notification_view.ui");
        title_label.set_text(title);

        let description_label: Label = builder
            .object("description")
            .expect("Couldn't find 'description' in notification_view.ui");
        description_label.set_wrap(true);
        description_label.set_text(description);

        let close: Button = builder
            .object("close")
            .expect("Couldn't find 'close' in notification_view.ui");

        close.connect_clicked({
            let root = root.clone();
            move |button| {
                /*
                if let Some(parent) = root.parent() {
                    if let Some(container) = parent.downcast_ref::<Container>() {
                        container.remove(&root);
                    }
                }
                */
            }
        });

        Self {
            root,
            title: title_label,
            description: description_label
        }
    }
}
