use crate::zfs::pools::Pool;
use crate::zfs::datasets::Dataset;
use crate::zfs::arc::ArcStats;
use crate::zfs::scrub::ScrubStatus;
use crate::zfs::snapshots::Snapshot;
use crate::net::Interface;
use ratatui::widgets::TableState;

pub struct App {
    pub nav: Nav,
    pub focus: Focus,

    pub input: InputState,
    pub zfs: ZfsState,
    pub network: NetworkState,
    pub system: SystemState,
}

pub struct InputState {
    pub mode: InputMode,
    pub action: Option<InputAction>,
    pub buffer: String,
}

pub struct ZfsState {
    pub pools: Vec<Pool>,
    pub pool_state: TableState,

    pub datasets: Vec<Dataset>,
    pub dataset_state: TableState,

    pub snapshots: Vec<Snapshot>,
    pub snapshot_state: TableState,

    pub pool_status: Vec<String>,
    pub selected_pool: Option<String>,
    pub scrub: Option<ScrubStatus>,

    pub share_state: TableState,
}

pub struct NetworkState {
    pub interfaces: Vec<Interface>,
    pub interface_state: TableState,
    pub sort: NetworkSort,
}

pub struct SystemState {
    pub hostname: String,
    pub uptime: String,
    pub arc: ArcStats,
    pub arc_history: Vec<u64>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum InputMode {
    Normal,
    Editing
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputAction {
    CreatingSnapshot,
    RenamingDataset
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Focus { Left, Right }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Nav {
    Datasets,
    PoolStatus,
    Snapshots,
    Scrub,
    Shares,
    Network,
}

#[derive(Clone, Copy)]
pub enum NetworkSort {
    Name,
    State,
    Mtu,
}

