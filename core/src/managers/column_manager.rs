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
        if column_name.len() == 0 {
            return Err(ColumnError::EmptyName);
        }

        let column_id = self.next_id as u8;
        let column = Column::new(column_id, column_name, self.columns.len() as u8);
        self.columns.insert(column_id, column);
        self.next_id += 1;
        Ok(column_id)
    }
}

impl Default for ColumnManager {
    fn default() -> Self {
        Self::new()
    }
}