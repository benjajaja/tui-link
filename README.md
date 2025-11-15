# tui-link

Clickable link widget for Ratatui programs.

```rust
use tui_link::Link;
use ratatui::text::Span;
use ratatui::prelude::Stylize;

let logo = Link::new(
    Span::from("ratatui website").blue().underlined(),
    "https://ratatui.rs",
);

link.render(area, &mut buf);
```

The link renders just the text. When clicked it opens the link.

![capture](https://github.com/user-attachments/assets/40ac5f9e-9025-4bd9-8e88-b5c5a235b8ec)

Uses "OSC (Operation System Command) 8" escape sequence and `CellDiffOption::ForcedWidth` to 
squeeze it all into the first cell of the buffer at the area.

```
OSC 8 ; params ; URI ST LINK_TEXT OSC 8 ; ; ST
```

[OSC 8 adoption in terminal emulators](https://github.com/Alhadis/OSC8-Adoption)

