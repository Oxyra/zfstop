use std::process::Command;

pub fn get_pool_status(pool: &str) -> Vec<String> {

    let output = Command::new("zpool")
        .args(["status", pool])
        .output();

    let Ok(output) = output else {
        return vec!["Error: Could not run zpool command".to_string()];
    };

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.lines()
        .filter(|line| {
            let t = line.trim();
            !t.is_empty() && (
                t.starts_with("state:") || 
                t.starts_with("scan:") || 
                t.starts_with("config:") || 
                t.contains("ONLINE") || 
                t.contains("DEGRADED") || 
                t.contains("FAULTED") ||
                line.starts_with(' ') || line.starts_with('\t')
            )
        })
        .map(|line| line.replace('\t', "    ")) 
        .collect()
}
