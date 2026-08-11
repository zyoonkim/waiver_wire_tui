use crate::app::App;
use ratatui::{
    Frame, layout::{self, Constraint, Direction, HorizontalAlignment::Center, Layout, Rect}, widgets::{Block, List, ListItem, ListState, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());
    let block = Block::bordered().title("Waiver Wire Watcher").title_alignment(Center);
    frame.render_widget(block, areas[0]);
    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(areas[1]);
    draw_wishlist(frame, panes[0], app);
    draw_search(frame, panes[1], app);
}

fn draw_wishlist(frame: &mut Frame, rect: Rect, app: &App) {
    let mut state = ListState::default();
    state.select(Some(app.selected_watchlist));

    let mut items: Vec<ListItem> = Vec::new();
    for player in app.wishlist.clone() {
        items.push(ListItem::new(format!(
            "{} {}",
            player.first_name, player.last_name
        )));
    }
    let list = List::new(items)
        .block(Block::bordered().title("Wishlist"))
        .highlight_symbol("> ");
    frame.render_stateful_widget(list, rect, &mut state);
}

fn draw_search(frame: &mut Frame, rect: Rect, app: &App) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(rect);
    let searchbar =
        Paragraph::new(app.searchbox_input.as_str()).block(Block::bordered().title("Search"));

    frame.render_widget(searchbar, areas[0]);

    let mut state = ListState::default();
    state.select(Some(app.selected_player));

    let mut items: Vec<ListItem> = Vec::new();
    for player in app.search_results.clone() {
        items.push(ListItem::new(format!(
            "{} {}",
            player.first_name, player.last_name
        )));
    }
    let list = List::new(items).highlight_symbol("> ")
        .block(Block::bordered());
    frame.render_stateful_widget(list, areas[1], &mut state);
}
