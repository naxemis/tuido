#[derive(Debug, PartialEq, Eq)]
pub enum CardMoveError {
    SourceColumnNotFound,
    TargetColumnNotFound,
    CardNotFound,
    InvalidCardPlace,
    CardAlreadyInserted,
}
