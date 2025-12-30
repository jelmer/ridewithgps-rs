//! Points of Interest related types and methods
//!
//! Note: These endpoints are only available to organization accounts.

use crate::{PaginatedResponse, Result, RideWithGpsClient};
use serde::{Deserialize, Serialize};

/// A point of interest
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PointOfInterest {
    /// POI ID
    pub id: u64,

    /// POI name
    pub name: Option<String>,

    /// POI description
    pub description: Option<String>,

    /// Latitude
    #[serde(alias = "latitude")]
    pub lat: Option<f64>,

    /// Longitude
    #[serde(alias = "longitude")]
    pub lng: Option<f64>,

    /// POI type/category
    #[serde(alias = "poi_type")]
    pub r#type: Option<String>,

    /// Type ID
    pub type_id: Option<u64>,

    /// Type name
    pub type_name: Option<String>,

    /// Icon identifier
    pub icon: Option<String>,

    /// User ID of the POI owner
    pub user_id: Option<u64>,

    /// Organization ID
    pub organization_id: Option<u64>,

    /// API URL
    pub url: Option<String>,

    /// Created timestamp
    pub created_at: Option<String>,

    /// Updated timestamp
    pub updated_at: Option<String>,

    /// Address
    pub address: Option<String>,

    /// Phone number
    pub phone: Option<String>,

    /// Website URL
    pub website: Option<String>,

    /// Tag names
    pub tag_names: Option<Vec<String>>,
}

