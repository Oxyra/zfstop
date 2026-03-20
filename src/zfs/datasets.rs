use std::process::Command;

pub struct Dataset {
    pub name: String,
    pub used: String,
    pub avail: String,
    pub mountpoint: String,
    pub compression: (String, String),
    pub recordsize: (String, String),
    pub compressratio: String,
}

pub fn list_datasets(pool: &str) -> Vec<Dataset> {
    let output = Command::new("zfs")
        .args([
            "list",
            "-H",
            "-o",
            "name,used,avail,mountpoint,compression,source:compression,recordsize,source:recordsize,compressratio",
            "-r",
            pool
        ])
        .output()
        .expect("failed to run zfs list");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('\t').collect();

            Dataset {
                name: parts.get(0).unwrap_or(&"").to_string(),
                used: parts.get(1).unwrap_or(&"").to_string(),
                avail: parts.get(2).unwrap_or(&"").to_string(),
                mountpoint: parts.get(3).unwrap_or(&"").to_string(),
                compression: (parts.get(4).unwrap_or(&"").to_string(), parts.get(5).unwrap_or(&"").to_string()),
                recordsize: (parts.get(6).unwrap_or(&"").to_string(), parts.get(7).unwrap_or(&"").to_string()),
                compressratio: parts.get(8).unwrap_or(&"").to_string(),
            }
        })
    .collect()
}
