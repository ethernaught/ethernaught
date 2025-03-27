use gtk::{gdk, Builder, Container, CssProvider, Label, ListBox, StyleContext};
use gtk::prelude::{BuilderExtManual, ContainerExt, CssProviderExt, LabelExt, ListBoxExt, WidgetExt};

pub struct BottomBar {
    pub root: gtk::Box,
    pub license: Label
}

impl BottomBar {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/bottom_bar.ui");


        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in bottom_bar.ui");

        let license: Label = builder
            .object("license")
            .expect("Couldn't find 'license' in bottom_bar.ui");

        license.set_label(format!("{}-{}", env!("PROFILE"), env!("CARGO_PKG_VERSION")).as_str());



        Self {
            root,
            license
        }
    }
}
