use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use warp::{
    reject::{self, Reject},
    Rejection,
};

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to fetch data")]
    FetchError { source: reqwest::Error },

    #[error("Failed to deserialize data")]
    DeserializationError { source: reqwest::Error },
}

impl Reject for StorageError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub data_type: String,
    pub path: String,
    pub value: Value,
}

#[async_trait]
pub trait Storer: Clone + Send + Sync {
    async fn get(&self, path: &str) -> Result<Data, Rejection>;
}

#[derive(Clone)]
pub struct RedactStorer {
    url: String,
}

impl RedactStorer {
    pub fn new(url: &str) -> RedactStorer {
        RedactStorer {
            url: url.to_string(),
        }
    }
}

#[async_trait]
impl Storer for RedactStorer {
    async fn get(&self, path: &str) -> Result<Data, Rejection> {
        match reqwest::get(&format!("{}/data?path={}", self.url, path)).await {
            Ok(r) => Ok(r
                .json::<Data>()
                .await
                .map_err(|source| reject::custom(StorageError::DeserializationError { source }))?),
            Err(source) => Err(reject::custom(StorageError::FetchError { source })),
        }
    }
}

// pub async fn get(url: &str, path: String) -> Result<Data, Rejection> {
//     match reqwest::get(&format!("{}/data?path={}", url, path)).await {
//         Ok(r) => Ok(r
//             .json::<Data>()
//             .await
//             .map_err(|source| reject::custom(StorageError::DeserializationError { source }))?),
//         Err(source) => Err(reject::custom(StorageError::FetchError { source })),
//     }
// }
