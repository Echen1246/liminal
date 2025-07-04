use crate::{config::TerminalConfig, errors::*};
use rgb::RGB8;
use std::collections::VecDeque;
use vte::{Params, Parser, Perform};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalCell {
    pub character: char,
    pub foreground_color: RGB8,
    pub background_color: RGB8,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Default for TerminalCell {
    fn default() -> Self {
        Self {
            character: ' ',
            foreground_color: RGB8::new(200, 200, 200),
            background_color: RGB8::new(0, 0, 0),
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

#[derive(Debug)]
pub struct TerminalBuffer {
    cells: Vec<Vec<TerminalCell>>,
    rows: usize,
    cols: usize,
    cursor_row: usize,
    cursor_col: usize,
    scrollback: VecDeque<Vec<TerminalCell>>,
    scrollback_limit: usize,
}

impl TerminalBuffer {
    pub fn new(rows: usize, cols: usize, scrollback_limit: usize) -> Self {
        let cells = vec![vec![TerminalCell::default(); cols]; rows];
        
        Self {
            cells,
            rows,
            cols,
            cursor_row: 0,
            cursor_col: 0,
            scrollback: VecDeque::with_capacity(scrollback_limit),
            scrollback_limit,
        }
    }
    
    pub fn resize(&mut self, new_rows: usize, new_cols: usize) {
        self.rows = new_rows;
        self.cols = new_cols;
        self.cells.resize(new_rows, vec![TerminalCell::default(); new_cols]);
        
        for row in &mut self.cells {
            row.resize(new_cols, TerminalCell::default());
        }
        
        // Clamp cursor position
        self.cursor_row = self.cursor_row.min(new_rows.saturating_sub(1));
        self.cursor_col = self.cursor_col.min(new_cols.saturating_sub(1));
    }
    
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&TerminalCell> {
        self.cells.get(row)?.get(col)
    }
    
    pub fn set_cell(&mut self, row: usize, col: usize, cell: TerminalCell) {
        if let Some(row_cells) = self.cells.get_mut(row) {
            if let Some(cell_ref) = row_cells.get_mut(col) {
                *cell_ref = cell;
            }
        }
    }
    
    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_row, self.cursor_col)
    }
    
    pub fn move_cursor(&mut self, row: usize, col: usize) {
        self.cursor_row = row.min(self.rows.saturating_sub(1));
        self.cursor_col = col.min(self.cols.saturating_sub(1));
    }
    
    pub fn scroll_up(&mut self, lines: usize) {
        for _ in 0..lines {
            if let Some(line) = self.cells.get(0).cloned() {
                self.scrollback.push_back(line);
                if self.scrollback.len() > self.scrollback_limit {
                    self.scrollback.pop_front();
                }
            }
            
            self.cells.remove(0);
            self.cells.push(vec![TerminalCell::default(); self.cols]);
        }
    }
    
    pub fn get_visible_content(&self) -> &[Vec<TerminalCell>] {
        &self.cells
    }
    
    pub fn get_scrollback(&self) -> &VecDeque<Vec<TerminalCell>> {
        &self.scrollback
    }
}

pub struct Terminal {
    buffer: TerminalBuffer,
    parser: Parser,
    current_style: TerminalCell,
}

impl Terminal {
    pub fn new(config: &TerminalConfig) -> Result<Self> {
        let buffer = TerminalBuffer::new(
            config.rows as usize,
            config.cols as usize,
            config.scrollback_limit,
        );
        
        Ok(Self {
            buffer,
            parser: Parser::new(),
            current_style: TerminalCell::default(),
        })
    }
    
    pub fn process_data(&mut self, data: &[u8]) {
        for &byte in data {
            let mut parser = std::mem::take(&mut self.parser);
            parser.advance(self, byte);
            self.parser = parser;
        }
    }
    
    pub fn resize(&mut self, rows: u32, cols: u32) {
        self.buffer.resize(rows as usize, cols as usize);
    }
    
    pub fn get_buffer(&self) -> &TerminalBuffer {
        &self.buffer
    }
    
    pub fn get_cursor_position(&self) -> (usize, usize) {
        self.buffer.cursor_position()
    }
}

impl Perform for Terminal {
    fn print(&mut self, c: char) {
        let (row, col) = self.buffer.cursor_position();
        
        let mut cell = self.current_style;
        cell.character = c;
        
        self.buffer.set_cell(row, col, cell);
        
        // Move cursor forward
        if col + 1 < self.buffer.cols {
            self.buffer.move_cursor(row, col + 1);
        } else if row + 1 < self.buffer.rows {
            self.buffer.move_cursor(row + 1, 0);
        } else {
            // Scroll up
            self.buffer.scroll_up(1);
            self.buffer.move_cursor(self.buffer.rows - 1, 0);
        }
    }
    
    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                // Line feed
                let (row, col) = self.buffer.cursor_position();
                if row + 1 < self.buffer.rows {
                    self.buffer.move_cursor(row + 1, col);
                } else {
                    self.buffer.scroll_up(1);
                }
            }
            b'\r' => {
                // Carriage return
                let (row, _) = self.buffer.cursor_position();
                self.buffer.move_cursor(row, 0);
            }
            b'\t' => {
                // Tab
                let (row, col) = self.buffer.cursor_position();
                let next_tab = ((col / 8) + 1) * 8;
                self.buffer.move_cursor(row, next_tab.min(self.buffer.cols - 1));
            }
            _ => {}
        }
    }
    
    fn csi_dispatch(&mut self, params: &Params, _intermediates: &[u8], _ignore: bool, c: char) {
        match c {
            'A' => {
                // Cursor up
                let lines = params.iter().next().and_then(|p| p.get(0)).map(|&v| v).unwrap_or(1) as usize;
                let (row, col) = self.buffer.cursor_position();
                self.buffer.move_cursor(row.saturating_sub(lines), col);
            }
            'B' => {
                // Cursor down
                let lines = params.iter().next().and_then(|p| p.get(0)).map(|&v| v).unwrap_or(1) as usize;
                let (row, col) = self.buffer.cursor_position();
                self.buffer.move_cursor((row + lines).min(self.buffer.rows - 1), col);
            }
            'C' => {
                // Cursor right
                let cols = params.iter().next().and_then(|p| p.get(0)).map(|&v| v).unwrap_or(1) as usize;
                let (row, col) = self.buffer.cursor_position();
                self.buffer.move_cursor(row, (col + cols).min(self.buffer.cols - 1));
            }
            'D' => {
                // Cursor left
                let cols = params.iter().next().and_then(|p| p.get(0)).map(|&v| v).unwrap_or(1) as usize;
                let (row, col) = self.buffer.cursor_position();
                self.buffer.move_cursor(row, col.saturating_sub(cols));
            }
            'H' => {
                // Cursor position
                let mut param_iter = params.iter();
                let row = param_iter.next()
                    .and_then(|p| p.get(0))
                    .map(|&v| v)
                    .unwrap_or(1)
                    .saturating_sub(1) as usize;
                let col = param_iter.next()
                    .and_then(|p| p.get(0))
                    .map(|&v| v)
                    .unwrap_or(1)
                    .saturating_sub(1) as usize;
                self.buffer.move_cursor(row, col);
            }
            'm' => {
                // SGR (Select Graphic Rendition)
                for param in params.iter() {
                    for &value in param {
                        self.apply_sgr(value);
                    }
                }
            }
            _ => {}
        }
    }
    
    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {
        // ESC sequences
    }
    
    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {}
    fn put(&mut self, _byte: u8) {}
    fn unhook(&mut self) {}
    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {}
}

