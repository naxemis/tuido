use std::collections::HashMap;

use crate::models::column::Column;

pub struct ColumnManager {
    columns: HashMap<u8, Column>,
    next_id: u8,
}

impl ColumnManager {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
            next_id: 0,
        }
    }
}

impl Default for ColumnManager {
    fn default() -> Self {
        Self::new()
    }
}