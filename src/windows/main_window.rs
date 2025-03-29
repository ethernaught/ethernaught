use std::cell::RefCell;
use std::collections::HashMap;
use std::process::exit;
use std::rc::Rc;
use gtk::{gdk, Application, ApplicationWindow, Builder, CssProvider, Stack, StyleContext, Window};
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{ActionGroupExt, BuilderExtManual, ContainerExt, CssProviderExt, GtkWindowExt, StackExt, StyleContextExt, WidgetExt};
use pcap::devices::Device;
use pcap::utils::interface_flags::InterfaceFlags;
use crate::actions::window_actions::{register_stack_actions, register_window_actions};
use crate::views::bottom_bar::BottomBar;
use crate::views::devices_view::DevicesView;
use crate::views::inter::stackable::Stackable;
use crate::views::title_bar::TitleBar;

#[derive(Clone)]
pub struct MainWindow {
    pub window: ApplicationWindow,
    pub title_bar: TitleBar,
    pub stack: Stack,
    pub bottom_bar: BottomBar,
    pub views: Rc<RefCell<HashMap<String, Box<dyn Stackable>>>>
}

impl MainWindow {

    pub fn new(app: &Application) -> Self {
        let builder = Builder::from_resource("/net/ethernaught/rust/res/ui/gtk3/window.ui");

        let provider = CssProvider::new();
        provider.load_from_resource("/net/ethernaught/rust/res/ui/gtk3/window.css");
        //provider.load_from_path("res/ui/gtk3/window.css").expect("Failed to load CSS file.");

        StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Failed to get default screen."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window: ApplicationWindow = builder
            .object("main_window")
            .expect("Failed to get the 'main_window' from window.ui");

        window.set_application(Some(app));
        window.connect_destroy(|_| exit(0));
        //window.set_decorated(false);
        window.set_border_width(1);

        #[cfg(profile = "nightly")]
        window.style_context().add_class("nightly");

        #[cfg(profile = "release")]
        window.style_context().add_class("release");

        //window.set_icon_from_file("res/icons/ic_launcher.svg").expect("Failed to load icon");

        let title_bar = TitleBar::new();
        window.set_titlebar(Some(&title_bar.root));

        let window_content: gtk::Box = builder
            .object("window_content")
            .expect("Failed to get the 'window_content' from window.ui");

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

        stack.show();

        let bottom_bar = BottomBar::new();
        window_content.add(&bottom_bar.root);


        let mut devices = Device::list().expect("Failed to get device list");
        devices.sort_by(|a, b| {
            b.flags.contains(&InterfaceFlags::Running).cmp(&a.flags.contains(&InterfaceFlags::Running))
        });

        let view = DevicesView::new(&window, devices);




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

        window.show();

        let _self = Self {
            window,
            title_bar,
            stack,
            bottom_bar,
            views
        };

        _self.add_view(Box::new(view));


        register_window_actions(&_self);
        register_stack_actions(&_self);

        _self
    }

    pub fn add_view(&self, view: Box<dyn Stackable>) {
        let name = view.get_name();
        match self.stack.child_by_name(&name) {
            Some(child) => {
                let pos = self.stack.child_position(&child) as usize;

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
                        for i in (pos + 1..children.len()).rev() {
                            let name = self.stack.child_name(&children[i]).unwrap().to_string();
                            self.views.borrow().get(&name).unwrap().on_destroy();
                            self.stack.remove(&children[i]);
                            self.views.borrow_mut().remove(&name);
                        }
                    }
                }
            }
        }

        self.stack.add_named(view.get_root(), &name);
        self.stack.set_visible_child_name(&name);
        view.on_create();
        self.views.borrow_mut().insert(name, view);
    }
}
