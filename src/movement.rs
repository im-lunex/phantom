#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Movement {
    // Basic movements
    Left,
    Right,
    Up,
    Down,

    // Word movements
    NextWord,
    PrevWord,
    EndOfWord,

    // Line movements
    StartOfLine,
    EndOfLine,
    FirstNonWhitespace,

    // File movements
    StartOfFile,
    EndOfFile,

    // Page movements
    PageDown,
    PageUp,
    HalfPageDown,
    HalfPageUp,
}

impl Movement {
    pub fn execute(&self, cx: usize, cy: usize, lines: &[String]) -> (usize, usize) {
        match self {
            Movement::Left => (cx.saturating_sub(1), cy),
            Movement::Right => {
                let line = lines.get(cy).map(|l| l.len()).unwrap_or(0);
                (std::cmp::min(cx + 1, line), cy)
            }
            Movement::Up => (cx, cy.saturating_sub(1)),
            Movement::Down => {
                if cy + 1 < lines.len() {
                    (cx, cy + 1)
                } else {
                    (cx, cy)
                }
            }

            Movement::NextWord => Self::next_word(cx, cy, lines),
            Movement::PrevWord => Self::prev_word(cx, cy, lines),
            Movement::EndOfWord => Self::end_of_word(cx, cy, lines),

            Movement::StartOfLine => (0, cy),
            Movement::EndOfLine => {
                let line = lines.get(cy).map(|l| l.len()).unwrap_or(0);
                (line.saturating_sub(1), cy)
            }
            Movement::FirstNonWhitespace => {
                if let Some(line) = lines.get(cy) {
                    let pos = line.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
                    (pos, cy)
                } else {
                    (cx, cy)
                }
            }

            Movement::StartOfFile => (0, 0),
            Movement::EndOfFile => {
                let last_line = lines.len().saturating_sub(1);
                let col = lines
                    .get(last_line)
                    .map(|l| l.len().saturating_sub(1))
                    .unwrap_or(0);
                (col, last_line)
            }

            Movement::PageDown => {
                let page_size = 10; // Adjust based on editor height
                (
                    cx,
                    std::cmp::min(cy + page_size, lines.len().saturating_sub(1)),
                )
            }
            Movement::PageUp => {
                let page_size = 10;
                (cx, cy.saturating_sub(page_size))
            }
            Movement::HalfPageDown => {
                let page_size = 5;
                (
                    cx,
                    std::cmp::min(cy + page_size, lines.len().saturating_sub(1)),
                )
            }
            Movement::HalfPageUp => {
                let page_size = 5;
                (cx, cy.saturating_sub(page_size))
            }
        }
    }

    fn next_word(cx: usize, cy: usize, lines: &[String]) -> (usize, usize) {
        if let Some(line) = lines.get(cy) {
            let rest = line.chars().skip(cx);

            // Skip current word if in middle of word
            let mut pos = cx;
            for ch in rest.clone() {
                if !ch.is_alphanumeric() && ch != '_' {
                    break;
                }
                pos += 1;
            }

            // Skip whitespace
            let rest = line.chars().skip(pos);
            for (i, ch) in rest.enumerate() {
                if ch.is_alphanumeric() || ch == '_' {
                    return (pos + i, cy);
                }
            }
        }
        (cx, cy)
    }

    fn prev_word(cx: usize, cy: usize, lines: &[String]) -> (usize, usize) {
        if let Some(line) = lines.get(cy) {
            if cx == 0 {
                return (cx, cy);
            }

            let line_str: String = line.chars().take(cx).collect();
            let chars: Vec<char> = line_str.chars().collect();

            let mut pos = cx.saturating_sub(1);

            // Skip whitespace backward
            while pos > 0 && !chars[pos].is_alphanumeric() && chars[pos] != '_' {
                pos -= 1;
            }

            // Skip word backward
            while pos > 0 && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                pos -= 1;
            }

            // Move forward once if we stopped at non-word char
            if pos > 0 && !chars[pos].is_alphanumeric() && chars[pos] != '_' {
                pos += 1;
            }

            return (pos, cy);
        }
        (cx, cy)
    }

    fn end_of_word(cx: usize, cy: usize, lines: &[String]) -> (usize, usize) {
        if let Some(line) = lines.get(cy) {
            let rest = line.chars().skip(cx);

            let mut pos = cx;
            let mut found_word = false;

            for ch in rest {
                if ch.is_alphanumeric() || ch == '_' {
                    found_word = true;
                    pos += 1;
                } else if found_word {
                    return (pos.saturating_sub(1), cy);
                } else {
                    pos += 1;
                }
            }

            if found_word {
                return (pos.saturating_sub(1), cy);
            }
        }
        (cx, cy)
    }
}
