use leptos::prelude::*;
use uuid::Uuid;

use crate::api::client::{clear_token, get_token, set_token};
use crate::api::types::UserClaims;

#[derive(Debug, Clone, PartialEq)]
pub struct AuthUser {
    pub id: Uuid,
    pub role: String,
    pub token: String,
}

/// Global auth state provided via context.
#[derive(Clone, Copy)]
pub struct AuthState {
    pub user: RwSignal<Option<AuthUser>>,
}

impl AuthState {
    pub fn new() -> Self {
        let user = RwSignal::new(Self::restore_from_storage());
        Self { user }
    }

    fn restore_from_storage() -> Option<AuthUser> {
        let token = get_token()?;
        decode_claims(&token)
    }

    pub fn login(&self, token: &str) {
        set_token(token);
        if let Some(user) = decode_claims(token) {
            self.user.set(Some(user));
        }
    }

    pub fn logout(&self) {
        clear_token();
        self.user.set(None);
    }

    pub fn is_authenticated(&self) -> bool {
        self.user.get_untracked().is_some()
    }

    pub fn current_user_id(&self) -> Option<Uuid> {
        self.user.get_untracked().map(|u| u.id)
    }
}

fn decode_claims(token: &str) -> Option<AuthUser> {
    // JWT is base64url: header.payload.signature
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return None;
    }

    // Decode the payload (index 1) — base64url without padding
    let payload = parts[1];
    let padded = match payload.len() % 4 {
        2 => format!("{}==", payload),
        3 => format!("{}=", payload),
        _ => payload.to_string(),
    };

    // Use window.atob for base64 decode in browser
    let window = web_sys::window()?;
    let decoded = window
        .atob(&padded.replace('-', "+").replace('_', "/"))
        .ok()?;

    let claims: UserClaims = serde_json::from_str(&decoded).ok()?;

    // Check expiration
    let now = (chrono::Utc::now().timestamp()) as usize;
    if claims.exp < now {
        return None;
    }

    Some(AuthUser {
        id: claims.sub,
        role: claims.role,
        token: token.to_string(),
    })
}
