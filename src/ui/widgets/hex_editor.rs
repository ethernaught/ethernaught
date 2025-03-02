use std::cell::RefCell;
use std::rc::Rc;
use gtk::{cairo, DrawingArea};
use gtk::gdk::EventMask;
use gtk::glib::{Propagation, PropertySet};
use gtk::prelude::{WidgetExt, WidgetExtManual};

const BYTES_PER_ROW: usize = 16;

#[derive(Clone)]
pub struct HexEditor {
    data: Rc<RefCell<Vec<u8>>>,
    cursor: Rc<RefCell<Option<usize>>>,
    selection: Rc<RefCell<Option<(usize, usize)>>>,
    line_number_color: Rc<RefCell<(f64, f64, f64)>>,
    text_color: Rc<RefCell<(f64, f64, f64)>>,
    cursor_color: Rc<RefCell<(f64, f64, f64)>>,
    selection_color: Rc<RefCell<(f64, f64, f64)>>,
    font_family: Rc<RefCell<String>>,
    font_size: Rc<RefCell<f64>>,
    padding: Rc<RefCell<f64>>,
    drawing_area: DrawingArea
}

impl Default for HexEditor {

    fn default() -> Self {
        let drawing_area = DrawingArea::new();
        drawing_area.set_widget_name("hex_editor");
        drawing_area.set_hexpand(true);
        drawing_area.set_vexpand(true);
        drawing_area.show();

        drawing_area.add_events(EventMask::POINTER_MOTION_MASK);

        let mut _self = Self {
            data: Rc::new(RefCell::new(Vec::new())),
            cursor: Rc::new(RefCell::new(None)),
            selection: Rc::new(RefCell::new(None)),
            line_number_color: Rc::new(RefCell::new((0.0, 0.0, 0.0))),
            text_color: Rc::new(RefCell::new((0.0, 0.0, 0.0))),
            cursor_color: Rc::new(RefCell::new((1.0, 0.0, 0.0))),
            selection_color: Rc::new(RefCell::new((1.0, 0.0, 1.0))),
            font_family: Rc::new(RefCell::new("Monospace".to_string())),
            font_size: Rc::new(RefCell::new(14.0)),
            padding: Rc::new(RefCell::new(20.0)),
            drawing_area
        };
        _self.update_content_size();

        _self
    }
}

