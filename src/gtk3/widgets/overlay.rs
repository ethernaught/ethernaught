use std::cell::RefCell;
use gtk::gdk::{EventMask, WindowAttr, WindowType, WindowWindowClass, RGBA};
use gtk::{gdk, glib, Align, Allocation, Buildable, Container, Misc, StateFlags, Widget};
use gtk::cairo::Context;
use gtk::glib::{Cast, ObjectExt, ParamFlags, ParamSpec, ParamSpecBuilderExt, ParamSpecString, Propagation, StaticType, ToValue, Value};
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{SettingsExt, StyleContextExt, WidgetExt};
use gtk::subclass::container::{Callback, ContainerImplExt};
use gtk::subclass::prelude::{ContainerImpl, ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassSubclassExt, WidgetImpl, WidgetImplExt};

#[derive(Default)]
pub struct OverlayImpl {
    children: RefCell<Vec<Widget>>
}

#[glib::object_subclass]
impl ObjectSubclass for OverlayImpl {

    const NAME: &'static str = "Overlay";
    type ParentType = Container;
    type Type = Overlay;

    fn class_init(class: &mut Self::Class) {
        class.set_css_name("overlay");
    }
}

impl ObjectImpl for OverlayImpl {}

impl WidgetImpl for OverlayImpl {

    fn draw(&self, cr: &Context) -> Propagation {
        self.parent_draw(cr);
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

        for child in self.children.borrow().iter() {
            let mut width = allocation.width();
            if !child.is_hexpand_set() {
                width = child.width_request();

                if width < 0 {
                    width = 0;
                }
            }

            let mut height = allocation.height();
            if !child.is_vexpand_set() {
                height = child.height_request();

                if height < 0 {
                    height = 0;
                }
            }

            let x = match child.halign() {
                Align::Fill | Align::Start => 0,
                Align::End => allocation.width() - width,
                Align::Center => (allocation.width() - width) / 2,
                _ => 0
            };

            let y = match child.valign() {
                Align::Fill | Align::Start => 0,
                Align::End => allocation.height() - height,
                Align::Center => (allocation.height() - height) / 2,
                _ => 0
            };

            child.size_allocate(&Allocation::new(
                x,
                y,
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

impl ContainerImpl for OverlayImpl {

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
    pub struct Overlay(ObjectSubclass<OverlayImpl>)
         @extends Container, Widget, @implements Buildable;
}

impl Overlay {

    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
