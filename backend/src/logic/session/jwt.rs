use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use thiserror::Error;
use uuid::Uuid;

use crate::database;

use super::Session;

/// Algorithm used to sign the keys.
static ALGORITHM: Algorithm = Algorithm::HS512;

/// Representation of the data encoded in the session tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtTokenClaims {
    pub session: Uuid,
    pub account: Uuid,
    pub exp: usize,
    pub nbf: usize,
    pub iss: String,
    pub sub: String,
}

/// session token representation.
#[derive(Debug)]
pub struct JwtToken {
    pub jwt: String,
    pub claims: JwtTokenClaims,
}

impl JwtToken {
    /// Renew an existing token.
    pub async fn renew(
        &self,
        conn: &mut PgConnection,
        key: &EncodingKey,
    ) -> Result<JwtToken, RenewError> {
        // Get the session from the database.
        let mut session = match database::session::get_by_id(conn, &self.claims.session)
            .await
            .map_err(RenewError::DatabaseError)?
        {
            Some(session) => session,
            None => return Err(RenewError::SessionUnknown),
        };

        // Update the last seen time to now.
        session.update_seen(conn).await?;

        // Encode the token.
        JwtToken::encode(&session, key).map_err(RenewError::TokenError)
    }

    /// Encode a session to an JWT token.
    pub fn encode(
        session: &Session,
        key: &EncodingKey,
    ) -> Result<JwtToken, jsonwebtoken::errors::Error> {
        // Create the not before and expiring time of the token.
        let now = Utc::now();
        let expires = now + Duration::minutes(7);

        // Define the claims in the token.
        let claims = JwtTokenClaims {
            session: session.id,
            account: session.account,
            exp: expires.timestamp() as usize,
            nbf: now.timestamp() as usize,
            iss: "Nexium".to_string(),
            sub: "session".to_string(),
        };

        // Encode the claims.
        let jwt = encode::<JwtTokenClaims>(&Header::new(ALGORITHM), &claims, key)?;

        Ok(JwtToken { jwt, claims })
    }

    /// Decode a JWT with the key.
    pub fn decode(jwt: String, key: &DecodingKey<'_>) -> Result<Self, jsonwebtoken::errors::Error> {
        // Validation setting to be used.
        let validation = Validation {
            // The session token expires in 7 minutes, but give the user 15 seconds of leeway.
            leeway: 15,
            validate_exp: true,
            validate_nbf: true,
            // Don't set the audience.
            aud: None,
            // Sanity Check if the JWT token comes from a Nexium instance.
            iss: Some("Nexium".to_string()),
            // Make sure this is a session token.
            sub: Some("session".to_string()),
            // Set the algorithm.
            algorithms: vec![ALGORITHM],
        };

        // decode the token.
        let decoded = decode::<JwtTokenClaims>(jwt.as_str(), key, &validation)?;

        Ok(JwtToken {
            claims: decoded.claims,
            jwt,
        })
    }

    /// Decode a session with the key.
    /// This does not validate the expiration time.
    pub fn decode_unchecked(
        jwt: String,
        key: &DecodingKey<'_>,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        // Validation setting to be used.
        let validation = Validation {
            // The session token expires in 7 minutes, but give the user 15 seconds of leeway.
            leeway: 15,
            validate_exp: false,
            validate_nbf: true,
            // Don't set the audience.
            aud: None,
            // Sanity Check if the JWT token comes from a Nexium instance.
            iss: Some("Nexium".to_string()),
            // Make sure this is a session token.
            sub: Some("session".to_string()),
            // Set the algorithm.
            algorithms: vec![ALGORITHM],
        };

        // decode the token.
        let decoded = decode::<JwtTokenClaims>(jwt.as_str(), key, &validation)?;

        Ok(JwtToken {
            claims: decoded.claims,
            jwt,
        })
    }
}

/// Possible errors when renewing an token.
#[derive(Debug, Error)]
pub enum RenewError {
    #[error("Session was unknown.")]
    SessionUnknown,
    #[error("Token encoding failed.")]
    TokenError(#[from] jsonwebtoken::errors::Error),
    #[error("An internal database error occured.")]
    DatabaseError(#[from] sqlx::Error),
}
