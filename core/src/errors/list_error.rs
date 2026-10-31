/// Errors that can occur while managing lists.
#[derive(Debug, PartialEq, Eq)]
pub enum ListError {
    /// The requested list does not exist.
    ListNotFound,
    /// A list name is empty after trimming whitespace.
    EmptyName,
    /// No more list IDs are available.
    ListIdLimitReached,
}
