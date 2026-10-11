pub struct List {
    id: u16,
    name: String,
}

impl List {
    pub fn new(id: u16, name: String) -> Self {
        Self { id, name }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_owned()
    }
}