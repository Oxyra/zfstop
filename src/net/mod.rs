use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize, Debug, Clone)]
pub struct Interface {
    #[serde(rename = "ifname")]
    pub name: String,
    pub operstate: String,
    #[serde(default)]
    pub addr_info: Vec<AddrInfo>,
    #[serde(default = "default_link_type")]
    pub link_type: String,
    #[serde(rename = "mtu")]
    pub mtu: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AddrInfo {
    pub local: String,
    pub family: String,
}

pub fn list_interfaces() -> Vec<Interface> {
    let output = Command::new("ip")
        .args(["-j", "addr"])
        .output();

    match output {
        Ok(out) => serde_json::from_slice(&out.stdout).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn default_link_type() -> String {
    "unknown".to_string()
}
