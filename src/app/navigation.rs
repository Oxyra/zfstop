use crate::app::{App, Nav};

impl App {
    pub fn next_nav(&mut self) {
        self.nav = match self.nav {
            Nav::Datasets => Nav::Snapshots,
            Nav::Snapshots => Nav::PoolStatus,
            Nav::PoolStatus => Nav::Network,
            Nav::Network => Nav::Scrub,
            Nav::Scrub => Nav::Shares,
            Nav::Shares => Nav::Datasets,
            Nav::Docker => Nav::Docker,
        };
    }

    pub fn prev_nav(&mut self) {
        self.nav = match self.nav {
            Nav::Datasets => Nav::Shares,
            Nav::Snapshots => Nav::Datasets,
            Nav::PoolStatus => Nav::Snapshots,
            Nav::Network => Nav::PoolStatus,
            Nav::Scrub => Nav::Network,
            Nav::Shares => Nav::Scrub,
            Nav::Docker => Nav::Docker,
        };
    }

    pub fn back(&mut self) {
        self.nav = Nav::Datasets;
    }
}
