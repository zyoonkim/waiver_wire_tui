// mod api;
mod apistub;
mod app;
mod input;
mod models;
mod search;
mod ui;

use std::io;

// use api::ApiClient;
use apistub::ApiClient;
use app::App;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use input::InputResult;
use ratatui::{Terminal, backend::CrosstermBackend};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut api = ApiClient::new();

    let players = api.get_players().await.unwrap();
    let wishlist = api.get_wishlist(&players).await.unwrap();

    let mut app = App::new(players, wishlist);

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app, &mut api).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    api: &mut ApiClient,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| {
            ui::draw(frame, app);
        })?;

        if let Event::Key(key) = event::read()? {
            match input::handle_key(app, key) {
                InputResult::Continue => {}

                InputResult::Quit => break,

                InputResult::AddToWishlist => {
                    if let Some(player_id) = app
                        .selected_search_player()
                        .map(|player| player.player_id.clone())
                    {
                        api.add_to_wishlist(&player_id).await.unwrap();

                        let wishlist = api.get_wishlist(&app.players).await.unwrap();

                        app.wishlist = wishlist;
                    }
                }

                InputResult::RemoveFromWishlist => {
                    if let Some(player_id) = app
                        .selected_wishlist_player()
                        .map(|player| player.player_id.clone())
                    {
                        api.remove_from_wishlist(&player_id).await.unwrap();

                        let wishlist = api.get_wishlist(&app.players).await.unwrap();

                        app.wishlist = wishlist;
                    }
                }
            }
        }
    }

    Ok(())
}
