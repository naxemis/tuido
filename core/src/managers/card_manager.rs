use std::collections::HashMap;

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
}

impl Default for CardManager {
    fn default() -> Self {
        Self::new()
    }
}