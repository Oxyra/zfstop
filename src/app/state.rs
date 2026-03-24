use crate::zfs::pools::Pool;
use crate::zfs::datasets::Dataset;
use crate::zfs::arc::ArcStats;
use crate::zfs::scrub::ScrubStatus;
use crate::zfs::snapshots::Snapshot;
use crate::net::Interface;
use ratatui::widgets::TableState;
use crate::docker::types::DockerContainer;

pub struct App {
    pub nav: Nav,
    pub focus: Focus,

    pub dialog: Dialog,

    pub zfs: ZfsState,
    pub network: NetworkState,
    pub docker: DockerState,
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

pub struct DockerState {
    pub containers: Vec<DockerContainer>,
    pub container_state: TableState,
}

pub struct SystemState {
    pub hostname: String,
    pub uptime: String,
    pub arc: ArcStats,
    pub arc_history: Vec<u64>,
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
    Docker,
}

#[derive(Clone, Copy)]
pub enum NetworkSort {
    Name,
    State,
    Mtu,
}

pub enum Dialog {
    None,
    Input {
        title: String,
        label: String,
        buffer: String,
        on_confirm: Box<dyn Fn(String) -> Action + Send + Sync>,
    },
    Confirm {
        title: String,
        message: String,
        action: Action,
    },
    Error {
        title: String,
        message: String,
    },
    Info {
        title: String,
        message: String,
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    CreateSnapshot { dataset: String, name: String },
    RenameDataset { old_name: String, new_name: String },
    DestroySnapshot { name: String },
    StartScrub { pool: String },
    StopScrub { pool: String },
    ChangeNav(Nav),

    DockerStart { id: String },
    DockerStop { id: String },
    DockerRestart { id: String },
    DockerRemove { id: String },
}
