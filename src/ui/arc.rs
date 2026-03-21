use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;
use crate::utils::format_bytes;

pub fn draw_arc(f: &mut Frame, app: &App, area: Rect) {
    let arc = &app.system.arc;
    let block = Block::default()
        .title(Line::from(" ARC Cache ").alignment(Alignment::Left))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(240)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ]).split(inner);

    let header_cols = Layout::horizontal([
        Constraint::Percentage(33),
        Constraint::Percentage(34),
        Constraint::Percentage(33),
    ]).split(layout[0]);

    f.render_widget(Paragraph::new(Line::from(vec![
        Span::raw("Usage: "),
        Span::styled(format_bytes(arc.size), Style::default().fg(Color::Cyan).bold()),
    ])), header_cols[0]);

    let hit_ratio = arc.hit_ratio();
    f.render_widget(Paragraph::new(Line::from(vec![
        Span::styled("● ", Style::default().fg(Color::Green)),
        Span::raw(format!("{:.1}% Hit", hit_ratio)),
    ]).alignment(Alignment::Center)), header_cols[1]);

    f.render_widget(Paragraph::new(Line::from(vec![
        Span::raw("Total: "),
        Span::styled(format_bytes(arc.max), Style::default().fg(Color::DarkGray)),
    ]).alignment(Alignment::Right)), header_cols[2]);

    let ratio = (arc.size as f64 / arc.max as f64).clamp(0.0, 1.0);
    f.render_widget(Paragraph::new(Line::from(vec![
        Span::raw("Utilization: "),
        Span::styled(format!("{:.1}%", ratio * 100.0), Style::default().bold()),
    ]).alignment(Alignment::Right)), layout[2]);

    f.render_widget(Gauge::default()
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::Rgb(30, 30, 30)))
        .ratio(ratio)
        .label("")
        .use_unicode(true), layout[3]);
}

pub fn draw_arc_breakdown(f: &mut Frame, app: &App, area: Rect) {
    let arc = &app.system.arc;
    let (d_data, d_meta, p_data, p_meta) = arc.breakdown();

    let block = Block::default()
        .title(Line::from(" Hit Breakdown ").alignment(Alignment::Left))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(240)))
        .padding(Padding::uniform(1));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ]).split(inner);

    let rows = [
        ("Demand Data", d_data, Color::Indexed(39)),
        ("Demand Meta", d_meta, Color::Indexed(37)),
        ("Prefetch Data", p_data, Color::Indexed(111)),
        ("Prefetch Meta", p_meta, Color::Indexed(147)),
    ];

    for (i, (label, value, color)) in rows.iter().enumerate() {
        let row_cols = Layout::horizontal([
            Constraint::Length(16),
            Constraint::Min(0),
            Constraint::Length(8),
        ]).split(layout[i]);

        f.render_widget(
            Paragraph::new(*label).style(Style::default().fg(Color::Indexed(244))), 
            row_cols[0]
        );

        let gauge = LineGauge::default()
            .ratio((*value).clamp(0.0, 1.0))
            .filled_style(Style::default().fg(*color))
            .filled_symbol(symbols::block::FULL)
            .unfilled_symbol(symbols::line::HORIZONTAL)
            .unfilled_style(Style::default().fg(Color::Indexed(236)));

        f.render_widget(gauge, row_cols[1]);

        let pct_text = format!("{:>6.1}%", *value * 100.0);
        f.render_widget(
            Paragraph::new(pct_text)
                .style(Style::default().fg(*color).bold())
                .alignment(Alignment::Right), 
            row_cols[2]
        );
    }
}

pub fn draw_arc_graph(f: &mut Frame, app: &App, area: Rect) {
    let raw_val = app.system.arc_history.last().cloned().unwrap_or(0);

    let current_hit = raw_val as f64 / 10.0;
    
    let spark_color = match current_hit {
        h if h > 90.0 => Color::Green,
        h if h > 70.0 => Color::Cyan,
        h if h > 40.0 => Color::Yellow,
        _ => Color::Red,
    };

    let spark = Sparkline::default()
        .block(
            Block::default()
                .title(Line::from(vec![
                    Span::raw(" ARC Hit Rate History "),
                    Span::styled(
                        format!("avg: {:.1}% ", current_hit), 
                        Style::default().fg(spark_color).bold()
                    ),
                ]))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Indexed(240)))
        )
        .data(&app.system.arc_history)
        .max(1000) 
        .style(Style::default().fg(spark_color));

    f.render_widget(spark, area);
}
