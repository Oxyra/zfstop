use crate::app::state::ZfsState;
use crate::zfs::snapshots::{list_snapshots};

impl ZfsState {
    pub fn load_snapshots(&mut self) {
        if let Some(i) = self.dataset_state.selected() {
            if let Some(ds) = self.datasets.get(i) {
                self.snapshots = list_snapshots(&ds.name);

                let len = self.snapshots.len();
                let selected = self.snapshot_state.selected().unwrap_or(0);

                if len == 0 {
                    self.snapshot_state.select(None);
                } else if selected >= len {
                    self.snapshot_state.select(Some(len - 1));
                }
            }
        }
    }

    pub fn open_snapshots(&mut self) {
        self.load_snapshots();
    }

    pub fn next_snapshot(&mut self) {
        if self.snapshots.is_empty() { return; }

        let i = self.snapshot_state.selected().unwrap_or(0);
        let next = ( i + 1 ).min(self.snapshots.len() - 1 );

        self.snapshot_state.select(Some(next));
    }

    pub fn previous_snapshot(&mut self) {
        if self.snapshots.is_empty() { return; }

        let i = self.snapshot_state.selected().unwrap_or(0);
        self.snapshot_state.select(Some(i.saturating_sub(1)));
    }
}
