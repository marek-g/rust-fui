pub struct TextBuffer {
    content: String,
    cursor_pos: usize,      // index measured in chars (chars().count())
    selection_start: usize, // start of the selection, no selection if equals cursor_pos
}

impl TextBuffer {
    pub fn new(initial_text: String) -> Self {
        let len = initial_text.chars().count();
        Self {
            content: initial_text,
            cursor_pos: len,
            selection_start: len,
        }
    }

    pub fn get_text(&self) -> &str {
        &self.content
    }

    pub fn set_text(&mut self, text: String) {
        self.content = text;
        let len = self.content.chars().count();
        self.cursor_pos = self.cursor_pos.min(len);
        self.selection_start = self.selection_start.min(len);
    }

    pub fn get_cursor(&self) -> usize {
        self.cursor_pos
    }

    pub fn set_cursor(&mut self, pos: usize, extend_selection: bool) {
        let len = self.content.chars().count();
        self.cursor_pos = pos.min(len);
        if !extend_selection {
            self.selection_start = self.cursor_pos;
        }
    }

    pub fn get_selection_start(&self) -> usize {
        self.selection_start
    }

    pub fn get_selection(&self) -> Option<(usize, usize)> {
        if self.selection_start == self.cursor_pos {
            None
        } else {
            let start = self.selection_start.min(self.cursor_pos);
            let end = self.selection_start.max(self.cursor_pos);
            Some((start, end))
        }
    }

    pub fn select_all(&mut self) {
        self.selection_start = 0;
        self.cursor_pos = self.content.chars().count();
    }

    pub fn get_selected_string(&self) -> Option<String> {
        self.get_selection()
            .map(|(start, end)| self.content.chars().skip(start).take(end - start).collect())
    }

    pub fn delete_selection(&mut self) -> bool {
        if let Some((start, end)) = self.get_selection() {
            let new_content: String = self
                .content
                .chars()
                .take(start)
                .chain(self.content.chars().skip(end))
                .collect();
            self.content = new_content;
            self.cursor_pos = start;
            self.selection_start = start;
            return true;
        }
        false
    }

    pub fn insert_str(&mut self, s: &str) {
        self.delete_selection();
        let start_pos = self.cursor_pos;
        let new_content: String = self
            .content
            .chars()
            .take(start_pos)
            .chain(s.chars())
            .chain(self.content.chars().skip(start_pos))
            .collect();
        self.content = new_content;
        self.cursor_pos = start_pos + s.chars().count();
        self.selection_start = self.cursor_pos;
    }

    pub fn backspace(&mut self) {
        if !self.delete_selection() && self.cursor_pos > 0 {
            let pos = self.cursor_pos - 1;
            let new_content: String = self
                .content
                .chars()
                .take(pos)
                .chain(self.content.chars().skip(pos + 1))
                .collect();
            self.content = new_content;
            self.cursor_pos = pos;
            self.selection_start = pos;
        }
    }

    pub fn delete(&mut self) {
        if !self.delete_selection() {
            let len = self.content.chars().count();
            if self.cursor_pos < len {
                let pos = self.cursor_pos;
                let new_content: String = self
                    .content
                    .chars()
                    .take(pos)
                    .chain(self.content.chars().skip(pos + 1))
                    .collect();
                self.content = new_content;
            }
        }
    }
}
