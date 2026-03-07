use std::cell::{Cell, RefCell};
use gtk4::gdk::RGBA;

use gtk4::{glib, gsk, Buildable, Orientation, Snapshot, Widget};
use gtk4::graphene::Rect;
use gtk4::cairo::{Content, RecordingSurface};
use gtk4::prelude::{ObjectExt, SnapshotExt, WidgetExt};
use gtk4::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassIsExt, WidgetClassExt, WidgetImpl};
use gtk4::cairo::{FontSlant, FontWeight};
use gtk4::pango::Weight;
use gtk4::prelude::StyleContextExt;
use gtk4::StateFlags;

const BYTES_PER_ROW: usize = 16;

mod imp {
    use gtk4::cairo::Context;
    use gtk4::subclass::prelude::{DerivedObjectProperties, ObjectSubclassExt};
    use super::*;

    #[derive(glib::Properties)]
    #[properties(wrapper_type = super::HexEditor)]
    pub struct HexEditorImpl {
        pub data: RefCell<Vec<u8>>,
        pub cursor: RefCell<Option<usize>>,
        pub selection: RefCell<Option<(usize, usize)>>,
        #[property(name = "line-number-color", get, set = Self::set_line_number_color, type = RGBA)]
        pub line_number_color: Cell<RGBA>,
        #[property(name = "cursor-color", get, set = Self::set_cursor_color, type = RGBA)]
        pub cursor_color: Cell<RGBA>,
        #[property(name = "selection-color", get, set = Self::set_selection_color, type = RGBA)]
        pub selection_color: Cell<RGBA>
    }

    impl Default for HexEditorImpl {

        fn default() -> Self {
            Self {
                data: RefCell::new(Vec::new()),
                cursor: RefCell::new(None),
                selection: RefCell::new(None),
                line_number_color: Cell::new(RGBA::new(0.7, 0.7, 0.7, 1.0)),
                cursor_color: Cell::new(RGBA::new(0.8, 0.8, 0.8, 1.0)),
                selection_color: Cell::new(RGBA::new(0.4, 0.0, 0.4, 1.0))
            }
        }
    }

    impl HexEditorImpl {

        pub fn set_line_number_color(&self, color: RGBA) {
            self.line_number_color.set(color);
            self.obj().queue_draw();
        }

        pub fn set_cursor_color(&self, color: RGBA) {
            self.cursor_color.set(color);
            self.obj().queue_draw();
        }

        pub fn set_selection_color(&self, color: RGBA) {
            self.selection_color.set(color);
            self.obj().queue_draw();
        }

        fn calculate_size(&self) -> (i32, i32) {
            let widget = self.obj();
            let style_context = widget.style_context();

            let padding = style_context.padding();

            let surface = RecordingSurface::create(Content::Color, None).unwrap();
            let cr = Context::new(&surface).unwrap();

            let font_desc = widget.pango_context().font_description().unwrap();

            let font_weight = match font_desc.weight() {
                Weight::Bold => FontWeight::Bold,
                _ => FontWeight::Normal
            };

            cr.select_font_face(font_desc.family().unwrap().split(',').next().unwrap().trim(), FontSlant::Normal, font_weight);
            cr.set_font_size(font_desc.size() as f64 / 1024.0);// * self.get_monitor_dpi() / 96.0);

            let extents = cr.font_extents().unwrap();
            let char_width = extents.max_x_advance() + 2.0;
            let row_height = extents.ascent() + extents.descent() + 2.0;

            let width = padding.left() as f64 + padding.right() as f64 + (extents.max_x_advance() * 2.0) + (BYTES_PER_ROW as f64 * (char_width * 3.0)) + (char_width * 8.0);
            let height = padding.top() as i32 + padding.bottom() as i32 + ((self.data.borrow().len() / BYTES_PER_ROW) as i32 + 1) * row_height as i32;

            (width as i32, height)
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HexEditorImpl {

        const NAME: &'static str = "HexEditor";
        type Type = super::HexEditor;
        type ParentType = Widget;

        fn class_init(class: &mut Self::Class) {
            class.set_css_name("hexeditor");
        }
    }

    impl ObjectImpl for HexEditorImpl {

        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }
    }

    impl WidgetImpl for HexEditorImpl {

