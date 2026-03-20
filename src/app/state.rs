use crate::zfs::pools::Pool;
use crate::zfs::datasets::Dataset;
use crate::zfs::arc::ArcStats;
use crate::zfs::scrub::ScrubStatus;
use crate::zfs::snapshots::Snapshot;
use crate::net::Interface;
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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Mode { Dashboard, PoolStatus }

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum View { DatasetDetails, ScrubStatus, Snapshots, Shares, Network }

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum InputMode { Normal, CreatingSnapshot, RenamingDataset }
