use ratatui::widgets::TableState;

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