        fn snapshot(&self, snapshot: &Snapshot) {
            let widget = self.obj();
            let width = widget.width() as f32;
            let height = widget.height() as f32;

            let cr = snapshot.append_cairo(&Rect::new(0.0, 0.0, width, height));

            let widget = self.obj();
            let style_context = widget.style_context();

            style_context.set_state(StateFlags::NORMAL);
            let text_color = style_context.color();

            let padding = style_context.padding();

            if let Some(background) = style_context.lookup_color("background-color") {
                cr.set_source_rgba(background.red() as f64, background.green() as f64, background.blue() as f64, background.alpha() as f64);
                cr.paint();
            }

            let font_desc = widget.pango_context().font_description().unwrap();

            let font_weight = match font_desc.weight() {
                Weight::Bold => FontWeight::Bold,
                _ => FontWeight::Normal
            };

            cr.select_font_face(font_desc.family().unwrap().split(',').next().unwrap().trim(), FontSlant::Normal, font_weight);
            cr.set_font_size(font_desc.size() as f64 / 1024.0);// * self.get_monitor_dpi() / 96.0);

            let extents = cr.font_extents().unwrap();
            let char_width = extents.max_x_advance() + 2.0;
            let row_padding = 2.0;
            let row_height = extents.ascent() + extents.descent() + row_padding;
            let hex_offset = padding.left() as f64 + extents.max_x_advance() + char_width * 8.0;
            let ascii_offset = hex_offset + extents.max_x_advance() + char_width * (BYTES_PER_ROW as f64 * 2.0);

            for (i, &byte) in self.data.borrow().iter().enumerate() {
                let row = i / BYTES_PER_ROW;
                let col = i % BYTES_PER_ROW;

                let hex_x = hex_offset + col as f64 * (char_width * 2.0);
                let y = padding.top() as f64 + (row as f64 * row_height);
                let ascii_x = ascii_offset + col as f64 * char_width;

                if col == 0 {
                    let color = match *self.cursor.borrow_mut() {
                        Some(cursor) => {
                            if cursor/BYTES_PER_ROW == row {
                                text_color

                            } else {
                                self.line_number_color.get()
                            }
                        }
                        None => self.line_number_color.get()
                    };

                    cr.set_source_rgba(color.red() as f64, color.green() as f64, color.blue() as f64, color.alpha() as f64);
                    let line_number = format!("{:08X}", row * BYTES_PER_ROW);

                    for (i, c) in line_number.chars().enumerate() {
                        cr.move_to(padding.left() as f64 + (i as f64 * char_width), y + extents.ascent() + row_padding);
                        cr.show_text(&c.to_string());
                    }
                }

                match *self.selection.borrow() {
                    Some((x, x2)) => {
                        if i >= x && i <= x+x2-1  {
                            let color = self.selection_color.get();
                            cr.set_source_rgba(color.red() as f64, color.green() as f64, color.blue() as f64, color.alpha() as f64);
                            cr.rectangle(hex_x - 1.0, y, char_width * 2.0, row_height);
                            cr.fill().unwrap();

                            cr.rectangle(ascii_x - 1.0, y, char_width - 2.0 + 2.0, row_height);
                            cr.fill().unwrap();
                        }
                    }
                    None => {}
                }

                if Some(i) == *self.cursor.borrow() {
                    let color = self.cursor_color.get();
                    cr.set_source_rgba(color.red() as f64, color.green() as f64, color.blue() as f64, color.alpha() as f64);
                    cr.rectangle(hex_x, y + 1.0, char_width * 2.0 - 2.0, row_height - 2.0);
                    cr.stroke().unwrap();

                    cr.rectangle(ascii_x, y + 1.0, char_width - 2.0, row_height - 2.0);
                    cr.stroke().unwrap();
                }

                let color = match byte {
                    0 => {
                        match *self.selection.borrow() {
                            Some((x, x2)) => {
                                if i >= x && i <= x+x2-1  {
                                    text_color
                                } else {
                                    self.line_number_color.get()
                                }
                            }
                            None => self.line_number_color.get()
                        }
                    },
                    _ => text_color
                };

                cr.set_source_rgba(color.red() as f64, color.green() as f64, color.blue() as f64, color.alpha() as f64);

                let hex = format!("{:02X}", byte);

                for (i, c) in hex.chars().enumerate() {
                    cr.move_to(hex_x + (i as f64 * char_width), y + extents.ascent() + row_padding);
                    cr.show_text(&c.to_string());
                }

                let ascii_char = if byte.is_ascii_graphic() { byte as char } else { '.' };
                cr.move_to(ascii_x, y + extents.ascent() + row_padding);
                cr.show_text(&ascii_char.to_string());
            }
        }

        fn measure(&self, orientation: Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            let (width, height) = self.calculate_size();
            match orientation {
                Orientation::Horizontal => (width, width, -1, -1),
                Orientation::Vertical => (height, height, -1, -1),
                _ => unimplemented!()
            }
        }
    }
}

glib::wrapper! {
    pub struct HexEditor(ObjectSubclass<imp::HexEditorImpl>)
        @extends Widget,
        @implements Buildable;
}

impl HexEditor {

    pub fn new() -> Self {
        glib::Object::builder::<HexEditor>().build()
    }

    pub fn set_data(&self, data: Vec<u8>) {
        *self.imp().data.borrow_mut() = data.to_vec();
    }

    pub fn set_selection(&self, a: usize, b: usize) {
        *self.imp().selection.borrow_mut() = Some((a, b));
        self.queue_draw();
    }

    pub fn get_selection(&self) -> Option<(usize, usize)> {
        self.imp().selection.borrow().clone()
    }
}

impl Default for HexEditor {

    fn default() -> Self {
        Self::new()
    }
}
