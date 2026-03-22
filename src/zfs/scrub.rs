use std::process::Command;

pub struct ScrubStatus {
    pub pool: String,
    pub state: String,
    pub progress: f64,
    pub scanned: String,
    pub total: String,
    pub issued: String,
    pub speed: String,
    pub repaired: String,
    pub eta: String,
}

pub fn start_scrub(pool: &str) -> Result<(), String> {
    let output = Command::new("sudo")
        .args(["zpool", "scrub", pool])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

pub fn get_scrub_status(pool: &str) -> Option<ScrubStatus> {
    let output = Command::new("zpool")
        .args(["status", pool])
        .output()
        .ok()?;

    let text = String::from_utf8_lossy(&output.stdout);

    if !text.contains("scrub in progress") {
        return None;
    }

    let mut progress = 0.0;
    let mut scanned = String::new();
    let mut issued = String::new();
    let mut total = String::new();
    let mut speed = String::new();
    let mut repaired = String::new();
    let mut eta = String::new();

    for line in text.lines() {
        let line = line.trim();

        if line.contains("scanned") && line.contains("issued") {
            let parts: Vec<&str> = line.split(',').collect();

            if parts.len() >= 2 {
                let scan_part = parts[0].trim();
                let issued_part = parts[1].trim();

                let scan_split: Vec<&str> = scan_part.split_whitespace().collect();
                if scan_split.len() >= 3 {
                    scanned = scan_split[0].to_string();
                    total = scan_split[2].to_string();
                }

                let issued_split: Vec<&str> = issued_part.split_whitespace().collect();
                if issued_split.len() >= 3 {
                    issued = issued_split[0].to_string();
                }

                if issued_part.contains("at") {
                    speed = issued_part
                        .split("at")
                        .nth(1)
                        .unwrap_or("")
                        .trim()
                        .to_string();
                }
            }
        }

        if line.contains("repaired") && line.contains("done") {
            let parts: Vec<&str> = line.split(',').collect();

            if parts.len() >= 3 {
                repaired = parts[0]
                    .split("repaired")
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();

                progress = parts[1]
                    .split('%')
                    .next()
                    .unwrap_or("0")
                    .trim()
                    .parse::<f64>()
                    .unwrap_or(0.0)
                    / 100.0;

                eta = parts[2]
                    .replace("to go", "")
                    .trim()
                    .to_string();
            }
        }
    }

    Some(ScrubStatus {
        pool: pool.to_string(),
        state: "scrubbing".into(),
        progress,
        scanned,
        total,
        issued,
        speed,
        repaired,
        eta,
    })
}

