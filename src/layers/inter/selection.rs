pub trait Selection {

    fn get_selection(&self, variable: &str) -> (usize, usize);
}
