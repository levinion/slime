use std::io::{stdout, Stdout};

use anyhow::Result;
use crossterm::{
    cursor::SetCursorStyle,
    event::{self, DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal};
use tokio::sync::mpsc::Receiver;

use crate::{
    keymap::ProcessState,
    widgets::{SlimeInput, SlimeTable},
};

pub struct App {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub rx: Receiver<String>,
    pub choices: Vec<String>,
    pub table: SlimeTable,
    pub input: SlimeInput,
    pub area: Option<Rect>,
}

impl App {
    pub fn new(rx: Receiver<String>) -> Result<Self> {
        stdout()
            .execute(EnterAlternateScreen)?
            .execute(EnableMouseCapture)?
            .execute(SetCursorStyle::BlinkingBar)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        let table = SlimeTable::new();
        let input = SlimeInput::new();

        Ok(Self {
            terminal,
            rx,
            choices: Vec::new(),
            table,
            input,
            area: None,
        })
    }

    pub fn run(mut self) -> Result<()> {
        self.render()?;
        self.update()?;
        let mut out = None;
        loop {
            match self.process()? {
                ProcessState::Exit => break,
                ProcessState::Update => self.update()?,
                ProcessState::Render => self.render()?,
                ProcessState::DoNothing => {
                    if self.read_choices() {
                        self.update()?;
                    }
                }
                ProcessState::Output(o) => {
                    out = Some(o);
                    break;
                }
            }
        }
        self.drop()?;
        if let Some(o) = out {
            println!("{}", o)
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.read_choices();
        self.table.update(
            &self.input.content,
            &self.choices,
            self.area.as_ref().unwrap(),
        );
        self.render()?;
        Ok(())
    }

    fn process(&mut self) -> Result<ProcessState> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                return Ok(self.process_key(key));
            }
        }
        Ok(ProcessState::DoNothing)
    }

    fn render(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let area = frame.size();
            self.area = Some(area);
            self.table.render(frame, &area);
            self.input.render(frame, &area);
        })?;
        Ok(())
    }

    fn drop(&self) -> Result<()> {
        stdout()
            .execute(LeaveAlternateScreen)?
            .execute(DisableMouseCapture)?
            .execute(SetCursorStyle::SteadyBlock)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn read_choices(&mut self) -> bool {
        if self.rx.is_empty() {
            return false;
        }
        while let Ok(choice) = self.rx.try_recv() {
            self.choices.push(choice);
        }
        true
    }
}
