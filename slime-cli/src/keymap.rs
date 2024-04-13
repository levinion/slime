use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::app::App;

pub enum ProcessState {
    Exit,
    Update,
    Render,
    DoNothing,
    Output(String),
}

impl App {
    pub fn process_key(&mut self, key: KeyEvent) -> ProcessState {
        if key.kind != KeyEventKind::Press {
            return ProcessState::DoNothing;
        }
        match key.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Char(ch) => self.input_char(ch),
            KeyCode::Backspace => self.delete_before(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            KeyCode::Delete => self.delete_after(),
            KeyCode::End => self.move_cursor_end(),
            KeyCode::Up => self.select_up(),
            KeyCode::Down => self.select_down(),
            KeyCode::Enter => self.enter(),
            _ => ProcessState::DoNothing,
        }
    }

    fn move_cursor_left(&mut self) -> ProcessState {
        if self.input.cursor_loc > 0 {
            self.input.cursor_loc -= 1;
        }
        ProcessState::Render
    }

    fn move_cursor_right(&mut self) -> ProcessState {
        if self.input.cursor_loc < self.input.content.len() as _ {
            self.input.cursor_loc += 1;
        }
        ProcessState::Render
    }

    fn input_char(&mut self, ch: char) -> ProcessState {
        self.input.content.push(ch);
        self.move_cursor_right();
        ProcessState::Update
    }

    fn delete_before(&mut self) -> ProcessState {
        if self.input.cursor_loc > 0 {
            self.input
                .content
                .remove(self.input.cursor_loc as usize - 1);
            self.input.cursor_loc -= 1;
        }
        ProcessState::Update
    }

    fn delete_after(&mut self) -> ProcessState {
        if self.input.cursor_loc < self.input.content.len() as _ {
            self.input.content.remove(self.input.cursor_loc as _);
        }
        ProcessState::Update
    }

    fn move_cursor_end(&mut self) -> ProcessState {
        self.input.cursor_loc = self.input.content.len() as _;
        ProcessState::Render
    }

    fn select_up(&mut self) -> ProcessState {
        if self.table.select_offset < self.table.content.len() - 1 {
            self.table.select_offset += 1;
        }
        ProcessState::Render
    }

    fn select_down(&mut self) -> ProcessState {
        if self.table.select_offset > 0 {
            self.table.select_offset -= 1;
        }
        ProcessState::Render
    }

    fn exit(&self) -> ProcessState {
        ProcessState::Exit
    }

    fn enter(&self) -> ProcessState {
        ProcessState::Output(
            self.table
                .content
                .get(self.table.select_offset)
                .unwrap()
                .clone(),
        )
    }
}
