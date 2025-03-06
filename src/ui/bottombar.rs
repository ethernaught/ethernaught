use gtk::{AboutDialog, ApplicationWindow, Builder, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label, CssProvider, StyleContext, gdk, Stack, Container, TreeView, Widget, Window, gio, MenuBar, MenuItem, Menu};
use gtk::prelude::*;
use gtk::prelude::{ActionMapExt, GtkWindowExt};
use crate::ui::application::OApplication;

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
        let builder = Builder::from_file("res/ui/bottombar-ui.xml");

        let root: gtk::Box = builder
            .object("bottombar")
            .expect("Couldn't find 'bottombar' in bottombar-ui.xml");

        self.root = Some(root.upcast());
        self.root.as_ref().unwrap()
    }
}
