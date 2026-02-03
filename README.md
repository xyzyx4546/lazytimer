# lazytimer

A terminal-based speedcubing timer written in Rust.

![Screenshot](screenshot.png)

## Features

- Timer with inspection mode
- Support for multiple puzzle types (2x2, 3x3, 4x4, 5x5, Skewb, Pyraminx)
- Solve statistics & graph
- Scramble generation
- Data persistence

## Installation

Make sure your terminal supports [progressive keyboard enhancements](https://sw.kovidgoyal.net/kitty/keyboard-protocol/) (also known as the Kitty keyboard protocol).

You can then install lazytimer using `cargo`

```bash
cargo install lazytimer
```

## Keybinds

| Key   | Action                  |
| ----- | ----------------------- |
| ?     | Show keybinds           |
| q     | Quit                    |
| Esc   | Close popup             |
| Enter | Confirm                 |
| Space | Start/stop timer        |
| h / ← | Previous puzzle type    |
| j / ↓ | Previous solve          |
| k / ↑ | Next solve              |
| l / → | Next puzzle type        |
| g     | Go to first solve       |
| G     | Go to last solve        |
| i     | Show solve details      |
| +     | Toggle +2 penalty       |
| -     | Toggle DNF penalty      |
| d     | Delete selected solve   |
