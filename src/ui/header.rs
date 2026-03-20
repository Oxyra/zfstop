use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::horizontal([
        Constraint::Length(15),
        Constraint::Length(25),
        Constraint::Min(0),
        Constraint::Length(40),
    ]).split(area);

    f.render_widget(
        Paragraph::new(Span::styled(" ZFStop ⚡ ", Style::default().fg(Color::Indexed(39)).bold())),
        chunks[0],
    );

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" HOST ", Style::default().bg(Color::Indexed(240)).fg(Color::White)),
            Span::raw(" "),
            Span::styled(&app.hostname, Style::default().fg(Color::Cyan).bold()),
        ])),
        chunks[1],
    );

    let now = chrono::Local::now().format("%H:%M:%S").to_string();
    let stats = Line::from(vec![
        Span::styled(" UP ", Style::default().fg(Color::Indexed(244))),
        Span::styled(&app.uptime, Style::default().fg(Color::White)),
        Span::raw("  "),
        Span::styled(" TIME ", Style::default().fg(Color::Indexed(244))),
        Span::styled(now, Style::default().fg(Color::White)),
    ]);

    f.render_widget(Paragraph::new(stats).alignment(Alignment::Right), chunks[3]);
}
