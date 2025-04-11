use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;
use gtk4::{gdk, style_context_add_provider_for_display, Application, ApplicationWindow, Builder, CssProvider, HeaderBar, Stack, StackPage, StyleContext, Widget};
use gtk4::prelude::{BoxExt, Cast, GtkWindowExt, ListModelExt, ObjectExt, StyleContextExt, WidgetExt};
use rlibpcap::devices::Device;
use rlibpcap::utils::interface_flags::InterfaceFlags;
use crate::gtk4::actions::window_actions::{register_stack_actions, register_window_actions};
use crate::gtk4::views::bottom_bar::BottomBar;
use crate::gtk4::views::devices_view::DevicesView;
use crate::gtk4::views::inter::stackable::Stackable;
use crate::gtk4::views::main_view::MainView;
use crate::gtk4::views::notification_view::{NotificationTypes, NotificationView};
use crate::gtk4::views::title_bar::TitleBar;
use crate::sniffer::Sniffer;

#[derive(Clone)]
pub struct MainWindow {
    pub window: ApplicationWindow,
    pub title_bar: TitleBar,
    pub stack: Stack,
    pub notifications: gtk4::Box,
    pub bottom_bar: BottomBar,
    pub views: Rc<RefCell<HashMap<String, Box<dyn Stackable>>>>
}

impl MainWindow {

    pub fn new(app: &Application) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/window.ui");

        let window: ApplicationWindow = builder
            .object("main_window")
            .expect("Failed to get the 'main_window' from window.ui");

        window.set_application(Some(app));
        window.connect_destroy(|_| exit(0));
        //window.set_decorated(false);
        //window.set_border_width(1);

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/window.css");

        style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        /*
        let (width, height) = window.default_size();

        let hints = Geometry::new(
            width,
            height,
            -1,
            -1,
            0,
            0,
            0,
            0,
            0.0,
            0.0,
            gdk::Gravity::NorthWest);
        window.set_geometry_hints(None::<&gtk::Widget>, Some(&hints), WindowHints::MIN_SIZE);
        */

        #[cfg(profile = "nightly")]
        window.style_context().add_class("nightly");

        #[cfg(profile = "release")]
        window.style_context().add_class("release");

        //window.set_icon_from_file("res/icons/ic_launcher.svg").expect("Failed to load icon");

        let title_bar = TitleBar::new(&window);
        window.set_titlebar(Some(&title_bar.root));

        let root: gtk4::Box = builder
            .object("root")
            .expect("Failed to get the 'root' from window.ui");

        //window_content.add(&create_alertbar());

        let stack: Stack = builder
            .object("stack")
            .expect("Failed to get the 'stack' from window.ui");

        let views: Rc<RefCell<HashMap<String, Box<dyn Stackable>>>> = Rc::new(RefCell::new(HashMap::new()));

        stack.connect_visible_child_name_notify({
            let views = views.clone();
            let mut previous = RefCell::new(String::new());
            move |stack| {
                let current = stack.visible_child_name().unwrap_or_default().to_string();

                if previous.borrow().is_empty() {
                    *previous.borrow_mut() = current;
                    return;
                }

                views.borrow().get(&*previous.borrow()).unwrap().on_pause();

                if views.borrow().contains_key(&current) {
                    views.borrow().get(&current).unwrap().on_resume();
                }

                *previous.borrow_mut() = current;
            }
        });

        let notifications: gtk4::Box = builder
            .object("notifications")
            .expect("Failed to get the 'notifications' from window.ui");

        let bottom_bar = BottomBar::new();
        root.append(&bottom_bar.root);

        let mut devices = Device::list().expect("Failed to get device list");
        devices.sort_by(|a, b| {
            b.get_flags().contains(&InterfaceFlags::Running).cmp(&a.get_flags().contains(&InterfaceFlags::Running))
        });

        /*
        window.connect_button_press_event({
            let window = window.clone();
            move |_, event| {
                match event.button() {
                    8 => {
                        window.activate_action("back", None);
                    }
                    9 => {
                        window.activate_action("next", None);
                    }
                    _ => {}
                }

                Proceed
            }
        });
        */

        window.show();

        let _self = Self {
            window,
            title_bar,
            stack,
            notifications,
            bottom_bar,
            views
        };

        _self.add_view(Box::new(DevicesView::new(&_self, devices)));

        register_window_actions(&_self);
        register_stack_actions(&_self);


        let sniffer = Sniffer::new();
        sniffer.run();

