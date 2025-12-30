#![deny(missing_docs)]
//! A Rust client for the RideWithGPS API.
//!
//! This crate provides a client for interacting with the RideWithGPS API v1,
//! allowing you to manage routes, trips, events, collections, and more.
//!
//! # Example
//!
//! ```rust,no_run
//! use ridewithgps_client::RideWithGpsClient;
//!
//! let client = RideWithGpsClient::new(
//!     "https://ridewithgps.com",
//!     "your-api-key",
//!     Some("your-auth-token")
//! );
//!
//! // Get current user
//! let user = client.get_current_user().unwrap();
//! println!("User: {:?}", user);
//! ```

use log::trace;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::fmt;
use url::Url;

mod auth;
mod collections;
mod events;
mod members;
mod poi;
mod routes;
mod sync;
mod trips;
mod users;

pub use auth::*;
pub use collections::*;
pub use events::*;
pub use members::*;
pub use poi::*;
pub use routes::*;
pub use sync::*;
pub use trips::*;
pub use users::*;

/// Error type for RideWithGPS API operations
#[derive(Debug)]
pub enum Error {
    /// HTTP request error
    Http(reqwest::Error),

    /// URL parsing error
    Url(url::ParseError),

    /// JSON serialization/deserialization error
    Json(serde_json::Error),

    /// API error response
    ApiError(String),

    /// Authentication error
    AuthError(String),

    /// Resource not found
    NotFound(String),

    /// Bad request
    BadRequest(String),

    /// Forbidden
    Forbidden(String),

    /// Validation error
    ValidationError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {}", e),
            Error::Url(e) => write!(f, "URL error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::ApiError(s) => write!(f, "API error: {}", s),
            Error::AuthError(s) => write!(f, "Authentication error: {}", s),
            Error::NotFound(s) => write!(f, "Resource not found: {}", s),
            Error::BadRequest(s) => write!(f, "Bad request: {}", s),
            Error::Forbidden(s) => write!(f, "Forbidden: {}", s),
            Error::ValidationError(s) => write!(f, "Validation error: {}", s),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::Url(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

/// Result type for RideWithGPS API operations
pub type Result<T> = std::result::Result<T, Error>;

/// Pagination information for list responses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pagination {
    /// Total number of records
    pub record_count: Option<u64>,

    /// Total number of pages
    pub page_count: Option<u64>,

    /// Current page size
    pub page_size: Option<u64>,

    /// URL for the next page
    pub next_page_url: Option<String>,
}

/// Common response wrapper for paginated lists
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedResponse<T> {
    /// The result items
    pub results: Vec<T>,

    /// Pagination information
    #[serde(flatten)]
    pub pagination: Pagination,
}

/// Main client for the RideWithGPS API
pub struct RideWithGpsClient {
    client: Client,
    base_url: Url,
    api_key: String,
    auth_token: Option<String>,
}

impl RideWithGpsClient {
    /// Create a new RideWithGPS API client
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for the API (e.g., "https://ridewithgps.com")
    /// * `api_key` - Your API key
    /// * `auth_token` - Optional authentication token for user-specific operations
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
    /// ```
    pub fn new(base_url: &str, api_key: &str, auth_token: Option<&str>) -> Self {
        Self {
            client: Client::new(),
            base_url: Url::parse(base_url).expect("Invalid base URL"),
            api_key: api_key.to_string(),
            auth_token: auth_token.map(|s| s.to_string()),
        }
    }

    /// Create a new client with authentication credentials
    ///
    /// This will create a client and authenticate using email and password
    /// to obtain an auth token.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for the API
    /// * `api_key` - Your API key
    /// * `email` - User email
    /// * `password` - User password
    pub fn with_credentials(
        base_url: &str,
        api_key: &str,
        email: &str,
        password: &str,
    ) -> Result<Self> {
        let mut client = Self::new(base_url, api_key, None);
        let auth_token = client.create_auth_token(email, password)?;
        client.auth_token = Some(auth_token.auth_token);
        Ok(client)
    }

    /// Set the authentication token
    pub fn set_auth_token(&mut self, token: &str) {
        self.auth_token = Some(token.to_string());
    }

    /// Get the authentication token
    pub fn auth_token(&self) -> Option<&str> {
        self.auth_token.as_deref()
    }

    /// Build headers for API requests
    fn build_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-rwgps-api-key",
            HeaderValue::from_str(&self.api_key)
                .map_err(|e| Error::AuthError(format!("Invalid API key format: {}", e)))?,
        );

        if let Some(token) = &self.auth_token {
            headers.insert(
                "x-rwgps-auth-token",
                HeaderValue::from_str(token)
                    .map_err(|e| Error::AuthError(format!("Invalid auth token format: {}", e)))?,
            );
        }

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(headers)
    }

