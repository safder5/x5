# Rust Text Editor

A simple terminal-based text editor built in Rust using the Ropey library.

## Features

- **Text Editing**: Insert and delete characters with real-time rendering
- **Auto Line Wrapping**: Automatically wraps to next line when reaching terminal width
- **Cursor Navigation**: Move cursor with arrow keys (up, down, left, right)
- (Incomplete): **Undo/Redo**: Full undo (Ctrl+Z) and redo (Ctrl+Y) support using operation stacking
- **Backspace**: Delete characters and merge lines
- **Enter Key**: Insert new lines at cursor position
- **Terminal-Aware**: Adapts to your terminal width for consistent display

## Controls

- **Arrow Keys**: Navigate cursor
- **Backspace**: Delete character before cursor
- **Enter**: Insert new line
- (Incomplete): **Ctrl+Z**: Undo last operation
- (Incomplete): **Ctrl+Y**: Redo last undone operation
- **CTRL+ q**: Exit editor

## Technical Details

- Uses **Ropey** for efficient text buffer management
- Uses **Crossterm** for terminal manipulation
- Operation-based undo/redo for memory efficiency
- Hard line wrapping at terminal width boundaries(did not implement soft-wrap intentionally)

## Usage

```bash
cargo run
```

## Requirements

- Rust (latest stable)
- Dependencies: `ropey`, `crossterm`
