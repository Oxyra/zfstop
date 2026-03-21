use crate::app::state::SystemState;
use crate::zfs::arc::get_arc_stats;

impl SystemState {
    pub fn update_arc(&mut self) {
        self.arc = get_arc_stats();
        let new_value = (self.arc.hit_ratio() * 10.0) as u64;

        self.arc_history.push(new_value);

        if self.arc_history.len() > 120 {
            self.arc_history.remove(0);
        }
    }

    pub fn update_uptime(&mut self) {
        if let Ok(uptime_str) = std::fs::read_to_string("/proc/uptime") {
            if let Some(seconds_str) = uptime_str.split_whitespace().next() {
                if let Ok(seconds) = seconds_str.parse::<f32>() {
                    let s = seconds as u64;
                    let days = s / 86400;
                    let hours = (s % 86400) / 3600;
                    let minutes = (s % 3600) / 60;

                    self.uptime = if days > 0 {
                        format!("{}d {:02}h {:02}m", days, hours, minutes)
                    } else {
                        format!("{:02}h {:02}m", hours, minutes)
                    };
                }
            }
        }
    }
}
