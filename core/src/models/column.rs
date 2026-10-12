use crate::errors::card_error::CardError;
use crate::errors::card_move_error::CardMoveError;
use crate::managers::card_manager::CardManager;
use crate::models::card::Card;

/// A column in a to-do list identified by an ID, name, and position.
pub struct Column {
    id: u8,
    name: String,
    column_place: u8,
    card_manager: CardManager,
}

impl Column {
    /// Creates a column with the given ID, name, and position.
    pub fn new(id: u8, name: String, column_place: u8) -> Self {
        Self {
            id,
            name,
            column_place,
            card_manager: CardManager::new(),
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

    pub fn add_card(&mut self, name: &str) -> Result<u16, CardError> {
        self.card_manager.add(name)
    }

    pub fn insert_card(&mut self, card: Card, target_place: u16) -> Result<u16, CardMoveError> {
        self.card_manager.insert(card, target_place)
    }

    pub fn remove_card(&mut self, id: u16) -> Option<Card> {
        self.card_manager.remove(id)
    }

    pub fn get_card_by_id(&self, id: u16) -> Option<&Card> {
        self.card_manager.get_by_id(id)
    }

    pub fn get_card_by_name(&self, name: &str) -> Vec<&Card> {
        self.card_manager.get_by_name(name)
    }

    pub fn rename_card(&mut self, id: u16, name: &str) -> Result<(), CardError> {
        self.card_manager.rename(id, name)
    }

    pub fn change_card_description(
        &mut self,
        id: u16,
        new_description: &str,
    ) -> Result<(), CardError> {
        self.card_manager.change_description(id, new_description)
    }

    pub fn move_card(&mut self, id: u16, new_place: u16) -> Result<u16, CardError> {
        self.card_manager.move_to(id, new_place)
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
