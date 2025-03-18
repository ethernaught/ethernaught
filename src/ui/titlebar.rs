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
use crate::ui::context::Context;

#[derive(Clone)]
pub struct TitleBar {
    context: Context,
    root: Option<Container>
}

impl TitleBar {

    pub fn new(context: Context) -> Self {
        Self {
            context,
            root: None
        }
    }

    pub fn on_create(&mut self) -> &Container {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/titlebar_ui.xml");

        let root: gtk::Box = builder
            .object("titlebar")
            .expect("Couldn't find 'titlebar' in titlebar_ui.xml");

        self.init_navigation_options(&builder);
        self.init_window_options(&builder);

        self.root = Some(root.upcast());

        self.root.as_ref().unwrap()
    }

    fn init_navigation_options(&mut self, builder: &Builder) {
        let navigation_menubar: MenuBar = builder
            .object("navigation_menubar")
            .expect("Couldn't find 'navigation_menubar' in titlebar_ui.xml");

        let menu_builder = Builder::from_resource("/net/ethernaught/rust/res/ui/ethernaught_ui.xml");
        let menu: gio::MenuModel = menu_builder
            .object("main_window_menu")
            .expect("Couldn't find 'main_window_menu' in ethernaught_ui.xml");

        navigation_menubar.bind_model(Some(&menu), None, false);


        let navigation_menubar = Rc::new(RefCell::new(navigation_menubar));
        let navigation_buttons = Rc::new(RefCell::new(builder
            .object::<gtk::Box>("navigation_buttons")
            .expect("Couldn't find 'navigation_buttons' in ethernaught_ui.xml")));


        let navigation_menubar_clone = Rc::clone(&navigation_menubar);
        let navigation_buttons_clone = Rc::clone(&navigation_buttons);

        navigation_menubar.borrow().connect_deactivate(move |_| {
            navigation_menubar_clone.borrow().hide();
            navigation_buttons_clone.borrow().show();
        });

        let menu_button: Button = builder
            .object("menu_button")
            .expect("Couldn't find 'menu_button' in titlebar_ui.xml");

        let navigation_menubar_clone = Rc::clone(&navigation_menubar);
        let navigation_buttons_clone = Rc::clone(&navigation_buttons);

        menu_button.connect_clicked(move |_| {
            navigation_buttons_clone.borrow().hide();
            navigation_menubar_clone.borrow().show_all();
            navigation_menubar_clone.borrow().select_first(true);
        });

        let back_button: Button = builder
            .object("back_button")
            .expect("Couldn't find 'back_button' in titlebar_ui.xml");

        let context = self.context.clone();
        back_button.connect_clicked(move |_| {
            context.on_back_pressed();
        });

        let next_button: Button = builder
            .object("next_button")
            .expect("Couldn't find 'next_button' in titlebar_ui.xml");

        let context = self.context.clone();
        next_button.connect_clicked(move |_| {
            context.on_next_pressed();
        });
    }

    fn init_window_options(&self, builder: &Builder) {
        let minimize_button: Button = builder
            .object("minimize_button")
            .expect("Couldn't find 'minimize_button' in titlebar_ui.xml");

        let window = self.context.get_window().unwrap();
        minimize_button.connect_clicked(move |_| {
            window.iconify();
        });

        let maximize_button: Button = builder
            .object("maximize_button")
            .expect("Couldn't find 'maximize_button' in titlebar_ui.xml");

        let window = self.context.get_window().unwrap();
        maximize_button.connect_clicked(move |_| {
            if window.is_maximized() {
                window.unmaximize();
                return;
            }

            window.maximize();
        });

        let close_button: Button = builder
            .object("close_button")
            .expect("Couldn't find 'close_button' in titlebar_ui.xml");

        let context = self.context.get_application();
        close_button.connect_clicked(move |_| {
            context.quit();
        });
    }
}
