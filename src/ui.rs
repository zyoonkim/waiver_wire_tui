use crate::app::{App, CurrentWidget};
use ratatui::{
    Frame,
    layout::{self, Constraint, Direction, HorizontalAlignment::Center, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());
    let block = Block::bordered()
        .title("Waiver Wire Watcher")
        .title_alignment(Center);
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
    state.select(Some(app.selected_wishlist()));

    let mut items: Vec<ListItem> = Vec::new();
    for player in app.wishlist() {
        items.push(ListItem::new(format!(
            "{} {}",
            player.first_name, player.last_name
        )));
    }
    let selected = app.current_widget() == CurrentWidget::Wishlist;
    let list = List::new(items)
        .block(
            Block::bordered()
                .title("Wishlist")
                .border_style(border_style(selected)),
        )
        .highlight_symbol("> ");
    frame.render_stateful_widget(list, rect, &mut state);
}

fn draw_search(frame: &mut Frame, rect: Rect, app: &App) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(rect);
    let binding = app.searchbox_input();
    let searchbar = Paragraph::new(binding).block(Block::bordered().title("Search"));

    frame.render_widget(searchbar, areas[0]);

    let mut state = ListState::default();
    state.select(Some(app.selected_player()));

    let mut items: Vec<ListItem> = Vec::new();
    for player in app.search_results().clone() {
        items.push(ListItem::new(format!(
            "{} {}",
            player.first_name, player.last_name
        )));
    }
    let selected = app.current_widget() == CurrentWidget::SearchBox;
    let list = List::new(items)
        .highlight_symbol("> ")
        .block(Block::bordered().border_style(border_style(selected)));

    frame.render_stateful_widget(list, areas[1], &mut state);
}

fn border_style(selected: bool) -> Style {
    if selected {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    }
}
