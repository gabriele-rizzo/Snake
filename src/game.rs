use crate::{
    apple::Apple, board::Board, pause_menu::PauseMenu, result_menu::ResultMenu, snake::Snake,
    state::GameState,
};
use crossterm::event;
use ratatui::prelude::*;
use std::{io, time::Duration};

#[derive(Clone)]
pub struct Game {
    pub state: GameState,
    score: u16,
    apple: Apple,
    snake: Snake,
    best: Option<u16>,
}

impl Game {
    pub fn new() -> Self {
        let apple = Apple::default();

        Self {
            state: GameState::default(),
            snake: Snake::new(&apple),
            score: 1,
            apple,
            best: None,
        }
    }

    pub fn input(&mut self, timeout: Duration) -> io::Result<()> {
        if event::poll(timeout)? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        event::KeyCode::Char('q') => self.state = GameState::Terminated,
                        event::KeyCode::Char('p') => {
                            if self.state == GameState::Running {
                                self.state = GameState::Paused;
                            }
                        }
                        event::KeyCode::Char('r') => match self.state {
                            GameState::Paused => {
                                self.state = GameState::Running;
                                self.snake.change_direction(None);
                            }
                            GameState::Finished(_) => self.reset(),
                            _ => {}
                        },
                        event::KeyCode::Up | event::KeyCode::Char('w') => self
                            .snake
                            .change_direction(Some(crate::snake::Direction::Up)),
                        event::KeyCode::Down | event::KeyCode::Char('s') => self
                            .snake
                            .change_direction(Some(crate::snake::Direction::Down)),
                        event::KeyCode::Left | event::KeyCode::Char('a') => self
                            .snake
                            .change_direction(Some(crate::snake::Direction::Left)),
                        event::KeyCode::Right | event::KeyCode::Char('d') => self
                            .snake
                            .change_direction(Some(crate::snake::Direction::Right)),
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    pub fn update(&mut self) {
        if self.state != GameState::Running || !self.snake.moving() {
            return;
        }

        let (state, ate) = self.snake.update(&self.apple);
        self.state = state;

        if ate {
            self.apple = Apple::default();
            self.score += 1;
        }

        if self.state == GameState::Finished(true) || self.state == GameState::Finished(false) {
            let best = self.best.unwrap_or(0);

            if self.score > best {
                self.best = Some(self.score);
            }
        }
    }

    fn reset(&mut self) {
        self.apple = Apple::default();
        self.snake = Snake::new(&self.apple);
        self.state = GameState::Running;
        self.score = 1;
    }
}

impl StatefulWidget for Game {
    type State = GameState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Board {
            score: self.score,
            best: self.best,
        }
        .render(area, buf);

        self.apple.render(area, buf);
        self.snake.render(area, buf);

        match *state {
            GameState::Paused => PauseMenu.render(area, buf),
            GameState::Finished(state) => ResultMenu { state }.render(area, buf),
            _ => {}
        };
    }
}
