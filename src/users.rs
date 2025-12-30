//! User-related types and methods

use crate::{Result, RideWithGpsClient};
use serde::{Deserialize, Serialize};

/// User information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    /// User ID
    pub id: u64,

    /// User name (full name or display name)
    pub name: Option<String>,

    /// First name
    pub first_name: Option<String>,

    /// Last name
    pub last_name: Option<String>,

    /// Display name
    pub display_name: Option<String>,

    /// User email
    pub email: Option<String>,

    /// User location
    pub location: Option<String>,

    /// User description/bio
    pub description: Option<String>,

    /// Avatar URL
    pub avatar_url: Option<String>,

    /// Whether the user has a premium account
    pub premium: Option<bool>,

    /// Account creation timestamp
    pub created_at: Option<String>,

    /// Account updated timestamp
    pub updated_at: Option<String>,
}

impl RideWithGpsClient {
    /// Get the current authenticated user's information
    ///
    /// Requires an auth token to be set.
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
    /// let user = client.get_current_user().unwrap();
    /// println!("User: {:?}", user);
    /// ```
    pub fn get_current_user(&self) -> Result<User> {
        #[derive(Deserialize)]
        struct UserWrapper {
            user: User,
        }

        let wrapper: UserWrapper = self.get("/api/v1/users/current.json")?;
        Ok(wrapper.user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_deserialization() {
        let json = r#"{
            "id": 123,
            "name": "Test User",
            "email": "test@example.com",
            "premium": true
        }"#;

        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.id, 123);
        assert_eq!(user.name.as_deref(), Some("Test User"));
        assert_eq!(user.email.as_deref(), Some("test@example.com"));
        assert_eq!(user.premium, Some(true));
    }

    #[test]
    fn test_user_wrapper_deserialization() {
        let json = r#"{
            "user": {
                "id": 456,
                "name": "Wrapped User",
                "first_name": "Wrapped",
                "last_name": "User",
                "email": "wrapped@example.com"
            }
        }"#;

        #[derive(Deserialize)]
        struct UserWrapper {
            user: User,
        }

        let wrapper: UserWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.user.id, 456);
        assert_eq!(wrapper.user.name.as_deref(), Some("Wrapped User"));
        assert_eq!(wrapper.user.first_name.as_deref(), Some("Wrapped"));
        assert_eq!(wrapper.user.last_name.as_deref(), Some("User"));
    }

    #[test]
    fn test_user_with_all_fields() {
        let json = r#"{
            "id": 789,
            "name": "Full User",
            "first_name": "Full",
            "last_name": "User",
            "display_name": "fulluser",
            "email": "full@example.com",
            "location": "San Francisco, CA",
            "description": "Avid cyclist",
            "avatar_url": "https://example.com/avatar.jpg",
            "premium": true,
            "created_at": "2020-01-01T00:00:00Z",
            "updated_at": "2025-01-01T00:00:00Z"
        }"#;

        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.id, 789);
        assert_eq!(user.display_name.as_deref(), Some("fulluser"));
        assert_eq!(user.location.as_deref(), Some("San Francisco, CA"));
        assert_eq!(user.description.as_deref(), Some("Avid cyclist"));
        assert_eq!(user.premium, Some(true));
    }
}
