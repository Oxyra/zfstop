use crate::zfs::pools::{list_pools, Pool};
use crate::zfs::status::get_pool_status;
use crate::zfs::datasets::{Dataset, list_datasets};
use crate::zfs::arc::{ArcStats, get_arc_stats};
use crate::zfs::scrub::{ScrubStatus, get_scrub_status};
use crate::zfs::snapshots::{Snapshot, list_snapshots, create_snapshot};

use crate::net::{Interface, list_interfaces};

use ratatui::widgets::TableState;

pub struct App {
    pub pools: Vec<Pool>,
    pub table_state: TableState,

    pub datasets: Vec<Dataset>,
    pub dataset_state: TableState,

    pub snapshots: Vec<Snapshot>,
    pub snapshot_state: TableState,

    pub pool_status: Vec<String>,
    pub selected_pool: Option<String>,
    
    pub arc: ArcStats,
    pub arc_history: Vec<u64>,

    pub mode: Mode,
    pub view: View,

    pub scrub: Option<ScrubStatus>,

    pub input_mode: InputMode,
    pub input_buffer: String,

    pub hostname: String,
    pub uptime: String,

    pub share_state: TableState,

    pub interfaces: Vec<Interface>,
    pub interface_state: TableState,
}

pub enum Mode {
    Dashboard,
    PoolStatus,
}

pub enum View {
    DatasetDetails,
    ScrubStatus,
    Snapshots,
    Shares,
    Network,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum InputMode {
    Normal,
    CreatingSnapshot,
    RenamingDataset,
}

impl App {
    pub fn new() -> Self {

        let hostname = std::fs::read_to_string("/proc/sys/kernel/hostname")
            .unwrap_or_else(|_| "unknown".into())
            .trim()
            .to_string();

        let pools = list_pools();

        let mut table_state = TableState::default();
        table_state.select(Some(0));

        let mut dataset_state = TableState::default();
        dataset_state.select(Some(0));

        let mut snapshot_state = TableState::default();
        snapshot_state.select(Some(0));

        let mut share_state = TableState::default();
        share_state.select(Some(0));

        let mut interface_state = TableState::default();
        interface_state.select(Some(0));
        let interfaces = list_interfaces();

        let arc = get_arc_stats();

        let init = (arc.hit_ratio() * 10.0) as u64;

        Self {
            pools,
            table_state,
            datasets: Vec::new(),
            dataset_state,
            snapshots: Vec::new(),
            snapshot_state,
            selected_pool: None,
            pool_status: Vec::new(),
            arc,
            arc_history: vec![init; 120],
            mode: Mode::Dashboard,
            view: View::DatasetDetails,
            scrub: None,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            hostname,
            uptime: "00:00:00".into(),
            share_state,
            interfaces,
            interface_state,
        }
    }

    pub fn update_network(&mut self) {
        self.interfaces = list_interfaces();
    }

    pub fn update_arc(&mut self) {
        self.arc = get_arc_stats();
        let new_value = (self.arc.hit_ratio() * 10.0) as u64;

        self.arc_history.push(new_value);

        if self.arc_history.len() > 120 {
            self.arc_history.remove(0);
        }
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
        let prev = if i >= shared_count - 1 { 0 } else { i - 1 };
        self.share_state.select(Some(prev));
    }

    pub fn open_scrub(&mut self) {
        if let Some(i) = self.table_state.selected() {
            if let Some(pool) = self.pools.get(i) {

                self.scrub = get_scrub_status(&pool.name);

                self.view = View::ScrubStatus;
                self.mode = Mode::Dashboard;
            }
        }
    }


    pub fn open_selected_pool(&mut self) {

        if let Some(i) = self.table_state.selected() {
            let name = self.pools[i].name.clone();

            self.pool_status = get_pool_status(&name);
            self.selected_pool = Some(name);

            self.mode = Mode::PoolStatus;
        }
    }

    pub fn load_datasets(&mut self) {
        if let Some(i) = self.table_state.selected() {
            let pool = &self.pools[i].name;
            self.datasets = list_datasets(pool);
            self.dataset_state.select(Some(0));
        }
    }

    pub fn next_dataset(&mut self) {

        if self.datasets.is_empty() { return; }

        let i = match self.dataset_state.selected() {
            Some(i) => {
                if i >= self.datasets.len() - 1 { 0 } else { i + 1 }
            }
            None => 0
        };

        self.dataset_state.select(Some(i));
    }

