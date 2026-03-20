mod app;
mod ui;
mod zfs;
mod utils;
mod net;

use app::{App, View, Mode};
use ui::draw;

use crate::app::InputMode;

use std::{io, time::Duration};
use std::sync::{Arc, Mutex};
use std::thread;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

fn main() -> Result<(), io::Error> {

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let app = Arc::new(Mutex::new(App::new()));

    app.lock().unwrap().load_datasets();

    let metrics_app = Arc::clone(&app);

    thread::spawn(move || {
        loop {
            {
                let mut app = metrics_app.lock().unwrap();

                if let Ok(uptime_str) = std::fs::read_to_string("/proc/uptime") {
                    if let Some(seconds_str) = uptime_str.split_whitespace().next() {
                        if let Ok(seconds) = seconds_str.parse::<f32>() {
                            let s = seconds as u64;
                            let days = s / 86400;
                            let hours = (s % 86400) / 3600;
                            let minutes = (s % 3600) / 60;

                            app.uptime = if days > 0 {
                                format!("{}d {:02}h {:02}m", days, hours, minutes)
                            } else {
                                format!("{:02}h {:02}m", hours, minutes)
                            };
                        }
                    }
                }

                if app.pools.is_empty() {
                    app.pools = zfs::pools::list_pools();
                }

                app.update_arc();

                for pool in &mut app.pools {
                    pool.update_io();
                }

                app.update_network();

                if matches!(app.view, View::ScrubStatus) {
                    if let Some(i) = app.table_state.selected() {
                        let pool = &app.pools[i].name;
                        app.scrub = zfs::scrub::get_scrub_status(pool);
                    }
                }

                if matches!(app.mode, Mode::PoolStatus) {
                    if let Some(i) = app.table_state.selected() {
                        let pool_name = &app.pools[i].name;
                        app.pool_status = zfs::status::get_pool_status(pool_name);
                    }
                }
            }

            thread::sleep(Duration::from_secs(1));
        }
    });

    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: Arc<Mutex<App>>,
) -> io::Result<()> {

    loop {
        {
            let mut app_locked = app.lock().unwrap();
            terminal.draw(|f| draw(f, &mut app_locked))?;
        }

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                let mut app = app.lock().unwrap();

                match app.input_mode {
                    InputMode::Normal => {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Up => app.previous(),
                            KeyCode::Down => app.next(),
                            KeyCode::Enter => app.open_selected_pool(),
                            KeyCode::Esc => app.back(),

                            KeyCode::Char('c') => {
                                if let View::DatasetDetails = app.view {
                                    app.start_create_snapshot();
                                }
                            },

                            KeyCode::Char('r') => {
                                if let View::DatasetDetails = app.view {
                                    app.start_rename_dataset();
                                }
                            },

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
                            KeyCode::Char('S') => app.open_snapshots(),
                            KeyCode::Char('s') => app.open_scrub(),

                            KeyCode::Char('n') => {
                                app.view = View::Shares;
                            }
                            KeyCode::Char('i') => {
                                app.view = View::Network;
                            }
                            _ => {}
                        }
                    }

                    InputMode::CreatingSnapshot => {
                        match key.code {
                            KeyCode::Enter => {
                                if let Err(err) = app.submit_snapshot() {
                                    println!("Error creating snapshot: {}", err);
                                }
                            }
                            KeyCode::Esc => app.cancel_snapshot(),
                            KeyCode::Char(c) => app.input_buffer.push(c),
                            KeyCode::Backspace => { app.input_buffer.pop(); }
                            _ => {}
                        }
                    }

                    InputMode::RenamingDataset => {
                        match key.code {
                            KeyCode::Enter => {
                                if let Err(err) = app.submit_rename() {
                                    println!("Error renaming dataset: {}", err);
                                }
                            }
                            KeyCode::Esc => {
                                app.input_mode = InputMode::Normal;
                                app.input_buffer.clear();
                            },
                            KeyCode::Char(r) => app.input_buffer.push(r),
                            KeyCode::Backspace => { app.input_buffer.pop(); }
                            _ => {}
                        }
                    }
                }
            }

        }
    }
}

