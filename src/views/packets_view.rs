use gtk::{gdk, Builder, Button, Container, CssProvider, Entry, Image, Label, ListBox, ScrolledWindow, StyleContext, TreeView};
use gtk::prelude::{BuilderExtManual, ContainerExt, CssProviderExt, LabelExt, ListBoxExt, WidgetExt};

#[derive(Clone)]
pub struct PacketsView {
    pub root: gtk::Box,
    pub search: Entry,
    pub scroll_layout: ScrolledWindow,
    pub tree_view: TreeView
}

impl PacketsView {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/packets_view.ui");

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in packet_view.ui");

        let search: Entry = builder
            .object("search")
            .expect("Couldn't find 'search' in packet_view.ui");

        let scroll_layout: ScrolledWindow = builder
            .object("list_scroll_layout")
            .expect("Couldn't find 'list_scroll_layout' in packet_view.ui");

        let tree_view: TreeView = builder
            .object("tree_view")
            .expect("Couldn't find 'tree_view' in packet_view.ui");

        Self {
            root,
            search,
            scroll_layout,
            tree_view,
        }
    }
}
