use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::refresh::RefreshToken;

/// Algorithm used to sign the keys.
static ALGORITHM: Algorithm = Algorithm::HS512;

/// Representation of the data encoded in the access tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub session: Uuid,
    pub account: Uuid,
    pub exp: usize,
    pub nbf: usize,
    pub iss: String,
    pub sub: String,
}

/// Access token representation.
#[derive(Debug)]
pub struct AccessToken {
    pub jwt: String,
    pub claims: AccessTokenClaims,
}

impl AccessToken {
    /// Encode a new access token from a refresh token.
    pub fn encode(
        refresh: &RefreshToken,
        key: &EncodingKey,
    ) -> Result<AccessToken, jsonwebtoken::errors::Error> {
        // Create the not before and expiring time of the token.
        let now = Utc::now();
        let expires = now + Duration::minutes(15);

        // Define the claims in the token.
        let claims = AccessTokenClaims {
            session: refresh.session.id,
            account: refresh.session.account,
            exp: expires.timestamp() as usize,
            nbf: now.timestamp() as usize,
            iss: "Nexium".to_string(),
            sub: "access".to_string(),
        };

        // Encode the claims.
        let jwt = encode::<AccessTokenClaims>(&Header::new(ALGORITHM), &claims, key)?;

        Ok(AccessToken { jwt, claims })
    }

    /// Decode the access token with the key.
    pub fn decode(jwt: String, key: &DecodingKey<'_>) -> Result<Self, jsonwebtoken::errors::Error> {
        // Validation setting to be used.
        let validation = Validation {
            // The access token expires in 15 minutes, but give the user 30 seconds of leeway.
            leeway: 30,
            validate_exp: true,
            validate_nbf: true,
            // Don't set the audience.
            aud: None,
            // Sanity Check if the JWT token comes from a Nexium instance.
            iss: Some("Nexium".to_string()),
            // Make sure this is a access token, not a refresh token.
            sub: Some("access".to_string()),
            // Set the algorithm.
            algorithms: vec![ALGORITHM],
        };

        // decode the token.
        let decoded = decode::<AccessTokenClaims>(jwt.as_str(), key, &validation)?;

        Ok(AccessToken {
            claims: decoded.claims,
            jwt,
        })
    }
}
