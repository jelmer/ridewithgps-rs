//! Authentication-related types and methods

use crate::{Result, RideWithGpsClient, User};
use serde::{Deserialize, Serialize};

/// Request to create an authentication token
#[derive(Debug, Clone, Serialize)]
pub struct CreateAuthTokenRequest {
    /// User email
    pub email: String,

    /// User password
    pub password: String,
}

/// Response containing an authentication token
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthToken {
    /// The authentication token
    pub auth_token: String,

    /// User ID
    pub user_id: Option<u64>,

    /// User object (included in the response)
    pub user: Option<User>,
}

impl RideWithGpsClient {
    /// Create an authentication token using email and password
    ///
    /// # Arguments
    ///
    /// * `email` - User email address
    /// * `password` - User password
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
    /// let auth = client.create_auth_token("user@example.com", "password").unwrap();
    /// println!("Auth token: {}", auth.auth_token);
    /// ```
    pub fn create_auth_token(&self, email: &str, password: &str) -> Result<AuthToken> {
        let request = CreateAuthTokenRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        self.post("/api/v1/auth_tokens", &request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_token_request_serialization() {
        let request = CreateAuthTokenRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test@example.com"));
        assert!(json.contains("password123"));
    }
}
