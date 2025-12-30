//! Collection-related types and methods

use crate::{PaginatedResponse, Result, RideWithGpsClient, Route, Trip};
use serde::{Deserialize, Serialize};

/// A collection of routes and trips
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Collection {
    /// Collection ID
    pub id: u64,

    /// Collection name
    pub name: Option<String>,

    /// Collection description
    pub description: Option<String>,

    /// User ID of the collection owner
    pub user_id: Option<u64>,

    /// Visibility
    pub visibility: Option<crate::Visibility>,

    /// API URL
    pub url: Option<String>,

    /// HTML/web URL
    pub html_url: Option<String>,

    /// Cover photo/image
    pub cover: Option<String>,

    /// Created timestamp
    pub created_at: Option<String>,

    /// Updated timestamp
    pub updated_at: Option<String>,

    /// Number of routes in the collection
    pub route_count: Option<u32>,

    /// Collection cover photo URL
    pub cover_photo_url: Option<String>,

    /// Routes in the collection (included when fetching a specific collection)
    pub routes: Option<Vec<Route>>,

    /// Trips in the collection (included when fetching a specific collection)
    pub trips: Option<Vec<Trip>>,
}

/// Parameters for listing collections
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListCollectionsParams {
    /// Filter by collection name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

impl RideWithGpsClient {
    /// List collections
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ridewithgps_client::RideWithGpsClient;
    ///
    /// let client = RideWithGpsClient::new(
    ///     "https://ridewithgps.com",
    ///     "your-api-key",
    ///     Some("your-auth-token")
    /// );
    ///
    /// let collections = client.list_collections(None).unwrap();
    /// println!("Found {} collections", collections.results.len());
    /// ```
    pub fn list_collections(
        &self,
        params: Option<&ListCollectionsParams>,
    ) -> Result<PaginatedResponse<Collection>> {
        let mut url = "/api/v1/collections.json".to_string();

        if let Some(params) = params {
            let query = serde_json::to_value(params)?;
            if let Some(obj) = query.as_object() {
                if !obj.is_empty() {
                    let query_str = serde_urlencoded::to_string(obj).map_err(|e| {
                        crate::Error::ApiError(format!("Failed to encode query: {}", e))
                    })?;
                    url.push('?');
                    url.push_str(&query_str);
                }
            }
        }

        self.get(&url)
    }

    /// Get a specific collection by ID
    ///
    /// This returns the full collection including its routes and trips.
    ///
    /// # Arguments
    ///
    /// * `id` - The collection ID
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ridewithgps_client::RideWithGpsClient;
    ///
    /// let client = RideWithGpsClient::new(
    ///     "https://ridewithgps.com",
    ///     "your-api-key",
    ///     None
    /// );
    ///
    /// let collection = client.get_collection(12345).unwrap();
    /// println!("Collection: {:?}", collection);
    ///
    /// // Access routes within the collection
    /// if let Some(routes) = &collection.routes {
    ///     for route in routes {
    ///         println!("Route: {} - {:?}", route.id, route.name);
    ///     }
    /// }
    ///
    /// // Access trips within the collection
    /// if let Some(trips) = &collection.trips {
    ///     for trip in trips {
    ///         println!("Trip: {} - {:?}", trip.id, trip.name);
    ///     }
    /// }
    /// ```
    pub fn get_collection(&self, id: u64) -> Result<Collection> {
        #[derive(Deserialize)]
        struct CollectionWrapper {
            collection: Collection,
        }

        let wrapper: CollectionWrapper = self.get(&format!("/api/v1/collections/{}.json", id))?;
        Ok(wrapper.collection)
    }

    /// Get the pinned collection
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ridewithgps_client::RideWithGpsClient;
    ///
    /// let client = RideWithGpsClient::new(
    ///     "https://ridewithgps.com",
    ///     "your-api-key",
    ///     Some("your-auth-token")
    /// );
    ///
    /// let collection = client.get_pinned_collection().unwrap();
    /// println!("Pinned collection: {:?}", collection);
    /// ```
    pub fn get_pinned_collection(&self) -> Result<Collection> {
        #[derive(Deserialize)]
        struct CollectionWrapper {
            collection: Collection,
        }

        let wrapper: CollectionWrapper = self.get("/api/v1/collections/pinned.json")?;
        Ok(wrapper.collection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_deserialization() {
        let json = r#"{
            "id": 999,
            "name": "My Favorite Routes",
            "description": "Best rides in the area",
            "user_id": 123,
            "route_count": 15
        }"#;

        let collection: Collection = serde_json::from_str(json).unwrap();
        assert_eq!(collection.id, 999);
        assert_eq!(collection.name.as_deref(), Some("My Favorite Routes"));
        assert_eq!(
            collection.description.as_deref(),
            Some("Best rides in the area")
        );
        assert_eq!(collection.route_count, Some(15));
    }

    #[test]
    fn test_list_collections_params() {
        let params = ListCollectionsParams {
            name: Some("favorites".to_string()),
            page: Some(1),
            ..Default::default()
        };

        let json = serde_json::to_value(&params).unwrap();
        assert!(json.get("name").is_some());
        assert!(json.get("page").is_some());
    }

    #[test]
    fn test_collection_wrapper_deserialization() {
        let json = r#"{
            "collection": {
                "id": 888,
                "name": "Wrapped Collection",
                "visibility": "public"
            }
        }"#;

        #[derive(Deserialize)]
        struct CollectionWrapper {
            collection: Collection,
        }

        let wrapper: CollectionWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.collection.id, 888);
        assert_eq!(
            wrapper.collection.name.as_deref(),
            Some("Wrapped Collection")
        );
        assert_eq!(
            wrapper.collection.visibility,
            Some(crate::Visibility::Public)
        );
    }

    #[test]
    fn test_collection_with_nested_routes() {
        let json = r#"{
            "id": 777,
            "name": "Collection with Routes",
            "routes": [
                {
                    "id": 1,
                    "name": "Route 1",
                    "distance": 10000.0
                },
                {
                    "id": 2,
                    "name": "Route 2",
                    "distance": 20000.0
                }
            ]
        }"#;

        let collection: Collection = serde_json::from_str(json).unwrap();
        assert_eq!(collection.id, 777);
        assert!(collection.routes.is_some());
        let routes = collection.routes.unwrap();
        assert_eq!(routes.len(), 2);
        assert_eq!(routes[0].id, 1);
        assert_eq!(routes[0].name.as_deref(), Some("Route 1"));
        assert_eq!(routes[1].id, 2);
    }

    #[test]
    fn test_collection_with_nested_trips() {
        let json = r#"{
            "id": 666,
            "name": "Collection with Trips",
            "trips": [
                {
                    "id": 10,
                    "name": "Trip 1",
                    "distance": 15000.0
                }
            ]
        }"#;

        let collection: Collection = serde_json::from_str(json).unwrap();
        assert_eq!(collection.id, 666);
        assert!(collection.trips.is_some());
        let trips = collection.trips.unwrap();
        assert_eq!(trips.len(), 1);
        assert_eq!(trips[0].id, 10);
        assert_eq!(trips[0].name.as_deref(), Some("Trip 1"));
    }
}