    pub fn previous_dataset(&mut self) {
        if self.datasets.is_empty() { return; }

        let i = match self.dataset_state.selected() {
            Some(i) => {
                if i == 0 { self.datasets.len() - 1 } else { i - 1 }
            }
            None => 0
        };
        
        self.dataset_state.select(Some(i));
    }

    pub fn selected_dataset_name(&self) -> Option<String> {
        self.dataset_state
            .selected()
            .and_then(|i| self.datasets.get(i))
            .map(|d| d.name.clone())
    }

    pub fn selected_share(&self) -> Option<&Dataset> {
        let shares: Vec<&Dataset> = self.datasets.iter().filter(|d| d.is_shares()).collect();
        self.share_state.selected().and_then(|i| shares.get(i).copied())
    }

    pub fn load_snapshots(&mut self) {
        if let Some(i) = self.dataset_state.selected() {
            if let Some(ds) = self.datasets.get(i) {
                self.snapshots = list_snapshots(&ds.name);
                self.snapshot_state.select(Some(0));
            }
        }
    }

    pub fn open_snapshots(&mut self) {
        self.load_snapshots();
        self.view = View::Snapshots;
        self.snapshot_state.select(Some(0));
    }

    pub fn next_snapshot(&mut self) {
        if self.snapshots.is_empty() {
            return;
        }

        let i = self.snapshot_state.selected().unwrap_or(0);

        let next = if i >= self.snapshots.len() - 1 {
            self.snapshots.len() - 1
        } else {
            i + 1
        };

        self.snapshot_state.select(Some(next));
    }

    pub fn previous_snapshot(&mut self) {
        if self.snapshots.is_empty() {
            return;
        }

        let i = self.snapshot_state.selected().unwrap_or(0);

        let prev = i.saturating_sub(1);

        self.snapshot_state.select(Some(prev));
    }

    pub fn start_create_snapshot(&mut self) {
        self.input_buffer.clear();
        self.input_mode = InputMode::CreatingSnapshot;
    }

    pub fn submit_snapshot(&mut self) -> Result<(), String> {
        if let Some(dataset) = self.selected_dataset_name() {
            create_snapshot(&dataset, &self.input_buffer)?;
            self.input_mode = InputMode::Normal;
            self.input_buffer.clear();
            Ok(())
        } else {
            Err("No dataset selected".into())
        }
    }

    pub fn cancel_snapshot(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }

    pub fn start_rename_dataset(&mut self) {
        if let Some(name) = self.selected_dataset_name() {
            self.input_buffer = name;
            self.input_mode = InputMode::RenamingDataset;
        }
    }
    
    pub fn submit_rename(&mut self) -> Result<(), String> {
        if let Some(old_name) = self.selected_dataset_name() {
            let new_name = self.input_buffer.trim().to_string();

            println!("Renaming {} to {}", old_name, new_name);
            
            self.input_mode = InputMode::Normal;
            self.input_buffer.clear();
            self.load_datasets();
            Ok(())
        } else {
            Err("No dataset selected".into())
        }
    }

    pub fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => if i >= self.pools.len() - 1 { 0 } else { i + 1 },
            None => 0
        };

        self.table_state.select(Some(i));
        self.refresh_current_view_data();
    }

    pub fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => if i == 0 { self.pools.len() - 1 } else { i - 1},
            None => 0
        };

        self.table_state.select(Some(i));
        self.refresh_current_view_data();
    }

    pub fn back(&mut self) {

        match self.mode {

            Mode::PoolStatus => {
                self.mode = Mode::Dashboard;
            }

            Mode::Dashboard => {
                self.view = View::DatasetDetails;
            }
        }
    }

    fn refresh_current_view_data(&mut self) {
        if let Some(i) = self.table_state.selected() {
            if let Some(pool) = self.pools.get(i).map(|p| p.name.clone()) {

                self.datasets = list_datasets(&pool);
                self.dataset_state.select(Some(0));

                if matches!(self.mode, Mode::PoolStatus) {
                    self.pool_status = get_pool_status(&pool);
                    self.selected_pool = Some(pool.clone());
                }

                if matches!(self.view, View::ScrubStatus) {
                    self.scrub = get_scrub_status(&pool);
                }

                if matches!(self.view, View::Snapshots) {
                    self.load_snapshots();
                }

                if matches!(self.view, View::Shares) {
                    self.datasets = list_datasets(&pool);
                }
            }
        }
    }
}


