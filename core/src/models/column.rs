/// A column in a to-do list identified by an ID, name, and position.
pub struct Column {
    id: u8,
    name: String,
    column_place: u8,
}

impl Column {
    /// Creates a column with the given ID, name, and position.
    pub fn new(id: u8, name: String, column_place: u8) -> Self {
        Self {
            id,
            name,
            column_place,
        }
    }

    /// Returns the column's ID.
    pub fn get_id(&self) -> u8 {
        self.id
    }

    /// Returns the column's name.
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the column's position in the list.
    pub fn get_place(&self) -> u8 {
        self.column_place
    }

    /// Replaces the column's name with `new_name`.
    pub(crate) fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_owned();
    }

    /// Changes the column's position to `new_place`.
    pub(crate) fn move_to(&mut self, new_place: u8) {
        self.column_place = new_place;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rename_column() {
        let mut column = Column::new(0, String::from("A"), 0);
        column.rename("B");

        assert_eq!(column.get_name(), "B");
    }
}
