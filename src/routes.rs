//! Route-related types and methods

use crate::{PaginatedResponse, PointOfInterest, Result, RideWithGpsClient};
use serde::{Deserialize, Serialize};

/// Visibility setting for a route
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    /// Public route
    Public,

    /// Private route
    Private,

    /// Unlisted route
    Unlisted,
}

/// Track point on a route
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrackPoint {
    /// Longitude
    pub x: Option<f64>,

    /// Latitude
    pub y: Option<f64>,

    /// Distance in meters
    pub d: Option<f64>,

    /// Elevation in meters
    pub e: Option<f64>,
}

/// Course point (turn-by-turn cue) on a route
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoursePoint {
    /// Longitude
    pub x: Option<f64>,

    /// Latitude
    pub y: Option<f64>,

    /// Distance in meters
    pub d: Option<f64>,

    /// Cue type
    pub t: Option<String>,

    /// Cue text/description
    pub n: Option<String>,
}

/// Photo attached to a route or trip
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Photo {
    /// Photo ID
    pub id: u64,

    /// Photo URL
    pub url: Option<String>,

    /// Whether the photo is highlighted
    pub highlighted: Option<bool>,

    /// Photo caption
    pub caption: Option<String>,

    /// Created timestamp
    pub created_at: Option<String>,
}

/// A route
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Route {
    /// Route ID
    pub id: u64,

    /// Route name
    pub name: Option<String>,

    /// Route description
    pub description: Option<String>,

    /// Distance in meters
    pub distance: Option<f64>,

    /// Elevation gain in meters
    pub elevation_gain: Option<f64>,

    /// Elevation loss in meters
    pub elevation_loss: Option<f64>,

    /// Route visibility
    pub visibility: Option<Visibility>,

    /// User ID of the route owner
    pub user_id: Option<u64>,

    /// Created timestamp
    pub created_at: Option<String>,

    /// Updated timestamp
    pub updated_at: Option<String>,

    /// Locality/location
    pub locality: Option<String>,

    /// Administrative area
    pub administrative_area: Option<String>,

    /// Country code
    pub country_code: Option<String>,

    /// Track type
    pub track_type: Option<String>,

    /// Whether the route has course points
    pub has_course_points: Option<bool>,

    /// Terrain rating
    pub terrain: Option<i32>,

    /// Difficulty rating
    pub difficulty: Option<i32>,

    /// Track points (included when fetching a specific route)
    pub track_points: Option<Vec<TrackPoint>>,

    /// Course points/cues (included when fetching a specific route)
    pub course_points: Option<Vec<CoursePoint>>,

    /// Points of interest along the route (included when fetching a specific route)
    pub points_of_interest: Option<Vec<PointOfInterest>>,

    /// Photos (included when fetching a specific route)
    pub photos: Option<Vec<Photo>>,
}

/// Polyline data for a route
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Polyline {
    /// Encoded polyline string
    pub polyline: String,

    /// Parent type (e.g., "route")
    pub parent_type: Option<String>,

    /// Parent ID
    pub parent_id: Option<u64>,
}

/// Parameters for listing routes
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListRoutesParams {
    /// Filter by route name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Filter by visibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,

    /// Filter by minimum distance (meters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_distance: Option<f64>,

    /// Filter by maximum distance (meters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distance: Option<f64>,

    /// Filter by minimum elevation gain (meters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_elevation_gain: Option<f64>,

    /// Filter by maximum elevation gain (meters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_elevation_gain: Option<f64>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

impl RideWithGpsClient {
    /// List routes for the authenticated user
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ridewithgps_client::{RideWithGpsClient, ListRoutesParams};
    ///
    /// let client = RideWithGpsClient::new(
    ///     "https://ridewithgps.com",
    ///     "your-api-key",
    ///     Some("your-auth-token")
    /// );
    ///
    /// let params = ListRoutesParams {
    ///     min_distance: Some(10000.0), // 10km
    ///     ..Default::default()
    /// };
    ///
    /// let routes = client.list_routes(Some(&params)).unwrap();
    /// println!("Found {} routes", routes.results.len());
    /// ```
    pub fn list_routes(
        &self,
        params: Option<&ListRoutesParams>,
    ) -> Result<PaginatedResponse<Route>> {
        let mut url = "/api/v1/routes.json".to_string();

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

    /// Get a specific route by ID
    ///
    /// # Arguments
    ///
    /// * `id` - The route ID
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
    /// let route = client.get_route(12345).unwrap();
    /// println!("Route: {:?}", route);
    /// ```
    pub fn get_route(&self, id: u64) -> Result<Route> {
        self.get(&format!("/api/v1/routes/{}.json", id))
    }

    /// Get the polyline for a specific route
    ///
    /// # Arguments
    ///
    /// * `id` - The route ID
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
    /// let polyline = client.get_route_polyline(12345).unwrap();
    /// println!("Polyline: {}", polyline.polyline);
    /// ```
    pub fn get_route_polyline(&self, id: u64) -> Result<Polyline> {
        self.get(&format!("/api/v1/routes/{}/polyline.json", id))
    }

    /// Delete a route
    ///
    /// # Arguments
    ///
    /// * `id` - The route ID
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
    /// client.delete_route(12345).unwrap();
    /// ```
    pub fn delete_route(&self, id: u64) -> Result<()> {
        self.delete(&format!("/api/v1/routes/{}.json", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_deserialization() {
        let json = r#"{
            "id": 123,
            "name": "Test Route",
            "distance": 10000.0,
            "elevation_gain": 500.0,
            "visibility": "public"
        }"#;

        let route: Route = serde_json::from_str(json).unwrap();
        assert_eq!(route.id, 123);
        assert_eq!(route.name.as_deref(), Some("Test Route"));
        assert_eq!(route.distance, Some(10000.0));
        assert_eq!(route.visibility, Some(Visibility::Public));
    }

    #[test]
    fn test_polyline_deserialization() {
        let json = r#"{
            "polyline": "encoded_string_here",
            "parent_type": "route",
            "parent_id": 123
        }"#;

        let polyline: Polyline = serde_json::from_str(json).unwrap();
        assert_eq!(polyline.polyline, "encoded_string_here");
        assert_eq!(polyline.parent_type.as_deref(), Some("route"));
        assert_eq!(polyline.parent_id, Some(123));
    }

    #[test]
    fn test_list_routes_params() {
        let params = ListRoutesParams {
            name: Some("test".to_string()),
            visibility: Some(Visibility::Public),
            min_distance: Some(5000.0),
            ..Default::default()
        };

        let json = serde_json::to_value(&params).unwrap();
        assert!(json.get("name").is_some());
        assert!(json.get("visibility").is_some());
        assert!(json.get("min_distance").is_some());
    }
}
