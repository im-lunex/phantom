use crate::movement::Movement;
use crate::Document;
use crate::Row;
use crate::Terminal;
use std::env;
use std::time::Duration;
use std::time::Instant;
use termion::color;
use termion::event::Key;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

#[derive(PartialEq, Copy, Clone)]
pub enum SearchDirection {
    Forward,
    Backward,
}

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

struct StatusMessage {
    text: String,
    time: Instant,
}
impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    quit_times: u8,
    highlighted_word: Option<String>,
    mode: Mode,
    pending_keys: String,
    clipboard: Vec<String>,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status =
            String::from("HELP: i=insert | :w=save | :q=quit | /=search | :help for more");

        let document = if let Some(file_name) = args.get(1) {
            let doc = Document::open(file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
            quit_times: QUIT_TIMES,
            highlighted_word: None,
            mode: Mode::Normal,
            pending_keys: String::new(),
            clipboard: Vec::new(),
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            match self.mode {
                Mode::Normal => print!("\x1b[2 q"),
                Mode::Insert => print!("\x1b[5 q"),
            }
            self.document.highlight(
                &self.highlighted_word,
                Some(
                    self.offset
                        .y
                        .saturating_add(self.terminal.size().height as usize),
                ),
            );
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            let line_num_width = 5; // 4 digits + 1 space
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x).saturating_add(line_num_width),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_string());
                return;
            }
            self.document.file_name = new_name;
        }

        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully.".to_string());
        } else {
            self.status_message = StatusMessage::from("Error writing file!".to_string());
        }
    }
    fn search(&mut self) {
        let old_position = self.cursor_position.clone();
        let mut direction = SearchDirection::Forward;
        let query = self
            .prompt(
                "Search (ESC to cancel, Arrows to navigate): ",
                |editor, key, query| {
                    let mut moved = false;
                    match key {
                        Key::Right | Key::Down => {
                            direction = SearchDirection::Forward;
                            editor.move_cursor(Movement::Right);
                            moved = true;
                        }
                        Key::Left | Key::Up => {
                            direction = SearchDirection::Backward;
                            editor.move_cursor(Movement::Left);
                            moved = true;
                        }
                        _ => direction = SearchDirection::Forward,
                    }
                    if let Some(position) =
                        editor
                            .document
                            .find(&query, &editor.cursor_position, direction)
                    {
                        editor.cursor_position = position;
                        editor.scroll();
                    } else if moved {
                        editor.move_cursor(Movement::Left);
                    }
                    editor.highlighted_word = Some(query.to_string());
                },
            )
            .unwrap_or(None);

        if query.is_none() {
            self.cursor_position = old_position;
            self.scroll();
        }
        self.highlighted_word = None;
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match self.mode {
            Mode::Normal => {
                self.pending_keys.push(match pressed_key {
                    Key::Char(c) => c,
                    _ => ' ',
                });
                match pressed_key {
                    Key::Char('i') => self.mode = Mode::Insert,
                    Key::Char('a') => {
                        // Insert after cursor
                        self.move_cursor(Movement::Right);
                        self.mode = Mode::Insert;
                    }
                    Key::Char('A') => {
                        // Insert at end of line
                        self.move_cursor(Movement::EndOfLine);
                        self.move_cursor(Movement::Right);
                        self.mode = Mode::Insert;
                    }
                    Key::Char('o') => {
                        // Insert new line below
                        self.move_cursor(Movement::EndOfLine);
                        self.document.insert(&self.cursor_position, '\n');
                        self.move_cursor(Movement::Down);
                        self.cursor_position.x = 0;
                        self.mode = Mode::Insert;
                    }
                    Key::Char('O') => {
                        // Insert new line above
                        self.cursor_position.x = 0;
                        self.document.insert(&self.cursor_position, '\n');
                        self.mode = Mode::Insert;
                    }
                    Key::Char('h') => self.move_cursor(Movement::Left),
                    Key::Char('j') => self.move_cursor(Movement::Down),
                    Key::Char('k') => self.move_cursor(Movement::Up),
                    Key::Char('l') => self.move_cursor(Movement::Right),
                    Key::Char('w') => self.move_cursor(Movement::NextWord),
                    Key::Char('b') => self.move_cursor(Movement::PrevWord),
                    Key::Char('e') => self.move_cursor(Movement::EndOfWord),
                    Key::Char('0') => self.move_cursor(Movement::StartOfLine),
                    Key::Char('$') => self.move_cursor(Movement::EndOfLine),
                    Key::Char('^') => self.move_cursor(Movement::FirstNonWhitespace),
                    Key::Char('G') => self.move_cursor(Movement::EndOfFile),
                    Key::Ctrl('d') => self.move_cursor(Movement::PageDown),
                    Key::Ctrl('u') => self.move_cursor(Movement::PageUp),
                    Key::Ctrl('s') => self.save(),
                    Key::Char('/') => self.search(),
                    Key::Char('u') => {
                        if let Some(pos) = self.document.undo() {
                            self.cursor_position = pos;
                            self.status_message = StatusMessage::from("Undo".to_string());
                        }
                    }
                    Key::Ctrl('r') => {
                        if let Some(pos) = self.document.redo() {
                            self.cursor_position = pos;
                            self.status_message = StatusMessage::from("Redo".to_string());
                        }
                    }
                    Key::Char('x') => {
                        // Delete character under cursor
                        self.document.delete(&self.cursor_position);
                    }
                    Key::Char('p') => {
                        // Paste below current line
                        if !self.clipboard.is_empty() {
                            let mut y = self.cursor_position.y.saturating_add(1);
                            for line in &self.clipboard {
                                self.document.insert_line(&Position { x: 0, y }, line);
                                y = y.saturating_add(1);
                            }
                            self.cursor_position.y = self.cursor_position.y.saturating_add(1);
                            self.cursor_position.x = 0;
                            self.status_message = StatusMessage::from("Pasted".to_string());
                        }
                    }
                    Key::Char('P') => {
                        // Paste above current line
                        if !self.clipboard.is_empty() {
                            let mut y = self.cursor_position.y;
                            for line in &self.clipboard {
                                self.document.insert_line(&Position { x: 0, y }, line);
                                y = y.saturating_add(1);
                            }
                            self.cursor_position.x = 0;
                            self.status_message = StatusMessage::from("Pasted".to_string());
                        }
                    }
                    Key::Char('y') => {
                        self.pending_keys.push('y');
                    }
                    Key::Char('d') => {
                        self.pending_keys.push('d');
                    }
                    Key::Char(':') => {
                        let command = self.prompt(":", |_, _, _| {}).unwrap_or(None);
                        if let Some(command) = command {
                            let parts: Vec<&str> = command.trim().split_whitespace().collect();
                            if parts.is_empty() {
                                return Ok(());
                            }
                            
                            match parts[0] {
                                "q" => {
                                    if self.document.is_dirty() {
                                        self.status_message = StatusMessage::from(
                                            "No write since last change (use :q! to override)".to_string()
                                        );
                                    } else {
                                        self.should_quit = true;
                                    }
                                }
                                "q!" => self.should_quit = true,
                                "w" => self.save(),
                                "wq" | "x" => {
                                    self.save();
                                    self.should_quit = true;
                                }
                                "wq!" => {
                                    self.save();
                                    self.should_quit = true;
                                }
                                cmd if cmd.starts_with('e') => {
                                    if parts.len() > 1 {
                                        let filename = parts[1..].join(" ");
                                        match Document::open(&filename) {
                                            Ok(doc) => {
                                                self.document = doc;
                                                self.cursor_position = Position::default();
                                                self.offset = Position::default();
                                                self.status_message = StatusMessage::from(
                                                    format!("Opened: {}", filename)
                                                );
                                            }
                                            Err(_) => {
                                                self.status_message = StatusMessage::from(
                                                    format!("Could not open file: {}", filename)
                                                );
                                            }
                                        }
                                    } else {
                                        self.status_message = StatusMessage::from(
                                            "Usage: :e <filename>".to_string()
                                        );
                                    }
                                }
                                cmd if cmd.parse::<usize>().is_ok() => {
                                    // Jump to line number
                                    if let Ok(line_num) = cmd.parse::<usize>() {
                                        let target_line = line_num.saturating_sub(1);
                                        if target_line < self.document.len() {
                                            self.cursor_position.y = target_line;
                                            self.cursor_position.x = 0;
                                            self.scroll();
                                        }
                                    }
                                }
                                "help" | "h" => {
                                    self.status_message = StatusMessage::from(
                                        "Commands: :w :q :wq :q! :e <file> :<number>".to_string()
                                    );
                                }
                                _ => {
                                    self.status_message = StatusMessage::from(format!(
                                        "Unknown command: {} (type :help for commands)",
                                        command
                                    ));
                                }
                            }
                        }
                    }
                    _ => {
                        self.pending_keys.clear();
                    }
                }
                if self.pending_keys.ends_with("gg") {
                    self.move_cursor(Movement::StartOfFile);
                    self.pending_keys.clear();
                } else if self.pending_keys.ends_with("yy") {
                    // Yank current line
                    if let Some(line) = self.document.get_line(self.cursor_position.y) {
                        self.clipboard = vec![line];
                        self.status_message = StatusMessage::from("Yanked line".to_string());
                    }
                    self.pending_keys.clear();
                } else if self.pending_keys.ends_with("dd") {
                    // Delete current line
                    if let Some(line) = self.document.delete_line(&self.cursor_position) {
                        self.clipboard = vec![line];
                        if self.cursor_position.y >= self.document.len() && self.cursor_position.y > 0 {
                            self.cursor_position.y = self.cursor_position.y.saturating_sub(1);
                        }
                        self.cursor_position.x = 0;
                        self.status_message = StatusMessage::from("Deleted line".to_string());
                    }
                    self.pending_keys.clear();
                } else if self.pending_keys.ends_with("d$") || self.pending_keys.ends_with("D") {
                    // Delete to end of line
                    if let Some(deleted) = self.document.delete_to_end_of_line(&self.cursor_position) {
                        self.clipboard = vec![deleted];
                        self.status_message = StatusMessage::from("Deleted to end of line".to_string());
                    }
                    self.pending_keys.clear();
                }
            }
            Mode::Insert => match pressed_key {
                Key::Ctrl('q') => {
                    if self.quit_times > 0 && self.document.is_dirty() {
                        self.status_message = StatusMessage::from(format!(
                            "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                            self.quit_times
                        ));
                        self.quit_times -= 1;
                        return Ok(());
                    }
                    self.should_quit = true
                }
                Key::Esc => self.mode = Mode::Normal,
                Key::Char('\n') => {
                    self.document.insert(&self.cursor_position, '\n');
                    self.move_cursor(Movement::Down);
                    self.cursor_position.x = 0;
                }
                Key::Char(c) => {
                    self.document.insert(&self.cursor_position, c);
                    self.move_cursor(Movement::Right);
                }
                Key::Delete => self.document.delete(&self.cursor_position),
                Key::Backspace => {
                    if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                        self.move_cursor(Movement::Left);
                        self.document.delete(&self.cursor_position);
                    }
                }
                Key::Up => self.move_cursor(Movement::Up),
                Key::Down => self.move_cursor(Movement::Down),
                Key::Left => self.move_cursor(Movement::Left),
                Key::Right => self.move_cursor(Movement::Right),
                _ => (),
            },
        }
        self.scroll();
        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_message = StatusMessage::from(String::new());
        }
        Ok(())
    }
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let line_num_width = 5; // 4 digits + 1 space
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let visible_width = width.saturating_sub(line_num_width);
        
        if y < self.offset.y {
            self.offset.y = y;
        } else if y >= self.offset.y.saturating_add(height) {
            self.offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < self.offset.x {
            self.offset.x = x;
        } else if x >= self.offset.x.saturating_add(visible_width) {
            self.offset.x = x.saturating_sub(visible_width).saturating_add(1);
        }
    }
    fn move_cursor(&mut self, movement: Movement) {
        let Position { x, y } = self.cursor_position;
        let lines = self.document.lines();
        let (new_x, new_y) = movement.execute(x, y, &lines);

        let mut x_pos = new_x;
        let y_pos = new_y;

        let width = if let Some(row) = self.document.row(y_pos) {
            row.len()
        } else {
            0
        };
        if x_pos > width {
            x_pos = width;
        }

        self.cursor_position = Position { x: x_pos, y: y_pos }
    }
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Phantom editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        #[allow(clippy::integer_arithmetic, clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
    pub fn draw_row(&self, row: &Row, line_number: usize) {
        let width = self.terminal.size().width as usize;
        let line_num_width = 4;
        
        // Print line number
        print!("{:>4} ", line_number);
        
        let start = self.offset.x;
        let end = self.offset.x.saturating_add(width.saturating_sub(line_num_width + 1));
        let row = row.render(start, end);
        println!("{}\r", row)
    }
    #[allow(clippy::integer_division, clippy::integer_arithmetic)]
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            let file_row = self.offset.y.saturating_add(terminal_row as usize);
            if let Some(row) = self.document.row(file_row) {
                self.draw_row(row, file_row.saturating_add(1));
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.size().width as usize;
        let modified_indicator = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };
        
        let mode_indicator = match self.mode {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
        };

        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }
        status = format!(
            "{} - {} lines{} - {}",
            file_name,
            self.document.len(),
            modified_indicator,
            mode_indicator
        );

        let line_indicator = format!(
            "{} | {}/{}",
            self.document.file_type(),
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        #[allow(clippy::integer_arithmetic)]
        let len = status.len() + line_indicator.len();
        status.push_str(&" ".repeat(width.saturating_sub(len)));
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }
    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }
    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, std::io::Error>
    where
        C: FnMut(&mut Self, Key, &String),
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            let key = Terminal::read_key()?;
            match key {
                Key::Backspace => result.truncate(result.len().saturating_sub(1)),
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }
            callback(self, key, &result);
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
