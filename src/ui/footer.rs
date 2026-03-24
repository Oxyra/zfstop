use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::App;
use crate::app::Nav;

pub fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let context_name = match app.nav {
        Nav::Snapshots => "Snapshots",
        Nav::Datasets => "Datasets",
        Nav::Scrub => "Scrub",
        Nav::Shares => "Shares",
        Nav::Network => "Interfaces",
        Nav::PoolStatus => "Status",
        Nav::Docker => "Docker",
    };

    let key_style = Style::default().fg(Color::Black).bg(Color::Yellow).bold();
    let desc_style = Style::default().fg(Color::White).bg(Color::Rgb(50, 50, 50));
    let action_style = Style::default().fg(Color::Cyan).bg(Color::Rgb(30, 30, 30)).bold();

    let mut spans = vec![
        Span::styled(" Tab ", key_style),
        Span::styled(format!(" Focus: {:?} ", app.focus), desc_style),
        Span::raw("  "),
        Span::styled(" 1-6 ", key_style),
        Span::styled(format!(" View: {} ", context_name), desc_style),
        Span::raw("  "),
    ];

    if app.nav == Nav::Datasets {
        spans.push(Span::styled(" C ", action_style));
        spans.push(Span::styled(" Create Snapshot ", desc_style));
        spans.push(Span::raw(" "));
        spans.push(Span::styled(" R ", action_style));
        spans.push(Span::styled(" Rename Dataset ", desc_style));
        spans.push(Span::raw("  "));
    }

    let global_keys = vec![
        (" Esc ", "Back"),
        (" Q ", "Quit"),
    ];

    for (key, desc) in global_keys {
        spans.push(Span::styled(key, key_style));
        spans.push(Span::styled(format!(" {} ", desc), desc_style));
        spans.push(Span::raw(" "));
    }

    let footer = Paragraph::new(Line::from(spans))
        .alignment(Alignment::Left)
        .style(Style::default().bg(Color::Reset));

    f.render_widget(footer, area);
}
