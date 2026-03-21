use crossterm::event::KeyCode;
use crate::app::{App, InputMode, InputAction, Nav, Focus, NetworkSort};

pub fn handle_key(app: &mut App, key: KeyCode) {
    match app.input.mode {
        InputMode::Normal => handle_normal_mode(app, key),
        InputMode::Editing => handle_input_mode(app, key),
    }
}

fn handle_normal_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Tab => {
            app.focus = match app.focus {
                Focus::Left => Focus::Right,
                Focus::Right => Focus::Left,
            };
        },

        KeyCode::Up | KeyCode::Char('k') => app.handle_up(),
        KeyCode::Down | KeyCode::Char('j') => app.handle_down(),

        KeyCode::Char('h') => app.focus = Focus::Left, 
        KeyCode::Char('l') => app.focus = Focus::Right, 
        KeyCode::Char('[') => app.prev_nav(),
        KeyCode::Char(']') => app.next_nav(),

        KeyCode::Char('1') => { app.nav = Nav::Datasets; app.focus = Focus::Right; },
        KeyCode::Char('2') => { app.nav = Nav::Snapshots; app.zfs.load_snapshots(); app.focus = Focus::Right; },
        KeyCode::Char('3') => { app.nav = Nav::PoolStatus; app.focus = Focus::Right; },
        KeyCode::Char('4') => { app.nav = Nav::Network; app.focus = Focus::Right; },
        KeyCode::Char('5') => { app.nav = Nav::Scrub; app.focus = Focus::Right; },
        KeyCode::Char('6') => { app.nav = Nav::Shares; app.focus = Focus::Right; },

        KeyCode::Char('c')
            if app.focus == Focus::Right && app.nav == Nav::Datasets
        => app.start_create_snapshot(),
        KeyCode::Char('r')
            if app.focus == Focus::Right && app.nav == Nav::Datasets
        => app.start_rename_dataset(),

        KeyCode::Char('n') if app.nav == Nav::Network => {
            app.network.sort = NetworkSort::Name;
            app.network.sort_interfaces();
        }
        KeyCode::Char('s') if app.nav == Nav::Network => {
            app.network.sort = NetworkSort::State;
            app.network.sort_interfaces();
        }
        KeyCode::Char('m') if app.nav == Nav::Network => {
            app.network.sort = NetworkSort::Mtu;
            app.network.sort_interfaces();
        }
        
        KeyCode::Esc => app.back(),
        _ => {}
    }
}

fn handle_input_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => {
            if let Some(action) = app.input.action {
                let _ = match action {
                    InputAction::CreatingSnapshot => app.submit_snapshot(),
                    InputAction::RenamingDataset => app.submit_rename(),
                };
                app.reset_input();
            }
        }
        KeyCode::Esc => app.reset_input(),
        KeyCode::Char(c) => app.input.buffer.push(c),
        KeyCode::Backspace => { app.input.buffer.pop(); }
        _ => {}
    }
}
