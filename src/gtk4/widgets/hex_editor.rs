use std::cell::{Cell, RefCell};
use gtk4::gdk::RGBA;

use gtk4::{glib, Buildable, Orientation, Snapshot, Widget};
use gtk4::graphene::Rect;
use gtk4::cairo::{Content, RecordingSurface};
use gtk4::prelude::{ObjectExt, SnapshotExt, WidgetExt, StyleContextExt};
use gtk4::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassIsExt, WidgetClassExt, WidgetImpl};
use gtk4::cairo::{FontSlant, FontWeight};
use gtk4::pango::{FontDescription, SCALE, Weight};
use gtk4::StateFlags;

const BYTES_PER_ROW: usize = 16;

mod imp {
    use gtk4::cairo::{Context, TextExtents};
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
                selection_color: Cell::new(RGBA::new(0.4, 0.0, 0.4, 1.0)),
            }
        }
    }

    #[derive(Clone, Copy)]
    struct Metrics {
        font_size: f64,
        ascent: f64,
        descent: f64,
        row_height: f64,
        digit_width: f64,
        hex_cell_width: f64,
        ascii_cell_width: f64,
        line_number_width: f64,
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

        fn editor_font_desc(&self) -> FontDescription {
            let mut desc = self.obj()
                .pango_context()
                .font_description()
                .unwrap_or_else(|| FontDescription::from_string("Monospace 11"));

            desc.set_family("Monospace");
            desc
        }

        fn setup_cairo_font(&self, cr: &Context) -> Metrics {
            let font_desc = self.editor_font_desc();

            let font_weight = match font_desc.weight() {
                Weight::Bold => FontWeight::Bold,
                _ => FontWeight::Normal,
            };

            let font_size = (font_desc.size() as f64 / SCALE as f64).max(1.0);

            cr.select_font_face("Monospace", FontSlant::Normal, font_weight);
            cr.set_font_size(font_size);

            let font_extents = cr.font_extents().unwrap();

            // Measure real strings instead of using max_x_advance().
            let digit_width = text_width(cr, "0").ceil();
            let hex_cell_width = text_width(cr, "00").ceil() + 4.0;
            let ascii_cell_width = text_width(cr, ".").ceil() + 2.0;
            let line_number_width = text_width(cr, "00000000").ceil();

            let row_height = (font_extents.ascent() + font_extents.descent() + 4.0).ceil();

            Metrics {
                font_size,
                ascent: font_extents.ascent(),
                descent: font_extents.descent(),
                row_height,
                digit_width,
                hex_cell_width,
                ascii_cell_width,
                line_number_width,
            }
        }

        fn calculate_size(&self) -> (i32, i32) {
            let widget = self.obj();
            let style_context = widget.style_context();
            let padding = style_context.padding();

            let surface = RecordingSurface::create(Content::Color, None).unwrap();
            let cr = Context::new(&surface).unwrap();

            let m = self.setup_cairo_font(&cr);

            let gutter_gap = 12.0;
            let section_gap = 16.0;

            let width = padding.left() as f64
                + padding.right() as f64
                + m.line_number_width
                + gutter_gap
                + (BYTES_PER_ROW as f64 * m.hex_cell_width)
                + section_gap
                + (BYTES_PER_ROW as f64 * m.ascii_cell_width);

            let rows = (self.data.borrow().len() + BYTES_PER_ROW - 1) / BYTES_PER_ROW;
            let height = padding.top() as f64
                + padding.bottom() as f64
                + ((rows.max(1)) as f64 * m.row_height);

            (width.ceil() as i32, height.ceil() as i32)
        }
    }

    fn text_width(cr: &Context, text: &str) -> f64 {
        let ext: TextExtents = cr.text_extents(text).unwrap();
        ext.x_advance()
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

            let style_context = widget.style_context();
            style_context.set_state(StateFlags::NORMAL);
            let text_color = style_context.color();
            let padding = style_context.padding();

            if let Some(background) = style_context.lookup_color("background-color") {
                cr.set_source_rgba(
                    background.red() as f64,
                    background.green() as f64,
                    background.blue() as f64,
                    background.alpha() as f64,
                );
                let _ = cr.paint();
            }

            let m = self.setup_cairo_font(&cr);

            let baseline_y_offset = m.ascent + 2.0;
            let gutter_gap = 12.0;
            let section_gap = 16.0;

            let line_number_x = padding.left() as f64;
            let hex_offset = line_number_x + m.line_number_width + gutter_gap;
            let ascii_offset = hex_offset + (BYTES_PER_ROW as f64 * m.hex_cell_width) + section_gap;

            for (idx, &byte) in self.data.borrow().iter().enumerate() {
                let row = idx / BYTES_PER_ROW;
                let col = idx % BYTES_PER_ROW;

                let y = padding.top() as f64 + (row as f64 * m.row_height);
                let baseline_y = y + baseline_y_offset;

                let hex_x = hex_offset + (col as f64 * m.hex_cell_width);
                let ascii_x = ascii_offset + (col as f64 * m.ascii_cell_width);

                if col == 0 {
                    let color = match *self.cursor.borrow() {
                        Some(cursor) if cursor / BYTES_PER_ROW == row => text_color,
                        _ => self.line_number_color.get(),
                    };

                    cr.set_source_rgba(
                        color.red() as f64,
                        color.green() as f64,
                        color.blue() as f64,
                        color.alpha() as f64,
                    );

                    let line_number = format!("{:08X}", row * BYTES_PER_ROW);
                    cr.move_to(line_number_x, baseline_y);
                    let _ = cr.show_text(&line_number);
                }

                match *self.selection.borrow() {
                    Some((x, len)) if idx >= x && idx < x + len => {
                        let color = self.selection_color.get();
                        cr.set_source_rgba(
                            color.red() as f64,
                            color.green() as f64,
                            color.blue() as f64,
                            color.alpha() as f64,
                        );

                        cr.rectangle(hex_x - 1.0, y, m.hex_cell_width - 2.0, m.row_height);
                        let _ = cr.fill();

                        cr.rectangle(ascii_x - 1.0, y, m.ascii_cell_width, m.row_height);
                        let _ = cr.fill();
                    }
                    _ => {}
                }

                if Some(idx) == *self.cursor.borrow() {
                    let color = self.cursor_color.get();
                    cr.set_source_rgba(
                        color.red() as f64,
                        color.green() as f64,
                        color.blue() as f64,
                        color.alpha() as f64,
                    );

                    cr.rectangle(hex_x, y + 1.0, m.hex_cell_width - 2.0, m.row_height - 2.0);
                    let _ = cr.stroke();

                    cr.rectangle(ascii_x, y + 1.0, m.ascii_cell_width - 2.0, m.row_height - 2.0);
                    let _ = cr.stroke();
                }

                let color = match byte {
                    0 => match *self.selection.borrow() {
                        Some((x, len)) if idx >= x && idx < x + len => text_color,
                        _ => self.line_number_color.get(),
                    },
                    _ => text_color,
                };

                cr.set_source_rgba(
                    color.red() as f64,
                    color.green() as f64,
                    color.blue() as f64,
                    color.alpha() as f64,
                );

                let hex = format!("{:02X}", byte);
                cr.move_to(hex_x, baseline_y);
                let _ = cr.show_text(&hex);

                let ascii_char = if byte.is_ascii_graphic() { byte as char } else { '.' };
                cr.move_to(ascii_x, baseline_y);
                let _ = cr.show_text(&ascii_char.to_string());
            }
        }

        fn measure(&self, orientation: Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            let (width, height) = self.calculate_size();

            match orientation {
                Orientation::Horizontal => (width, width, -1, -1),
                Orientation::Vertical => (height, height, -1, -1),
                _ => unreachable!(),
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
        *self.imp().data.borrow_mut() = data;
        self.queue_resize();
        self.queue_draw();
    }

    pub fn set_selection(&self, a: usize, b: usize) {
        *self.imp().selection.borrow_mut() = Some((a, b));
        self.queue_draw();
    }

    pub fn get_selection(&self) -> Option<(usize, usize)> {
        *self.imp().selection.borrow()
    }
}

impl Default for HexEditor {

    fn default() -> Self {
        Self::new()
    }
}
