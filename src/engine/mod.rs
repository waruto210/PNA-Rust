//! Provide different engines for our k/v store
use crate::Result;
use async_trait::async_trait;

/// trait for k/v store engin
#[async_trait]
pub trait KvsEngine: Clone + Send + 'static {
    /// Sets the string value of a given string key.
    ///
    /// If the given key already exists, the previous value will be overwitten.
    async fn set(&self, key: String, value: String) -> Result<()>;

    /// Get the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    async fn get(&self, key: String) -> Result<Option<String>>;

    /// Remove a given string key.
    ///
    /// # Errors
    ///
    /// Returns `KvsError::KeyNotFound` if the given ket does not exixt.
    async fn remove(&self, key: String) -> Result<()>;
}
pub use self::sled::SledKvsEngine;
pub use kv::KvStore;
mod kv;
mod sled;
