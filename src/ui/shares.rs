use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw_shares(f: &mut Frame, app: &mut App, area: Rect) {
    let shared_datasets: Vec<_> = app.datasets
        .iter()
        .filter(|d| d.is_shares())
        .collect();

    let rows = shared_datasets.iter().map(|d| {
        let nfs_style = if d.sharenfs == "off" { 
            Style::default().fg(Color::Indexed(240)) 
        } else { 
            Style::default().fg(Color::Green) 
        };
        
        let smb_style = if d.sharesmb == "off" { 
            Style::default().fg(Color::Indexed(240)) 
        } else { 
            Style::default().fg(Color::Yellow) 
        };

        Row::new(vec![
            Cell::from(d.name.as_str()).style(Style::default().fg(Color::White).bold()),
            Cell::from(d.sharenfs.as_str()).style(nfs_style),
            Cell::from(d.sharesmb.as_str()).style(smb_style),
            Cell::from(d.mountpoint.as_str()).style(Style::default().fg(Color::Indexed(244))),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Fill(1),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Max(30),
        ],
    )
    .header(
        Row::new(vec![
            "  DATASET", 
            "NFS", 
            "SMB", 
            "MOUNTPOINT"
        ])
        .style(Style::default().fg(Color::Indexed(39)).bold()) // Matches Snapshot Blue
        .bottom_margin(1)
    )
    .block(
        Block::default()
            .title(Line::from(vec![
                Span::styled(" Network Shares ", Style::default().fg(Color::Cyan)),
                Span::styled(format!(" ({} active) ", shared_datasets.len()), Style::default().fg(Color::Indexed(240))),
            ]))
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

    f.render_stateful_widget(table, area, &mut app.share_state);
}
