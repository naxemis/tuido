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
}