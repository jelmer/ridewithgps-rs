//! Trip-related types and methods

use crate::{PaginatedResponse, Photo, Polyline, Result, RideWithGpsClient, Visibility};
use serde::{Deserialize, Serialize};

/// Track point on a trip with telemetry data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TripTrackPoint {
    /// Longitude
    pub x: Option<f64>,

    /// Latitude
    pub y: Option<f64>,

    /// Distance in meters
    pub d: Option<f64>,

    /// Elevation in meters
    pub e: Option<f64>,

    /// Unix timestamp
    pub t: Option<i64>,

    /// Speed in km/h
    pub s: Option<f64>,

    /// Temperature in Celsius
    #[serde(rename = "T")]
    pub temp: Option<f64>,

    /// Heart rate in BPM
    pub h: Option<f64>,

    /// Cadence in RPM
    pub c: Option<f64>,

    /// Power in watts
    pub p: Option<f64>,

    /// Power balance (L/R percentage)
    pub pb: Option<f64>,

    /// Lap marker
    pub lap: Option<bool>,

    /// Exclude from metrics
    pub k: Option<bool>,

    /// User modified point
    pub m: Option<bool>,
}

/// Gear/equipment used for a trip
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Gear {
    /// Gear ID
    pub id: u64,

    /// Make/brand
    pub make: Option<String>,

    /// Model
    pub model: Option<String>,

    /// Description
    pub description: Option<String>,

    /// Whether to exclude from totals
    pub exclude_from_totals: Option<bool>,

    /// Created timestamp
    pub created_at: Option<String>,
}

/// A trip (recorded ride)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trip {
    /// Trip ID
    pub id: u64,

    /// Trip name
    pub name: Option<String>,

    /// Trip description
    pub description: Option<String>,

    /// Distance in meters
    pub distance: Option<f64>,

    /// Elevation gain in meters
    pub elevation_gain: Option<f64>,

    /// Elevation loss in meters
    pub elevation_loss: Option<f64>,

    /// Trip visibility
    pub visibility: Option<Visibility>,

    /// User ID of the trip owner
    pub user_id: Option<u64>,

    /// API URL
    pub url: Option<String>,

    /// HTML/web URL
    pub web_url: Option<String>,

    /// Departed at timestamp
    pub departed_at: Option<String>,

    /// Time zone
    pub time_zone: Option<String>,

    /// Created timestamp
    pub created_at: Option<String>,

    /// Updated timestamp
    pub updated_at: Option<String>,

    /// Duration in seconds
    pub duration: Option<f64>,

    /// Moving time in seconds
    pub moving_time: Option<f64>,

    /// Average speed in m/s
    pub avg_speed: Option<f64>,

    /// Maximum speed in m/s
    pub max_speed: Option<f64>,

    /// Average cadence (RPM)
    pub avg_cad: Option<f64>,

    /// Minimum cadence (RPM)
    pub min_cad: Option<f64>,

    /// Maximum cadence (RPM)
    pub max_cad: Option<f64>,

    /// Average heart rate (BPM)
    pub avg_hr: Option<f64>,

    /// Minimum heart rate (BPM)
    pub min_hr: Option<f64>,

    /// Maximum heart rate (BPM)
    pub max_hr: Option<f64>,

    /// Average power (watts)
    pub avg_watts: Option<f64>,

    /// Minimum power (watts)
    pub min_watts: Option<f64>,

    /// Maximum power (watts)
    pub max_watts: Option<f64>,

    /// Calories burned
    pub calories: Option<f64>,

    /// Recording device name
    pub device: Option<String>,

    /// Locality/location
    pub locality: Option<String>,

    /// Administrative area
    pub administrative_area: Option<String>,

    /// Country code
    pub country_code: Option<String>,

    /// Activity type
    pub activity_type: Option<String>,

    /// FIT file sport type
    pub fit_sport: Option<i32>,

    /// FIT file sub-sport type
    pub fit_sub_sport: Option<i32>,

    /// Whether the trip is stationary
    pub stationary: Option<bool>,

    /// Track type
    pub track_type: Option<String>,

    /// Terrain rating
    pub terrain: Option<i32>,

    /// Difficulty rating
    pub difficulty: Option<i32>,

    /// First point latitude
    pub first_lat: Option<f64>,

    /// First point longitude
    pub first_lng: Option<f64>,

    /// Last point latitude
    pub last_lat: Option<f64>,

    /// Last point longitude
    pub last_lng: Option<f64>,

    /// Southwest corner latitude (bounding box)
    pub sw_lat: Option<f64>,

    /// Southwest corner longitude (bounding box)
    pub sw_lng: Option<f64>,

    /// Northeast corner latitude (bounding box)
    pub ne_lat: Option<f64>,

    /// Northeast corner longitude (bounding box)
    pub ne_lng: Option<f64>,

    /// Track points with telemetry (included when fetching a specific trip)
    pub track_points: Option<Vec<TripTrackPoint>>,

    /// Gear/equipment used (included when fetching a specific trip)
    pub gear: Option<Gear>,

    /// Photos (included when fetching a specific trip)
    pub photos: Option<Vec<Photo>>,
}

