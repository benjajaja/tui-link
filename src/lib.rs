#![no_std]
//! The [`Link`] widget renders a clickable link

extern crate alloc;

use core::num::NonZero;

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
    /// use tui_link::Link;
    /// use ratatui::text::Span;
    /// use ratatui::prelude::Stylize;
    ///
    /// let logo = Link::new(
    ///     Span::from("ratatui website").blue().underlined(),
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
        let capacity = 7 + self.url.width() + 2 + self.text.content.width() + 8;
        let mut s = String::with_capacity(capacity);
        s.push_str("\x1b]8;;");
        s.push_str(&self.url);
        s.push_str("\x1b\\");
        s.push_str(&self.text.content);
        s.push_str("\x1b]8;;\x1b\\");
        cell.set_symbol(&s);
        cell.set_style(self.text.style);
        cell.set_diff_option(CellDiffOption::ForcedWidth(
            NonZero::new(self.text.content.width()).unwrap_or_else(|| NonZero::new(1).unwrap()),
        ));
    }
}

#[cfg(test)]
mod tests {
    use ratatui::prelude::*;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn new() {
        let link = Link::new(Span::from("text"), "url");
        assert_eq!(link.text.content, "text");
        assert_eq!(link.url, "url");
    }

    #[rstest]
    fn escape() {
        let link = Link::new(Span::from("Link🔗"), "url");
        let area = Rect::new(0, 0, 10, 1);
        let mut buf = Buffer::empty(area);
        link.render(area, &mut buf);
        let cell = buf.cell((0, 0)).unwrap();
        assert_eq!("\x1b]8;;url\x1b\\Link🔗\x1b]8;;\x1b\\", cell.symbol());
        assert_eq!(
            CellDiffOption::ForcedWidth(NonZero::new(6).unwrap()),
            cell.diff_option
        );
    }
}
