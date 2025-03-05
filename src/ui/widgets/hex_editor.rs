use std::cell::RefCell;
use gtk::gdk::{EventMask, EventMotion, WindowAttr, WindowType, RGBA};
use gtk::{gdk, glib, pango, Buildable, Misc, StateFlags, Widget};
use gtk::cairo::{Context, FontSlant, FontWeight};
use gtk::glib::Propagation;
use gtk::glib::Propagation::Proceed;
use gtk::pango::Weight;
use gtk::prelude::{StyleContextExt, StyleContextExtManual, WidgetExt, WidgetExtManual};
use gtk::subclass::prelude::{ObjectImpl, ObjectSubclass, ObjectSubclassExt, ObjectSubclassIsExt, WidgetClassSubclassExt, WidgetImpl};

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
            data: RefCell::new(vec![]),
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

    fn draw(&self, cr: &Context) -> Propagation {
        let widget = self.obj();
        let style_context = widget.style_context();
        let pango_context = widget.create_pango_context();

        style_context.set_state(StateFlags::NORMAL);
        let text_color = style_context.color(StateFlags::NORMAL);

        let font_desc = style_context.font(StateFlags::NORMAL);
        let metrics = pango_context.metrics(Some(&font_desc), None);

        let ascent = metrics.ascent() as f64 / pango::SCALE as f64;
        let decent = metrics.descent() as f64 / pango::SCALE as f64;

        let font_size = if font_desc.is_size_absolute() {
            font_desc.size() as f64
        } else {
            font_desc.size() as f64 / pango::SCALE as f64
        };

        let font_weight = match font_desc.weight() {
            Weight::Bold => FontWeight::Bold,
            _ => {
                FontWeight::Normal
            }
        };

        let padding = style_context.padding(StateFlags::NORMAL);

        if let Ok(background) = style_context.style_property_for_state("background-color", StateFlags::NORMAL).get::<RGBA>() {
            cr.set_source_rgba(background.red(), background.green(), background.blue(), background.alpha());
            cr.paint();
        }

        let char_width = metrics.approximate_char_width() as f64 / pango::SCALE as f64;
        let row_height = ascent + decent;
        let ascii_offset = (BYTES_PER_ROW as f64) * (char_width * 2.0) + 10.0;
        let line_numbers_width = padding.left as f64 + 8.0 * char_width + 15.0;

        cr.select_font_face(font_desc.family().unwrap().as_str(), FontSlant::Normal, font_weight);
        cr.set_font_size(font_size);

        for (i, &byte) in self.data.borrow().iter().enumerate() {
            let row = i / BYTES_PER_ROW;
            let col = i % BYTES_PER_ROW;

            let hex_x = col as f64 * (char_width * 2.0) + line_numbers_width;
            let y = padding.top as f64 + (row as f64 * row_height);
            let ascii_x = ascii_offset + col as f64 * char_width + line_numbers_width;

            if col == 0 {
                let color = match *self.cursor.borrow() {
                    Some(cursor) => {
                        if cursor/BYTES_PER_ROW == row {
                            text_color

                        } else {
                            *self.line_number_color.borrow()
                        }
                    }
                    None => {
                        *self.line_number_color.borrow()
                    }
                };

                cr.set_source_rgba(color.red(), color.green(), color.blue(), color.alpha());
                let line_number = format!("{:08X}", row * BYTES_PER_ROW);
                cr.move_to(padding.left as f64, y + row_height - decent);
                cr.show_text(&line_number);
            }

            match *self.selection.borrow() {
                Some((x, x2)) => {
                    if i >= x && i <= x+x2-1  {
                        let color = self.selection_color.borrow();
                        cr.set_source_rgba(color.red(), color.green(), color.blue(), color.alpha());
                        cr.rectangle(hex_x - 2.0, y, char_width * 2.0, row_height);
                        cr.fill().unwrap();

                        cr.rectangle(ascii_x - 1.0, y, char_width - 2.0 + 2.0, row_height);
                        cr.fill().unwrap();
                    }
                }
                None => {}
            }

            if Some(i) == *self.cursor.borrow() {
                let color = self.cursor_color.borrow();
                cr.set_source_rgba(color.red(), color.green(), color.blue(), color.alpha());
                cr.rectangle(hex_x - 1.0, y + 1.0, char_width * 2.0 - 2.0, row_height - 2.0);
                cr.stroke().unwrap();

                cr.rectangle(ascii_x, y + 1.0, char_width - 2.0, row_height - 2.0);
                cr.stroke().unwrap();
            }

            cr.set_source_rgba(text_color.red(), text_color.green(), text_color.blue(), text_color.alpha());
            let hex = format!("{:02X}", byte);
            cr.move_to(hex_x, y + row_height - decent);
            cr.show_text(&hex);

            let ascii_char = if byte.is_ascii_graphic() { byte as char } else { '.' };
            cr.move_to(ascii_x, y + row_height - decent);
            cr.show_text(&ascii_char.to_string());
        }

        Proceed
    }

    fn realize(&self) {
        let widget = self.obj();
        let allocation = widget.allocation();

        let mut attr = WindowAttr::default();
        attr.window_type = WindowType::Child;
        attr.x = Some(allocation.x());
        attr.y = Some(allocation.y());
        attr.width = allocation.width();
        attr.height = allocation.height();

        let parent_window = widget.parent_window().unwrap();
        let window = gdk::Window::new(Some(&parent_window), &attr);

        widget.register_window(&window);
        widget.set_window(window);
        widget.set_realized(true);

        widget.add_events(EventMask::POINTER_MOTION_MASK);
    }

    fn motion_notify_event(&self, event: &EventMotion) -> Propagation {
        let widget = self.obj();
        let style_context = widget.style_context();
        let pango_context = widget.create_pango_context();

        style_context.set_state(StateFlags::NORMAL);

        let font_desc = style_context.font(StateFlags::NORMAL);
        let metrics = pango_context.metrics(Some(&font_desc), None);

        let ascent = metrics.ascent() as f64 / pango::SCALE as f64;
        let decent = metrics.descent() as f64 / pango::SCALE as f64;

        let padding = style_context.padding(StateFlags::NORMAL);

        let char_width = metrics.approximate_char_width() as f64 / pango::SCALE as f64;
        let row_height = ascent + decent;
        let ascii_offset = (BYTES_PER_ROW as f64) * (char_width * 2.0) + 10.0;
        let line_numbers_width = padding.left as f64 + 8.0 * char_width + 13.0;


        let (x, y) = event.position();

        let mut col = ((x - line_numbers_width) / (char_width * 2.0)).floor() as isize;
        let row = ((y - padding.top as f64) / row_height).floor() as isize;

        if x-line_numbers_width >= ascii_offset {
            let ascii_col = ((x - line_numbers_width - ascii_offset) / char_width).floor() as isize;
            col = ascii_col;
        }

        if col >= BYTES_PER_ROW as isize || row < 0 {
            *self.cursor.borrow_mut() = None;
            return Proceed;
        }

        if row >= 0 && col >= 0 {
            let index = (row * BYTES_PER_ROW as isize + col) as usize;
            if index < self.data.borrow().len() {
                *self.cursor.borrow_mut() = Some(index);
            } else {
                *self.cursor.borrow_mut() = None;
            }
        } else {
            *self.cursor.borrow_mut() = None;
        }

        self.obj().queue_draw();

        Proceed
    }
}

glib::wrapper! {
    pub struct HexEditor(ObjectSubclass<HexEditorImpl>)
         @extends Misc, Widget, @implements Buildable;
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
