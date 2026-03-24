use crate::docker::types::DockerState;
use ratatui::widgets::{Table, Row, Block, Borders, BorderType};
use ratatui::layout::{Constraint, Rect};
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub fn draw_docker(f: &mut Frame, docker: &mut DockerState, area: Rect) {
    if docker.containers.is_empty() {
        let empty = Paragraph::new("No containers running")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                .title(" Docker ")
                .borders(Borders::ALL)
            );

        f.render_widget(empty, area);
        return;
    }

    let selected_idx = docker.container_state.selected().unwrap_or(0);

    let rows = docker.containers.iter().enumerate().map(|(i, c)| {
        let is_selected = i == selected_idx;

        let short_id = &c.ID[..12.min(c.ID.len())];

        let status_style = if c.Status.contains("Up") {
            Style::default().fg(Color::Green)
        } else if c.Status.contains("Exited") {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::Yellow)
        };

        let name_style = if is_selected {
            Style::default().fg(Color::Cyan).bold()
        } else {
            Style::default().fg(Color::White)
        };

        Row::new(vec![
            Cell::from(c.ID.clone()).style(Style::default().fg(Color::Indexed(244))),
            Cell::from(c.Image.clone()).style(name_style),
            Cell::from(c.Status.clone()).style(status_style),
            Cell::from(Line::from(vec![
                Span::styled("● ", status_style),
                Span::raw(&c.Status),
            ])),
            Cell::from(Line::from(c.Ports.clone()).alignment(Alignment::Right)),
            Cell::from(format!(" {}", c.Name)).style(Style::default().fg(Color::Indexed(244))),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(12),
            Constraint::Length(20),
            Constraint::Length(18),
            Constraint::Length(15),
            Constraint::Fill(1),
        ]
    )
    .header(
        Row::new(vec![
            Cell::from(" ID"),
            Cell::from("IMAGE"),
            Cell::from("STATUS"),
            Cell::from(Line::from("PORTS").alignment(Alignment::Right)),
            Cell::from("NAME"),
        ])
        .style(Style::default().fg(Color::Indexed(244)).add_modifier(Modifier::UNDERLINED))
        .bottom_margin(1)
    )
    .block(
        Block::default()
        .title(Line::from(vec![
            Span::styled(" Docker Containers ", Style::default().fg(Color::White).bold()),
        ]))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(240)))
    )
    .row_highlight_style(Style::default().bg(Color::Indexed(235)))
    .highlight_symbol(" ");

    f.render_stateful_widget(table, area, &mut docker.container_state);
}

pub fn draw_docker_details(f: &mut Frame, docker: &DockerState, area: Rect) {
    let selected_idx = docker.container_state.selected().unwrap_or(0);
    let container = docker.containers.get(selected_idx);

    let block = Block::default()
        .title(Line::from(" Selection Details ").alignment(Alignment::Left))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(240)))
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some(c) = container {
        let chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ]).split(inner);

        f.render_widget(Paragraph::new(Line::from(vec![
            Span::styled(" CONTAINER ", Style::default().bg(Color::Indexed(39)).fg(Color::Black).bold()),
            Span::raw(" "),
            Span::styled(&c.Name, Style::default().fg(Color::White).bold()),
        ])), chunks[0]);

        let props = vec![
            ("ID", &c.ID),
            ("Image", &c.Image),
            ("Command", &c.Command),
            ("Status", &c.Status),
            ("Ports", &c.Ports),
            ("Created", &c.CreatedAt),
        ];

        let rows = props.into_iter().map(|(label, val)| {
            Row::new(vec![
                Cell::from(label).style(Style::default().fg(Color::Indexed(245))),
                Cell::from(val.as_str()).style(Style::default().fg(Color::White).bold()),
            ])
        });

        let table = Table::new(
            rows,
            [
                Constraint::Length(14),
                Constraint::Min(0),
            ]
        )
        .column_spacing(1);

        f.render_widget(table, chunks[2]);
    }
}

