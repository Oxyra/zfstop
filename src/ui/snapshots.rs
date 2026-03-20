use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;
use crate::utils::format_age;

pub fn draw_snapshots(f: &mut Frame, app: &mut App, area: Rect) {
    let dataset_name = app.datasets
        .get(app.dataset_state.selected().unwrap_or(0))
        .map(|d| d.name.clone())
        .unwrap_or_else(|| "none".to_string());

    let rows = app.snapshots.iter().map(|s| {
        let parts: Vec<&str> = s.name.split('@').collect();
        let dataset = parts.get(0).unwrap_or(&"");
        let snapshot = parts.get(1).unwrap_or(&"");

        Row::new(vec![
            Cell::from(dataset.to_string()).style(Style::default().fg(Color::Indexed(240))),
            Cell::from(snapshot.to_string()).style(Style::default().fg(Color::White).bold()),
            Cell::from(Line::from(format_age(s.created)).alignment(Alignment::Right)),
            Cell::from(Line::from(s.used.as_str()).alignment(Alignment::Right)),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Fill(1),
            Constraint::Length(10),
            Constraint::Length(12),
        ],
    )
    .header(
        Row::new(vec![
            Cell::from("  DATASET"),
            Cell::from("SNAPSHOT"),
            Cell::from(Line::from("AGE").alignment(Alignment::Right)),
            Cell::from(Line::from("USED").alignment(Alignment::Right)),
        ])
        .style(Style::default().fg(Color::Indexed(39)).bold())
        .bottom_margin(1)
    )
    .block(
        Block::default()
            .title(Line::from(vec![
                Span::styled(" Snapshots ", Style::default().fg(Color::Cyan)),
                Span::styled(format!(" {} ", dataset_name), Style::default().fg(Color::White).bold()),
                Span::styled(format!("({} total) ", app.snapshots.len()), Style::default().fg(Color::Indexed(240))),
            ]).alignment(Alignment::Left))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Indexed(240)))
    )
    .row_highlight_style(Style::default().bg(Color::Indexed(235)).bold())
    .highlight_symbol(Text::from(vec![
        Line::from(vec![
            Span::styled("▶ ", Style::default().fg(Color::Cyan).bold())
        ])
    ]));

    f.render_stateful_widget(table, area, &mut app.snapshot_state);
}
