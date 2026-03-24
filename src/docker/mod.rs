use serde::Deserialize;
use std::process::Command;
use serde_json::from_str;
use ratatui::widgets::TableState;

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

impl DockerState {
    pub fn previous_container(&mut self) {
        crate::app::utils::previous_index(&mut self.container_state, self.containers.len());
    }

    pub fn next_container(&mut self) {
        crate::app::utils::next_index(&mut self.container_state, self.containers.len());
    }

    pub fn refresh(&mut self) {
        self.containers = list_containers();
    }
}

pub fn list_containers() -> Vec<DockerContainer> {
    let output = Command::new("docker")
        .args(["ps", "--format", "{{json .}}"])
        .output()
        .expect("docker ps failed");

    let text = String::from_utf8_lossy(&output.stdout);
    let json_text = format!("[{}]", text.lines().collect::<Vec<_>>().join(","));

    from_str(&json_text).unwrap_or_default()
}
