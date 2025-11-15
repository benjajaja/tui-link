# tui-link

### ⚠️ Works only on a PR branch of ratatui

https://github.com/ratatui/ratatui/pull/1605

### What it does

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

![capture](https://github.com/user-attachments/assets/40ac5f9e-9025-4bd9-8e88-b5c5a235b8ec)

The link renders just the text, marked with the URL, so that when clicked, the terminal emulator 
opens the link in the browser.
The terminal emulator may also decorate the text when hovered (e.g. Kitty uses squiggly lines, see
screenshot).
Exact implementation details are up to the terminal.

### Why

URLs are already clickable in most terminals, which saves you needing to tediously select the URL
text and then copying and pasting.

But with TUIs, sometimes URLs get line-broken or wrapped, but not at the edge of the terminal:

```
╔═ box ══════╗
║            ║
║http://examp║
║le.com      ║
║            ║
╚════════════╝
```

This would either do nothing or open `http::/examp`. The terminal emulator has no insight into the
fact that the URL has been wrapped.

The URL could also be partially hidden, to further demonstrate the problem:

```
╔═ file1 ════╗
║          ╔═ file2 ═══╗
║http://exa║Lorem ipsum║
║le.com    ║ dolor sit ║
║          ║amet, conse║
╚══════════║ctetur adip║
           ╚═══════════╝
```


### How

Uses "OSC (Operation System Command) 8" escape sequence and `CellDiffOption::ForcedWidth` to 
squeeze it all into the first cell of the buffer at the area.

```
OSC 8 ; params ; URI ST LINK_TEXT OSC 8 ; ; ST
```

[OSC 8 adoption in terminal emulators](https://github.com/Alhadis/OSC8-Adoption)

