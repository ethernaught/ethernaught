use std::cell::RefCell;
use gtk4::{glib, Align, Allocation, Buildable, Orientation, Widget};
use gtk4::prelude::{Cast, CellAreaExt, SnapshotExt, WidgetExt};
use gtk4::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassExt, WidgetImpl, WidgetImplExt};

const MIN_WIDTH: i32 = 20;
const MIN_HEIGHT: i32 = 20;

#[derive(Default)]
pub struct OverlayImpl {}

#[glib::object_subclass]
impl ObjectSubclass for OverlayImpl {

    const NAME: &'static str = "Overlay";
    type ParentType = Widget;
    type Type = Overlay;

    fn class_init(class: &mut Self::Class) {
        class.set_css_name("overlay");
    }
}

impl ObjectImpl for OverlayImpl {

    /*fn dispose(&self) {
        for child in &*self.children.borrow() {
            child.unparent();
        }
    }*/
}

impl WidgetImpl for OverlayImpl {

    /*
    fn snapshot(&self, snapshot: &Snapshot) {
        let widget = self.obj();
        let width = widget.width() as f32;
        let height = widget.height() as f32;

        // Draw a solid background with Cairo
        let cr = snapshot.append_cairo(&Rect::new(0.0, 0.0, width, height));
        cr.set_source_rgb(0.2, 0.4, 0.6);
        cr.rectangle(0.0, 0.0, width as f64, height as f64);
        cr.fill().unwrap();

        for child in self.children.borrow().iter() {
            WidgetExt::snapshot(child, snapshot);
        }
    }*/

    fn measure(&self, orientation: Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        (0, 0, -1, -1)
    }

    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        let widget = self.obj();
        self.parent_size_allocate(width, height, baseline);

        let mut child = widget.first_child();
        while let Some(w) = &child {
            let mut child_width = width;
            //if !w.hexpands() {
            //    child_width = w.preferred_width().0;
            //}

            let mut child_height = height;
            //if !w.vexpands() {
            //    child_height = w.preferred_height().0;
            //}

            let x = match w.halign() {
                Align::Start | Align::Fill => 0,
                Align::End => width - child_width,
                Align::Center => (width - child_width) / 2,
                _ => 0,
            };

            let y = match w.valign() {
                Align::Start | Align::Fill => 0,
                Align::End => height - child_height,
                Align::Center => (height - child_height) / 2,
                _ => 0,
            };

            w.size_allocate(&Allocation::new(x, y, child_width, child_height), -1);
            child = w.next_sibling();
        }
    }
}

glib::wrapper! {
    pub struct Overlay(ObjectSubclass<OverlayImpl>)
        @extends Widget, @implements Buildable;
}

impl Overlay {

    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    /*
    pub fn add_child(&self, widget: &Widget) {
        widget.set_parent(self);
        self.imp().children.borrow_mut().push(widget.clone());
        self.queue_resize();
    }

    pub fn remove_child(&self, widget: &Widget) {
        if let Some(pos) = self.imp().children.borrow().iter().position(|w| w == widget) {
            self.imp().children.borrow_mut().remove(pos);
            widget.unparent();
            self.queue_resize();
        }
    }

    pub fn for_each_child<F: FnMut(&Widget)>(&self, mut f: F) {
        for child in self.imp().children.borrow().iter() {
            f(child);
        }
    }*/
}