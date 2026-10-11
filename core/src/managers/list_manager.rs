use std::collections::HashMap;
use crate::models::list::List;

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
        let new_list = List::new(self.next_id, name.to_string());
        let new_list_id = self.next_id;
        self.lists.insert(new_list_id, new_list);
        self.next_id += 1;
        new_list_id
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
}