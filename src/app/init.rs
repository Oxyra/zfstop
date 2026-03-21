use crate::app::{App, Nav, Focus, InputMode, NetworkSort};
use crate::app::state::{ZfsState, SystemState, NetworkState, InputState};
use crate::zfs::pools::list_pools;
use crate::zfs::arc::get_arc_stats;
use crate::net::list_interfaces;
use ratatui::widgets::TableState;

impl App {
    pub fn new() -> Self {
        let hostname = std::fs::read_to_string("/proc/sys/kernel/hostname")
            .unwrap_or_else(|_| "unknown".into())
            .trim()
            .to_string();

        let pools = list_pools();
        let interfaces = list_interfaces();
        let arc = get_arc_stats();

        let mut pool_state = TableState::default();
        pool_state.select(Some(0));

        let mut dataset_state = TableState::default();
        dataset_state.select(Some(0));

        let mut snapshot_state = TableState::default();
        snapshot_state.select(Some(0));
        
        let mut interface_state = TableState::default();
        interface_state.select(Some(0));

        let mut share_state = TableState::default();
        share_state.select(Some(0));


        let init = (arc.hit_ratio() * 10.0) as u64;

        Self {
            nav: Nav::Datasets,
            focus: Focus::Left,
            
            input: InputState {
                mode: InputMode::Normal,
                action: None,
                buffer: String::new(),
            },

            zfs: ZfsState {
                pools,
                pool_state,
                datasets: Vec::new(),
                dataset_state,
                snapshots: Vec::new(),
                snapshot_state,
                pool_status: Vec::new(),
                selected_pool: None,
                scrub: None,
                share_state,
            },

            network: NetworkState {
                interfaces,
                interface_state,
                sort: NetworkSort::Name,
            },

            system: SystemState {
                hostname,
                uptime: "00:00:00".into(),
                arc,
                arc_history: vec![init; 120],
            }
        }
    }
}
