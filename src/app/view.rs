use crate::app::state::{ZfsState, NetworkState};

pub enum ActiveView<'a> {
    Datasets(&'a mut ZfsState),
    Snapshots(&'a mut ZfsState),
    Network(&'a mut NetworkState),
    Shares(&'a mut ZfsState),
    PoolStatus,
    Scrub,
}

impl<'a> ActiveView<'a> {
    pub fn handle_up(self) {
        match self {
            ActiveView::Datasets(zfs) => zfs.previous_dataset(),
            ActiveView::Snapshots(zfs) => zfs.previous_snapshot(),
            ActiveView::Network(net) => net.previous_interface(),
            ActiveView::Shares(zfs) => zfs.previous_share(),
            _ => {}
        }
    }

    pub fn handle_down(self) {
        match self {
            ActiveView::Datasets(zfs) => zfs.next_dataset(),
            ActiveView::Snapshots(zfs) => zfs.next_snapshot(),
            ActiveView::Network(net) => net.next_interface(),
            ActiveView::Shares(zfs) => zfs.next_share(),
            _ => {}
        }
    }
}

