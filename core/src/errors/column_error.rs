/// Errors that can occur while managing columns.
#[derive(Debug, PartialEq, Eq)]
pub enum ColumnError {
    /// The requested column does not exist.
    ColumnNotFound,
    /// A column name is empty after trimming whitespace.
    EmptyName,
    /// No more column IDs are available.
    ColumnIdLimitReached,
    /// No more column places are available.
    ColumnPlaceLimitReached,
    /// No valid column place was found.
    InvalidColumnPlace,
}
