use gtk::{AboutDialog, ApplicationWindow, Builder, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label, CssProvider, StyleContext, gdk, Stack, Container, TreeView, Widget, Window, gio, MenuBar, MenuItem, Menu};
use gtk::gio::SimpleAction;
use gtk::prelude::*;

#[derive(Clone)]
pub struct TitleBar {
    pub root: gtk::Box,
    //pub menubar: MenuBar,
    //pub navigation_buttons: gtk::Box,
    pub back: Button,
    pub next: Button,
    pub network_type_icon: Image,
    pub network_type_label: Label,
    pub app_options: gtk::Box,
    pub start: Button,
    pub stop: Button
}

impl TitleBar {

    pub fn new(window: &ApplicationWindow) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/title_bar.ui");

        let root: gtk::Box = builder
            .object("root")
            .expect("Couldn't find 'root' in title_bar.ui");

        #[cfg(target_os = "linux")]
        {
            let menubar: MenuBar = builder
                .object("menubar")
                .expect("Couldn't find 'menubar' in title_bar.ui");

            let navigation_buttons: gtk::Box = builder
                .object("navigation_buttons")
                .expect("Couldn't find 'navigation_buttons' in ethernaught_ui.xml");

            menubar.connect_deactivate({
                let navigation_menubar = menubar.clone();
                let navigation_buttons = navigation_buttons.clone();
                move |_| {
                    navigation_menubar.hide();
                    navigation_buttons.show();
                }
            });

            let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/ethernaught_ui.xml");
            let model: gio::MenuModel = builder
                .object("main_window_menu")
                .expect("Couldn't find 'main_window_menu' in ethernaught_ui.xml");
            menubar.bind_model(Some(&model), None, false);
            menubar.show_all();
            menubar.hide();

            let action = SimpleAction::new("menu", None);
            action.connect_activate({
                let navigation_buttons = navigation_buttons.clone();
                move |_, _| {
                    navigation_buttons.hide();
                    menubar.show_all();
                    menubar.select_first(true);
                }
            });
            window.add_action(&action);
        }

        let back: Button = builder
            .object("back")
            .expect("Couldn't find 'back' in ethernaught_ui.xml");

        let next: Button = builder
            .object("next")
            .expect("Couldn't find 'next' in ethernaught_ui.xml");

        let network_type_icon: Image = builder
            .object("network_type_icon")
            .expect("Couldn't find 'network_type_icon' in title_bar.ui");

        let network_type_label: Label = builder
            .object("network_type_label")
            .expect("Couldn't find 'network_type_label' in title_bar.ui");

        let app_options: gtk::Box = builder
            .object("app_options")
            .expect("Couldn't find 'app_options' in title_bar.ui");

        let start: Button = builder
            .object("start")
            .expect("Couldn't find 'start' in title_bar.ui");

        let stop: Button = builder
            .object("stop")
            .expect("Couldn't find 'stop' in title_bar.ui");

        Self {
            root,
            back,
            next,
            network_type_icon,
            network_type_label,
            app_options,
            start,
            stop
        }
    }
}
