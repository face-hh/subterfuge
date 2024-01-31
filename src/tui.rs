use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{stdout, Result};

use ratatui::{prelude::*, widgets::*};

fn create_span(content: &str, color: Color, modifier: Modifier) -> Span {
    Span::styled(
        content,
        Style::new()
            .fg(color)
            .add_modifier(modifier),
    )
}

fn create_text(spans: Vec<Span>) -> Text {
    Text::from(vec![Line::from(spans)])
}

pub fn display_errors(errors: Vec<String>) {
    if errors.len() == 0 { return };

    for error in errors.iter().rev() {
        println!("{}", error);
    }
}

pub fn init() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let areas = Layout::new(
                Direction::Vertical,
                [
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(0),
                ],
            )
            .split(frame.size());

            let info = "Welcome! Use \":h\" for an introduction!";
            let eyes = vec!["𓃑", "𐂃", "𓃐", "𓃍, "];

            let current_eyes = eyes.get(1).unwrap();
            let stars = "𓃐".repeat(info.len());

            let title = format!("{current_eyes}  Subterfuge");

            let comment_border = format!("|𓃍{stars}|");
            let comment_text =format!("| {info}");
            
            let span1 = create_span("𓃍𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃏𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃏𓃍𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃏𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃏𓃍𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃏𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃏𓃍𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃏𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃐𓃐𓃍𓃐𓃐𓃐𓃏", Color::Indexed(196), Modifier::BOLD);
            let span2 = create_span("𓃑   ", Color::Indexed(196), Modifier::BOLD);
            let span3 = create_span(&title, Color::Indexed(210), Modifier::BOLD);
            let span4 = create_span(&comment_border, Color::Indexed(8), Modifier::ITALIC);
            let span5 = create_span(&comment_text, Color::Indexed(8), Modifier::ITALIC);
            
            let text1 = create_text(vec![span1.clone()]);
            let text2 = create_text(vec![span2.clone(), span3.clone()]);
            let text3 = create_text(vec![span2.clone(), span4.clone()]);
            let text4 = create_text(vec![span2.clone(), span5.clone()]);
            let newline = create_text(vec![span2.clone()]);
            
            let texts = vec![text1.clone(), text2, text3.clone(), text4, text3, newline.clone(), newline.clone(), newline.clone(), text1];
            
            for (i, text) in texts.iter().enumerate() {
                frame.render_widget(Paragraph::new(text.clone()), areas[i]);
            }
            
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
