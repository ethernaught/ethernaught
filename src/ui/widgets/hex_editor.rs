use gtk::cairo;

const BYTES_PER_ROW: usize = 16;

pub struct HexEditor {
    data: Vec<u8>,
    cursor: Option<usize>,
    selection: Option<(usize, usize)>,
    text_color: (f64, f64, f64),
    cursor_color: (f64, f64, f64),
    selection_color: (f64, f64, f64),
    font_family: String,
    font_size: f64,
    padding: f64
}

impl HexEditor {

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            cursor: None,
            selection: Some((0, 10)),
            text_color: (0.0, 0.0, 0.0),
            cursor_color: (1.0, 0.0, 0.0),
            selection_color: (1.0, 0.0, 1.0),
            font_family: "Monospace".to_string(),
            font_size: 14.0,
            padding: 20.0
        }
    }

    pub fn update_cursor(&mut self, x: f64, y: f64) {
        let char_width = self.font_size * 0.6;
        let hex_spacing = 3.0;
        let row_height = self.font_size + 4.0;

        let mut col = ((x - self.padding) / (char_width * 2.0 + hex_spacing)).floor() as isize;
        let row = ((y - (self.padding / 2.0)) / row_height).floor() as isize;

        let ascii_offset = (BYTES_PER_ROW as f64) * (char_width * 2.0 + hex_spacing) + 30.0;

        if x >= ascii_offset {
            let ascii_col = ((x - ascii_offset) / char_width).floor() as isize;
            col = ascii_col;
        }

        if col >= BYTES_PER_ROW as isize || row < 0 {
            self.cursor = None;
            return;
        }

        if row >= 0 && col >= 0 {
            let index = (row * BYTES_PER_ROW as isize + col) as usize;
            if index < self.data.len() {
                self.cursor = Some(index);
            } else {
                self.cursor = None;
            }
        } else {
            self.cursor = None;
        }
    }

    pub fn draw_hex_editor(&self, cr: &cairo::Context) {
        let char_width = self.font_size * 0.6;
        let hex_spacing = 3.0;
        let row_height = self.font_size + 4.0;
        let ascii_offset = (BYTES_PER_ROW as f64) * (char_width * 2.0 + hex_spacing) + 30.0;

        cr.select_font_face(self.font_family.as_str(), cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        cr.set_font_size(self.font_size);

        for (i, &byte) in self.data.iter().enumerate() {
            let row = i / BYTES_PER_ROW;
            let col = i % BYTES_PER_ROW;

            let hex_x = self.padding + col as f64 * (char_width * 2.0 + hex_spacing) + 100.0;//100 = TEMP
            let y = self.padding + row as f64 * row_height;
            let ascii_x = ascii_offset + col as f64 * char_width + 100.0;//100 = TEMP


            let line_number = format!("{:08X}", row * BYTES_PER_ROW);
            cr.set_source_rgb(self.text_color.0, self.text_color.1, self.text_color.2);
            cr.move_to(self.padding, y);
            cr.show_text(&line_number);



            if Some(i) == self.cursor {
                cr.set_source_rgb(self.cursor_color.0, self.cursor_color.1, self.cursor_color.2);
                cr.rectangle(hex_x - 2.0, y - self.font_size, char_width * 2.0 + 4.0, self.font_size + 4.0);
                cr.stroke().unwrap();

                cr.rectangle(ascii_x - 2.0, y - self.font_size, char_width + 4.0, self.font_size + 4.0);
                cr.stroke().unwrap();
            }

            cr.set_source_rgb(self.text_color.0, self.text_color.1, self.text_color.2);
            let hex = format!("{:02X}", byte);
            cr.move_to(hex_x, y);
            cr.show_text(&hex);

            let ascii_char = if byte.is_ascii_graphic() { byte as char } else { '.' };
            cr.move_to(ascii_x, y);
            cr.show_text(&ascii_char.to_string());
        }
    }

    pub fn content_size(&self) -> (i32, i32) {
        let char_width = self.font_size * 0.6;
        let hex_spacing = 3.0;
        let row_height = self.font_size + 4.0;
        let num_rows = (self.data.len() as f64 / BYTES_PER_ROW as f64).ceil() as usize;
        let hex_width = (BYTES_PER_ROW as f64) * (char_width * 2.0 + hex_spacing);
        let ascii_width = (BYTES_PER_ROW as f64) * char_width + 30.0;
        let content_width = hex_width + ascii_width + self.padding;
        let content_height = (num_rows as f64) * row_height + self.padding;

        (content_width as i32, content_height as i32)
    }

    pub fn set_text_color(&mut self, red: f64, green: f64, blue: f64) {
        self.text_color = (red, green, blue);
    }

    pub fn set_selection_color(&mut self, red: f64, green: f64, blue: f64) {
        self.selection_color = (red, green, blue);
    }

    pub fn set_cursor_color(&mut self, red: f64, green: f64, blue: f64) {
        self.cursor_color = (red, green, blue);
    }

    pub fn set_font_family(&mut self, font_family: &str) {
        self.font_family = font_family.to_string();
    }

    pub fn set_font_size(&mut self, font_size: f64) {
        self.font_size = font_size;
    }

    pub fn set_padding(&mut self, padding: f64) {
        self.padding = padding;
    }
}