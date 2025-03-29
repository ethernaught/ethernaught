use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use gtk::glib::{Cast, IsA};
use gtk::prelude::{BinExt, ContainerExt, GtkApplicationExt, GtkWindowExt, LabelExt, StackExt, StyleContextExt, WidgetExt};
use gtk::{glib, Application, Container, Stack, Widget, Window, WindowType};
use gtk::glib::ControlFlow::Continue;
use crate::oldui::activity::inter::activity::Activity;
use crate::oldui::handlers::bundle::Bundle;
use crate::oldui::handlers::event_handler::EventHandler;

#[derive(Clone)]
pub struct Context {
    app: Application,
    handler: EventHandler,
    stack: Rc<RefCell<Vec<Box<dyn Activity>>>>
}

impl Context {

    pub fn new(app: Application) -> Self {
        Self {
            app,
            handler: EventHandler::new(),
            stack: Rc::new(RefCell::new(Vec::new()))
        }
    }

    pub fn get_application(&self) -> Application {
        self.app.clone()
    }

    pub fn get_window(&self) -> Option<Window> {
        self.app.active_window()
    }

    pub fn get_titlebar(&self) -> Option<Widget> {
        self.app.active_window().unwrap().titlebar()
    }

    pub fn get_bottombar(&self) -> Option<Widget> {
        None
    }

    pub fn start_activity(&self, mut activity: Box<dyn Activity>, bundle: Option<Bundle>) {
        let stack = self.get_application().active_window().unwrap().child().unwrap().downcast_ref::<Container>().unwrap().children()[1].clone().downcast::<Stack>().unwrap();

        match stack.child_by_name(activity.get_name().as_ref()) {
            Some(child) => {
                let pos = stack.child_position(&child) as usize;
                self.stack.borrow().get(pos-1).unwrap().on_pause();

                let back_button = self.get_child_by_name::<Widget>(self.get_application().active_window().unwrap().titlebar().unwrap().upcast_ref(), "back_button").unwrap();
                back_button.style_context().add_class("active");

                let next_button = self.get_child_by_name::<Widget>(self.get_application().active_window().unwrap().titlebar().unwrap().upcast_ref(), "next_button").unwrap();
                next_button.style_context().remove_class("active");

                let children = stack.children();
                for i in (pos..children.len()).rev() {
                    self.stack.borrow().get(i).unwrap().on_pause();
                    self.stack.borrow().get(i).unwrap().on_destroy();
                    stack.remove(&children[i]);
                    self.stack.borrow_mut().remove(i);
                }
            }
            None => {
                let children = stack.children();
                if let Some(current) = stack.visible_child() {
                    if let Some(pos) = children.iter().position(|child| child == &current) {
                        self.stack.borrow().get(pos).unwrap().on_pause();

                        let back_button = self.get_child_by_name::<Widget>(self.get_application().active_window().unwrap().titlebar().unwrap().upcast_ref(), "back_button").unwrap();
                        back_button.style_context().add_class("active");

                        let next_button = self.get_child_by_name::<Widget>(self.get_application().active_window().unwrap().titlebar().unwrap().upcast_ref(), "next_button").unwrap();
                        next_button.style_context().remove_class("active");

                        for i in (pos + 1..children.len()).rev() {
                            self.stack.borrow().get(i).unwrap().on_pause();
                            self.stack.borrow().get(i).unwrap().on_destroy();
                            stack.remove(&children[i]);
                            self.stack.borrow_mut().remove(i);
                        }
                    }
                }
            }
        }

        let name = activity.get_name();
        let title = activity.get_title();
        let root = activity.on_create(bundle);
        stack.add_titled(root, &name, &title);

        let name = activity.get_name();
        self.stack.borrow_mut().push(activity);

        stack.set_visible_child_name(&name);
    }

    pub fn on_back_pressed(&self) {
        let stack = self.get_window().unwrap().child().unwrap().downcast_ref::<Container>().unwrap().children()[1].clone().downcast::<Stack>().unwrap();

        let children = stack.children();
        if let Some(current) = stack.visible_child() {
            if let Some(pos) = children.iter().position(|child| child == &current) {
                if pos > 0 {
                    self.stack.borrow().get(pos).unwrap().on_pause();
                    self.stack.borrow().get(pos - 1).unwrap().on_resume();
                    stack.set_visible_child(&children[pos - 1]);

                    let next_button = self.get_child_by_name::<Widget>(self.get_window().unwrap().titlebar().unwrap().upcast_ref(), "next_button").unwrap();
                    next_button.style_context().add_class("active");

                    let back_button = self.get_child_by_name::<Widget>(self.get_window().unwrap().titlebar().unwrap().upcast_ref(), "back_button").unwrap();
                    back_button.style_context().remove_class("active");
                }
            }
        }
    }

    pub fn on_next_pressed(&self) {
        let stack = self.get_window().unwrap().child().unwrap().downcast_ref::<Container>().unwrap().children()[1].clone().downcast::<Stack>().unwrap();

        let children = stack.children();
        if let Some(current) = stack.visible_child() {
            if let Some(pos) = children.iter().position(|child| child == &current) {
                if pos < children.len() - 1 {
                    self.stack.borrow().get(pos).unwrap().on_pause();
                    self.stack.borrow().get(pos + 1).unwrap().on_resume();
                    stack.set_visible_child(&children[pos + 1]);

                    let back_button = self.get_child_by_name::<Widget>(self.get_window().unwrap().titlebar().unwrap().upcast_ref(), "back_button").unwrap();
                    back_button.style_context().add_class("active");

                    let next_button = self.get_child_by_name::<Widget>(self.get_window().unwrap().titlebar().unwrap().upcast_ref(), "next_button").unwrap();
                    next_button.style_context().remove_class("active");
                }
            }
        }
    }

    pub fn get_child_by_name<T>(&self, widget: &Widget, name: &str) -> Option<T>
    where
        T: IsA<Widget> + 'static
    {
        if widget.widget_name().as_str() == name {
            return widget.downcast_ref::<T>().map(|w| w.clone());
        }

        if let Some(container) = widget.dynamic_cast_ref::<Container>() {
            for child in container.children() {
                if let Some(found) = self.get_child_by_name::<T>(&child, name) {
                    return Some(found);
                }
            }
        }

        None
    }

    pub fn create_window_from_activity(&self, mut activity: Box<dyn Activity>, bundle: Option<Bundle>) {
        let window = Window::new(WindowType::Toplevel);
        window.set_title(&activity.get_title());
        window.set_default_size(1200, 700);

        window.add(activity.on_create(bundle));
        window.show();
    }

    pub fn alert(&self, message: &str) {
        let alert = self.get_window().unwrap().child().unwrap().downcast_ref::<Container>().unwrap().children()[0].clone().downcast::<gtk::Box>().unwrap();

        let label = alert.children()[0].clone().downcast::<gtk::Label>().unwrap();
        label.set_label(message);

        alert.show();
    }

    pub fn alert_with_timeout(&self, message: &str, timeout: Duration) {
        let alert = self.get_window().unwrap().child().unwrap().downcast_ref::<Container>().unwrap().children()[0].clone().downcast::<gtk::Box>().unwrap();

        let label = alert.children()[0].clone().downcast::<gtk::Label>().unwrap();
        label.set_label(message);

        alert.show();

        glib::timeout_add_local(timeout, move || {
            alert.hide();
            Continue
        });
    }

    pub fn get_event_handler(&self) -> &EventHandler {
        &self.handler
    }
}
