use std::error::Error;
use std::io;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;

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
    Ok(())
}