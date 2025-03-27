use gtk::{gdk, Builder, Container, CssProvider, Label, ListBox, StyleContext};
use gtk::prelude::{BuilderExtManual, ContainerExt, CssProviderExt, LabelExt, ListBoxExt, WidgetExt};

#[derive(Clone)]
pub struct TitleBar {
    pub root: gtk::Box
}

impl TitleBar {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/title_bar.ui");


        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in title_bar.ui");

        Self {
            root
        }
    }
}
