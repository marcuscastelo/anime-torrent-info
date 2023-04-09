pub trait Handler {
    fn handle(&self, filename: &str) -> (String, Vec<String>);
}