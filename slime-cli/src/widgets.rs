use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, Borders, Paragraph, Row, Table, TableState},
    Frame,
};

pub struct SlimeTable {
    pub select_offset: usize,
    pub content: Vec<String>,
}

impl SlimeTable {
    pub fn new() -> Self {
        Self {
            select_offset: 0,
            content: vec![],
        }
    }

    pub fn update(&mut self, input: &str, choices: &[String], area: &Rect) {
        self.content = slime_core::fuzzy_search(input, choices, (area.height - 3) as usize);
        self.select_offset = std::cmp::min(self.content.len() - 1, self.select_offset);
    }

    pub fn render(&self, frame: &mut Frame, area: &Rect) {
        let table_area = Rect::new(
            0,
            area.height - self.content.len() as u16 - 3,
            area.width,
            area.height - 3,
        );

        let mut table_state = TableState::new();
        table_state.select(self.select());
        let table = create_table(self.content.clone());

        frame.render_stateful_widget(&table, table_area, &mut table_state);
    }

    fn select(&self) -> Option<usize> {
        if self.select_offset < self.content.len() {
            Some(self.content.len() - 1 - self.select_offset)
        } else {
            None
        }
    }
}

pub struct SlimeInput {
    pub content: String,
    pub cursor_loc: u16,
}

impl SlimeInput {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_loc: 0,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: &Rect) {
        let input_area = Rect::new(0, area.height - 3, area.width, 3);
        let input = create_input(&self.content);
        frame.render_widget(&input, input_area);
        frame.set_cursor(self.cursor_loc + 1, area.height - 2);
    }
}

fn create_table<'a>(items: Vec<String>) -> Table<'a> {
    items
        .into_iter()
        .map(|item| Row::new(vec![item]))
        .collect::<Table>()
        .block(Block::default())
        .highlight_style(Style::new().reversed())
        .highlight_symbol("> ")
}

fn create_input(input: &str) -> Paragraph {
    Paragraph::new(input).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::new().blue()),
    )
}

