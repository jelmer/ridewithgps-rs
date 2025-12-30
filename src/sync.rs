//! Sync-related types and methods

use crate::{Result, RideWithGpsClient};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Types of items that can be synchronized
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    /// Route
    Route,

    /// Trip
    Trip,

    /// Event
    Event,

    /// Collection
    Collection,
}

/// A synchronized item
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncItem {
    /// Item ID
    pub id: u64,

    /// Item type
    pub item_type: ItemType,

    /// When the item was updated
    pub updated_at: DateTime<Utc>,

    /// Whether the item was deleted
    pub deleted: Option<bool>,
}

/// Response from the sync endpoint
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncResponse {
    /// List of changed items
    pub items: Vec<SyncItem>,

    /// Server datetime for use in next sync request
    pub server_datetime: DateTime<Utc>,
}

impl RideWithGpsClient {
    /// Get items that have changed since a specific datetime
    ///
    /// This endpoint is useful for efficiently synchronizing a local library
    /// with the server by only fetching items that have changed.
    ///
    /// # Arguments
    ///
    /// * `since` - DateTime since which to fetch changes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ridewithgps_client::RideWithGpsClient;
    /// use chrono::{Utc, TimeZone};
    ///
    /// let client = RideWithGpsClient::new(
    ///     "https://ridewithgps.com",
    ///     "your-api-key",
    ///     Some("your-auth-token")
    /// );
    ///
    /// // Get all changes since January 1, 2025
    /// let since = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    /// let sync = client.sync(&since).unwrap();
    ///
    /// println!("Found {} changed items", sync.items.len());
    /// for item in sync.items {
    ///     println!("{:?} {} updated at {}",
    ///         item.item_type, item.id, item.updated_at);
    /// }
    ///
    /// // Use server_datetime for next sync
    /// let next_sync = client.sync(&sync.server_datetime).unwrap();
    /// ```
    pub fn sync(&self, since: &DateTime<Utc>) -> Result<SyncResponse> {
        let since_str = since.to_rfc3339();
        let url = format!(
            "/api/v1/sync.json?since={}",
            urlencoding::encode(&since_str)
        );
        self.get(&url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_sync_item_deserialization() {
        let json = r#"{
            "id": 123,
            "item_type": "route",
            "updated_at": "2025-01-15T10:30:00Z",
            "deleted": false
        }"#;

        let item: SyncItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.id, 123);
        assert_eq!(item.item_type, ItemType::Route);
        assert_eq!(item.deleted, Some(false));
    }

    #[test]
    fn test_sync_response_deserialization() {
        let json = r#"{
            "items": [
                {
                    "id": 123,
                    "item_type": "route",
                    "updated_at": "2025-01-15T10:30:00Z",
                    "deleted": false
                },
                {
                    "id": 456,
                    "item_type": "trip",
                    "updated_at": "2025-01-15T11:00:00Z",
                    "deleted": true
                }
            ],
            "server_datetime": "2025-01-15T12:00:00Z"
        }"#;

        let response: SyncResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].item_type, ItemType::Route);
        assert_eq!(response.items[1].item_type, ItemType::Trip);
        assert_eq!(
            response.server_datetime,
            Utc.with_ymd_and_hms(2025, 1, 15, 12, 0, 0).unwrap()
        );
    }

    #[test]
    fn test_item_type_serialization() {
        assert_eq!(
            serde_json::to_string(&ItemType::Route).unwrap(),
            r#""route""#
        );
        assert_eq!(serde_json::to_string(&ItemType::Trip).unwrap(), r#""trip""#);
    }
}
