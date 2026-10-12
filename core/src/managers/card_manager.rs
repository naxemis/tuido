use std::collections::HashMap;

use crate::errors::card_error::CardError;
use crate::models::card::Card;

pub(crate) struct CardManager {
    cards: HashMap<u16, Card>,
    next_id: u32,
}

impl CardManager {
    pub(crate) fn new() -> Self {
        Self {
            cards: HashMap::new(),
            next_id: 0,
        }
    }

    pub(crate) fn add(&mut self, name: &str) -> Result<u16, CardError> {
        if self.next_id > u32::from(u16::MAX) {
            return Err(CardError::CardIdLimitReached);
        }

        if self.cards.len() > u16::MAX as usize {
            return Err(CardError::CardPlaceLimitReached);
        }

        let card_name = name.trim().to_string();
        if card_name.is_empty() {
            return Err(CardError::EmptyName);
        }

        let card_id = self.next_id as u16;
        let card = Card::new(card_id, card_name, self.cards.len() as u16);
        self.cards.insert(card_id, card);
        self.next_id += 1;
        Ok(card_id)
    }

    pub(crate) fn remove(&mut self, id: u16) -> Option<Card> {
        let removed_card = self.cards.remove(&id)?;

        let removed_card_place = removed_card.get_place();

        for card in self.cards.values_mut() {
            if card.get_place() > removed_card_place {
                card.move_to(card.get_place() - 1);
            }
        }

        Some(removed_card)
    }

    pub(crate) fn get_by_id(&self, id: u16) -> Option<&Card> {
        self.cards.get(&id)
    }

    pub(crate) fn get_by_name(&self, name: &str) -> Vec<&Card> {
        let mut matching_cards = Vec::new();

        for card in self.cards.values() {
            if card.get_name() == name {
                matching_cards.push(card);
            }
        }

        matching_cards
    }

    pub(crate) fn rename(&mut self, id: u16, new_name: &str) -> Result<(), CardError> {
        let name = new_name.trim();
        if name.is_empty() {
            return Err(CardError::EmptyName);
        }

        match self.cards.get_mut(&id) {
            Some(card) => {
                card.rename(name);
                Ok(())
            }
            None => Err(CardError::CardNotFound),
        }
    }

    pub(crate) fn change_description(
        &mut self,
        id: u16,
        new_description: &str,
    ) -> Result<(), CardError> {
        match self.cards.get_mut(&id) {
            Some(card) => {
                card.change_description(new_description);
                Ok(())
            }
            None => Err(CardError::CardNotFound),
        }
    }

    fn is_valid_place_for_move(&self, new_place: u16) -> bool {
        if usize::from(new_place) >= self.cards.len() {
            return false;
        }
        true
    }

    pub(crate) fn move_to(&mut self, id: u16, new_place: u16) -> Result<u16, CardError> {
        let old_place = match self.get_by_id(id) {
            Some(card) => card.get_place(),
            None => return Err(CardError::CardNotFound),
        };

        if old_place == new_place {
            return Ok(old_place);
        }

        if !self.is_valid_place_for_move(new_place) {
            return Err(CardError::InvalidCardPlace);
        }

        let move_left = new_place < old_place;
        let _move_right = new_place > old_place;

        for card in self.cards.values_mut() {
            if move_left {
                if card.get_place() >= new_place && card.get_place() < old_place {
                    card.move_to(card.get_place() + 1);
                }
            } else {
                if card.get_place() <= new_place && card.get_place() > old_place {
                    card.move_to(card.get_place() - 1);
                }
            }
        }

        let card = self.cards.get_mut(&id).unwrap();
        card.move_to(new_place);

        Ok(new_place)
    }
}

impl Default for CardManager {
    fn default() -> Self {
        Self::new()
    }
}
