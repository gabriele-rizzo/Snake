use std::collections::VecDeque;

use rand::Rng;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Widget},
};

use crate::{apple::Apple, state::GameState, terminal::Terminal};

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Snake {
    pub body: VecDeque<(u16, u16)>,
    direction: Option<Direction>,
}

impl Snake {
    pub fn new(apple: &Apple) -> Self {
        let mut generator = rand::thread_rng();
        let area = Terminal::size();

        let mut x = generator.gen_range(1..area.0 - 1);
        let mut y = generator.gen_range(1..area.1 - 1);

        while x == apple.position.0 {
            x = generator.gen_range(1..area.0 - 1);
        }

        while y == apple.position.0 {
            y = generator.gen_range(1..area.1 - 1);
        }

        Self {
            body: VecDeque::from([(x, y)]),
            direction: None,
        }
    }

    pub fn change_direction(&mut self, direction: Option<Direction>) {
        if direction.is_none() || self.direction.is_none() {
            self.direction = direction.clone();
        }

        let direction = direction.unwrap();
        match self.direction.clone().unwrap() {
            Direction::Up => {
                if direction != Direction::Down {
                    self.direction = Some(direction)
                }
            }
            Direction::Down => {
                if direction != Direction::Up {
                    self.direction = Some(direction)
                }
            }
            Direction::Left => {
                if direction != Direction::Right {
                    self.direction = Some(direction)
                }
            }
            Direction::Right => {
                if direction != Direction::Left {
                    self.direction = Some(direction)
                }
            }
        }
    }

    pub fn moving(&self) -> bool {
        self.direction.is_some()
    }

    pub fn update(&mut self, apple: &Apple) -> (GameState, bool) {
        let size = Terminal::size();
        let old = self.body.front().unwrap();
        let mut new = old.clone();

        match self.direction.clone().unwrap() {
            Direction::Up => new.1 -= 1,
            Direction::Down => new.1 += 1,
            Direction::Left => new.0 -= 1,
            Direction::Right => new.0 += 1,
        }

        if self.body.contains(&new) {
            return (GameState::Finished(false), false);
        }

        self.body.push_front(new);
        self.body.pop_back();

        let inside_bounds = new.0 != 0 && new.0 != size.0 - 1 && new.1 != 0 && new.1 != size.1 - 1;
        let ate = self.body.front().unwrap() == &apple.position;

        if ate {
            self.body.push_front(apple.position);
        }

        let new_state = if !inside_bounds {
            GameState::Finished(false)
        } else if self.body.len() == (size.0 as usize - 2) * (size.1 as usize - 2) {
            GameState::Finished(true)
        } else {
            GameState::Running
        };

        (new_state, ate)
    }
}

impl Widget for Snake {
    fn render(self, _: Rect, buf: &mut Buffer) {
        let mut body = self.body.clone();
        let head = body.pop_front().unwrap();

        Paragraph::new("▮".bold()).render(Rect::new(head.0, head.1, 1, 1), buf);

        for (x, y) in body {
            Paragraph::new("▮".bold()).render(Rect::new(x, y, 1, 1), buf);
        }
    }
}
