use anyhow::Result;
use futures::StreamExt;
use mongodb::bson::Document;
use mongodb::{bson, Cursor};
use serde::de::DeserializeOwned;

pub enum MongoDB {
    Profiles,
    Tweets,
}

impl MongoDB {
    pub fn name(&self) -> &str {
        match self {
            Self::Profiles => "profiles-db",
            Self::Tweets => "tweets-db",
        }
    }

    pub fn uri(&self) -> String {
        format!("mongodb://{}:27017", self.name())
    }
}

pub enum MongoCollection {
    Profiles,
    Tweets,
}

impl MongoCollection {
    pub fn name(&self) -> &str {
        match self {
            Self::Profiles => "profiles",
            Self::Tweets => "tweets",
        }
    }
}

/// A helper function to collect items from a MongoDB cursor
///
/// # Arguments
///
/// * `cursor` - The MongoDB cursor
/// * `limit` - The maximum number of items to collect
///
/// # Returns
///     
/// A vector of items
pub async fn collect_deserialize<T: DeserializeOwned>(
    mut cursor: Cursor<Document>,
    limit: Option<usize>,
) -> Result<Vec<T>, tonic::Status> {
    let mut items = Vec::new();
    while let Some(result) = cursor.next().await {
        // Check if we've reached the limit
        if let Some(limit) = limit {
            if items.len() >= limit {
                break;
            }
        }

        // Deserialize the document
        match result {
            Ok(document) => match bson::from_document::<T>(document) {
                Ok(item) => items.push(item),
                Err(_) => return Err(tonic::Status::internal("Failed to deserialize item")),
            },
            Err(_) => return Err(tonic::Status::internal("Failed to get item")),
        }
    }

    Ok(items)
}