/// Parameters for listing trips
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListTripsParams {
    /// Filter by trip name
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
    /// List trips for the authenticated user
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ridewithgps_client::{RideWithGpsClient, ListTripsParams};
    ///
    /// let client = RideWithGpsClient::new(
    ///     "https://ridewithgps.com",
    ///     "your-api-key",
    ///     Some("your-auth-token")
    /// );
    ///
    /// let params = ListTripsParams {
    ///     min_distance: Some(20000.0), // 20km
    ///     ..Default::default()
    /// };
    ///
    /// let trips = client.list_trips(Some(&params)).unwrap();
    /// println!("Found {} trips", trips.results.len());
    /// ```
    pub fn list_trips(&self, params: Option<&ListTripsParams>) -> Result<PaginatedResponse<Trip>> {
        let mut url = "/api/v1/trips.json".to_string();

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

    /// Get a specific trip by ID
    ///
    /// # Arguments
    ///
    /// * `id` - The trip ID
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
    /// let trip = client.get_trip(12345).unwrap();
    /// println!("Trip: {:?}", trip);
    /// ```
    pub fn get_trip(&self, id: u64) -> Result<Trip> {
        #[derive(Deserialize)]
        struct TripWrapper {
            trip: Trip,
        }

        let wrapper: TripWrapper = self.get(&format!("/api/v1/trips/{}.json", id))?;
        Ok(wrapper.trip)
    }

    /// Get the polyline for a specific trip
    ///
    /// # Arguments
    ///
    /// * `id` - The trip ID
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
    /// let polyline = client.get_trip_polyline(12345).unwrap();
    /// println!("Polyline: {}", polyline.polyline);
    /// ```
    pub fn get_trip_polyline(&self, id: u64) -> Result<Polyline> {
        self.get(&format!("/api/v1/trips/{}/polyline.json", id))
    }

    /// Delete a trip
    ///
    /// # Arguments
    ///
    /// * `id` - The trip ID
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
    /// client.delete_trip(12345).unwrap();
    /// ```
    pub fn delete_trip(&self, id: u64) -> Result<()> {
        self.delete(&format!("/api/v1/trips/{}.json", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trip_deserialization() {
        let json = r#"{
            "id": 456,
            "name": "Morning Ride",
            "distance": 25000.0,
            "elevation_gain": 300.0,
            "visibility": "private",
            "duration": 3600.0,
            "avg_speed": 6.94
        }"#;

        let trip: Trip = serde_json::from_str(json).unwrap();
        assert_eq!(trip.id, 456);
        assert_eq!(trip.name.as_deref(), Some("Morning Ride"));
        assert_eq!(trip.distance, Some(25000.0));
        assert_eq!(trip.visibility, Some(Visibility::Private));
        assert_eq!(trip.duration, Some(3600.0));
    }

    #[test]
    fn test_list_trips_params() {
        let params = ListTripsParams {
            name: Some("ride".to_string()),
            visibility: Some(Visibility::Public),
            min_distance: Some(10000.0),
            page: Some(2),
            ..Default::default()
        };

        let json = serde_json::to_value(&params).unwrap();
        assert!(json.get("name").is_some());
        assert!(json.get("visibility").is_some());
        assert!(json.get("min_distance").is_some());
        assert!(json.get("page").is_some());
    }
}
