use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RepositoryInfo {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub branch: String,
    #[serde(rename = "gameVersion")]
    pub game_version: String,
    #[serde(rename = "gameVersions")]
    pub game_versions: Vec<String>,
    pub server: String,
    #[serde(rename = "serverOptions")]
    pub server_options: Vec<String>,
    #[serde(rename = "hasWarning")]
    pub has_warning: bool,
}
