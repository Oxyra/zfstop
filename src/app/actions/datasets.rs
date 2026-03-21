use crate::app::state::ZfsState;
use crate::app::utils::{next_index, previous_index};
use crate::zfs::datasets::{list_datasets, Dataset};

impl ZfsState {
    pub fn load_datasets(&mut self) {
        if let Some(i) = self.pool_state.selected() {
            let pool = &self.pools[i].name;
            self.datasets = list_datasets(pool);

            if self.datasets.is_empty() {
                self.dataset_state.select(None);
            } else {
                let i = self.dataset_state.selected().unwrap_or(0);
                let i = i.min(self.datasets.len() - 1 );
                self.dataset_state.select(Some(i));
            }
        }
    }

    pub fn next_dataset(&mut self) {
        next_index(&mut self.dataset_state, self.datasets.len());
    }

    pub fn previous_dataset(&mut self) {
        previous_index(&mut self.dataset_state, self.datasets.len());
    }

    pub fn selected_dataset_name(&self) -> Option<String> {
        self.dataset_state
            .selected()
            .and_then(|i| self.datasets.get(i))
            .map(|d| d.name.clone())
    }
    
    pub fn next_share(&mut self) {
        let shared_count = self.datasets.iter().filter(|d| d.is_shares()).count();
        if shared_count == 0 { return; }
        let i = self.share_state.selected().unwrap_or(0);
        let next = if i >= shared_count - 1 { 0 } else { i + 1 };
        self.share_state.select(Some(next));
    }

    pub fn previous_share(&mut self) {
        let shared_count = self.datasets.iter().filter(|d| d.is_shares()).count();
        if shared_count == 0 { return; }
        let i = self.share_state.selected().unwrap_or(0);
        let prev = if i == 0 { shared_count - 1 } else { i - 1 };
        self.share_state.select(Some(prev));
    }

    pub fn selected_share(&self) -> Option<&Dataset> {
        let shares: Vec<&Dataset> = self.datasets.iter().filter(|d| d.is_shares()).collect();
        self.share_state.selected().and_then(|i| shares.get(i).copied())
    }

}
