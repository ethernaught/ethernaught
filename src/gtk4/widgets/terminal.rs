use std::cell::RefCell;
use gtk4::{glib, Buildable, Orientation, Snapshot, StateFlags, Widget};
use gtk4::gdk::RGBA;
use gtk4::glib::property::PropertyGet;
use gtk4::graphene::Rect;
use gtk4::prelude::{DisplayExt, ObjectExt, SnapshotExt, StyleContextExt, WidgetExt};
use gtk4::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassExt, WidgetImpl, WidgetImplExt};

pub struct TerminalImpl {
    cursor: RefCell<Option<usize>>,
    selection: RefCell<Option<(usize, usize)>>,
    cursor_color: RefCell<RGBA>,
    selection_color: RefCell<RGBA>
}

impl Default for TerminalImpl {

    fn default() -> Self {
        Self {
            cursor: RefCell::new(None),
            selection: RefCell::new(None),
            cursor_color: RefCell::new(RGBA::new(0.8, 0.8, 0.8, 1.0)),
            selection_color: RefCell::new(RGBA::new(0.4, 0.0, 0.4, 1.0))
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for TerminalImpl {

    const NAME: &'static str = "Terminal";
    type ParentType = Widget;
    type Type = Terminal;

    fn class_init(class: &mut Self::Class) {
        class.set_css_name("terminal");
    }
}

impl ObjectImpl for TerminalImpl {}

impl WidgetImpl for TerminalImpl {

    fn snapshot(&self, snapshot: &Snapshot) {
        let widget = self.obj();
        let width = widget.width() as f32;
        let height = widget.height() as f32;

        let cr = snapshot.append_cairo(&Rect::new(0.0, 0.0, width, height));
    }

    fn measure(&self, orientation: Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        match orientation {
            Orientation::Horizontal => (0, 0, -1, -1),
            Orientation::Vertical => (0, 0, -1, -1),
            _ => unimplemented!()
        }
    }
}

glib::wrapper! {
    pub struct Terminal(ObjectSubclass<TerminalImpl>)
        @extends Widget, @implements Buildable;
}

impl Terminal {

    pub fn new() -> Self {
        let _self = glib::Object::builder::<Terminal>().build();
        _self
    }

    pub fn set_cursor_color(&self, color: RGBA) {
        *self.imp().cursor_color.borrow_mut() = color;
    }

    pub fn get_cursor_color(&self) -> RGBA {
        self.imp().cursor_color.borrow().clone()
    }

    pub fn set_selection_color(&self, color: RGBA) {
        *self.imp().selection_color.borrow_mut() = color;
    }

    pub fn get_selection_color(&self) -> RGBA {
        self.imp().selection_color.borrow().clone()
    }

    pub fn set_selection(&self, a: usize, b: usize) {
        *self.imp().selection.borrow_mut() = Some((a, b));
        self.queue_draw();
    }

    pub fn get_selection(&self) -> Option<(usize, usize)> {
        self.imp().selection.borrow().clone()
    }
}
