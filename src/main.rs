use std::env::split_paths;
use std::error::Error;
use std::io;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::{Frame, Terminal};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

fn main() -> Result<(), Box<dyn Error>> {
    // startup
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let app_result = main_loop(&mut terminal);

    // teardown
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(e) = app_result {
        eprintln!("{e:?}");
    }

    Ok(())
}

fn main_loop<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue
            }

            if key.code == KeyCode::Char('q') {
                return Ok(())
            }
        }
    }
}

fn render(frame: &mut Frame) {
    let outer_block = Block::default().borders(Borders::ALL);
    frame.render_widget(outer_block, frame.area());
    
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1), Constraint::Max(1)])
        .split(frame.area());
    
    let text = Paragraph::new(
        Span::styled("Gee Whiz! press 'q' to quit", Style::default().fg(Color::Yellow))
    ).block(Block::default()).centered();
     
    frame.render_widget(text, layout[1]);
}