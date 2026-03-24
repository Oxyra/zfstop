use crossterm::event::KeyCode;
use crate::app::{App, Nav, Focus, NetworkSort, Action, Dialog};

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
        KeyCode::Char('7') => { app.nav = Nav::Docker; app.focus = Focus::Right; },

        KeyCode::Char('c')
            if app.focus == Focus::Right && app.nav == Nav::Datasets
        => {
            if let Some(ds_name) = app.zfs.selected_dataset_name() {
                app.dialog = Dialog::Input {
                    title: "New Snapshot".into(),
                    label: "Name".into(),
                    buffer: String::new(),
                    on_confirm: Box::new(move |name| Action::CreateSnapshot {
                        dataset: ds_name.clone(),
                        name
                    }),
                };
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
                        action: Action::DestroySnapshot { name: snap.name.clone() },
                    }
                }
            }
        },

        KeyCode::Char('r')
            if app.focus == Focus::Right && app.nav == Nav::Datasets
        => {
            if let Some(name) = app.zfs.selected_dataset_name() {
                let old_name = name.clone();
                app.dialog = Dialog::Input {
                    title: "Rename Dataset".into(),
                    label: "New name".into(),
                    buffer: name,
                    on_confirm: Box::new(move |new_name| {
                        Action::RenameDataset {
                            old_name: old_name.clone(),
                            new_name: new_name
                        }
                    }),
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

        KeyCode::Char('S') if app.nav == Nav::Scrub || app.nav == Nav::PoolStatus => {
            if let Some(i) = app.zfs.pool_state.selected() {
                let pool_name = app.zfs.pools[i].name.clone();
                app.dispatch(Action::StartScrub { pool: pool_name });
            }
        }

        KeyCode::Char('X') if app.nav == Nav::Scrub || app.nav == Nav::PoolStatus => {
            if let Some(i) = app.zfs.pool_state.selected() {
                let pool_name = app.zfs.pools[i].name.clone();

                app.dialog = Dialog::Confirm {
                    title: "Stop Scrub".into(),
                    message: format!("Are you sure you want to stop the scrub on {}?", pool_name),
                    action: Action::StopScrub { pool: pool_name },
                };
            }
        }
        
        KeyCode::Esc => app.back(),
        _ => {}
    }
}

fn handle_dialog(app: &mut App, key: KeyCode) {
    match &mut app.dialog {
        Dialog::Input { buffer, on_confirm, .. } => {
            match key {
                KeyCode::Enter => {
                    let input = std::mem::take(buffer);
                    let action = on_confirm(input);
                    app.dialog = Dialog::None;
                    app.dispatch(action);
                }
                KeyCode::Esc => app.dialog = Dialog::None,
                KeyCode::Char(c) => buffer.push(c),
                KeyCode::Backspace => { buffer.pop(); }
                _ => {}
            }
        }

        Dialog::Confirm { action, .. } => {
            if let KeyCode::Char('y') = key {
                let action = action.clone();
                app.dialog = Dialog::None;
                app.dispatch(action);
            } else if matches!(key, KeyCode::Char('n') | KeyCode::Esc) {
                app.dialog = Dialog::None;
            }
        }

        Dialog::Error { .. } => {
            match key {
                KeyCode::Esc | KeyCode::Enter => {
                    app.dialog = Dialog::None;
                }
                _ => {}
            }
        }

        Dialog::Info { .. } => {
            match key {
                KeyCode::Esc | KeyCode::Enter => {
                    app.dialog = Dialog::None;
                }
                _ => {}
            }
        }

        Dialog::None => {}
    }
}


