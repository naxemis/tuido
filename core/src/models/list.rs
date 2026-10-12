use crate::errors::column_error::ColumnError;
use crate::managers::column_manager::ColumnManager;
use crate::models::column::Column;

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
    pub fn get_list_id(&self) -> u16 {
        self.id
    }

    /// Returns the list's name.
    pub fn get_list_name(&self) -> &str {
        self.name.as_str()
    }

    /// Replaces the list's name with `new_name`.
    pub(crate) fn rename_list(&mut self, new_name: &str) {
        self.name = new_name.to_owned();
    }

    /// Adds a column to the list and returns its generated ID.
    ///
    /// Leading and trailing whitespace is removed from `name`.
    ///
    /// Returns [`ColumnError::ColumnIdLimitReached`] when no more column IDs
    /// are available, [`ColumnError::ColumnPlaceLimitReached`] when no more
    /// column places are available, or [`ColumnError::EmptyName`] when the
    /// trimmed name is empty.
    pub fn add_column(&mut self, name: &str) -> Result<u8, ColumnError> {
        self.column_manager.add(name)
    }

    /// Removes the column with the given ID.
    ///
    /// Returns the removed column, or [`ColumnError::ColumnNotFound`] if no
    /// column has that ID.
    pub fn remove_column(&mut self, id: u8) -> Result<Column, ColumnError> {
        self.column_manager
            .remove(id)
            .ok_or(ColumnError::ColumnNotFound)
    }

    /// Finds a column by its ID.
    ///
    /// Returns `None` if no column has that ID.
    pub fn get_column_by_id(&self, id: u8) -> Option<&Column> {
        self.column_manager.get_by_id(id)
    }

    /// Finds all columns whose names exactly match `name`.
    pub fn get_column_by_name(&self, name: &str) -> Vec<&Column> {
        self.column_manager.get_by_name(name)
    }

    /// Renames the column with the given ID.
    ///
    /// Leading and trailing whitespace is removed from `new_name`.
    ///
    /// Returns [`ColumnError::EmptyName`] if the trimmed name is empty, or
    /// [`ColumnError::ColumnNotFound`] if no column has that ID.
    pub fn rename_column(&mut self, id: u8, new_name: &str) -> Result<(), ColumnError> {
        self.column_manager.rename(id, new_name)
    }

    /// Moves a column to a new position and returns its final position.
    ///
    /// Columns between the old and new positions are shifted to preserve
    /// consecutive positions.
    ///
    /// Returns [`ColumnError::ColumnNotFound`] if no column has the given ID,
    /// or [`ColumnError::InvalidColumnPlace`] if `new_place` is outside the
    /// current range of column positions.
    pub fn move_column(&mut self, id: u8, new_place: u8) -> Result<u8, ColumnError> {
        self.column_manager.move_to(id, new_place)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_list() {
        let mut list = List::new(1, String::from("A"));
        list.rename_list("B");

        assert_eq!(list.get_list_name(), "B");
    }

    #[test]
    fn check_for_column_manager() {
        let mut list = List::new(1, String::from("A"));

        assert!(!list.get_column_by_id(0).is_some());

        let column_id = list.add_column("B").unwrap();

        assert!(list.get_column_by_id(column_id).is_some());
    }
}
