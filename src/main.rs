use constants::TICK_RATE;
use game::Game;
use state::GameState;
use std::{io, time::Instant};
use terminal::Terminal;

mod apple;
mod board;
mod constants;
mod game;
mod pause_menu;
mod snake;
mod state;
mod terminal;
mod result_menu;

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new()?;
    let mut last_tick = Instant::now();

    let mut game = Game::new();

    while game.state != GameState::Terminated {
        terminal.draw(|f| {
            f.render_stateful_widget(game.clone(), f.size(), &mut game.state);
        })?;

        let timeout = TICK_RATE.saturating_sub(last_tick.elapsed());
        game.input(timeout)?;

        if last_tick.elapsed() >= TICK_RATE {
            game.update();

            last_tick = Instant::now();
        }
    }

    Ok(())
}
