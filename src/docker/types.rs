use ratatui::widgets::TableState;
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

pub struct DockerState {
    pub containers: Vec<DockerContainer>,
    pub container_state: TableState,
}
