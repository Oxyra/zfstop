use std::process::Command;
use std::fs;
use std::time::Instant;

pub struct PoolIO {
    pub read_bytes: u64,
    pub write_bytes: u64,
}

pub struct Pool {
    pub name: String,
    pub health: String,
    pub size: u64,
    pub alloc: u64,

    pub read_bps: u64,
    pub write_bps: u64,

    pub last_read: u64,
    pub last_write: u64,

    pub read_history: Vec<u64>,
    pub write_history: Vec<u64>,

    pub last_update: Instant,
}

impl Pool {
    pub fn capacity_percent(&self) -> f64 {
        if self.size == 0 {
            0.0
        } else {
            (self.alloc as f64 / self.size as f64) * 100.0
        }
    }

    pub fn update_io(&mut self) {
        let counters = read_pool_counters(&self.name);
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f64();

        if dt < 0.1 { return; }

        let read_delta = counters.read_bytes.saturating_sub(self.last_read);
        let write_delta = counters.write_bytes.saturating_sub(self.last_write);

        self.read_bps = (read_delta as f64 / dt) as u64;
        self.write_bps = (write_delta as f64 / dt) as u64;

        let prev_r = *self.read_history.last().unwrap_or(&0);
        let prev_w = *self.write_history.last().unwrap_or(&0);

        let smooth_r = ((prev_r as f64 * 0.1) + (self.read_bps as f64 * 0.9)) as u64;
        let smooth_w = ((prev_w as f64 * 0.1) + (self.write_bps as f64 * 0.9)) as u64;

        self.read_history.push(smooth_r);
        self.write_history.push(smooth_w);

        if self.read_history.len() > 60 { self.read_history.remove(0); }
        if self.write_history.len() > 60 { self.write_history.remove(0); }

        self.last_read = counters.read_bytes;
        self.last_write = counters.write_bytes;
        self.last_update = now;
    }
}

pub fn list_pools() -> Vec<Pool> {
    let output = Command::new("zpool")
        .args(["list", "-H", "-p", "-o", "name,health,size,alloc"])
        .output()
        .expect("failed to run zpool");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            Pool {
                name: parts[0].to_string(),
                health: parts[1].to_string(),
                size: parts[2].parse().unwrap_or(0),
                alloc: parts[3].parse().unwrap_or(0),

                read_bps: 0,
                write_bps: 0,

                last_read: 0,
                last_write: 0,

                read_history: Vec::new(),
                write_history: Vec::new(),

                last_update: Instant::now(),
            }
        })
        .collect()
}

fn read_pool_counters(pool: &str) -> PoolIO {
    let path = format!("/proc/spl/kstat/zfs/{}/iostats", pool);

    let text = fs::read_to_string(path).unwrap_or_default();

    let mut arc_read = 0;
    let mut arc_write = 0;
    let mut direct_read = 0;
    let mut direct_write = 0;

    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 3 {
            continue;
        }

        let value = parts[2].parse::<u64>().unwrap_or(0);

        match parts[0] {
            "arc_read_bytes" => arc_read = value,
            "arc_write_bytes" => arc_write = value,
            "direct_read_bytes" => direct_read = value,
            "direct_write_bytes" => direct_write = value,
            _ => {}
        }
    }

    PoolIO {
        read_bytes: arc_read + direct_read,
        write_bytes: arc_write + direct_write,
    }
}

