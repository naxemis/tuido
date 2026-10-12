pub struct Card {
    id: u16,
    name: String,
    description: String,
    card_place: u16,
}

impl Card {
    pub fn new(id: u16, name: String, card_place: u16) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            card_place,
        }
    }
}