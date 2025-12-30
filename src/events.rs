//! Event-related types and methods

use crate::{PaginatedResponse, Photo, Result, RideWithGpsClient, Visibility};
use serde::{Deserialize, Serialize};

/// Event organizer information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Organizer {
    /// Organizer ID
    pub id: Option<u64>,

    /// Organizer name
    pub name: Option<String>,

    /// Created timestamp
    pub created_at: Option<String>,

    /// Updated timestamp
    pub updated_at: Option<String>,
}

/// An event
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    /// Event ID
    pub id: u64,

    /// Event name
    pub name: Option<String>,

    /// Event description
    pub description: Option<String>,

    /// Event location
    pub location: Option<String>,

    /// Latitude
    pub lat: Option<f64>,

    /// Longitude
    pub lng: Option<f64>,

    /// Event visibility
    pub visibility: Option<Visibility>,

    /// API URL
    pub url: Option<String>,

    /// HTML/web URL
    pub html_url: Option<String>,

    /// Time zone
    pub time_zone: Option<String>,

    /// Start date
    pub start_date: Option<String>,

    /// Start time (e.g., "09:00")
    pub start_time: Option<String>,

    /// End date
    pub end_date: Option<String>,

    /// End time (e.g., "17:00")
    pub end_time: Option<String>,

    /// Whether it's an all-day event
    pub all_day: Option<bool>,

    /// Event start date/time (combined)
    pub starts_at: Option<String>,

    /// Event end date/time (combined)
    pub ends_at: Option<String>,

    /// Registration opens at
    pub registration_opens_at: Option<String>,

    /// Registration closes at
    pub registration_closes_at: Option<String>,

    /// User ID of the event owner
    pub user_id: Option<u64>,

    /// Created timestamp
    pub created_at: Option<String>,

    /// Updated timestamp
    pub updated_at: Option<String>,

    /// Event URL slug
    pub slug: Option<String>,

    /// Logo URL
    pub logo_url: Option<String>,

    /// Banner URL
    pub banner_url: Option<String>,

    /// Whether registration is required
    pub registration_required: Option<bool>,

    /// Maximum attendees
    pub max_attendees: Option<u32>,

    /// Current number of attendees
    pub attendee_count: Option<u32>,

    /// Event organizers (included when fetching a specific event)
    pub organizers: Option<Vec<Organizer>>,

    /// Photos (included when fetching a specific event)
    pub photos: Option<Vec<Photo>>,
}