/// Parameters for listing POIs
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListPointsOfInterestParams {
    /// Filter by POI name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Filter by POI type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poi_type: Option<String>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// Request to create or update a POI
#[derive(Debug, Clone, Serialize)]
pub struct PointOfInterestRequest {
    /// POI name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// POI description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Latitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,

    /// Longitude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,

    /// POI type/category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poi_type: Option<String>,

    /// Icon identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Website URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

impl RideWithGpsClient {
    /// List points of interest
    ///
    /// Note: This endpoint is only available to organization accounts.
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
    /// let pois = client.list_points_of_interest(None).unwrap();
    /// println!("Found {} POIs", pois.results.len());
    /// ```
    pub fn list_points_of_interest(
        &self,
        params: Option<&ListPointsOfInterestParams>,
    ) -> Result<PaginatedResponse<PointOfInterest>> {
        let mut url = "/api/v1/points_of_interest.json".to_string();

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

    /// Create a new point of interest
    ///
    /// Note: This endpoint is only available to organization accounts.
    ///
    /// # Arguments
    ///
    /// * `poi` - The POI data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ridewithgps_client::{RideWithGpsClient, PointOfInterestRequest};
    ///
    /// let client = RideWithGpsClient::new(
    ///     "https://ridewithgps.com",
    ///     "your-api-key",
    ///     Some("your-auth-token")
    /// );
    ///
    /// let poi_req = PointOfInterestRequest {
    ///     name: Some("Coffee Shop".to_string()),
    ///     description: Some("Great coffee stop".to_string()),
    ///     latitude: Some(37.7749),
    ///     longitude: Some(-122.4194),
    ///     poi_type: Some("cafe".to_string()),
    ///     icon: Some("coffee".to_string()),
    ///     address: None,
    ///     phone: None,
    ///     website: None,
    /// };
    ///
    /// let poi = client.create_point_of_interest(&poi_req).unwrap();
    /// println!("Created POI: {}", poi.id);
    /// ```
    pub fn create_point_of_interest(
        &self,
        poi: &PointOfInterestRequest,
    ) -> Result<PointOfInterest> {
        #[derive(Deserialize)]
        struct PoiWrapper {
            point_of_interest: PointOfInterest,
        }

        let wrapper: PoiWrapper = self.post("/api/v1/points_of_interest.json", poi)?;
        Ok(wrapper.point_of_interest)
    }

    /// Get a specific point of interest by ID
    ///
    /// Note: This endpoint is only available to organization accounts.
    ///
    /// # Arguments
    ///
    /// * `id` - The POI ID
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
    /// let poi = client.get_point_of_interest(12345).unwrap();
    /// println!("POI: {:?}", poi);
    /// ```
    pub fn get_point_of_interest(&self, id: u64) -> Result<PointOfInterest> {
        #[derive(Deserialize)]
        struct PoiWrapper {
            point_of_interest: PointOfInterest,
        }

        let wrapper: PoiWrapper = self.get(&format!("/api/v1/points_of_interest/{}.json", id))?;
        Ok(wrapper.point_of_interest)
    }

    /// Update a point of interest
    ///
    /// Note: This endpoint is only available to organization accounts.
    ///
    /// # Arguments
    ///
    /// * `id` - The POI ID
    /// * `poi` - The updated POI data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ridewithgps_client::{RideWithGpsClient, PointOfInterestRequest};
    ///
    /// let client = RideWithGpsClient::new(
    ///     "https://ridewithgps.com",
    ///     "your-api-key",
    ///     Some("your-auth-token")
    /// );
    ///
    /// let poi_req = PointOfInterestRequest {
    ///     name: Some("Updated Coffee Shop".to_string()),
    ///     description: None,
    ///     latitude: None,
    ///     longitude: None,
    ///     poi_type: None,
    ///     icon: None,
    ///     address: None,
    ///     phone: None,
    ///     website: None,
    /// };
    ///
    /// let poi = client.update_point_of_interest(12345, &poi_req).unwrap();
    /// println!("Updated POI: {:?}", poi);
    /// ```
    pub fn update_point_of_interest(
        &self,
        id: u64,
        poi: &PointOfInterestRequest,
    ) -> Result<PointOfInterest> {
        #[derive(Deserialize)]
        struct PoiWrapper {
            point_of_interest: PointOfInterest,
        }

        let wrapper: PoiWrapper =
            self.put(&format!("/api/v1/points_of_interest/{}.json", id), poi)?;
        Ok(wrapper.point_of_interest)
    }

    /// Delete a point of interest
    ///
    /// Note: This endpoint is only available to organization accounts.
    ///
    /// # Arguments
    ///
    /// * `id` - The POI ID
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
    /// client.delete_point_of_interest(12345).unwrap();
    /// ```
    pub fn delete_point_of_interest(&self, id: u64) -> Result<()> {
        self.delete(&format!("/api/v1/points_of_interest/{}.json", id))
    }

    /// Associate a point of interest with a route
    ///
    /// Note: This endpoint is only available to organization accounts.
    ///
    /// # Arguments
    ///
    /// * `poi_id` - The POI ID
    /// * `route_id` - The route ID
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
    /// client.associate_poi_with_route(12345, 67890).unwrap();
    /// ```
    pub fn associate_poi_with_route(&self, poi_id: u64, route_id: u64) -> Result<()> {
        let url = format!(
            "/api/v1/points_of_interest/{}/routes/{}.json",
            poi_id, route_id
        );
        let response = self
            .client
            .post(self.base_url.join(&url)?)
            .headers(self.build_headers()?)
            .send()?;

        match response.status().as_u16() {
            200 | 201 | 204 => Ok(()),
            _ => {
                let status = response.status();
                let text = response.text().unwrap_or_default();
                Err(self.error_from_status(status.as_u16(), &text))
            }
        }
    }

    /// Disassociate a point of interest from a route
    ///
    /// Note: This endpoint is only available to organization accounts.
    ///
    /// # Arguments
    ///
    /// * `poi_id` - The POI ID
    /// * `route_id` - The route ID
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
    /// client.disassociate_poi_from_route(12345, 67890).unwrap();
    /// ```
    pub fn disassociate_poi_from_route(&self, poi_id: u64, route_id: u64) -> Result<()> {
        let url = format!(
            "/api/v1/points_of_interest/{}/routes/{}.json",
            poi_id, route_id
        );
        self.delete(&url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poi_deserialization() {
        let json = r#"{
            "id": 999,
            "name": "Coffee Shop",
            "description": "Great coffee",
            "latitude": 37.7749,
            "longitude": -122.4194,
            "poi_type": "cafe",
            "icon": "coffee"
        }"#;

        let poi: PointOfInterest = serde_json::from_str(json).unwrap();
        assert_eq!(poi.id, 999);
        assert_eq!(poi.name.as_deref(), Some("Coffee Shop"));
        assert_eq!(poi.lat, Some(37.7749));
        assert_eq!(poi.lng, Some(-122.4194));
        assert_eq!(poi.r#type.as_deref(), Some("cafe"));
    }

    #[test]
    fn test_poi_request_serialization() {
        let req = PointOfInterestRequest {
            name: Some("Bike Shop".to_string()),
            description: Some("Full service".to_string()),
            latitude: Some(40.7128),
            longitude: Some(-74.0060),
            poi_type: Some("bike_shop".to_string()),
            icon: Some("bicycle".to_string()),
            address: Some("123 Main St".to_string()),
            phone: Some("555-1234".to_string()),
            website: Some("https://example.com".to_string()),
        };

        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json.get("name").unwrap(), "Bike Shop");
        assert_eq!(json.get("latitude").unwrap(), 40.7128);
        assert_eq!(json.get("poi_type").unwrap(), "bike_shop");
    }
}
