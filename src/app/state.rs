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

    pub dialog: Dialog,

    pub zfs: ZfsState,
    pub network: NetworkState,
    pub system: SystemState,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputAction {
    RenamingDataset,
    CreatingSnapshot,
    DestroyingSnapshot
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

#[derive(Clone)]
pub enum Dialog {
    None,

    Input {
        title: String,
        label: String,
        buffer: String,
        action: InputAction,
    },

    Confirm {
        title: String,
        message: String,
        action: ConfirmAction,
    },
}

#[derive(Clone)]
pub enum ConfirmAction {
    DestroySnapshot(String),
}
