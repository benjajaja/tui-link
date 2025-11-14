#![no_std]
//! The [`Link`] widget renders a clickable link

extern crate alloc;

use alloc::string::String;

use ratatui_core::buffer::Buffer;
use ratatui_core::buffer::CellDiffOption;
use ratatui_core::layout::Rect;
use ratatui_core::text::Span;
use ratatui_core::widgets::Widget;
use unicode_width::UnicodeWidthStr;

/// Displays a clickable link
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Link<'a> {
    text: Span<'a>,
    url: &'a str,
}

impl<'a> Link<'a> {
    /// Create a new Link widget
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::{Link, LinkSize};
    ///
    /// let logo = Link::new(
    ///     Span::from("ratatui website").fg(Color::Blue).underlined(),
    ///     "https://ratatui.rs",
    /// );
    /// ```
    pub const fn new(text: Span<'a>, url: &'a str) -> Self {
        Self { text, url }
    }
}

impl<'a> Widget for Link<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(cell) = buf.cell_mut(area) else {
            return;
        };
        let capacity = 7 + self.url.len() + 2 + self.text.content.len() + 8;
        let mut s = String::with_capacity(capacity);
        s.push_str("\x1b]8;;");
        s.push_str(&self.url);
        s.push_str("\x1b\\");
        s.push_str(&self.text.content);
        s.push_str("\x1b]8;;\x1b\\");
        cell.set_symbol(&s);
        cell.set_style(self.text.style);
        cell.set_diff_option(CellDiffOption::ForcedWidth(self.text.content.width()));
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn new() {
        let link = Link::new(Span::from("text"), "url");
        assert_eq!(link.text.content, "text");
    }
}
