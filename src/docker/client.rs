use std::process::Command;
use crate::docker::types::{DockerContainer};
use crate::app::state::{DockerState};
use serde_json::from_str;

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
        .args(["ps", "-a", "--format", "{{json .}}"])
        .output()
        .expect("docker ps failed");

    let text = String::from_utf8_lossy(&output.stdout);
    let json_text = format!("[{}]", text.lines().collect::<Vec<_>>().join(","));

    from_str(&json_text).unwrap_or_default()
}

pub fn docker_start(id: &str) -> Result<(), String> {
    Command::new("docker")
        .args(["start", id])
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn docker_stop(id: &str) -> Result<(), String> {
    Command::new("docker")
        .args(["stop", id])
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn docker_restart(id: &str) -> Result<(), String> {
    Command::new("docker")
        .args(["restart", id])
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn docker_remove(id: &str) -> Result<(), String> {
    Command::new("docker")
        .args(["rm", "-f", id])
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn docker_cmd(args: &[&str]) -> Command {
    let mut cmd = Command::new("docker");
    cmd.args(args);
    cmd
}

