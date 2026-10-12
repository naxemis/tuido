/// A to-do list identified by a numeric ID and a name.
pub struct List {
    id: u16,
    name: String,
}

impl List {
    /// Creates a list with the given ID and name.
    pub fn new(id: u16, name: String) -> Self {
        Self { id, name }
    }

    /// Returns the list's ID.
    pub fn get_id(&self) -> u16 {
        self.id
    }

    /// Returns the list's name.
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// Replaces the list's name with `new_name`.
    pub(crate) fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_list() {
        let mut list = List::new(1, String::from("a"));
        list.rename("b");
        assert_eq!(list.get_name(), "b");
    }
}


