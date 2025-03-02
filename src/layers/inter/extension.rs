pub trait LayerExt {

    fn get_selection(&self, variable: &str) -> (usize, usize);

    fn to_string(&self) -> String;
}
