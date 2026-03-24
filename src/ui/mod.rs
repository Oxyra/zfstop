pub mod pools;
pub mod datasets;
pub mod snapshots;
pub mod arc;
pub mod scrub;
pub mod status;
pub mod footer;
pub mod header;
pub mod shares;
pub mod network;
pub mod docker;

use ratatui::{Frame, layout::*};
use ratatui::widgets::{Block, Borders, BorderType, Paragraph, Clear};
use crate::app::{App, Nav};
use crate::app::state::Dialog;
use crate::app::utils::centered_rect;

use header::draw_header;
use pools::draw_pools;
use datasets::{draw_datasets, draw_dataset_details};
use snapshots::draw_snapshots;
use arc::{draw_arc, draw_arc_graph, draw_arc_breakdown};
use scrub::draw_scrub;
use status::draw_status;
use footer::draw_footer;
use shares::draw_shares;
use network::{draw_network, draw_network_details};
use docker::{draw_docker, draw_docker_details};

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let vertical_mode = area.width < 120;

    let main_layout = Layout::vertical([
        Constraint::Length(1), // Header
        Constraint::Min(0),    // Body
        Constraint::Length(1), // Footer
    ]).split(area);

    draw_header(f, app, main_layout[0]);
    draw_footer(f, app, main_layout[2]);

    let body_area = main_layout[1];

    if vertical_mode {
        draw_vertical_layout(f, app, body_area);
    } else {
        draw_horizontal_layout(f, app, body_area);
    }

    draw_dialog(f, app);
}

fn draw_horizontal_layout(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::horizontal([
        Constraint::Percentage(35), // Sidebar
        Constraint::Percentage(65), // Detail Pane
    ]).split(area);

    let left_layout = Layout::vertical([
        Constraint::Length(6),  // ARC
        Constraint::Length(8),  // Graph
        Constraint::Length(10), // Breakdown
        Constraint::Min(0),     // Pools
    ]).split(chunks[0]);

    draw_arc(f, app, left_layout[0]);
    draw_arc_graph(f, app, left_layout[1]);
    draw_arc_breakdown(f, app, left_layout[2]);
    draw_pools(f, app, left_layout[3]);

    match app.nav {
        Nav::Datasets => {
            let right_layout = Layout::vertical([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ]).split(chunks[1]);
            draw_datasets(f, app, right_layout[0]);
            draw_dataset_details(f, app, right_layout[1]);
        }
        Nav::PoolStatus => draw_status(f, app, chunks[1]),
        Nav::Snapshots  => draw_snapshots(f, app, chunks[1]),
        Nav::Scrub      => draw_scrub(f, app, chunks[1]),
        Nav::Shares     => draw_shares(f, app, chunks[1]),
        Nav::Network    => {
            let right_layout = Layout::vertical([
                Constraint::Percentage(65),
                Constraint::Percentage(35),
            ]).split(chunks[1]);
            draw_network(f, app, right_layout[0]);
            draw_network_details(f, app, right_layout[1]);
        }
        Nav::Docker     => {
            let right_layout = Layout::vertical([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ]).split(chunks[1]);

            draw_docker(f, &mut app.docker, right_layout[0]);
            draw_docker_details(f, &app.docker, right_layout[1]);
        }
    }
}

fn draw_vertical_layout(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(7),  // ARC
        Constraint::Length(8),  // ARC Breakdown/Graph
        Constraint::Min(10),    // Pools
        Constraint::Length(15), // Detail View
    ]).split(area);

    draw_arc(f, app, chunks[0]);
    draw_arc_breakdown(f, app, chunks[1]);
    draw_pools(f, app, chunks[2]);

    match app.nav {
        Nav::Datasets   => draw_datasets(f, app, chunks[3]),
        Nav::PoolStatus => draw_status(f, app, chunks[3]),
        Nav::Snapshots  => draw_snapshots(f, app, chunks[3]),
        Nav::Scrub      => draw_scrub(f, app, chunks[3]),
        Nav::Shares     => draw_shares(f, app, chunks[3]),
        Nav::Network    => draw_network(f, app, chunks[3]),
        Nav::Docker     => {
            let right_layout = Layout::vertical([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ]).split(chunks[1]);

            draw_docker(f, &mut app.docker, right_layout[0]);
            draw_docker_details(f, &app.docker, right_layout[1]);
        }
    }
}

use ratatui::style::{Style, Color};
use ratatui::widgets::{Padding};

fn draw_dialog(f: &mut Frame, app: &App) {
    if matches!(app.dialog, Dialog::None) {
        return;
    }

    let area = f.area();
    let popup = centered_rect(50, 30, area);

    f.render_widget(Clear, popup);

    match &app.dialog {
        Dialog::Input { title, label, buffer, .. } => {
            let block = Block::default()
                .title(format!(" {} ", title))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan))
                .padding(Padding::horizontal(2));

            let inner = block.inner(popup);
            f.render_widget(block, popup);

            let layout = Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(inner);

            let text = Paragraph::new(format!("{}:", label));
            f.render_widget(text, layout[0]);

            let input = Paragraph::new(buffer.as_str())
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(input, layout[1]);

            // cursor
            f.set_cursor_position((
                layout[1].x + buffer.len() as u16,
                layout[1].y,
            ));
        }

        Dialog::Confirm { title, message, .. } => {
            let block = Block::default()
                .title(format!(" {} ", title))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Red))
                .padding(Padding::horizontal(2));

            let inner = block.inner(popup);
            f.render_widget(block, popup);

            let layout = Layout::vertical([
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(inner);

            let text = Paragraph::new(message.as_str());
            f.render_widget(text, layout[0]);

            let hint = Paragraph::new("[y] Yes    [n] No")
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(hint, layout[1]);
        }

        Dialog::Error { title, message } => {
            let block = Block::default()
                .title(format!(" {} ", title))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow));

            let inner = block.inner(popup);
            f.render_widget(block, popup);

            let text = Paragraph::new(message.as_str())
                .style(Style::default().fg(Color::Red));

            f.render_widget(text, inner);
        }

        Dialog::Info { title, message } => {
            let block = Block::default()
                .title(format!(" {} ", title))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Blue));

            let inner = block.inner(popup);
            f.render_widget(block, popup);

            let text = Paragraph::new(message.as_str())
                .style(Style::default().fg(Color::White));

            f.render_widget(text, inner);
        }

        Dialog::None => {}
    }
}
