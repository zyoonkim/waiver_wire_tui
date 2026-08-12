use crate::models::Player;
use crate::search;

#[derive(Clone, PartialEq, Eq)]
pub enum CurrentWidget {
    Wishlist,
    SearchBox,
}

pub struct App {
    current_widget: CurrentWidget,
    searchbox_input: String,
    players: Vec<Player>,
    wishlist: Vec<Player>,
    selected_player: usize,
    selected_wishlist: usize,
    search_results: Vec<Player>,
}

impl App {
    pub fn new(players: Vec<Player>, wishlist: Vec<Player>) -> App {
        App {
            current_widget: CurrentWidget::Wishlist,
            searchbox_input: String::new(),
            players: players,
            wishlist: wishlist,
            selected_player: 0 as usize,
            selected_wishlist: 0 as usize,
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
            if self.selected_wishlist < self.wishlist.len() - 1 {
                self.selected_wishlist += 1;
            }
        }
    }

    pub fn previous_wishlist(&mut self) {
        if self.selected_wishlist > 0 {
            self.selected_wishlist -= 1;
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

    pub fn selected_search_player(&self) -> Option<&Player> {
        self.search_results.get(self.selected_player)
    }

    pub fn selected_wishlist_player(&self) -> Option<&Player> {
        self.wishlist.get(self.selected_wishlist)
    }

    pub fn current_widget(&self) -> CurrentWidget {
        self.current_widget.clone()
    }

    pub fn searchbox_input(&self) -> String {
        self.searchbox_input.clone()
    }

    pub fn players(&self) -> Vec<Player> {
        self.players.clone()
    }
    pub fn wishlist(&self) -> Vec<Player> {
        self.wishlist.clone()
    }
    pub fn selected_player(&self) -> usize {
        self.selected_player
    }

    pub fn selected_wishlist(&self) -> usize {
        self.selected_wishlist
    }
    pub fn search_results(&self) -> Vec<Player> {
        self.search_results.clone()
    }

    pub fn reset_wishlist(&mut self, new_wishlist: Vec<Player>) {
        self.wishlist = new_wishlist;
    }
}
