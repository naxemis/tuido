use std::collections::HashMap;
use crate::errors::column_error::ColumnError;
use crate::models::column::Column;

pub struct ColumnManager {
    columns: HashMap<u8, Column>,
    next_id: u16,
}

impl ColumnManager {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add(&mut self, name: &str) -> Result<u8, ColumnError> {
        if self.next_id > u16::from(u8::MAX) {
            return Err(ColumnError::ColumnIdLimitReached);
        }

        if self.columns.len() > u8::MAX as usize {
            return Err(ColumnError::ColumnPlaceLimitReached);
        }

        let column_name = name.trim().to_string();
        if column_name.is_empty() {
            return Err(ColumnError::EmptyName);
        }

        let column_id = self.next_id as u8;
        let column = Column::new(column_id, column_name, self.columns.len() as u8);
        self.columns.insert(column_id, column);
        self.next_id += 1;
        Ok(column_id)
    }

    pub fn remove(&mut self, id: u8) -> Option<Column> {
        self.columns.remove(&id)
    }

    pub fn get_by_id(&self, id: u8) -> Option<&Column> {
        self.columns.get(&id)
    }

    pub fn get_by_name(&self, name: &str) -> Vec<&Column> {
        let mut matching_columns = Vec::new();

        for column in self.columns.values() {
            if column.get_name() == name {
                matching_columns.push(column);
            }
        }

        matching_columns
    }

    pub fn rename(&mut self, id: u8, new_name: &str) -> Result<(), ColumnError> {
        let name = new_name.trim();
        if name.is_empty() {
            return Err(ColumnError::EmptyName);
        }

        match self.columns.get_mut(&id) {
            Some(column) => {
                column.rename(name);
                Ok(())
            }
            None => Err(ColumnError::ColumnNotFound),
        }

    }
}

impl Default for ColumnManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::column_error::ColumnError;
    use crate::managers::column_manager::ColumnManager;

    #[test]
    fn add_column() {
        let mut manager = ColumnManager::new();
        let result = manager.add("A");
        assert_eq!(result, Ok(0))
    }

    #[test]
    fn add_column_empty_name() {
        let mut manager = ColumnManager::new();
        let result = manager.add("   ");
        assert_eq!(result, Err(ColumnError::EmptyName));
    }

    #[test]
    fn get_by_id() {
        let mut manager = ColumnManager::new();
        let result = manager.add("A");
        assert!(manager.get_by_id(result.unwrap()).is_some());
    }

    #[test]
    fn get_by_name() {
        let mut manager = ColumnManager::new();
        manager.add("A").unwrap();
        let result = manager.get_by_name("A");
        assert!(!result.is_empty())
    }

    #[test]
    fn remove_colum() {
        let mut manager = ColumnManager::new();
        let id = manager.add("A").unwrap();
        let result = manager.remove(id);
        assert!(result.is_some());
        assert!(manager.get_by_id(id).is_none());
    }

    #[test]
    fn remove_missing_column() {
        let mut manager = ColumnManager::new();
        let result = manager.remove(0);
        assert!(result.is_none());
    }
}