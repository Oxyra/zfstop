use crate::app::state::ZfsState;
use crate::zfs::status::get_pool_status;
use crate::zfs::scrub::get_scrub_status;
use crate::app::Nav;

impl ZfsState {
    pub fn on_pool_changed(&mut self, nav: Nav) {
        self.load_datasets();

        match nav {
            Nav::Snapshots => self.load_snapshots(),
            Nav::PoolStatus => {
                if let Some(i) = self.pool_state.selected() {
                    let name = self.pools[i].name.clone();
                    self.pool_status = crate::zfs::status::get_pool_status(&name);
                    self.selected_pool = Some(name);
                }
            }
            Nav::Scrub => {
                if let Some(i) = self.pool_state.selected() {
                    let name = self.pools[i].name.clone();
                    self.scrub = crate::zfs::scrub::get_scrub_status(&name);
                }
            }
            _ => {}
        }
    }


    pub fn next_pool(&mut self) {
        let i = match self.pool_state.selected() {
            Some(i) => if i >= self.pools.len() - 1 { 0 } else { i + 1 },
            None => 0
        };

        self.pool_state.select(Some(i));
    }

    pub fn previous_pool(&mut self) {
        let i = match self.pool_state.selected() {
            Some(i) => if i == 0 { self.pools.len() - 1 } else { i - 1 },
            None => 0
        };

        self.pool_state.select(Some(i));
    }

    pub fn open_selected_pool(&mut self) {
        if let Some(i) = self.pool_state.selected() {
            let name = self.pools[i].name.clone();

            self.pool_status = get_pool_status(&name);
            self.selected_pool = Some(name);
        }
    }

    pub fn open_scrub(&mut self) {
        if let Some(i) = self.pool_state.selected() {
            if let Some(pool) = self.pools.get(i) {
                self.scrub = get_scrub_status(&pool.name);
            }
        }
    }
}
