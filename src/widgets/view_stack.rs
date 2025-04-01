use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use gtk::gdk::{EventMask, WindowAttr, WindowType, WindowWindowClass, RGBA};
use gtk::{gdk, glib, Allocation, Buildable, Container, Misc, StateFlags, Widget};
use gtk::cairo::Context;
use gtk::glib::{Cast, Propagation};
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{StyleContextExt, WidgetExt};
use gtk::subclass::container::{Callback, ContainerImplExt};
use gtk::subclass::prelude::{ContainerImpl, ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassSubclassExt, WidgetImpl, WidgetImplExt};

#[derive(Default)]
pub struct ViewStackImpl {
    names: RefCell<Vec<String>>,
    children: RefCell<Vec<Widget>>,
    //event_listeners: Rc<RefCell<Vec<String, Box<dyn Fn(Box<dyn Event>)>>>>
}

#[glib::object_subclass]
impl ObjectSubclass for ViewStackImpl {

    const NAME: &'static str = "ViewStack";
    type ParentType = Container;
    type Type = ViewStack;

    fn class_init(class: &mut Self::Class) {
        class.set_css_name("viewstack");
    }
}

impl ObjectImpl for ViewStackImpl {}

impl WidgetImpl for ViewStackImpl {

    fn draw(&self, cr: &Context) -> Propagation {
        //cr.set_source_rgba(0.2, 0.0, 0.0, 1.0);
        //cr.paint();

        self.parent_draw(cr);

        /*
        for child in self.children.borrow().iter() {
            child.draw(cr);
        }*/

        Proceed
    }

    fn realize(&self) {
        let widget = self.obj();
        let allocation = widget.allocation();

        let attr = WindowAttr {
            title: None,
            event_mask: EventMask::empty(),
            x: Some(allocation.x()),
            y: Some(allocation.y()),
            width: allocation.width(),
            height: allocation.height(),
            wclass: WindowWindowClass::InputOutput,
            visual: None,
            window_type: WindowType::Child,
            cursor: None,
            override_redirect: false,
            type_hint: None
        };

        let parent_window = widget.parent_window().unwrap();
        let window = gdk::Window::new(Some(&parent_window), &attr);

        widget.register_window(&window);
        widget.set_window(window);
        widget.set_realized(true);

        widget.set_can_focus(true);
    }

    fn size_allocate(&self, allocation: &Allocation) {
        let widget = self.obj();

        //let allocation2 = Allocation::new(0, 0, allocation.width(), allocation.height());
        for child in self.children.borrow().iter() {
            let mut width = allocation.width();
            if !child.is_hexpand_set() {
                width = child.width_request();
            }

            let mut height = allocation.height();
            if !child.is_vexpand_set() {
                height = child.height_request();
            }

            //let mut width = child.width_request();
            //let mut height = child.height_request();

            child.size_allocate(&Allocation::new(
                0,
                0,
                width,
                height
            ));
        }

        widget.set_allocation(allocation);
        if widget.is_realized() {
            if let Some(window) = widget.window() {
                window.move_resize(allocation.x(), allocation.y(), allocation.width(), allocation.height());
            }
        }
    }
}

impl ContainerImpl for ViewStackImpl {

    fn add(&self, widget: &Widget) {
        let container = self.obj();
        let mut children = self.children.borrow_mut();

        widget.set_parent(container.upcast_ref::<Widget>());
        children.push(widget.clone());

        container.queue_resize();
    }

    fn remove(&self, widget: &Widget) {
        let container = self.obj();
        let mut children = self.children.borrow_mut();

        if let Some(pos) = children.iter().position(|w| w == widget) {
            children.remove(pos);
            widget.unparent();

            container.queue_resize();
        }
    }

    fn forall(&self, include_internals: bool, callback: &Callback) {
        let children = self.children.borrow();
        for child in children.iter() {
            callback.call(child);
        }
    }
}

glib::wrapper! {
    pub struct ViewStack(ObjectSubclass<ViewStackImpl>)
         @extends Container, Widget, @implements Buildable;
}

impl ViewStack {

    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn select(&self, name: &str) {
        //self.imp().names.borrow_mut().push(name.into());
    }

    pub fn get_by_name(&self, name: &str) -> Option<Widget> {
        todo!()
    }

    pub fn get_selected_widget(&self) -> Option<Widget> {
        todo!()
    }

    pub fn get_selected_index(&self) -> usize {
        self.imp().children.borrow().len()
    }

    pub fn connect_stack_change(&self) {

    }

    pub fn connect_(&self) {

    }
}
