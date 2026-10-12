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

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn card_place(&self) -> u16 {
        self.card_place
    }

    pub(crate) fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_owned();
    }

    pub(crate) fn change_description(&mut self, new_description: &str) {
        self.description = new_description.to_owned();
    }

    pub(crate) fn move_to(&mut self, new_place: u16) {
        self.card_place = new_place;
    }
}