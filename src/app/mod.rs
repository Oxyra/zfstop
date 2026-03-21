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
    InputMode, 
    InputAction,
    NetworkSort
};

use crate::app::view::ActiveView;
use crate::zfs::snapshots::create_snapshot;
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

    pub fn reset_input(&mut self) {
        self.input.mode = InputMode::Normal;
        self.input.action = None;
        self.input.buffer.clear();
    }
    
    pub fn start_create_snapshot(&mut self) {
        self.input.buffer.clear();
        self.input.mode = InputMode::Editing;
        self.input.action = Some(InputAction::CreatingSnapshot);
    }

    pub fn submit_snapshot(&mut self) -> Result<(), String> {
        if let Some(dataset) = self.zfs.selected_dataset_name() {
            create_snapshot(&dataset, &self.input.buffer)?;
            self.input.mode = InputMode::Normal;
            self.input.buffer.clear();
            Ok(())
        } else {
            Err("No dataset selected".into())
        }
    }
    
    pub fn start_rename_dataset(&mut self) {
        if let Some(name) = self.zfs.selected_dataset_name() {
            self.input.buffer = name;
            self.input.mode = InputMode::Editing;
            self.input.action = Some(InputAction::RenamingDataset);
        }
    }
    
    pub fn submit_rename(&mut self) -> Result<(), String> {
        if let Some(old_name) = self.zfs.selected_dataset_name() {
            let new_name = self.input.buffer.trim().to_string();

            println!("Renaming {} to {}", old_name, new_name);
            
            self.input.mode = InputMode::Normal;
            self.input.buffer.clear();
            self.zfs.load_datasets();
            Ok(())
        } else {
            Err("No dataset selected".into())
        }
    }
}


