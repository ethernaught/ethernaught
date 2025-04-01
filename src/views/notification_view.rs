use gtk::{AboutDialog, ApplicationWindow, Builder, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label, CssProvider, StyleContext, gdk, Stack, Container, TreeView, Widget, Window, gio, MenuBar, MenuItem, Menu, Align};
use gtk::pango::WrapMode;
use gtk::prelude::*;

#[derive(Clone)]
pub struct NotificationView {
    pub root: gtk::Box,
    icon: Image,
    pub title: Label,
    pub description: Label
}

impl NotificationView {

    pub fn new(message: &str) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/notification_view.ui");

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in notification_view.ui");

        let icon: Image = builder
            .object("icon")
            .expect("Couldn't find 'icon' in notification_view.ui");

        let title: Label = builder
            .object("title")
            .expect("Couldn't find 'title' in notification_view.ui");
        title.set_text("Permissions");

        let description: Label = builder
            .object("description")
            .expect("Couldn't find 'description' in notification_view.ui");
        description.set_wrap(true);
        description.set_text(message);

        Self {
            root,
            icon,
            title,
            description
        }
    }
}
