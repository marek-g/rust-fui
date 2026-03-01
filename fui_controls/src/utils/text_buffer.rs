use fui_core::Property;

pub struct TextBuffer {
    content: Property<String>,
    cursor_pos: usize,      // index measured in chars (chars().count())
    selection_start: usize, // start of the selection, no selection if equals cursor_pos
}

impl TextBuffer {
    pub fn new(content: Property<String>) -> Self {
        let len = content.read().chars().count();
        Self {
            content,
            cursor_pos: len,
            selection_start: len,
        }
    }

    pub fn get_text_property(&self) -> &Property<String> {
        &self.content
    }

    pub fn get_cursor(&self) -> usize {
        self.cursor_pos
    }

    pub fn set_cursor(&mut self, pos: usize, extend_selection: bool) {
        let len = self.content.read().chars().count();
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
        self.cursor_pos = self.content.read().chars().count();
    }

    pub fn get_selected_string(&self) -> Option<String> {
        self.get_selection().map(|(start, end)| {
            self.content
                .read()
                .chars()
                .skip(start)
                .take(end - start)
                .collect()
        })
    }

    pub fn delete_selected_text(&mut self) -> Option<String> {
        if let Some((start, end)) = self.get_selection() {
            let selected = self.get_selected_string();
            let mut writer = self.content.write();
            let new_content: String = writer
                .chars()
                .enumerate()
                .filter(|&(i, _)| i < start || i >= end)
                .map(|(_, c)| c)
                .collect();
            *writer = new_content;

            self.cursor_pos = start;
            self.selection_start = start;
            return selected;
        }
        None
    }

    pub fn insert_str(&mut self, s: &str) {
        self.delete_selected_text();
        let start_pos = self.cursor_pos;
        let mut writer = self.content.write();

        let new_content: String = writer
            .chars()
            .take(start_pos)
            .chain(s.chars())
            .chain(writer.chars().skip(start_pos))
            .collect();

        *writer = new_content;
        self.cursor_pos = start_pos + s.chars().count();
        self.selection_start = self.cursor_pos;
    }

    pub fn backspace(&mut self) {
        if self.delete_selected_text().is_none() && self.cursor_pos > 0 {
            let pos = self.cursor_pos - 1;
            let mut writer = self.content.write();

            let new_content: String = writer
                .chars()
                .enumerate()
                .filter(|&(i, _)| i != pos)
                .map(|(_, c)| c)
                .collect();

            *writer = new_content;
            self.cursor_pos = pos;
            self.selection_start = pos;
        }
    }

    pub fn delete(&mut self) {
        if self.delete_selected_text().is_none() {
            let pos = self.cursor_pos;
            let mut writer = self.content.write();
            let len = writer.chars().count();

            if pos < len {
                let new_content: String = writer
                    .chars()
                    .enumerate()
                    .filter(|&(i, _)| i != pos)
                    .map(|(_, c)| c)
                    .collect();
                *writer = new_content;
            }
        }
    }

    pub fn move_word_left(&mut self, extend_selection: bool) {
        let chars = {
            let text = self.content.read();
            text.chars().take(self.cursor_pos).collect::<Vec<_>>()
        };

        if chars.is_empty() {
            return;
        }

        let mut pos = self.cursor_pos;

        // skip initial whitespace to the left
        while pos > 0 && chars[pos - 1].is_whitespace() {
            pos -= 1;
        }
        // skip the word itself
        while pos > 0 && !chars[pos - 1].is_whitespace() {
            pos -= 1;
        }

        self.set_cursor(pos, extend_selection);
    }

    pub fn move_word_right(&mut self, extend_selection: bool) {
        let chars = {
            let text = self.content.read();
            text.chars().collect::<Vec<_>>()
        };

        let mut pos = self.cursor_pos;

        // skip initial whitespace to the right
        while pos < chars.len() && chars[pos].is_whitespace() {
            pos += 1;
        }
        // skip the word itself
        while pos < chars.len() && !chars[pos].is_whitespace() {
            pos += 1;
        }

        self.set_cursor(pos, extend_selection);
    }
}
