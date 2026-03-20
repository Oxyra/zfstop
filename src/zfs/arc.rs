use std::fs;

#[derive(Default, Clone)]
pub struct ArcStats {
    pub size: u64,
    pub max: u64,
    pub hits: u64,
    pub misses: u64,

    pub demand_data_hits: u64,
    pub demand_metadata_hits: u64,
    pub prefetch_data_hits: u64,
    pub prefetch_metadata_hits: u64,
}

impl ArcStats {
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 { 0.0 } else { (self.hits as f64 / total as f64) * 100.0 }
    }

    pub fn breakdown_total(&self) -> u64 {
        self.demand_data_hits
        + self.demand_metadata_hits
        + self.prefetch_data_hits
        + self.prefetch_metadata_hits
    }

    pub fn breakdown(&self) -> (f64, f64, f64, f64) {
        let total = self.breakdown_total() as f64;

        if total == 0.0 {
            return (0.0, 0.0, 0.0, 0.0);
        }

        (
            self.demand_data_hits as f64 / total,
            self.demand_metadata_hits as f64 / total,
            self.prefetch_data_hits as f64 / total,
            self.prefetch_metadata_hits as f64 / total,
        )
    }
}

pub fn get_arc_stats() -> ArcStats {

    let text = fs::read_to_string("/proc/spl/kstat/zfs/arcstats").unwrap_or_default();

    let mut stats = ArcStats::default();

    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 { continue }
        let value = parts[2].parse::<u64>().unwrap_or(0);

        match parts[0] {
            "size" => stats.size = value,
            "c_max" => stats.max = value,
            "hits" => stats.hits = value,
            "misses" => stats.misses = value,
            "demand_data_hits" => stats.demand_data_hits = value,
            "demand_metadata_hits" => stats.demand_metadata_hits = value,
            "prefetch_data_hits" => stats.prefetch_data_hits = value,
            "prefetch_metadata_hits" => stats.prefetch_metadata_hits = value,
            _ => {}
        }
    }

    stats
}

