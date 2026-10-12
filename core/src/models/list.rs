use crate::managers::column_manager::ColumnManager;

/// A to-do list identified by a numeric ID and a name.
pub struct List {
    id: u16,
    name: String,
    column_manager: ColumnManager,
}

impl List {
    /// Creates a list with the given ID and name.
    pub fn new(id: u16, name: String) -> Self {
        Self {
            id,
            name,
            column_manager: ColumnManager::new(),
        }
    }

    /// Returns the list's ID.
    pub fn get_id(&self) -> u16 {
        self.id
    }

    /// Returns the list's name.
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_column_manager(&self) -> &ColumnManager {
        &self.column_manager
    }

    pub fn get_column_manager_mut(&mut self) -> &mut ColumnManager {
        &mut self.column_manager
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
        let mut list = List::new(1, String::from("A"));
        list.rename("B");
        
        assert_eq!(list.get_name(), "B");
    }
}
