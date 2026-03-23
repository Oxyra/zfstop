pub mod state;
pub mod view;
pub mod init;
pub mod tick;
pub mod handlers;
pub mod navigation;
pub mod utils;
pub mod system;

pub mod actions;

pub use self::state::{
    App, 
    Nav, 
    Focus, 
    NetworkSort,
    Action,
    Dialog,
};

use crate::app::view::ActiveView;
use crate::zfs::snapshots::{create_snapshot, destroy_snapshot};
use crate::zfs::scrub::{start_scrub, stop_scrub, get_scrub_status};
use crossterm::event::KeyCode;

impl App {
    fn active_view_mut(&mut self) -> ActiveView<'_> {
        match self.nav {
            Nav::Datasets => ActiveView::Datasets(&mut self.zfs),
            Nav::Snapshots => ActiveView::Snapshots(&mut self.zfs),
            Nav::Network => ActiveView::Network(&mut self.network),
            Nav::Shares => ActiveView::Shares(&mut self.zfs),
            Nav::PoolStatus => ActiveView::PoolStatus,
            Nav::Scrub => ActiveView::Scrub,
        }
    }

    pub fn handle_up(&mut self) {
        match self.focus {
            Focus::Left => {
                self.zfs.previous_pool();
                self.zfs.on_pool_changed(self.nav);
            }
            Focus::Right => self.active_view_mut().handle_up(),
        }
    }

    pub fn handle_down(&mut self) {
        match self.focus {
            Focus::Left => {
                self.zfs.next_pool();
                self.zfs.on_pool_changed(self.nav);
            }
            Focus::Right => self.active_view_mut().handle_down(),
        }
    }

    pub fn map_key(&mut self, key: KeyCode) {
        handlers::handle_key(self, key);
    }

    pub fn dispatch(&mut self, action: Action) {
        let result = match action {
            Action::CreateSnapshot { dataset, name } => {
                create_snapshot(&dataset, &name).map(|_| self.zfs.load_snapshots())
            }
            
            Action::RenameDataset { old_name, new_name } => {
                println!("Renaming {} to {}", old_name, new_name);
                Ok(()) 
            }
            
            Action::DestroySnapshot { name } => {
                destroy_snapshot(&name).map(|_| self.zfs.load_snapshots())
            }

            Action::StartScrub { pool } => {
                match start_scrub(&pool) {
                    Ok(_) => {
                        self.zfs.scrub = get_scrub_status(&pool);

                        self.dialog = Dialog::Info {
                            title: "Scrub Started".into(),
                            message: format!("Scrub started on pool {}", pool),
                        };

                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }

            Action::StopScrub { pool } => {
                stop_scrub(&pool).map(|_| {
                    self.zfs.scrub = get_scrub_status(&pool);
                })
            }

            Action::ChangeNav(nav) => {
                self.nav = nav;
                self.focus = Focus::Right;
                Ok(())
            }
        };

        if let Err(e) = result {
            self.show_error(e);
        }
    }

    pub fn show_error(&mut self, msg: String) {
        self.dialog = Dialog::Error {
            title: "Error".into(),
            message: msg,
        };
    }
}


