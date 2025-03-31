pub trait LayerExt: Send {

    fn get_fields(&self) -> Vec<&str>;

    fn get_selection(&self, key: &str) -> (usize, usize);

    fn get_field_name(&self, key: &str) -> String;

    fn get_title(&self, key: &str) -> String;

    fn get_value(&self, key: &str) -> String;

    fn get_description(&self, key: &str) -> String {
        match key {
            "frame" => self.get_title(key),
            _ => format!("{}: {}", self.get_title(key), self.get_value(key)),
        }
    }

    fn get_value_as_bytes(&self, key: &str) -> Vec<u8>;

    fn to_string(&self) -> String;

    fn clone_ext(&self) -> Box<dyn LayerExt>;
}
