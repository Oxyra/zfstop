use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let selected_pool_name = app.pools.get(app.table_state.selected().unwrap_or(0))
        .map(|p| p.name.as_str())
        .unwrap_or("Unknown");

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(" Pool Topology & Status ", Style::default().fg(Color::Cyan)),
        ]))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(240)))
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
    ]).split(inner);

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" STATUS ", Style::default().bg(Color::Indexed(39)).fg(Color::Black).bold()),
            Span::raw(" "),
            Span::styled(selected_pool_name, Style::default().fg(Color::White).bold()),
        ])),
        chunks[0],
    );

    let lines: Vec<Line> = app.pool_status.iter().map(|line: &String| {
        let trimmed = line.trim();
        
        if trimmed.contains("ONLINE") {
            Line::from(vec![
                Span::raw(line.clone()),
                Span::styled(" ●", Style::default().fg(Color::Green)),
            ])
        } else if trimmed.contains("DEGRADED") || trimmed.contains("FAULTED") || trimmed.contains("OFFLINE") {
            Line::from(Span::styled(line.clone(), Style::default().fg(Color::Red).bold()))
        } else if trimmed.starts_with("state:") {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            Line::from(vec![
                Span::styled(parts[0], Style::default().fg(Color::Indexed(244))),
                Span::raw(":"),
                Span::styled(parts.get(1).unwrap_or(&"").to_string(), Style::default().fg(Color::Indexed(214)).bold()),
            ])
        } else if trimmed.starts_with("config:") || trimmed.starts_with("pool:") {
             Line::from(Span::styled(line.clone(), Style::default().fg(Color::Cyan).bold()))
        } else if trimmed.contains("/") || (trimmed.len() > 2 && trimmed.chars().all(|c| c.is_alphanumeric() || c == '-')) {
            Line::from(Span::styled(line.clone(), Style::default().fg(Color::White)))
        } else {
            Line::from(Span::styled(line.clone(), Style::default().fg(Color::Indexed(240))))
        }
    }).collect();

    f.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false }), 
        chunks[2]
    );
}