    /// Execute a GET request
    fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T> {
        let url = self.base_url.join(path)?;
        trace!("GET {}", url);

        let headers = self.build_headers()?;
        let response = self.client.get(url).headers(headers).send()?;

        self.handle_response(response)
    }

    /// Execute a POST request
    fn post<T: for<'de> Deserialize<'de>, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let url = self.base_url.join(path)?;
        trace!("POST {}", url);

        let headers = self.build_headers()?;
        let response = self.client.post(url).headers(headers).json(body).send()?;

        self.handle_response(response)
    }

    /// Execute a PUT request
    fn put<T: for<'de> Deserialize<'de>, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let url = self.base_url.join(path)?;
        trace!("PUT {}", url);

        let headers = self.build_headers()?;
        let response = self.client.put(url).headers(headers).json(body).send()?;

        self.handle_response(response)
    }

    /// Execute a DELETE request
    fn delete(&self, path: &str) -> Result<()> {
        let url = self.base_url.join(path)?;
        trace!("DELETE {}", url);

        let headers = self.build_headers()?;
        let response = self.client.delete(url).headers(headers).send()?;

        match response.status().as_u16() {
            204 => Ok(()),
            _ => {
                let status = response.status();
                let text = response.text().unwrap_or_default();
                Err(self.error_from_status(status.as_u16(), &text))
            }
        }
    }

    /// Handle API response and convert to typed result
    fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        response: reqwest::blocking::Response,
    ) -> Result<T> {
        let status = response.status();

        match status.as_u16() {
            200 | 201 => {
                let text = response.text()?;
                serde_json::from_str(&text).map_err(Error::Json)
            }
            _ => {
                let text = response.text().unwrap_or_default();
                Err(self.error_from_status(status.as_u16(), &text))
            }
        }
    }

    /// Convert HTTP status code to Error
    fn error_from_status(&self, status: u16, body: &str) -> Error {
        match status {
            400 => Error::BadRequest(body.to_string()),
            401 => Error::AuthError(body.to_string()),
            403 => Error::Forbidden(body.to_string()),
            404 => Error::NotFound(body.to_string()),
            422 => Error::ValidationError(body.to_string()),
            _ => Error::ApiError(format!("HTTP {}: {}", status, body)),
        }
    }
}

impl fmt::Debug for RideWithGpsClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RideWithGpsClient")
            .field("base_url", &self.base_url.as_str())
            .field("api_key", &"***")
            .field("auth_token", &self.auth_token.as_ref().map(|_| "***"))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = RideWithGpsClient::new(
            "https://ridewithgps.com",
            "test-api-key",
            Some("test-token"),
        );

        assert_eq!(client.base_url.as_str(), "https://ridewithgps.com/");
        assert_eq!(client.api_key, "test-api-key");
        assert_eq!(client.auth_token.as_deref(), Some("test-token"));
    }

    #[test]
    fn test_set_auth_token() {
        let mut client = RideWithGpsClient::new("https://ridewithgps.com", "test-api-key", None);

        assert_eq!(client.auth_token(), None);

        client.set_auth_token("new-token");
        assert_eq!(client.auth_token(), Some("new-token"));
    }
}
