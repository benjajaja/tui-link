//! # [Ratatui] `Link` example
use std::env::args;

use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Stylize;
use ratatui::style::Color;
use ratatui::text::Span;
use ratatui::{Frame, TerminalOptions, Viewport};
use tui_link::Link;

fn main() -> Result<(), std::io::Error> {
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(3),
    });
    let arg1 = args().nth(1);
    let arg2 = args().nth(2);
    let (text, url) = match (arg1.as_deref(), arg2.as_deref()) {
        (Some(text), Some(url)) => (text, url),
        (Some(text), None) => (text, "https://ratatui.rs"),
        _ => ("Click me!", "https://ratatui.rs"),
    };
    terminal.draw(|frame| render(frame, (text, url)))?;
    ratatui::restore();
    println!();
    Ok(())
}

/// Render the UI with a logo.
fn render(frame: &mut Frame, (text, url): (&str, &str)) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
    let [top, bottom] = frame.area().layout(&layout);

    frame.render_widget("Clickable link:", top);
    frame.render_widget(
        Link::new(Span::from(text).fg(Color::Blue).underlined(), url),
        bottom,
    );
}