impl Terminal {
    fn apply_sgr(&mut self, value: u16) {
        match value {
            0 => {
                // Reset
                self.current_style = TerminalCell::default();
            }
            1 => {
                // Bold
                self.current_style.bold = true;
            }
            3 => {
                // Italic
                self.current_style.italic = true;
            }
            4 => {
                // Underline
                self.current_style.underline = true;
            }
            30..=37 => {
                // Foreground colors
                self.current_style.foreground_color = match value {
                    30 => RGB8::new(0, 0, 0),       // Black
                    31 => RGB8::new(205, 49, 49),   // Red
                    32 => RGB8::new(13, 188, 121),  // Green
                    33 => RGB8::new(229, 229, 16),  // Yellow
                    34 => RGB8::new(36, 114, 200),  // Blue
                    35 => RGB8::new(188, 63, 188),  // Magenta
                    36 => RGB8::new(17, 168, 205),  // Cyan
                    37 => RGB8::new(229, 229, 229), // White
                    _ => self.current_style.foreground_color,
                };
            }
            40..=47 => {
                // Background colors
                self.current_style.background_color = match value {
                    40 => RGB8::new(0, 0, 0),       // Black
                    41 => RGB8::new(205, 49, 49),   // Red
                    42 => RGB8::new(13, 188, 121),  // Green
                    43 => RGB8::new(229, 229, 16),  // Yellow
                    44 => RGB8::new(36, 114, 200),  // Blue
                    45 => RGB8::new(188, 63, 188),  // Magenta
                    46 => RGB8::new(17, 168, 205),  // Cyan
                    47 => RGB8::new(229, 229, 229), // White
                    _ => self.current_style.background_color,
                };
            }
            _ => {}
        }
    }
} 