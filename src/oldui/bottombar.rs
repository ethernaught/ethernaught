use gtk::{AboutDialog, ApplicationWindow, Builder, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label, CssProvider, StyleContext, gdk, Stack, Container, TreeView, Widget, Window, gio, MenuBar, MenuItem, Menu};
use gtk::prelude::*;
use gtk::prelude::{ActionMapExt, GtkWindowExt};
use crate::oldui::application::OApplication;

#[derive(Clone)]
pub struct BottomBar {
    app: OApplication,
    root: Option<Container>
}

impl BottomBar {

    pub fn new(app: OApplication) -> Self {
        Self {
            app,
            root: None
        }
    }

    pub fn on_create(&mut self) -> &Container {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/bottom_bar.ui");

        let root: gtk::Box = builder
            .object("bottombar")
            .expect("Couldn't find 'bottombar' in bottom_bar.ui");

        let license: Label = builder
            .object("license")
            .expect("Couldn't find 'license' in bottom_bar.ui");

        license.set_label(format!("{}-{}", env!("PROFILE"), env!("CARGO_PKG_VERSION")).as_str());

        self.root = Some(root.upcast());
        self.root.as_ref().unwrap()
    }
}
