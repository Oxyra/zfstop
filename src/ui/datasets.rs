use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_datasets(f: &mut Frame, app: &mut App, area: Rect) {
    let selected_idx = app.dataset_state.selected().unwrap_or(0);
    
    let rows = app.datasets.iter().enumerate().map(|(i, d)| {
        let is_parent = app.datasets.iter().any(|x| x.name.starts_with(&(d.name.clone() + "/")));
        let is_selected = i == selected_idx;

        let mut display_name = d.name.clone();
        if let Some(pool) = app.pools.get(app.table_state.selected().unwrap_or_default()) {
            if d.name != pool.name {
                let path_without_pool = d.name.trim_start_matches(&pool.name).trim_start_matches('/');
                let depth = path_without_pool.split('/').count();
                let leaf = path_without_pool.split('/').last().unwrap_or(path_without_pool);
                
                let indent = "  ".repeat(depth.saturating_sub(1));
                let connector = if is_parent { "▼ " } else { "• " };
                display_name = format!("{indent}{connector}{leaf}");
            } else {
                display_name = format!("🏠 {display_name}");
            }
        }

        let name_style = if is_selected {
            Style::default().fg(Color::Cyan).bold()
        } else if is_parent {
            Style::default().fg(Color::Indexed(39)) // Bright blue for folders
        } else {
            Style::default().fg(Color::White)
        };

        Row::new(vec![
            Cell::from(display_name).style(name_style),
            Cell::from(Line::from(d.used.as_str()).alignment(Alignment::Right)),
            Cell::from(Line::from(d.avail.as_str()).alignment(Alignment::Right)),
            Cell::from(format!(" {}", d.mountpoint)).style(Style::default().fg(Color::Indexed(244))),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Fill(1),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Max(35),
        ],
    )
    .header(
        Row::new(vec![
            Cell::from("  NAME"),
            Cell::from(Line::from("USED").alignment(Alignment::Right)),
            Cell::from(Line::from("AVAIL").alignment(Alignment::Right)),
            Cell::from("  MOUNTPOINT"),
        ])
        .style(Style::default().fg(Color::Indexed(244)).add_modifier(Modifier::UNDERLINED))
        .bottom_margin(1)
    )
    .block(
        Block::default()
            .title(Line::from(vec![
                Span::styled(" Datasets ", Style::default().fg(Color::White).bold()),
            ]))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Indexed(240)))
    )
    .row_highlight_style(Style::default().bg(Color::Indexed(235)))
    .highlight_symbol(" ");

    f.render_stateful_widget(table, area, &mut app.dataset_state);
}

pub fn draw_dataset_details(f: &mut Frame, app: &App, area: Rect) {
    let selected_idx = app.dataset_state.selected().unwrap_or(0);
    let dataset = app.datasets.get(selected_idx);

    let block = Block::default()
        .title(Line::from(" Selection Details ").alignment(Alignment::Left))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(240)))
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some(ds) = dataset {
        let chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ]).split(inner);

        f.render_widget(Paragraph::new(Line::from(vec![
            Span::styled(" DATASET ", Style::default().bg(Color::Indexed(39)).fg(Color::Black).bold()),
            Span::raw(" "),
            Span::styled(&ds.name, Style::default().fg(Color::White).bold()),
        ])), chunks[0]);

        let props = vec![
            ("Used", &ds.used, "local"),
            ("Available", &ds.avail, "local"),
            ("Mountpoint", &ds.mountpoint, "local"),
            ("Compression", &ds.compression.0, ds.compression.1.as_str()),
            ("Recordsize", &ds.recordsize.0, ds.recordsize.1.as_str()),
            ("Ratio", &ds.compressratio, "local"),
        ];

        let rows = props.into_iter().map(|(label, val, source)| {
            let color = match source {
                "local" => Color::Cyan,
                "default" => Color::Indexed(240),
                _ => Color::Indexed(244),
            };

            Row::new(vec![
                Cell::from(label).style(Style::default().fg(Color::Indexed(245))),
                Cell::from(val.as_str()).style(Style::default().fg(Color::White).bold()),
                Cell::from(format!("({})", source)).style(Style::default().fg(color).italic()),
            ])
        });

        let table = Table::new(rows, [
            Constraint::Length(14),
            Constraint::Length(15),
            Constraint::Min(0)
        ])
        .column_spacing(1);

        f.render_widget(table, chunks[2]);
    }
}
