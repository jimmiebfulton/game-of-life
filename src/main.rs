mod engine;

use std::io::{stdout, Write, Error};
use crossterm::{
    event,
    execute, queue,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, MoveTo},
    style::{self, Color, Stylize, ResetColor, SetBackgroundColor, SetForegroundColor},
    Result,
};
use crossterm::event::{Event, KeyEvent, KeyCode, poll, read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crate::engine::{GameOfLife, GameMatrix, CellState};
use std::time::Duration;
use rand::prelude::*;

fn main() -> Result<()> {
    let sparcity = 7;
    let sleep = 50;

    execute!(stdout(), EnterAlternateScreen)?;

    let (rows, columns) = crossterm::terminal::size().map(|(x, y)| (x as usize, y as usize))?;

    let mut game = GameOfLife::new(rows, columns);

    let (rows, columns) = game.shape();
    for row in 0..rows {
        for column in 0..columns {
            let x: u8 = rand::random();
            if x % sparcity == 0 {
                game.current_mut().set_state((row, column), CellState::Alive);
            }
        }
    }

    // game.current_mut().set_state((20, 5), CellState::Alive);
    // game.current_mut().set_state((20, 6), CellState::Alive);
    // game.current_mut().set_state((20, 7), CellState::Alive);
    // game.current_mut().set_state((19, 7), CellState::Alive);
    // game.current_mut().set_state((18, 6), CellState::Alive);

    enable_raw_mode()?;

    let mut paused = false;
    loop {
        match check_commands() {
            Ok(Some(Command::Paused)) => {
                paused = !paused;
            }
            Ok(None) => {
            }
            _ => { break; }
        }

        if !paused {
            render(&mut game, &mut stdout());
            game.tick();
        }

        std::thread::sleep(Duration::from_millis(sleep));
    }
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, Hide)?;

    Ok(())
}

fn render<W>(game: &GameOfLife, write: &mut W) -> Result<()>
    where W: Write
{
    let (rows, columns) = game.shape();

    for row in 0..rows {
        for column in 0..columns {
            let previous_state = game.previous().get_state((row, column));
            let current_state = game.current().get_state((row, column));
            if previous_state != current_state {
                queue!(write, MoveTo(row as u16, column as u16))?;
                match current_state {
                    CellState::Alive => {
                        // queue!(write, SetForegroundColor(Color::White))?;
                        queue!(write, style::PrintStyledContent( "█".white()))?;
                    }
                    CellState::Dead => {
                        // queue!(write, SetForegroundColor(Color::Black))?;
                        queue!(write, style::PrintStyledContent( "█".black()))?;
                    }
                }
            }
        }
    }
    write.flush()?;
    Ok(())
}

enum Command {
    Paused,
    Quit,
}

fn check_commands() -> Result<Option<Command>> {
    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(0))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(KeyEvent { code, modifiers: _ }) if code == KeyCode::Char(' ') => {
                    return Ok(Some(Command::Paused));
                }
                Event::Key(KeyEvent { code, modifiers: _ }) if code == KeyCode::Char('q') => {
                    return Ok(Some(Command::Quit));
                }
                _ => return Ok(None)
            }
        } else {
            return Ok(None);
        }
    }
}

pub fn read_char() -> Result<char> {
    loop {
        if let Event::Key(KeyEvent {
                              code: KeyCode::Char(c),
                              ..
                          }) = event::read()?
        {
            return Ok(c);
        }
    }
}