        _self
    }

    pub fn from_file(app: &Application, path: &PathBuf) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/window.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/window.css");
        //provider.load_from_path("res/ui/gtk3/window.css").expect("Failed to load CSS file.");

        style_context_add_provider_for_display(&gdk::Display::default().unwrap(), &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let window: ApplicationWindow = builder
            .object("main_window")
            .expect("Failed to get the 'main_window' from window.ui");

        window.set_application(Some(app));
        window.connect_destroy(|_| exit(0));
        //window.set_decorated(false);
        //window.set_border_width(1);

        /*
        let (width, height) = window.default_size();

        let hints = Geometry::new(
            width,
            height,
            -1,
            -1,
            0,
            0,
            0,
            0,
            0.0,
            0.0,
            gdk::Gravity::NorthWest);
        window.set_geometry_hints(None::<&gtk::Widget>, Some(&hints), WindowHints::MIN_SIZE);
        */

        #[cfg(profile = "nightly")]
        window.style_context().add_class("nightly");

        #[cfg(profile = "release")]
        window.style_context().add_class("release");

        //window.set_icon_from_file("res/icons/ic_launcher.svg").expect("Failed to load icon");

        let title_bar = TitleBar::new(&window);
        window.set_titlebar(Some(&title_bar.root));

        let root: gtk4::Box = builder
            .object("root")
            .expect("Failed to get the 'root' from window.ui");

        //window_content.add(&create_alertbar());

        let stack: Stack = builder
            .object("stack")
            .expect("Failed to get the 'stack' from window.ui");

        let views: Rc<RefCell<HashMap<String, Box<dyn Stackable>>>> = Rc::new(RefCell::new(HashMap::new()));

        stack.connect_visible_child_name_notify({
            let views = views.clone();
            let mut previous = RefCell::new(String::new());
            move |stack| {
                let current = stack.visible_child_name().unwrap_or_default().to_string();

                if previous.borrow().is_empty() {
                    *previous.borrow_mut() = current;
                    return;
                }

                views.borrow().get(&*previous.borrow()).unwrap().on_pause();

                if views.borrow().contains_key(&current) {
                    views.borrow().get(&current).unwrap().on_resume();
                }

                *previous.borrow_mut() = current;
            }
        });

        let notifications: gtk4::Box = builder
            .object("notifications")
            .expect("Failed to get the 'notifications' from window.ui");

        let bottom_bar = BottomBar::new();
        root.append(&bottom_bar.root);

        let mut devices = Device::list().expect("Failed to get device list");
        devices.sort_by(|a, b| {
            b.get_flags().contains(&InterfaceFlags::Running).cmp(&a.get_flags().contains(&InterfaceFlags::Running))
        });

        /*
        window.connect_button_press_event({
            let window = window.clone();
            move |_, event| {
                match event.button() {
                    8 => {
                        window.activate_action("back", None);
                    }
                    9 => {
                        window.activate_action("next", None);
                    }
                    _ => {}
                }

                Proceed
            }
        });
        */

        window.show();

        let _self = Self {
            window,
            title_bar,
            stack,
            notifications,
            bottom_bar,
            views
        };

        _self.add_view(Box::new(DevicesView::new(&_self, devices)));
        //_self.add_view(Box::new(MainView::from_pcap(&_self, path)));

        register_window_actions(&_self);
        register_stack_actions(&_self);


        let sniffer = Sniffer::new();
        sniffer.run();

        _self
    }

    pub fn add_view(&self, view: Box<dyn Stackable>) {
        let name = view.get_name();

        match self.stack.child_by_name(&name) {
            Some(child) => {
                self.title_bar.back.style_context().add_class("active");
                self.title_bar.next.style_context().remove_class("active");


                let pages = self.stack.pages();
                for i in (0..pages.n_items()).rev() {
                    let page = pages.item(i).expect("Failed to get page")
                        .downcast::<StackPage>()
                        .expect("Item is not a StackPage");

                    let name = page.name().unwrap().to_string();
                    self.views.borrow().get(&name).unwrap().on_destroy();
                    self.stack.remove(&page.child());
                    self.views.borrow_mut().remove(&name);

                    if child.eq(&page.child()) {
                        break;
                    }
                }
            }
            None => {
                if let Some(current) = self.stack.visible_child() {
                    let pages = self.stack.pages();
                    for i in (0..pages.n_items()).rev() {
                        let page = pages.item(i).expect("Failed to get page")
                            .downcast::<StackPage>()
                            .expect("Item is not a StackPage");

                        if current.eq(&page.child()) {
                            break;
                        }

                        let name = page.name().unwrap().to_string();
                        self.views.borrow().get(&name).unwrap().on_destroy();
                        self.stack.remove(&page.child());
                        self.views.borrow_mut().remove(&name);
                    }
                }
            }
        }

        /*
        let name = view.get_name();
        match self.stack.child_by_name(&name) {
            Some(child) => {
                let pos = self.stack.child_position(&child) as usize;

                self.title_bar.back.style_context().add_class("active");
                self.title_bar.next.style_context().remove_class("active");

                let children = self.stack.children();
                for i in (pos..children.len()).rev() {
                    let name = self.stack.child_name(&children[i]).unwrap().to_string();
                    self.views.borrow().get(&name).unwrap().on_destroy();
                    self.stack.remove(&children[i]);
                    self.views.borrow_mut().remove(&name);
                }
            }
            None => {
                let children = self.stack.children();
                if let Some(current) = self.stack.visible_child() {
                    if let Some(pos) = children.iter().position(|child| child == &current) {
                        self.title_bar.back.style_context().add_class("active");
                        self.title_bar.next.style_context().remove_class("active");

                        for i in (pos + 1..children.len()).rev() {
                            let name = self.stack.child_name(&children[i]).unwrap().to_string();
                            self.views.borrow().get(&name).unwrap().on_destroy();
                            self.stack.remove(&children[i]);
                            self.views.borrow_mut().remove(&name);
                        }
                    }
                }
            }
        }*/

        self.stack.add_named(view.get_root(), Some(&name));
        self.stack.set_visible_child_name(&name);
        view.on_create();
        self.views.borrow_mut().insert(name, view);
    }

    pub fn notify(&self, _type: NotificationTypes, title: &str, description: &str) {
        self.notifications.append(&NotificationView::new(_type, title, description).root);
    }
}
