use gtk::{AboutDialog, ApplicationWindow, Builder, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label, CssProvider, StyleContext, gdk, Stack, Container, TreeView, Widget, Window, gio, MenuBar, MenuItem, Menu};
use gtk::prelude::*;

#[derive(Clone)]
pub struct TitleBar {
    pub root: gtk::Box,
    pub navigation_menubar: MenuBar,
    pub navigation_buttons: gtk::Box,
    pub back_button: Button,
    pub next_button: Button,
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

        let navigation_menubar: MenuBar = builder
            .object("navigation_menubar")
            .expect("Couldn't find 'navigation_menubar' in title_bar.ui");


        let navigation_buttons: gtk::Box = builder
            .object("navigation_buttons")
            .expect("Couldn't find 'navigation_buttons' in ethernaught_ui.xml");

        navigation_menubar.connect_deactivate({
            let navigation_menubar = navigation_menubar.clone();
            let navigation_buttons = navigation_buttons.clone();
            move |_| {
                navigation_menubar.hide();
                navigation_buttons.show();
            }
        });

        let back_button: Button = builder
            .object("back_button")
            .expect("Couldn't find 'back_button' in ethernaught_ui.xml");

        let next_button: Button = builder
            .object("next_button")
            .expect("Couldn't find 'next_button' in ethernaught_ui.xml");

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

        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/ethernaught_ui.xml");
        let model: gio::MenuModel = builder
            .object("main_window_menu")
            .expect("Couldn't find 'main_window_menu' in ethernaught_ui.xml");
        navigation_menubar.bind_model(Some(&model), None, false);
        navigation_menubar.show_all();
        navigation_menubar.hide();

        Self {
            root,
            navigation_menubar,
            back_button,
            next_button,
            navigation_buttons,
            network_type_icon,
            network_type_label,
            app_options,
            start_button,
            stop_button
        }
    }

    pub fn open_menubar(&self) {
        self.navigation_buttons.hide();
        self.navigation_menubar.show_all();
        self.navigation_menubar.select_first(true);
    }

    pub fn close_menubar(&self) {
        self.navigation_menubar.hide();
        self.navigation_buttons.show();
    }
}
