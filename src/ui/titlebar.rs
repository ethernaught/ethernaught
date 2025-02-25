use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;
use gtk::{AboutDialog, ApplicationWindow, Builder, Image, Application, TreeViewColumn, CellRendererText, ScrolledWindow, Button, ListBoxRow, Label, CssProvider, StyleContext, gdk, Stack, Container, TreeView, Widget, Window, gio, MenuBar, MenuItem, Menu};
use gtk::ffi::GtkMenuBar;
use gtk::gdk_pixbuf::PixbufLoader;
use gtk::prelude::*;
use gtk::gio::SimpleAction;
use gtk::glib::{clone, Propagation, PropertyGet};
use gtk::prelude::{ActionMapExt, GtkWindowExt};
use crate::ui::activity::inter::activity::Activity;
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






        //self.init_menu_bar();

        //self.app.set_menubar(Some(&menubar));



        self.root = Some(root.upcast());

        self.root.as_ref().unwrap()
    }

    fn init_navigation_options(&mut self, builder: &Builder) {

        let navigation_menubar: MenuBar = builder
            .object("navigation_menubar")
            .expect("Couldn't find 'navigation_menubar' in titlebar-ui.xml");

        let menu_builder = Builder::from_file("res/ui/ethernaut-ui.xml");
        let menu: gio::MenuModel = menu_builder
            .object("main_window_menu")
            .expect("Couldn't find 'main_window_menu' in ethernaut-ui.xml");

        navigation_menubar.bind_model(Some(&menu), None, false);
        //navigation_menubar.show_all();

        let is_menu_open = Rc::new(RefCell::new(false));


        for child in navigation_menubar.children() {
            if let Some(menu_item) = child.downcast_ref::<MenuItem>() {
                //if let Some(menu) = menu_item.submenu().unwrap().downcast_ref::<MenuItem>() {
                let mut update_menu_state = Rc::clone(&is_menu_open);
                menu_item.connect_select(move |_| {
                    *update_menu_state.borrow_mut() = true;
                });

                let mut update_menu_state = Rc::clone(&is_menu_open);
                menu_item.connect_deselect(move |_| {
                    *update_menu_state.borrow_mut() = false;
                });
                //}
            }
        }

        /*
        navigation_menubar.connect_event(move |_, event| {
            //println!("Event! {:?}", event);
            Propagation::Proceed
        });
        */




        //let toggle = Rc::new(RefCell::new(false));


        let navigation_buttons: gtk::Box = builder
            .object("navigation_buttons")
            .expect("Couldn't find 'navigation_buttons' in ethernaut-ui.xml");


        let navigation_menubar_clone = navigation_menubar.clone();
        let navigation_buttons_clone = navigation_buttons.clone();

        let update_menu_state = Rc::clone(&is_menu_open);
        navigation_menubar.connect_button_press_event(move |_, event| {
            if *update_menu_state.borrow() {
                navigation_menubar_clone.hide();
                navigation_buttons_clone.show_all();
            }
            Propagation::Proceed
        });







        let menu_button: Button = builder
            .object("menu_button")
            .expect("Couldn't find 'menu_button' in titlebar-ui.xml");

        menu_button.connect_clicked(move |_| {
            //let mut state = toggle.borrow_mut();
            //*state = !*state;

            //if *state {
                navigation_buttons.hide();
                navigation_menubar.show_all();
                navigation_menubar.select_first(true);
            //    return;
            //}
            //navigation_menubar.hide();
            //navigation_buttons.show_all();
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
