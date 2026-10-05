//! Data-transfer types shared by the KumoRust client and kumo-server.

use serde::{Deserialize, Serialize};

/// Stable characteristics used to identify an executable without sending the
/// executable itself to the server.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct ExecutableFingerprint {
    pub sha256: String,
    pub size: u64,
    pub file_name: String,
}

/// User-visible metadata for a game. The server owns the canonical values;
/// the client keeps a copy for displaying games while offline.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct GameMetadata {
    pub title: String,
    #[serde(default)]
    pub rj_code: Option<String>,
    #[serde(default)]
    pub maker: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub source_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameLookupRequest {
    pub fingerprint: ExecutableFingerprint,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameLookupResponse {
    pub game: Option<GameMetadata>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateGameRequest {
    pub fingerprint: ExecutableFingerprint,
    pub metadata: GameMetadata,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DlsiteLookupRequest {
    pub rj_code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DlsiteLookupResponse {
    pub metadata: GameMetadata,
}
