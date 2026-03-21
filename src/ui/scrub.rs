use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_scrub(f: &mut Frame, app: &App, area: Rect) {
    let selected_pool_name = app.zfs.pool_state.selected()
        .and_then(|i| app.zfs.pools.get(i))
        .map(|p| p.name.as_str())
        .unwrap_or("No Pool Selected");

    let block = Block::default()
        .title(Line::from(vec![
            Span::raw(" Pool: "),
            Span::styled(format!(" {} ", selected_pool_name), Style::default().fg(Color::Black).bg(Color::Indexed(39)).bold()),
            Span::raw(" "),
        ]))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(240)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some(scrub) = &app.zfs.scrub {
        let layout = Layout::vertical([
            Constraint::Length(1), // Status Line
            Constraint::Length(1), // Spacer
            Constraint::Length(2), // Gauge
            Constraint::Length(1), // Spacer
            Constraint::Min(0),    // Stats Table
        ])
        .margin(1)
        .split(inner);

        let status_color = if scrub.state.contains("progress") { Color::Green } else { Color::Yellow };
        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled("STATUS: ", Style::default().fg(Color::Indexed(244))),
                Span::styled("● ", Style::default().fg(status_color)),
                Span::styled(scrub.state.to_uppercase(), Style::default().fg(Color::White).bold()),
            ])),
            layout[0],
        );

        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Indexed(39)).bg(Color::Indexed(235)))
            .ratio(scrub.progress)
            .label(format!("{:.1}% Complete", scrub.progress * 100.0))
            .use_unicode(true);
        f.render_widget(gauge, layout[2]);

        let error_color = if scrub.repaired != "0B" { Color::Red } else { Color::Green };
        let rows = vec![
            Row::new(vec![Cell::from("Scanned"), Cell::from(scrub.scanned.as_str()).style(Style::default().bold())]),
            Row::new(vec![Cell::from("Issued"), Cell::from(scrub.issued.as_str()).style(Style::default().bold())]),
            Row::new(vec![Cell::from("Speed"), Cell::from(scrub.speed.as_str()).style(Style::default().fg(Color::Indexed(111)))]),
            Row::new(vec![Cell::from("Repaired"), Cell::from(scrub.repaired.as_str()).style(Style::default().fg(error_color).bold())]),
            Row::new(vec![Cell::from("ETA"), Cell::from(scrub.eta.as_str()).style(Style::default().fg(Color::Indexed(214)).bold())]),
        ];

        let table = Table::new(rows, [Constraint::Length(12), Constraint::Min(0)])
            .column_spacing(2)
            .style(Style::default().fg(Color::Indexed(250)));

        f.render_widget(table, layout[4]);

    } else {
        f.render_widget(
            Paragraph::new(vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(" 󰄬 ", Style::default().fg(Color::Green)),
                    Span::styled(selected_pool_name, Style::default().fg(Color::White).bold()),
                    Span::raw(" is healthy"),
                ]),
                Line::from(Span::styled("No scrub in progress", Style::default().fg(Color::Indexed(240)))),
            ])
            .alignment(Alignment::Center)
            .block(Block::default().padding(Padding::vertical(2))),
            inner,
        );
    }
}
