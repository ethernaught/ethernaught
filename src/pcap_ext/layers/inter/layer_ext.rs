pub trait LayerExt: Send {

    fn get_fields(&self) -> Vec<&str>;

    fn get_selection(&self, key: &str) -> Option<(usize, usize)>;

    fn get_field_name(&self, key: &str) -> Option<String>;

    fn get_title(&self, key: &str) -> Option<String>;

    fn get_value(&self, key: &str) -> Option<String>;

    fn get_description(&self, key: &str) -> Option<String> {
        match key {
            "frame" => self.get_title(key),
            _ => Some(format!("{}: {}", self.get_title(key)?, self.get_value(key)?))
        }
    }

    fn get_value_as_bytes(&self, key: &str) -> Option<Vec<u8>>;

    fn to_string(&self) -> String;

    fn clone_ext(&self) -> Box<dyn LayerExt>;
}
