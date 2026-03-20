use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_scrub(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(Line::from(" Pool Scrub Status ").alignment(Alignment::Left))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(240)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some(scrub) = &app.scrub {
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .margin(1)
        .split(inner);

        let header = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Length(20),
        ]).split(layout[0]);

        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(" POOL ", Style::default().bg(Color::Indexed(39)).fg(Color::Black).bold()),
                Span::raw(" "),
                Span::styled(&scrub.pool, Style::default().fg(Color::White).bold()),
            ])),
            header[0],
        );

        let status_color = if scrub.state.contains("progress") { Color::Green } else { Color::Yellow };
        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled("● ", Style::default().fg(status_color)),
                Span::styled(&scrub.state, Style::default().fg(Color::White)),
            ])).alignment(Alignment::Right),
            header[1],
        );

        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Indexed(39)).bg(Color::Indexed(235)))
            .ratio(scrub.progress)
            .label(format!("{:.1}% Complete", scrub.progress * 100.0))
            .use_unicode(true);
        f.render_widget(gauge, layout[2]);

        let error_color = if scrub.repaired != "0B" { Color::Red } else { Color::Indexed(244) };
        
        let props = vec![
            ("Scanned", &scrub.scanned, Color::White),
            ("Issued", &scrub.issued, Color::White),
            ("Speed", &scrub.speed, Color::Indexed(111)),
            ("Repaired", &scrub.repaired, error_color),
            ("Total Size", &scrub.total, Color::White),
            ("Time Left", &scrub.eta, Color::Indexed(214)),
        ];

        let rows = props.into_iter().map(|(label, val, color)| {
            Row::new(vec![
                Cell::from(label).style(Style::default().fg(Color::Indexed(244))),
                Cell::from(val.as_str()).style(Style::default().fg(color).bold()),
            ])
        });

        let table = Table::new(rows, [Constraint::Length(12), Constraint::Min(0)])
            .column_spacing(2);

        f.render_widget(table, layout[4]);

    } else {
        f.render_widget(
            Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled("󰄬 No scrub in progress", Style::default().fg(Color::Indexed(240)))),
                Line::from(Span::styled("Idle", Style::default().fg(Color::Indexed(237)))),
            ])
            .alignment(Alignment::Center),
            inner,
        );
    }
}
