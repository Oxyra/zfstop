mod app;
mod ui;
mod zfs;
mod utils;
mod net;
mod docker;

use app::App;
use app::state::Dialog;

use ui::draw;

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

    {
        let mut app_locked = app.lock().unwrap();

        if app_locked.zfs.pools.is_empty() {
            app_locked.zfs.pools = zfs::pools::list_pools();
        }

        let nav = app_locked.nav;
        app_locked.zfs.on_pool_changed(nav);
    }

    let metrics_app = Arc::clone(&app);

    thread::spawn(move || {
        loop {
            if let Ok(mut app) = metrics_app.lock() {
                app.system.update_uptime();
                app.tick();
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

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                let mut app = app.lock().unwrap();

                if matches!(app.dialog, Dialog::None) && key.code == KeyCode::Char('q') {
                    return Ok(());
                }

                app.map_key(key.code);
            }
        }
    }
}

