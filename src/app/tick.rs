use crate::app::{App, Nav};
use crate::zfs::status::get_pool_status;
use crate::zfs::scrub::get_scrub_status;

impl App {
    pub fn tick(&mut self) {
        self.system.update_arc();

        if matches!(self.nav, Nav::Network) {
            self.network.update_network();
        }

        if matches!(self.nav, Nav::Docker) {
            self.docker.refresh();
        }

        for pool in &mut self.zfs.pools {
            pool.update_io();
        }

        if let Some(i) = self.zfs.pool_state.selected() {
            let pool_name = self.zfs.pools[i].name.clone();
            match self.nav {
                Nav::PoolStatus => self.zfs.pool_status = get_pool_status(&pool_name),
                Nav::Scrub => self.zfs.scrub = get_scrub_status(&pool_name),
                _ => {}
            }
        }
    }
}
