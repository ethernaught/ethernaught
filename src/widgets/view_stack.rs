use std::cell::RefCell;
use std::cmp::max;
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
    pub children: RefCell<Vec<Widget>>
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
        cr.set_source_rgba(0.2, 0.0, 0.0, 1.0);
        cr.paint();

        self.parent_draw(cr);

        for child in self.children.borrow().iter() {
            child.draw(cr);
        }

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
        widget.set_allocation(allocation);

        for child in self.children.borrow().iter() {
            child.size_allocate(allocation);
        }

        if widget.is_realized() {
            if let Some(window) = widget.window() {
                window.move_resize(
                    allocation.x(),
                    allocation.y(),
                    allocation.width(),
                    allocation.height(),
                );
            }
        }
    }
}

impl ContainerImpl for ViewStackImpl {

    fn add(&self, widget: &Widget) {
        let container = self.obj(); // Get the parent container
        let mut children = self.children.borrow_mut();

        // Add the child and set its parent
        widget.set_parent(container.upcast_ref::<gtk::Widget>());
        children.push(widget.clone());

        // Queue a resize since a child was added
        container.queue_resize();
    }

    fn remove(&self, widget: &Widget) {
        let container = self.obj();
        let mut children = self.children.borrow_mut();

        if let Some(pos) = children.iter().position(|w| w == widget) {
            children.remove(pos);
            widget.unparent();

            // Queue a resize after removing a child
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
        //let _self = glib::Object::builder::<ViewStack>().build();
        //_self
    }
}
