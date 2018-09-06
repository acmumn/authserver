//! Common types used between acmumn-identity-client and acmumn-identity-server.
extern crate serde;
#[macro_use]
extern crate serde_derive;

/// The representation of an authentication token.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Token {
    /// The time, in seconds since the Unix epoch in UTC, at which the token was issued.
    pub iat: Option<i64>,

    /// The time, in seconds since the Unix epoch in UTC, at which the token expires.
    pub exp: Option<i64>,

    /// The client data in the token.
    #[serde(flatten)]
    pub data: ClientData,
}

/// The client data in the token.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "type")]
pub enum ClientData {
    /// A token representing a member.
    Member {
        /// The member's database ID.
        id: u32,

        /// The member's name.
        name: String,

        /// The member's X.500 (or another identifier).
        ///
        /// Since an X.500 is not required for membership, this value may instead be an arbitrary
        /// string of lower-case ASCII letters preceded by a `!`.
        x500: String,

        /// The member's card number, if known.
        card: Option<String>,

        /// The member's preferred email address.
        email: String,

        /// Whether the member is an administrator.
        admin: bool,

        /// Whether the member is currently paid up.
        paid: bool,
    },

    /// A token representing a service.
    Service {
        /// The service's name.
        name: String,
    },
}
