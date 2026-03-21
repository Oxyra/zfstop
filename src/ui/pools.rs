use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;
use crate::utils::format_bytes;

pub fn draw_pools(f: &mut Frame, app: &App, area: Rect) {
    let selected_idx = app.zfs.pool_state.selected().unwrap_or(0);

    let main_block = Block::default()
        .title(Line::from(" ZFS Pools ").alignment(Alignment::Left))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(240)));
    
    let inner_area = main_block.inner(area);
    f.render_widget(main_block, area);

    let pool_count = app.zfs.pools.len().max(1);
    let constraints: Vec<Constraint> = app.zfs.pools
        .iter()
        .map(|_| Constraint::Ratio(1, pool_count as u32)) 
        .collect();

    let pool_chunks = Layout::vertical(constraints).split(inner_area);

    for (i, pool) in app.zfs.pools.iter().enumerate() {
        if i >= pool_chunks.len() { break; }
        
        let pool_area = pool_chunks[i];
        let is_selected = i == selected_idx;

        if is_selected {
            f.render_widget(Block::default().bg(Color::Indexed(235)), pool_area);
        }

        let health_color = match pool.health.as_str() {
            "ONLINE" => Color::Green,
            "DEGRADED" => Color::Yellow,
            _ => Color::Red,
        };

        let block = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Indexed(237)))
            .title(Line::from(vec![
                Span::styled(if is_selected { " ▶ " } else { "   " }, Style::default().fg(Color::Cyan).bold()),
                Span::styled(format!("{:<12}", pool.name), Style::default()
                    .fg(if is_selected { Color::Cyan } else { Color::White })
                    .bold()),
                Span::styled(format!(" {} ", pool.health), Style::default().fg(health_color)),
            ]));

        let inner = block.inner(pool_area);
        f.render_widget(block, pool_area);

        let row_constraints = if inner.height >= 3 {
            vec![
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
        } else {
            vec![Constraint::Length(1), Constraint::Min(0)]
        };

        let rows = Layout::vertical(row_constraints).split(inner);

        let cap_percent = pool.capacity_percent();
        let cap_color = if cap_percent > 90.0 { Color::Red } 
                        else if cap_percent > 75.0 { Color::Yellow } 
                        else { Color::Indexed(39) };

        let cap_layout = Layout::horizontal([
            Constraint::Length(12),
            Constraint::Min(10),
            Constraint::Length(25),
        ]).split(rows[0]);

        f.render_widget(
            Paragraph::new("  Capacity").style(Style::default().fg(Color::Indexed(244))), 
            cap_layout[0]
        );
        
        f.render_widget(
            LineGauge::default()
                .ratio((cap_percent / 100.0).clamp(0.0, 1.0))
                .filled_style(Style::default().fg(cap_color))
                .unfilled_style(Style::default().fg(Color::Indexed(236)))
                .filled_symbol(symbols::line::THICK_VERTICAL)
                .unfilled_symbol(symbols::line::HORIZONTAL),
            cap_layout[1],
        );
        
        let usage_text = format!("{}/{} ({:.1}%)", 
            format_bytes(pool.alloc), 
            format_bytes(pool.size), 
            cap_percent
        );
        f.render_widget(
            Paragraph::new(usage_text)
                .alignment(Alignment::Right)
                .style(Style::default().fg(Color::Indexed(245))), 
            cap_layout[2]
        );

        if rows.len() > 2 && rows[2].height >= 1 {
            let io_cols = Layout::horizontal([
                Constraint::Percentage(50),
                Constraint::Length(2), 
                Constraint::Percentage(50),
            ]).split(rows[2]);

            render_io_widget(f, io_cols[0], "  READ ", pool.read_bps, &pool.read_history, Color::Indexed(111));
            render_io_widget(f, io_cols[2], "  WRITE", pool.write_bps, &pool.write_history, Color::Indexed(214));
        }
    }
}

fn render_io_widget(f: &mut Frame, area: Rect, label: &str, val: u64, history: &[u64], color: Color) {
    let layout = Layout::horizontal([
        Constraint::Length(8),
        Constraint::Length(10),
        Constraint::Min(0),
    ]).split(area);

    f.render_widget(Paragraph::new(label).style(Style::default().fg(Color::Indexed(245))), layout[0]);
    f.render_widget(Paragraph::new(format_bytes(val)).style(Style::default().fg(color).bold()), layout[1]);
    
    let max = history.iter().max().copied().unwrap_or(0).max(1024 * 1024); 
    let spark = Sparkline::default()
        .data(history)
        .max(max)
        .style(Style::default().fg(color));
    f.render_widget(spark, layout[2]);
}
