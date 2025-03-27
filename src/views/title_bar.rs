use gtk::{gdk, Builder, Button, Container, CssProvider, Image, Label, ListBox, StyleContext};
use gtk::prelude::{BuilderExtManual, ContainerExt, CssProviderExt, LabelExt, ListBoxExt, WidgetExt};

#[derive(Clone)]
pub struct TitleBar {
    pub root: gtk::Box,
    pub network_type_icon: Image,
    pub network_type_label: Label,
    pub app_options: gtk::Box,
    pub start_button: Button,
    pub stop_button: Button
}

impl TitleBar {

    pub fn new() -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/title_bar.ui");

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in title_bar.ui");

        let network_type_icon: Image = builder
            .object("network_type_icon")
            .expect("Couldn't find 'network_type_icon' in title_bar.ui");

        let network_type_label: Label = builder
            .object("network_type_label")
            .expect("Couldn't find 'network_type_label' in title_bar.ui");

        let app_options: gtk::Box = builder
            .object("app_options")
            .expect("Couldn't find 'app_options' in title_bar.ui");

        let start_button: Button = builder
            .object("start_button")
            .expect("Couldn't find 'start_button' in title_bar.ui");

        let stop_button: Button = builder
            .object("stop_button")
            .expect("Couldn't find 'stop_button' in title_bar.ui");

        Self {
            root,
            network_type_icon,
            network_type_label,
            app_options,
            start_button,
            stop_button
        }
    }
}
