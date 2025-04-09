use gtk::{AboutDialog, ApplicationWindow, Builder, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label, CssProvider, StyleContext, gdk, Stack, Container, TreeView, Widget, Window, gio, MenuBar, MenuItem, Menu, Align};
use gtk::prelude::*;

#[derive(Clone)]
pub struct TerminalView {
    pub root: gtk::Box
}

impl TerminalView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/terminal_view.ui");

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in terminal_view.ui");

        Self {
            root
        }
    }
}
