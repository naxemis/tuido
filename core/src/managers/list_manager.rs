use std::collections::HashMap;

use crate::models::list::List;
use crate::errors::list_error::ListError;

pub struct ListManager {
    lists: HashMap<u16, List>,
    next_id: u16,
}

impl ListManager {
    pub fn new() -> Self {
        Self {
            lists: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add(&mut self, name: &str) -> u16 {
        let list_name = name.trim().to_string();
        let list_id = self.next_id;
        let list = List::new(list_id, list_name);
        self.lists.insert(list_id, list);
        self.next_id += 1;
        list_id
    }

    pub fn remove(&mut self, id: u16) -> Option<List> {
        self.lists.remove(&id)
    }

    pub fn get_by_id(&self, id: u16) -> Option<&List> {
        self.lists.get(&id)
    }

    pub fn get_by_name(&self, name: &str) -> Vec<&List> {
        let mut matching_lists = Vec::new();

        for list in self.lists.values() {
            if list.get_name() == name {
                matching_lists.push(list);
            }
        }

        matching_lists
    }

    pub fn rename(&mut self, id: u16, new_name: &str) -> Result<(), ListError> {
        let name = new_name.trim();

        if name.is_empty() {
            return Err(ListError::EmptyName)
        }

        match self.lists.get_mut(&id) {
            Some(list) => {
                list.rename(name);
                Ok(())
            }
            None => Err(ListError::ListNotFound),
        }
    }
}