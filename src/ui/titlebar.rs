use std::process::exit;
use gtk::{AboutDialog, ApplicationWindow, Builder, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label, CssProvider, StyleContext, gdk, Stack, Container, TreeView, Widget, Window, gio, MenuBar, MenuItem, Menu};
use gtk::gdk_pixbuf::PixbufLoader;
use gtk::prelude::*;
use gtk::gio::SimpleAction;
use gtk::glib::PropertyGet;
use gtk::prelude::{ActionMapExt, GtkWindowExt};
use crate::ui::activity::devices_activity::DevicesActivity;
use crate::ui::activity::inter::activity::Activity;
use crate::ui::activity::main_activity::MainActivity;
use crate::ui::application::OApplication;

pub struct TitleBar {
    app: OApplication,
    root: Option<Container>
}

impl TitleBar {

    pub fn new(app: OApplication) -> Self {
        Self {
            app,
            root: None
        }
    }

    pub fn on_create(&mut self) -> &Container {
        let builder = Builder::from_file("res/ui/titlebar-ui.xml");

        let root: gtk::Box = builder
            .object("titlebar")
            .expect("Couldn't find 'titlebar' in titlebar-ui.xml");

        self.init_navigation_options(&builder);
        self.init_window_options(&builder);








        //self.app.set_menubar(Some(&menubar));

        root.add(&self.init_menu_bar());


        self.root = Some(root.upcast());

        self.root.as_ref().unwrap()
    }

    fn init_menu_bar(&self) -> Container {
        let builder = Builder::from_file("res/ui/omniscient-ui.xml");
        let menubar: MenuBar = builder
            .object("main_window_menu")
            .expect("Couldn't find 'main_window_menu' in omniscient-ui.xml");



        let file_menu_item: MenuItem = builder
            .object("file_menu_item")
            .expect("Couldn't find 'file_menu_item' in omniscient-ui.xml");

        let file_menu: Menu = builder
            .object("file_menu")
            .expect("Couldn't find 'file_menu' in omniscient-ui.xml");

        file_menu_item.set_submenu(Some(&file_menu));

        menubar.upcast()
    }

    fn init_navigation_options(&mut self, builder: &Builder) {
        let menu_button: Button = builder
            .object("menu_button")
            .expect("Couldn't find 'menu_button' in titlebar-ui.xml");

        menu_button.connect_clicked(move |_| {
            println!("ON CLICK");
        });



        /*
        let back_button: Button = builder
            .object("back_button")
            .expect("Couldn't find 'back_button' in titlebar-ui.xml");

        let _self = self.clone();
        back_button.connect_clicked(move |_| {
            _self.on_back_pressed();
        });


        let next_button: Button = builder
            .object("next_button")
            .expect("Couldn't find 'next_button' in titlebar-ui.xml");

        let _self = self.clone();
        next_button.connect_clicked(move |_| {
            _self.on_next_pressed();
        });
        */
    }

    fn init_window_options(&self, builder: &Builder) {
        let minimize_button: Button = builder
            .object("minimize_button")
            .expect("Couldn't find 'minimize_button' in titlebar-ui.xml");

        let window = self.app.get_window().unwrap();
        minimize_button.connect_clicked(move |_| {
            window.iconify();
        });

        let maximize_button: Button = builder
            .object("maximize_button")
            .expect("Couldn't find 'maximize_button' in titlebar-ui.xml");

        let window = self.app.get_window().unwrap();
        maximize_button.connect_clicked(move |_| {
            if window.is_maximized() {
                window.unmaximize();
                return;
            }

            window.maximize();
        });

        let close_button: Button = builder
            .object("close_button")
            .expect("Couldn't find 'close_button' in titlebar-ui.xml");

        let app = self.app.get_application();
        close_button.connect_clicked(move |_| {
            app.quit();
        });
    }
}
