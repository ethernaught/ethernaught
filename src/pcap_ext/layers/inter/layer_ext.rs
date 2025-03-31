pub trait LayerExt: Send {

    fn get_variables(&self) -> Vec<&str>;

    fn get_selection(&self, variable: &str) -> (usize, usize);

    fn get_field_name(&self, variable: &str) -> String;

    fn get_title(&self, variable: &str) -> String;

    fn get_value(&self, variable: &str) -> String;

    fn get_description(&self, variable: &str) -> String;

    fn get_value_as_bytes(&self, variable: &str) -> Vec<u8>;

    fn to_string(&self) -> String;

    fn clone_ext(&self) -> Box<dyn LayerExt>;
}
