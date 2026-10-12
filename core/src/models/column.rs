pub struct Column {
    id: u8,
    name: String,
    list_id: u16,
    column_place: u8,
}

impl Column {
    pub fn new(id: u8, name: String, list_id: u16, column_place: u8) -> Self {
        Self {
            id,
            name,
            list_id,
            column_place,
        }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_list_id(&self) -> u16 {
        self.list_id
    }

    pub fn get_place(&self) -> u8 {
        self.column_place
    }

    pub(crate) fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_owned();
    }

    pub(crate) fn change_place(&mut self, new_place: u8) {
        self.column_place = new_place;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rename_column() {
        let mut column = Column::new(0, String::from("a"), 0, 0);
        column.rename("b");
        assert_eq!(column.get_name(), "b");
    }
}
