use crossterm::event::KeyCode;
use crate::app::{App, InputAction, Nav, Focus, NetworkSort};
use crate::app::state::{Dialog, ConfirmAction};

pub fn handle_key(app: &mut App, key: KeyCode) {
    if !matches!(app.dialog, Dialog::None) {
        handle_dialog(app, key);
        return;
    }

    handle_normal_mode(app, key);
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
        => {
            app.dialog = Dialog::Input {
                title: "New Snapshot".into(),
                label: "Name".into(),
                buffer: String::new(),
                action: InputAction::CreatingSnapshot,
            }
        },
        KeyCode::Char('d')
            if app.focus == Focus::Right && app.nav == Nav::Snapshots
        => {
            if let Some(i) = app.zfs.snapshot_state.selected() {
                if let Some(snap) = app.zfs.snapshots.get(i) {
                    app.dialog = Dialog::Confirm {
                        title: "Destroy Snapshot".into(),
                        message: format!("Destroy {} ?", snap.name),
                        action: ConfirmAction::DestroySnapshot(snap.name.clone()),
                    }
                }
            }
        },

        KeyCode::Char('r')
            if app.focus == Focus::Right && app.nav == Nav::Datasets
        => {
            if let Some(name) = app.zfs.selected_dataset_name() {
                app.dialog = Dialog::Input {
                    title: "Rename Dataset".into(),
                    label: "New name".into(),
                    buffer: name,
                    action: InputAction::RenamingDataset,
                };
            }
        },

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

fn handle_dialog(app: &mut App, key: KeyCode) {
    let mut action_to_run = None;

    match &mut app.dialog {
        Dialog::Input { buffer, action, .. } => {
            match key {
                KeyCode::Enter => {
                    action_to_run = Some((*action, buffer.clone()));
                    app.dialog = Dialog::None;
                }
                KeyCode::Esc => app.dialog = Dialog::None,
                KeyCode::Char(c) => buffer.push(c),
                KeyCode::Backspace => { buffer.pop(); }
                _ => {}
            }
        }

        Dialog::Confirm { action, .. } => {
            if let KeyCode::Char('y') = key {
                action_to_run = Some((InputAction::DestroyingSnapshot, match action {
                    ConfirmAction::DestroySnapshot(name) => name.clone(),
                }));
                app.dialog = Dialog::None;
            } else if matches!(key, KeyCode::Char('n') | KeyCode::Esc) {
                app.dialog = Dialog::None;
            }
        }

        Dialog::None => {}
    }

    if let Some((action, value)) = action_to_run {
        match action {
            InputAction::CreatingSnapshot => {
                let _ = app.submit_snapshot_with_name(value);
            }
            InputAction::RenamingDataset => {
                let _ = app.submit_rename_with_name(value);
            }
            InputAction::DestroyingSnapshot => {
                let _ = app.submit_destroy_snapshot(value);
            }
        }
    }
}


