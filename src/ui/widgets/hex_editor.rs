use gtk::cairo;

const BYTES_PER_ROW: usize = 16;

pub struct HexEditor {
    data: Vec<u8>,
    cursor: Option<usize>,
    selection: Option<(usize, usize)>,
    line_number_color: (f64, f64, f64),
    text_color: (f64, f64, f64),
    cursor_color: (f64, f64, f64),
    selection_color: (f64, f64, f64),
    font_family: String,
    font_size: f64,
    padding: f64
}

impl Default for HexEditor {

    fn default() -> Self {
        Self {
            data: Vec::new(),
            cursor: None,
            selection: None,
            line_number_color: (0.0, 0.0, 0.0),
            text_color: (0.0, 0.0, 0.0),
            cursor_color: (1.0, 0.0, 0.0),
            selection_color: (1.0, 0.0, 1.0),
            font_family: "Monospace".to_string(),
            font_size: 14.0,
            padding: 20.0
        }
    }
}

impl HexEditor {

    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }

    pub fn update_cursor(&mut self, x: f64, y: f64) {
        let char_width = self.font_size * 0.6;
        let hex_spacing = 3.0;
        let row_height = self.font_size + 4.0;
        let ascii_offset = (BYTES_PER_ROW as f64) * (char_width * 2.0 + hex_spacing) + 10.0;
        let line_numbers_width = self.padding + 8.0 * char_width + 15.0;

        let mut col = ((x - line_numbers_width) / (char_width * 2.0 + hex_spacing)).floor() as isize;
        let row = ((y - (self.padding / 2.0)) / row_height).floor() as isize;

        if x-line_numbers_width >= ascii_offset {
            let ascii_col = ((x - line_numbers_width - ascii_offset) / char_width).floor() as isize;
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
        let ascii_offset = (BYTES_PER_ROW as f64) * (char_width * 2.0 + hex_spacing) + 10.0;
        let line_numbers_width = self.padding + 8.0 * char_width + 15.0;

        cr.select_font_face(self.font_family.as_str(), cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        cr.set_font_size(self.font_size);

        let mut prev_row = 0;

        for (i, &byte) in self.data.iter().enumerate() {
            let row = i / BYTES_PER_ROW;
            let col = i % BYTES_PER_ROW;

            let hex_x = col as f64 * (char_width * 2.0 + hex_spacing) + line_numbers_width;
            let y = self.padding + row as f64 * row_height;
            let ascii_x = ascii_offset + col as f64 * char_width + line_numbers_width;

            if col == 0 {
                cr.set_source_rgb(self.line_number_color.0, self.line_number_color.1, self.line_number_color.2);
                let line_number = format!("{:08X}", row * BYTES_PER_ROW);
                cr.move_to(self.padding, y);
                cr.show_text(&line_number);
                prev_row = row;
            }


            match self.selection {
                None => {}
                Some((x, x2)) => {
                    if i >= x && i <= x2-1  {
                        cr.set_source_rgb(self.selection_color.0, self.selection_color.1, self.selection_color.2);
                        cr.rectangle(hex_x - 2.0, y - self.font_size, char_width * 2.0 + 4.0, self.font_size + 4.0);
                        cr.fill().unwrap();

                        cr.rectangle(ascii_x - 2.0, y - self.font_size, char_width + 4.0, self.font_size + 4.0);
                        cr.fill().unwrap();
                    }
                }
            }

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
        let line_numbers_width = self.padding + 8.0 * char_width + 15.0;
        let content_width = line_numbers_width + hex_width + ascii_width;
        let content_height = (num_rows as f64) * row_height + self.padding;

        (content_width as i32, content_height as i32)
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn set_selection(&mut self, x: usize, x2: usize) {
        self.selection = Some((x, x2));
    }

    pub fn set_line_number_color(&mut self, red: f64, green: f64, blue: f64) {
        self.line_number_color = (red, green, blue);
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