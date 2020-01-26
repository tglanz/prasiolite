use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct MapConfig {
    pub display: String,
    pub file_name: String,
    pub description: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct MapsConfig {
    pub entries: Vec<MapConfig>
}