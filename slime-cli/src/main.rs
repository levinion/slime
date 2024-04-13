use std::io::{stdin, IsTerminal};

use app::App;
use tokio::sync::mpsc::channel;
mod app;
mod keymap;
mod widgets;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, rx) = channel(100);
    tokio::spawn(async move {
        loop {
            let mut s = String::new();
            if !stdin().is_terminal() && stdin().read_line(&mut s).unwrap() == 0
                || tx.send(s.trim().to_string()).await.is_err()
            {
                return;
            };
        }
    });
    let app = App::new(rx)?;
    app.run()
}
