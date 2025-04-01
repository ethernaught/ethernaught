use gtk::{AboutDialog, ApplicationWindow, Builder, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label, CssProvider, StyleContext, gdk, Stack, Container, TreeView, Widget, Window, gio, MenuBar, MenuItem, Menu};
use gtk::prelude::*;

#[derive(Clone)]
pub struct NotificationView {
    pub root: gtk::Box,
    pub label: Label
}

impl NotificationView {

    pub fn new(message: &str) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/notification_view.ui");

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in notification_view.ui");

        let label: Label = builder
            .object("label")
            .expect("Couldn't find 'label' in notification_view.ui");
        label.set_text(message);

        Self {
            root,
            label
        }
    }
}