impl HexEditor {

    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self {
            data: Rc::new(RefCell::new(data)),
            ..Default::default()
        }
    }

    pub fn update_cursor(&mut self, x: f64, y: f64) {
        let char_width = *self.font_size.borrow() * 0.6;
        let hex_spacing = 3.0;
        let row_height = *self.font_size.borrow() + 4.0;
        let ascii_offset = (BYTES_PER_ROW as f64) * (char_width * 2.0 + hex_spacing) + 10.0;
        let line_numbers_width = *self.padding.borrow() + 8.0 * char_width + 15.0;

        let mut col = ((x - line_numbers_width) / (char_width * 2.0 + hex_spacing)).floor() as isize;
        let row = ((y - (*self.padding.borrow() / 2.0)) / row_height).floor() as isize;

        if x-line_numbers_width >= ascii_offset {
            let ascii_col = ((x - line_numbers_width - ascii_offset) / char_width).floor() as isize;
            col = ascii_col;
        }

        if col >= BYTES_PER_ROW as isize || row < 0 {
            *self.cursor.borrow_mut() = None;
            return;
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
    }

    pub fn draw_hex_editor(&self, cr: &cairo::Context) {
        let char_width = *self.font_size.borrow() * 0.6;
        let hex_spacing = 3.0;
        let row_height = *self.font_size.borrow() + 4.0;
        let ascii_offset = (BYTES_PER_ROW as f64) * (char_width * 2.0 + hex_spacing) + 10.0;
        let line_numbers_width = *self.padding.borrow() + 8.0 * char_width + 15.0;

        cr.select_font_face(self.font_family.borrow().as_str(), cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        cr.set_font_size(*self.font_size.borrow());

        let mut prev_row = 0;

        for (i, &byte) in self.data.borrow().iter().enumerate() {
            let row = i / BYTES_PER_ROW;
            let col = i % BYTES_PER_ROW;

            let hex_x = col as f64 * (char_width * 2.0 + hex_spacing) + line_numbers_width;
            let y = *self.padding.borrow() + row as f64 * row_height;
            let ascii_x = ascii_offset + col as f64 * char_width + line_numbers_width;

            if col == 0 {
                cr.set_source_rgb(self.line_number_color.borrow().0, self.line_number_color.borrow().1, self.line_number_color.borrow().2);
                let line_number = format!("{:08X}", row * BYTES_PER_ROW);
                cr.move_to(*self.padding.borrow(), y);
                cr.show_text(&line_number);
                prev_row = row;
            }


            match *self.selection.borrow() {
                Some((x, x2)) => {
                    if i >= x && i <= x+x2-1  {
                        cr.set_source_rgb(self.selection_color.borrow().0, self.selection_color.borrow().1, self.selection_color.borrow().2);
                        cr.rectangle(hex_x - 2.0, y - *self.font_size.borrow(), char_width * 2.0 + 4.0, *self.font_size.borrow() + 4.0);
                        cr.fill().unwrap();

                        cr.rectangle(ascii_x - 2.0, y - *self.font_size.borrow(), char_width + 4.0, *self.font_size.borrow() + 4.0);
                        cr.fill().unwrap();
                    }
                }
                None => {}
            }

            if Some(i) == *self.cursor.borrow() {
                cr.set_source_rgb(self.cursor_color.borrow().0, self.cursor_color.borrow().1, self.cursor_color.borrow().2);
                cr.rectangle(hex_x - 2.0, y - *self.font_size.borrow(), char_width * 2.0 + 4.0, *self.font_size.borrow() + 4.0);
                cr.stroke().unwrap();

                cr.rectangle(ascii_x - 2.0, y - *self.font_size.borrow(), char_width + 4.0, *self.font_size.borrow() + 4.0);
                cr.stroke().unwrap();
            }

            cr.set_source_rgb(self.text_color.borrow().0, self.text_color.borrow().1, self.text_color.borrow().2);
            let hex = format!("{:02X}", byte);
            cr.move_to(hex_x, y);
            cr.show_text(&hex);

            let ascii_char = if byte.is_ascii_graphic() { byte as char } else { '.' };
            cr.move_to(ascii_x, y);
            cr.show_text(&ascii_char.to_string());
        }
    }

    pub fn update_content_size(&self) {
        let char_width = *self.font_size.borrow() * 0.6;
        let hex_spacing = 3.0;
        let row_height = *self.font_size.borrow() + 4.0;
        let num_rows = (self.data.borrow().len() as f64 / BYTES_PER_ROW as f64).ceil() as usize;
        let hex_width = (BYTES_PER_ROW as f64) * (char_width * 2.0 + hex_spacing);
        let ascii_width = (BYTES_PER_ROW as f64) * char_width + 30.0;
        let line_numbers_width = *self.padding.borrow() + 8.0 * char_width + 15.0;
        let content_width = line_numbers_width + hex_width + ascii_width;
        let content_height = (num_rows as f64) * row_height + *self.padding.borrow();

        self.drawing_area.set_size_request(content_width as i32, content_height as i32);
    }

    pub fn get_drawing_area(&self) -> &DrawingArea {
        &self.drawing_area
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        *self.data.borrow_mut() = data;
    }

    pub fn set_selection(&mut self, x: usize, x2: usize) {
        *self.selection.borrow_mut() = Some((x, x2));
        self.drawing_area.queue_draw();
    }

    pub fn set_line_number_color(&mut self, red: f64, green: f64, blue: f64) {
        *self.line_number_color.borrow_mut() = (red, green, blue);
    }

    pub fn set_text_color(&mut self, red: f64, green: f64, blue: f64) {
        *self.text_color.borrow_mut() = (red, green, blue);
    }

    pub fn set_selection_color(&mut self, red: f64, green: f64, blue: f64) {
        *self.selection_color.borrow_mut() = (red, green, blue);
    }

    pub fn set_cursor_color(&mut self, red: f64, green: f64, blue: f64) {
        *self.cursor_color.borrow_mut() = (red, green, blue);
    }

    pub fn set_font_family(&mut self, font_family: &str) {
        *self.font_family.borrow_mut() = font_family.to_string();
    }

    pub fn set_font_size(&mut self, font_size: f64) {
        *self.font_size.borrow_mut() = font_size;
    }

    pub fn set_padding(&mut self, padding: f64) {
        *self.padding.borrow_mut() = padding;
    }
}