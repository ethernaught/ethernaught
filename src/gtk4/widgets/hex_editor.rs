use std::cell::RefCell;
use gtk4::{glib, Buildable, Orientation, Snapshot, StateFlags, Widget};
use gtk4::glib::property::PropertyGet;
use gtk4::graphene::Rect;
use gtk4::prelude::{DisplayExt, ObjectExt, SnapshotExt, StyleContextExt, WidgetExt};
use gtk4::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassExt, WidgetImpl, WidgetImplExt};

#[derive(Default)]
pub struct HexEditorImpl {
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

    pub fn new() -> Self {
        let _self = glib::Object::builder::<HexEditor>().build();
        _self
    }
}
