//! [POST /_matrix/client/r0/user_directory/search](https://matrix.org/docs/spec/client_server/r0.6.1#post-matrix-client-r0-user-directory-search)

use js_int::{uint, UInt};
use ruma_api::{ruma_api, Outgoing};
use ruma_identifiers::UserId;
use serde::Serialize;

ruma_api! {
    metadata: {
        description: "Performs a search for users.",
        method: POST,
        name: "search_users",
        path: "/_matrix/client/r0/user_directory/search",
        rate_limited: true,
        requires_authentication: true,
    }

    request: {
        /// The term to search for.
        pub search_term: &'a str,

        /// The maximum number of results to return.
        ///
        /// Defaults to 10.
        #[serde(default = "default_limit", skip_serializing_if = "is_default_limit")]
        pub limit: UInt,
    }

    response: {
        /// Ordered by rank and then whether or not profile info is available.
        pub results: &'a [User<'a>],

        /// Indicates if the result list has been truncated by the limit.
        pub limited: bool,
    }

    error: crate::Error
}

fn default_limit() -> UInt {
    uint!(10)
}

fn is_default_limit(limit: &UInt) -> bool {
    limit == &default_limit()
}

/// User data as result of a search.
#[derive(Clone, Debug, Outgoing, Serialize)]
pub struct User<'a> {
    /// The user's matrix user ID.
    pub user_id: &'a UserId,

    /// The display name of the user, if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<&'a str>,

    /// The avatar url, as an MXC, if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<&'a str>,
}
