use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_network(f: &mut Frame, app: &mut App, area: Rect) {
    let rows: Vec<Row> = app.interfaces.iter()
        .filter(|i| i.name != "lo")
        .map(|iface| {
            let state_style = match iface.operstate.as_str() {
                "UP" => Style::default().fg(Color::Green),
                "DOWN" => Style::default().fg(Color::Red),
                _ => Style::default().fg(Color::Indexed(240)),
            };

            let ip = iface.addr_info.iter()
                .find(|a| a.family == "inet")
                .map(|a| a.local.clone())
                .unwrap_or_else(|| "-".to_string());

            Row::new(vec![
                Cell::from(iface.name.clone()).style(Style::default().fg(Color::White).bold()),
                Cell::from(iface.link_type.clone()).style(Style::default().fg(Color::Indexed(240))),
                Cell::from(iface.operstate.clone()).style(state_style),
                Cell::from(ip),
                Cell::from(Line::from(iface.mtu.to_string()).alignment(Alignment::Right)),
            ])
        }).collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(10),
            Constraint::Fill(1),
            Constraint::Length(10),
        ],
    )
    .header(
        Row::new(vec![
            Cell::from(" INTERFACE"),
            Cell::from("TYPE"),
            Cell::from("STATE"),
            Cell::from("IP ADDRESS"),
            Cell::from(Line::from("MTU").alignment(Alignment::Right)),
        ])
        .style(Style::default().fg(Color::Indexed(39)).bold()) // Matching Snapshot blue
        .bottom_margin(1)
    )
    .block(
        Block::default()
            .title(Line::from(vec![
                Span::styled(" Network Interfaces ", Style::default().fg(Color::Yellow)),
                Span::styled(format!(" ({} active) ", app.interfaces.len().saturating_sub(1)), Style::default().fg(Color::Indexed(240))),
            ]).alignment(Alignment::Left))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Indexed(240)))
    )
    .row_highlight_style(Style::default().bg(Color::Indexed(235)).bold())
    .highlight_symbol(Text::from(vec![
        Line::from(vec![
            Span::styled("▶ ", Style::default().fg(Color::Yellow).bold())
        ])
    ]));

    f.render_stateful_widget(table, area, &mut app.interface_state);
}
