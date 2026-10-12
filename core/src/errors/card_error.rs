#[derive(Debug, PartialEq, Eq)]
pub enum CardError {
    CardNotFound,
    EmptyName,
    CardIdLimitReached,
    CardPlaceLimitReached,
    InvalidCardPlace,
}