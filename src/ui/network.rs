use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::{App, NetworkSort};

pub fn draw_network(f: &mut Frame, app: &mut App, area: Rect) {
    let sort_label = match app.network.sort {
        NetworkSort::Name => "Name",
        NetworkSort::State => "State",
        NetworkSort::Mtu => "MTU",
    };

    let rows: Vec<Row> = app.network.interfaces.iter()
        .map(|iface| {
            let (state_icon, state_style) = match iface.operstate.as_str() {
                "UP" => ("●", Style::default().fg(Color::Green)),
                "DOWN" => ("○", Style::default().fg(Color::Red)),
                _ => ("-", Style::default().fg(Color::Indexed(240))),
            };

            let ip = iface.addr_info.iter()
                .find(|a| a.family == "inet")
                .map(|a| a.local.clone())
                .unwrap_or_else(|| "-".to_string());

            let name_style = if iface.name == "lo" {
                Style::default().fg(Color::Indexed(240))
            } else {
                Style::default().fg(Color::White).bold()
            };

            Row::new(vec![
                Cell::from(iface.name.clone()).style(name_style),
                Cell::from(iface.link_type.clone()).style(Style::default().fg(Color::Indexed(240))),
                Cell::from(Line::from(vec![
                    Span::styled(format!("{} ", state_icon), state_style),
                    Span::styled(&iface.operstate, state_style),
                ])),
                Cell::from(ip).style(Style::default().fg(Color::Cyan)),
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
        .style(Style::default().fg(Color::Indexed(39)).bold())
        .bottom_margin(1)
    )
    .block(
        Block::default()
            .title(Line::from(vec![
                Span::styled(" Network Interfaces ", Style::default().fg(Color::Yellow)),
                Span::styled(format!(" [{}] ", sort_label), Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!(" ({} active) ", app.network.interfaces.len().saturating_sub(1)),
                    Style::default().fg(Color::Indexed(240))
                ),
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

    f.render_stateful_widget(table, area, &mut app.network.interface_state);
}

pub fn draw_network_details(f: &mut Frame, app: &App, area: Rect) {
    let selected = app.network.interface_state.selected().unwrap_or(0);

    let block = Block::default()
        .title(Span::styled(" Interface Details ", Style::default().fg(Color::Yellow)))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(240)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some(iface) = app.network.interfaces.get(selected) {
        let chunks = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).split(inner);

        let left_rows = vec![
            Row::new(vec![Cell::from("Name"), Cell::from(iface.name.as_str()).style(Style::default().bold())]),
            Row::new(vec![Cell::from("State"), Cell::from(iface.operstate.as_str())]),
            Row::new(vec![Cell::from("MTU"), Cell::from(iface.mtu.to_string())]),
        ];

        let ipv4 = iface.addr_info.iter().find(|a| a.family == "inet").map(|a| a.local.as_str()).unwrap_or("-");
        let right_rows = vec![
            Row::new(vec![Cell::from("Type"), Cell::from(iface.link_type.as_str())]),
            Row::new(vec![Cell::from("IPv4"), Cell::from(ipv4).style(Style::default().fg(Color::Cyan))]),
        ];

        let table_style = |rows| Table::new(rows, [Constraint::Length(8), Constraint::Min(0)]).column_spacing(1);

        f.render_widget(table_style(left_rows), chunks[0]);
        f.render_widget(table_style(right_rows), chunks[1]);
    }
}

