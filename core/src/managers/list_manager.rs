use std::collections::HashMap;

use crate::errors::list_error::ListError;
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

    pub fn add(&mut self, name: &str) -> Result<u16, ListError> {
        if self.next_id > u16::MAX {
            return Err(ListError::IdExhausted);
        }

        let list_name = name.trim().to_string();
        let list_id = self.next_id;
        let list = List::new(list_id, list_name);
        self.lists.insert(list_id, list);
        self.next_id += 1;
        Ok(list_id)
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
            return Err(ListError::EmptyName);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_list() {
        let mut manager = ListManager::new();
        let result = manager.add("Shopping");
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn get_by_id() {
        let mut manager = ListManager::new();
        let result = manager.add("Shopping");
        assert!(manager.get_by_id(result.unwrap()).is_some());
    }

    #[test]
    fn get_by_name() {
        let mut manager = ListManager::new();
        manager.add("Shopping").unwrap();
        let result = manager.get_by_name("Shopping");
        assert!(!result.is_empty());
    }

    #[test]
    fn remove_list() {
        let mut manager = ListManager::new();
        let id = manager.add("Shopping").unwrap();
        let result = manager.remove(id);
        assert!(result.is_some());
        assert!(manager.get_by_id(id).is_none());
    }

    #[test]
    fn remove_missing_list() {
        let mut manager = ListManager::new();
        let result = manager.remove(0);
        assert!(result.is_none());
    }

    #[test]
    fn rename_list() {
        let mut manager = ListManager::new();
        let id = manager.add("Shopping").unwrap();
        manager.rename(id, "NotShopping").unwrap();
        let renamed_list = manager.get_by_id(id).unwrap();
        assert_eq!(renamed_list.get_name(), "NotShopping");
    }

    #[test]
    fn rename_missing_list() {
        let mut manager = ListManager::new();
        let id = 0;
        let result = manager.rename(id, "NotShopping");
        assert_eq!(manager.lists.len(), 0);
        assert_eq!(result, Err(ListError::ListNotFound));
    }

    #[test]
    fn rename_with_empty_name() {
        let mut manager = ListManager::new();
        let id = manager.add("Shopping").unwrap();
        let result = manager.rename(id, "");
        assert_eq!(result, Err(ListError::EmptyName));
    }
}
