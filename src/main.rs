use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::{Frame, Terminal};
use std::error::Error;
use std::io;
use std::process::Command;

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

pub struct App {
    message: String,
    status: Option<String>,
}

fn main_loop<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App {
        message: String::from("Gee Whiz!"),
        status: None,
    };
    loop {
        terminal.draw(|frame| render(frame, &app.message))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char(' ') => run_cmd(&mut app),
                _ => {}
            };

            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn run_cmd(app: &mut App) {
    let output = Command::new("ls").output().expect("ls failed to execute");

    if output.status.success() {
        app.message = String::from_utf8(output.stdout).unwrap()
    } else {
        app.message = format!("ERR: {:?}", output.stderr)
    }
}

#[cfg(test)]
mod test_run_cmd {
    use std::process::Command;

    #[test]
    fn test_run_cmd() {
        let output = Command::new("ls").output().expect("ls failed to execute");

        assert!(output.status.success());
        assert_ne!
        (String::from_utf8(output.stdout), Ok(String::from("")));
    }
}

fn render(frame: &mut Frame, message: &str) {
    let outer_block = Block::default().borders(Borders::ALL);
    frame.render_widget(outer_block, frame.area());

    let text_parts: Vec<&str> = message.trim().split('\n').collect();

    let mut lines = Vec::new();
    for line in text_parts {
        lines.push(ListItem::new(
            Line::styled(line, Style::default().fg(Color::Yellow)).alignment(Alignment::Center),
        ))
    }
    lines.push(ListItem::new(
        Line::styled(
            String::from("press 'q' to quit"),
            Style::default().fg(Color::Yellow),
        )
        .alignment(Alignment::Center),
    ));

    let lines = List::new(lines).block(Block::default());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1), Constraint::Max(lines.len() as u16)])
        .split(frame.area());

    frame.render_widget(lines, layout[1]);
}
