use ratatui::widgets::TableState;
use ratatui::layout::{Constraint, Layout, Rect};

pub fn next_index(state: &mut TableState, len: usize) {
    if len == 0 { return; }

    let i = state.selected().unwrap_or(0);
    let next = (i + 1) % len;
    state.select(Some(next));
}

pub fn previous_index(state: &mut TableState, len: usize) {
    if len == 0 { return; }

    let i = state.selected().unwrap_or(0);
    let prev = if i == 0 { len - 1 } else { i - 1 };
    state.select(Some(prev));
}

pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

