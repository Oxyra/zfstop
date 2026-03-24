use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DockerContainer {
    pub ID: String,
    pub Image: String,
    pub Command: String,
    pub CreatedAt: String,
    pub Status: String,
    pub Ports: String,
    #[serde(rename = "Names")]
    pub Name: String,
}