/// Parameters for listing events
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListEventsParams {
    /// Filter by event name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Filter by visibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// Request to create or update an event
#[derive(Debug, Clone, Serialize)]
pub struct EventRequest {
    /// Event name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Event description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Event location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Event visibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,

    /// Event start date/time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,

    /// Event end date/time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,

    /// Registration opens at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_opens_at: Option<String>,

    /// Registration closes at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_closes_at: Option<String>,

    /// Whether registration is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_required: Option<bool>,

    /// Maximum attendees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attendees: Option<u32>,
}

impl RideWithGpsClient {
    /// List events
    ///
    /// # Arguments
    ///
    /// * `params` - Optional parameters for filtering and pagination
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ridewithgps_client::{RideWithGpsClient, ListEventsParams};
    ///
    /// let client = RideWithGpsClient::new(
    ///     "https://ridewithgps.com",
    ///     "your-api-key",
    ///     Some("your-auth-token")
    /// );
    ///
    /// let events = client.list_events(None).unwrap();
    /// println!("Found {} events", events.results.len());
    /// ```
    pub fn list_events(
        &self,
        params: Option<&ListEventsParams>,
    ) -> Result<PaginatedResponse<Event>> {
        let mut url = "/api/v1/events.json".to_string();

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

    /// Create a new event
    ///
    /// # Arguments
    ///
    /// * `event` - The event data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ridewithgps_client::{RideWithGpsClient, EventRequest, Visibility};
    ///
    /// let client = RideWithGpsClient::new(
    ///     "https://ridewithgps.com",
    ///     "your-api-key",
    ///     Some("your-auth-token")
    /// );
    ///
    /// let event_req = EventRequest {
    ///     name: Some("My Event".to_string()),
    ///     description: Some("A great ride".to_string()),
    ///     location: Some("San Francisco, CA".to_string()),
    ///     visibility: Some(Visibility::Public),
    ///     starts_at: Some("2025-06-01T09:00:00".to_string()),
    ///     ends_at: Some("2025-06-01T17:00:00".to_string()),
    ///     registration_opens_at: None,
    ///     registration_closes_at: None,
    ///     registration_required: Some(false),
    ///     max_attendees: None,
    /// };
    ///
    /// let event = client.create_event(&event_req).unwrap();
    /// println!("Created event: {}", event.id);
    /// ```
    pub fn create_event(&self, event: &EventRequest) -> Result<Event> {
        #[derive(Deserialize)]
        struct EventWrapper {
            event: Event,
        }

        let wrapper: EventWrapper = self.post("/api/v1/events.json", event)?;
        Ok(wrapper.event)
    }

    /// Get a specific event by ID
    ///
    /// # Arguments
    ///
    /// * `id` - The event ID
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
    /// let event = client.get_event(12345).unwrap();
    /// println!("Event: {:?}", event);
    /// ```
    pub fn get_event(&self, id: u64) -> Result<Event> {
        #[derive(Deserialize)]
        struct EventWrapper {
            event: Event,
        }

        let wrapper: EventWrapper = self.get(&format!("/api/v1/events/{}.json", id))?;
        Ok(wrapper.event)
    }

    /// Update an event
    ///
    /// # Arguments
    ///
    /// * `id` - The event ID
    /// * `event` - The updated event data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ridewithgps_client::{RideWithGpsClient, EventRequest};
    ///
    /// let client = RideWithGpsClient::new(
    ///     "https://ridewithgps.com",
    ///     "your-api-key",
    ///     Some("your-auth-token")
    /// );
    ///
    /// let event_req = EventRequest {
    ///     name: Some("Updated Event Name".to_string()),
    ///     description: None,
    ///     location: None,
    ///     visibility: None,
    ///     starts_at: None,
    ///     ends_at: None,
    ///     registration_opens_at: None,
    ///     registration_closes_at: None,
    ///     registration_required: None,
    ///     max_attendees: None,
    /// };
    ///
    /// let event = client.update_event(12345, &event_req).unwrap();
    /// println!("Updated event: {:?}", event);
    /// ```
    pub fn update_event(&self, id: u64, event: &EventRequest) -> Result<Event> {
        #[derive(Deserialize)]
        struct EventWrapper {
            event: Event,
        }

        let wrapper: EventWrapper = self.put(&format!("/api/v1/events/{}.json", id), event)?;
        Ok(wrapper.event)
    }

    /// Delete an event
    ///
    /// # Arguments
    ///
    /// * `id` - The event ID
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
    /// client.delete_event(12345).unwrap();
    /// ```
    pub fn delete_event(&self, id: u64) -> Result<()> {
        self.delete(&format!("/api/v1/events/{}.json", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_deserialization() {
        let json = r#"{
            "id": 789,
            "name": "Test Event",
            "location": "Portland, OR",
            "visibility": "public",
            "starts_at": "2025-06-01T09:00:00",
            "attendee_count": 25
        }"#;

        let event: Event = serde_json::from_str(json).unwrap();
        assert_eq!(event.id, 789);
        assert_eq!(event.name.as_deref(), Some("Test Event"));
        assert_eq!(event.location.as_deref(), Some("Portland, OR"));
        assert_eq!(event.visibility, Some(Visibility::Public));
        assert_eq!(event.attendee_count, Some(25));
    }

    #[test]
    fn test_event_request_serialization() {
        let req = EventRequest {
            name: Some("My Event".to_string()),
            description: Some("Fun ride".to_string()),
            location: None,
            visibility: Some(Visibility::Public),
            starts_at: None,
            ends_at: None,
            registration_opens_at: None,
            registration_closes_at: None,
            registration_required: Some(true),
            max_attendees: Some(100),
        };

        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json.get("name").unwrap(), "My Event");
        assert_eq!(json.get("visibility").unwrap(), "public");
        assert_eq!(json.get("registration_required").unwrap(), true);
        assert_eq!(json.get("max_attendees").unwrap(), 100);
    }
}
