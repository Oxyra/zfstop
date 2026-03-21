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

use ratatui::{Frame, layout::*};
use ratatui::widgets::{Block, Borders, BorderType, Paragraph, Clear, Padding};
use ratatui::style::{Style, Color};
use ratatui::text::{Span, Line};
use crate::app::{App, Nav, InputMode, InputAction};

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

    draw_snapshot_input_popup(f, app);
    draw_rename_input_popup(f, app);
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
    }
}

fn draw_snapshot_input_popup(f: &mut Frame, app: &App) {
    if app.input.mode != InputMode::Editing
        || app.input.action != Some(InputAction::CreatingSnapshot) {
        return;
    }

    let area = f.area();
    let popup_width = 60;
    let popup_height = 8;
    let vertical_margin = (area.height.saturating_sub(popup_height)) / 2;
    let horizontal_margin = (area.width.saturating_sub(popup_width)) / 2;

    let popup_area = Rect::new(horizontal_margin, vertical_margin, popup_width, popup_height);

    f.render_widget(Clear, popup_area); 

    let dataset_name = app.zfs.datasets
        .get(app.zfs.dataset_state.selected().unwrap_or(0))
        .map(|d| d.name.as_str())
        .unwrap_or("Unknown");

    let block = Block::default()
        .title(Line::from(" New Snapshot "))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
        .padding(Padding::horizontal(2));

    let inner = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
    ]).split(inner);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("For: ", Style::default().fg(Color::Indexed(244))),
            Span::styled(dataset_name, Style::default().fg(Color::White).italic()),
        ])),
        chunks[0],
    );

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("Name: ", Style::default().fg(Color::Indexed(244))),
            Span::styled(&app.input.buffer, Style::default().fg(Color::Yellow).bold()),
        ])),
        chunks[1],
    );

    f.set_cursor_position((
        chunks[1].x + 6 + (app.input.buffer.len() as u16),
        chunks[1].y,
    ));
}

fn draw_rename_input_popup(f: &mut Frame, app: &App) {
    if app.input.mode != InputMode::Editing
        || app.input.action != Some(InputAction::RenamingDataset) {
        return;
    }

    let area = f.area();
    let popup_width = 60;
    let popup_height = 8;
    let vertical_margin = (area.height.saturating_sub(popup_height)) / 2;
    let horizontal_margin = (area.width.saturating_sub(popup_width)) / 2;
    let popup_area = Rect::new(horizontal_margin, vertical_margin, popup_width, popup_height);

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(Line::from(" Rename Dataset "))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Yellow))
        .padding(Padding::horizontal(2));

    let inner = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
    ]).split(inner);

    let label = "Rename to: ";
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(label, Style::default().fg(Color::Indexed(244))),
            Span::styled(&app.input.buffer, Style::default().fg(Color::Yellow).bold()),
        ])),
        chunks[1],
    );
    
    f.set_cursor_position((
        chunks[1].x + 11 + (app.input.buffer.len() as u16),
        chunks[1].y,
    ));
}
