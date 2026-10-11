pub struct List {
    id: u16,
    name: String,
}
impl List {
    pub fn new(id: u16, name: String) -> Self {
        Self { id, name }
    }
}