use crate::models::Player;
use crate::search;

#[derive(PartialEq, Eq)]
pub enum CurrentWidget {
    Wishlist,
    SearchBox,
}

pub struct App {
    pub current_widget: CurrentWidget,
    pub searchbox_input: String,
    pub players: Vec<Player>,
    pub wishlist: Vec<Player>,
    pub selected_player: usize,
    pub selected_watchlist: usize,
    pub search_results: Vec<Player>,
}

impl App {
    pub fn new(players: Vec<Player>, wishlist: Vec<Player>) -> App {
        App {
            current_widget: CurrentWidget::Wishlist,
            searchbox_input: String::new(),
            players: players,
            wishlist: wishlist,
            selected_player: 0 as usize,
            selected_watchlist: 0 as usize,
            search_results: Vec::new(),
        }
    }

    pub fn next_widget(&mut self) {
        let next_widget = match self.current_widget {
            CurrentWidget::SearchBox => CurrentWidget::Wishlist,
            CurrentWidget::Wishlist => CurrentWidget::SearchBox,
        };
        self.current_widget = next_widget;
        if self.current_widget == CurrentWidget::SearchBox {
            self.search();
        }
    }

    pub fn previous_widget(&mut self) {
        let next_widget = match self.current_widget {
            CurrentWidget::SearchBox => CurrentWidget::Wishlist,
            CurrentWidget::Wishlist => CurrentWidget::SearchBox,
        };
        self.current_widget = next_widget;
        if self.current_widget == CurrentWidget::SearchBox {
            self.search();
        }
    }

    pub fn add_search_char(&mut self, c: char) {
        if self.current_widget != CurrentWidget::SearchBox {
            return;
        }
        self.searchbox_input.push(c);
        self.search();
        self.selected_player = 0;
    }

    pub fn remove_search_char(&mut self) {
        if self.current_widget != CurrentWidget::SearchBox {
            return;
        }
        self.searchbox_input.pop();
        self.search();
        self.selected_player = 0;
    }

    pub fn next_search_result(&mut self) {
        if self.search_results.len() > 0 {
            if self.selected_player < self.search_results.len() - 1 {
                self.selected_player += 1;
            }
        }
    }

    pub fn previous_search_result(&mut self) {
        if self.selected_player > 0 {
            self.selected_player -= 1;
        }
    }

    pub fn next_wishlist(&mut self) {
        if self.wishlist.len() > 0 {
            if self.selected_watchlist < self.wishlist.len() - 1 {
                self.selected_watchlist += 1;
            }
        }
    }

    pub fn previous_wishlist(&mut self) {
        if self.selected_watchlist > 0 {
            self.selected_watchlist -= 1;
        }
    }

    pub fn next_result(&mut self) {
        match self.current_widget {
            CurrentWidget::SearchBox => {
                self.next_search_result();
            }
            CurrentWidget::Wishlist => {
                self.next_wishlist();
            }
        }
    }

    pub fn previous_result(&mut self) {
        match self.current_widget {
            CurrentWidget::SearchBox => {
                self.previous_search_result();
            }
            CurrentWidget::Wishlist => {
                self.previous_wishlist();
            }
        }
    }

    fn search(&mut self) {
        self.search_results = search::search(self.searchbox_input.clone(), &self.players);
    }
}
