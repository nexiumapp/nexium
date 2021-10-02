use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::database;

use super::Session;

/// Algorithm used to sign the keys.
static ALGORITHM: Algorithm = Algorithm::HS512;

/// Representation of the data encoded in the refresh tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub session: Uuid,
    pub account: Uuid,
    pub iss: String,
    pub sub: String,
}

/// Refresh token representation.
#[derive(Debug)]
pub struct RefreshToken {
    pub jwt: String,
    pub claims: RefreshTokenClaims,
    pub session: Session,
}

impl RefreshToken {
    /// Encode a session in a JWT token.
    pub fn encode(
        session: Session,
        key: &EncodingKey,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        // The claims to encode in the token.
        let claims = RefreshTokenClaims {
            session: session.id,
            account: session.account,
            iss: "Nexium".to_string(),
            sub: "refresh".to_string(),
        };

        // Encode the token.
        let jwt = encode::<RefreshTokenClaims>(&Header::new(ALGORITHM), &claims, key)?;

        Ok(RefreshToken {
            jwt,
            claims,
            session,
        })
    }

    /// Decode an JWT to get a refresh token.
    /// This also validates it against the database.
    pub async fn decode(
        conn: &mut PgConnection,
        jwt: String,
        key: &DecodingKey<'_>,
    ) -> Result<Self, DecodeError> {
        // Validation settings to be used.
        let validation = Validation {
            // The refresh token does not expire, no need to check it.
            leeway: 0,
            validate_exp: false,
            validate_nbf: false,
            // Don't set the audience.
            aud: None,
            // Sanity Check if the JWT token comes from a Nexium instance.
            iss: Some("Nexium".to_string()),
            // Make sure this is a refresh token, not a session token.
            sub: Some("refresh".to_string()),
            // Set the algorithm.
            algorithms: vec![ALGORITHM],
        };

        // Decode the JWT token.
        let decoded = decode::<RefreshTokenClaims>(jwt.as_str(), key, &validation)
            .map_err(|_| DecodeError::DecodeError)?;

        // Get the session from the database.
        let mut session = match database::session::get_by_id(conn, &decoded.claims.session)
            .await
            .map_err(DecodeError::DatabaseError)?
        {
            Some(session) => session,
            None => return Err(DecodeError::SessionUnknown),
        };

        // Update the last seen time to now.
        session.update_seen(conn).await?;

        Ok(RefreshToken {
            claims: decoded.claims,
            jwt,
            session,
        })
    }
}

/// Possible errors when decoding the refresh token.
#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("The session was unknown.")]
    SessionUnknown,
    #[error("Token decoding failed.")]
    DecodeError,
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}
