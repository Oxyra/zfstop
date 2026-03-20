use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::App;
use crate::app::View;

pub fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let context = match app.view {
        View::Snapshots => "Snapshots",
        View::DatasetDetails => "Datasets",
        View::ScrubStatus => "Scrub",
        View::Shares => "Shares",
    };

    let key_style = Style::default().fg(Color::Black).bg(Color::Yellow).bold();
    let desc_style = Style::default().fg(Color::White).bg(Color::Rgb(50, 50, 50));

    let mut spans = vec![
        Span::styled(" ▲▼ ", key_style),
        Span::styled(format!(" {} ", context), desc_style),
        Span::raw(" "), // Spacer
    ];

    let hotkeys = vec![
        (" S ", "Snapshots"),
        (" s ", "Scrub"),
        (" N ", "Shares"),
        (" ↵ ", "Status"),
        (" Esc ", "Back"),
        (" Q ", "Quit"),
    ];

    for (key, desc) in hotkeys {
        spans.push(Span::styled(key, key_style));
        spans.push(Span::styled(format!(" {} ", desc), desc_style));
        spans.push(Span::raw(" "));
    }

    let footer = Paragraph::new(Line::from(spans))
        .alignment(Alignment::Center)
        .style(Style::default().bg(Color::Reset));

    f.render_widget(footer, area);
}
