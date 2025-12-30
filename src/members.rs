//! Club member related types and methods
//!
//! Note: These endpoints are only available to organization accounts.

use crate::{PaginatedResponse, Result, RideWithGpsClient};
use serde::{Deserialize, Serialize};

/// A club member
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Member {
    /// Member ID
    pub id: u64,

    /// User ID
    pub user_id: Option<u64>,

    /// Organization ID
    pub organization_id: Option<u64>,

    /// API URL
    pub url: Option<String>,

    /// Whether the member is active
    pub active: Option<bool>,

    /// Whether the member is an admin
    pub admin: Option<bool>,

    /// Whether the member can manage routes
    pub manages_routes: Option<bool>,

    /// Whether the member can manage members
    pub manages_members: Option<bool>,

    /// Whether the member can manage billing
    pub manages_billing: Option<bool>,

    /// When the member was approved
    pub approved_at: Option<String>,

    /// Member role
    pub role: Option<String>,

    /// Member status
    pub status: Option<String>,

    /// User name
    pub name: Option<String>,

    /// User email
    pub email: Option<String>,

    /// Joined timestamp
    pub joined_at: Option<String>,

    /// Created timestamp
    pub created_at: Option<String>,

    /// Updated timestamp
    pub updated_at: Option<String>,

    /// Permissions
    pub permissions: Option<MemberPermissions>,

    /// User object
    pub user: Option<crate::User>,
}

/// Member permissions
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberPermissions {
    /// Can manage routes
    pub manage_routes: Option<bool>,

    /// Can manage events
    pub manage_events: Option<bool>,

    /// Can manage members
    pub manage_members: Option<bool>,

    /// Can view analytics
    pub view_analytics: Option<bool>,
}

/// Parameters for listing members
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListMembersParams {
    /// Filter by member name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Filter by member role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Filter by member status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Page size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// Request to update member permissions/status
#[derive(Debug, Clone, Serialize)]
pub struct UpdateMemberRequest {
    /// Member role
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Member status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<MemberPermissions>,
}

impl RideWithGpsClient {
    /// List club members
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
    /// let members = client.list_members(None).unwrap();
    /// println!("Found {} members", members.results.len());
    /// ```
    pub fn list_members(
        &self,
        params: Option<&ListMembersParams>,
    ) -> Result<PaginatedResponse<Member>> {
        let mut url = "/api/v1/members.json".to_string();

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

    /// Get a specific member by ID
    ///
    /// Note: This endpoint is only available to organization accounts.
    ///
    /// # Arguments
    ///
    /// * `id` - The member ID
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
    /// let member = client.get_member(12345).unwrap();
    /// println!("Member: {:?}", member);
    /// ```
    pub fn get_member(&self, id: u64) -> Result<Member> {
        #[derive(Deserialize)]
        struct MemberWrapper {
            member: Member,
        }

        let wrapper: MemberWrapper = self.get(&format!("/api/v1/members/{}.json", id))?;
        Ok(wrapper.member)
    }

    /// Update a member's permissions or status
    ///
    /// Note: This endpoint is only available to organization accounts.
    ///
    /// # Arguments
    ///
    /// * `id` - The member ID
    /// * `member` - The updated member data
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use ridewithgps_client::{RideWithGpsClient, UpdateMemberRequest, MemberPermissions};
    ///
    /// let client = RideWithGpsClient::new(
    ///     "https://ridewithgps.com",
    ///     "your-api-key",
    ///     Some("your-auth-token")
    /// );
    ///
    /// let member_req = UpdateMemberRequest {
    ///     role: Some("admin".to_string()),
    ///     status: Some("active".to_string()),
    ///     permissions: Some(MemberPermissions {
    ///         manage_routes: Some(true),
    ///         manage_events: Some(true),
    ///         manage_members: Some(true),
    ///         view_analytics: Some(true),
    ///     }),
    /// };
    ///
    /// let member = client.update_member(12345, &member_req).unwrap();
    /// println!("Updated member: {:?}", member);
    /// ```
    pub fn update_member(&self, id: u64, member: &UpdateMemberRequest) -> Result<Member> {
        #[derive(Deserialize)]
        struct MemberWrapper {
            member: Member,
        }

        let wrapper: MemberWrapper = self.put(&format!("/api/v1/members/{}.json", id), member)?;
        Ok(wrapper.member)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_member_deserialization() {
        let json = r#"{
            "id": 123,
            "user_id": 456,
            "name": "John Doe",
            "email": "john@example.com",
            "role": "admin",
            "status": "active",
            "permissions": {
                "manage_routes": true,
                "manage_events": true,
                "manage_members": false,
                "view_analytics": true
            }
        }"#;

        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.id, 123);
        assert_eq!(member.name.as_deref(), Some("John Doe"));
        assert_eq!(member.role.as_deref(), Some("admin"));
        assert!(member.permissions.is_some());
        let perms = member.permissions.unwrap();
        assert_eq!(perms.manage_routes, Some(true));
        assert_eq!(perms.manage_members, Some(false));
    }

    #[test]
    fn test_update_member_request_serialization() {
        let req = UpdateMemberRequest {
            role: Some("moderator".to_string()),
            status: Some("active".to_string()),
            permissions: Some(MemberPermissions {
                manage_routes: Some(true),
                manage_events: Some(false),
                manage_members: Some(false),
                view_analytics: Some(true),
            }),
        };

        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json.get("role").unwrap(), "moderator");
        assert_eq!(json.get("status").unwrap(), "active");
        assert!(json.get("permissions").is_some());
    }

    #[test]
    fn test_member_wrapper_deserialization() {
        let json = r#"{
            "member": {
                "id": 888,
                "user_id": 777,
                "name": "Wrapped Member",
                "role": "admin",
                "status": "active"
            }
        }"#;

        #[derive(Deserialize)]
        struct MemberWrapper {
            member: Member,
        }

        let wrapper: MemberWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.member.id, 888);
        assert_eq!(wrapper.member.user_id, Some(777));
        assert_eq!(wrapper.member.name.as_deref(), Some("Wrapped Member"));
        assert_eq!(wrapper.member.role.as_deref(), Some("admin"));
    }

    #[test]
    fn test_member_permissions_deserialization() {
        let json = r#"{
            "manage_routes": true,
            "manage_events": false,
            "manage_members": true,
            "view_analytics": true
        }"#;

        let perms: MemberPermissions = serde_json::from_str(json).unwrap();
        assert_eq!(perms.manage_routes, Some(true));
        assert_eq!(perms.manage_events, Some(false));
        assert_eq!(perms.manage_members, Some(true));
        assert_eq!(perms.view_analytics, Some(true));
    }

    #[test]
    fn test_member_with_user_object() {
        let json = r#"{
            "id": 999,
            "user_id": 555,
            "name": "Full Member",
            "admin": true,
            "manages_routes": true,
            "user": {
                "id": 555,
                "name": "User Name",
                "email": "user@example.com"
            }
        }"#;

        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.id, 999);
        assert_eq!(member.admin, Some(true));
        assert!(member.user.is_some());
        let user = member.user.unwrap();
        assert_eq!(user.id, 555);
        assert_eq!(user.name.as_deref(), Some("User Name"));
    }
}
