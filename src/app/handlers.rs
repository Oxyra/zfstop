use crossterm::event::KeyCode;
use crate::app::{App, InputMode, View};

pub fn handle_key(app: &mut App, key: KeyCode) {
    match app.input_mode {
        InputMode::Normal => handle_normal_mode(app, key),
        InputMode::CreatingSnapshot | InputMode::RenamingDataset => handle_input_mode(app, key),
    }
}

fn handle_normal_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Up => app.previous(),
        KeyCode::Down => app.next(),
        KeyCode::Enter => app.open_selected_pool(),
        KeyCode::Esc => app.back(),

        // Sub-navigation (j/k)
        KeyCode::Char('j') => match app.view {
            View::DatasetDetails => app.next_dataset(),
            View::Snapshots => app.next_snapshot(),
            _ => {}
        },
        KeyCode::Char('k') => match app.view {
            View::DatasetDetails => app.previous_dataset(),
            View::Snapshots => app.previous_snapshot(),
            _ => {}
        },

        // View Switching
        KeyCode::Char('S') => app.open_snapshots(),
        KeyCode::Char('s') => app.open_scrub(),
        KeyCode::Char('n') => app.view = View::Shares,
        KeyCode::Char('i') => app.view = View::Network,

        // Contextual Actions
        KeyCode::Char('c') if app.view == View::DatasetDetails => app.start_create_snapshot(),
        KeyCode::Char('r') if app.view == View::DatasetDetails => app.start_rename_dataset(),
        _ => {}
    }
}

fn handle_input_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => {
            let _ = match app.input_mode {
                InputMode::CreatingSnapshot => app.submit_snapshot(),
                InputMode::RenamingDataset => app.submit_rename(),
                _ => Ok(()),
            };
        }
        KeyCode::Esc => app.cancel_snapshot(),
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => { app.input_buffer.pop(); }
        _ => {}
    }
}
