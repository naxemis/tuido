use std::io;

#[derive(Debug)]
pub enum StorageError {
    Io(io::Error),
    Serialization(serde_json::Error),
    Deserialization(serde_json::Error),
}