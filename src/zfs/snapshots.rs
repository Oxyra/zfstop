use std::process::Command;

#[derive(Clone)]
pub struct Snapshot {
    pub name: String,
    pub used: String,
    pub created: u64,
}

pub fn list_snapshots(dataset: &str) -> Vec<Snapshot> {
    let output = Command::new("zfs")
        .args([
            "list",
            "-t", "snapshot",
            "-o", "name,used,creation",
            "-H", "-p",
            "-s", "creation",
            "-r", dataset,
        ])
        .output()
        .expect("failed to run zfs list snapshots");

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut snapshots: Vec<Snapshot> = stdout
        .lines()
        .filter(|line| line.starts_with(dataset))
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            Snapshot {
                name: parts.get(0).unwrap_or(&"").to_string(),
                used: parts.get(1).unwrap_or(&"").to_string(),
                created: parts
                    .get(2)
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(0),
            }
        })
    .collect();

    snapshots.sort_by(|a, b| b.created.cmp(&a.created));

    snapshots
}

pub fn create_snapshot(dataset: &str, name: &str) -> Result<(), String> {
    let output = std::process::Command::new("sudo")
        .args(["/usr/sbin/zfs", "snapshot", &format!("{}@{}", dataset, name)])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(err.trim().to_string())
    }
}

pub fn destroy_snapshot(name: &str) -> Result<(), String> {
    let output = Command::new("sudo")
        .args(["/usr/sbin/zfs", "destroy", name])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}
