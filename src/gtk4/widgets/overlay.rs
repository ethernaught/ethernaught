use std::cell::RefCell;
use gtk4::{glib, Align, Allocation, Buildable, Orientation, Snapshot, Widget};
use gtk4::prelude::{SnapshotExt, WidgetExt};
use gtk4::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassExt, WidgetImpl, WidgetImplExt};

const MIN_WIDTH: i32 = 20;
const MIN_HEIGHT: i32 = 20;

#[derive(Default)]
pub struct OverlayImpl {
    children: RefCell<Vec<Widget>>
}

#[glib::object_subclass]
impl ObjectSubclass for OverlayImpl {

    const NAME: &'static str = "Overlay";
    type ParentType = Widget;
    type Type = Overlay;

    fn class_init(class: &mut Self::Class) {
        class.set_css_name("overlay");
    }
}

impl ObjectImpl for OverlayImpl {}

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
    }
*/
    fn measure(&self, orientation: Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        /*
        match orientation {
            Orientation::Horizontal => (0, 0, -1, -1),
            Orientation::Vertical => (0, 0, -1, -1),
            _ => unimplemented!()
        }
        */

        let children = self.children.borrow();

        let mut min = 0;
        let mut nat = 0;

        for child in children.iter() {
            let (child_min, child_nat, _, _) = child.measure(orientation, -1);
            min = min.max(child_min);
            nat = nat.max(child_nat);
        }

        (min, nat, -1, -1)
    }

    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        let widget = self.obj();
        self.parent_size_allocate(width, height, baseline);

        let allocation = Allocation::new(0, 0, width, height);
        for child in self.children.borrow().iter() {
            let mut child_width = width;
            /*if !child.hexpands() {
                child_width = child.preferred_width().0;
            }*/

            let mut child_height = height;
            /*if !child.vexpands() {
                child_height = child.preferred_height().0;
            }*/

            let x = match child.halign() {
                Align::Start | Align::Fill => 0,
                Align::End => width - child_width,
                Align::Center => (width - child_width) / 2,
                _ => 0,
            };

            let y = match child.valign() {
                Align::Start | Align::Fill => 0,
                Align::End => height - child_height,
                Align::Center => (height - child_height) / 2,
                _ => 0,
            };

            child.size_allocate(&Allocation::new(x, y, child_width, child_height), -1);
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
}