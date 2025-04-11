use std::cell::RefCell;
use gtk4::{glib, Buildable, Orientation, Snapshot, StateFlags, Widget};
use gtk4::gdk::RGBA;
use gtk4::glib::property::PropertyGet;
use gtk4::graphene::Rect;
use gtk4::prelude::{DisplayExt, ObjectExt, SnapshotExt, StyleContextExt, WidgetExt};
use gtk4::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassExt, WidgetImpl, WidgetImplExt};

const BYTES_PER_ROW: usize = 16;

pub struct HexEditorImpl {
    data: RefCell<Vec<u8>>,
    cursor: RefCell<Option<usize>>,
    selection: RefCell<Option<(usize, usize)>>,
    line_number_color: RefCell<RGBA>,
    cursor_color: RefCell<RGBA>,
    selection_color: RefCell<RGBA>
}

impl Default for HexEditorImpl {

    fn default() -> Self {
        Self {
            data: RefCell::new(Vec::new()),
            cursor: RefCell::new(None),
            selection: RefCell::new(None),
            line_number_color: RefCell::new(RGBA::new(0.7, 0.7, 0.7, 1.0)),
            cursor_color: RefCell::new(RGBA::new(0.8, 0.8, 0.8, 1.0)),
            selection_color: RefCell::new(RGBA::new(0.4, 0.0, 0.4, 1.0))
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for HexEditorImpl {

    const NAME: &'static str = "HexEditor";
    type ParentType = Widget;
    type Type = HexEditor;

    fn class_init(class: &mut Self::Class) {
        class.set_css_name("hexeditor");
    }
}

impl ObjectImpl for HexEditorImpl {}

impl WidgetImpl for HexEditorImpl {

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
    pub struct HexEditor(ObjectSubclass<HexEditorImpl>)
        @extends Widget, @implements Buildable;
}

impl HexEditor {

    pub fn new(data: Vec<u8>) -> Self {
        let _self = glib::Object::builder::<HexEditor>().build();
        *_self.imp().data.borrow_mut() = data.to_vec();
        _self
    }

    pub fn set_data(&self, data: Vec<u8>) {
        *self.imp().data.borrow_mut() = data.to_vec();
    }

    pub fn set_line_number_color(&self, color: RGBA) {
        *self.imp().line_number_color.borrow_mut() = color;
    }

    pub fn get_line_number_color(&self) -> RGBA {
        self.imp().line_number_color.borrow().clone()
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
