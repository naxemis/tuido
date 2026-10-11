#[derive(Debug, PartialEq, Eq)]
pub enum ListError {
    ListNotFound,
    EmptyName,
    IdExhausted,
}
