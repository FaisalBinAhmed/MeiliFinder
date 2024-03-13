use anyhow::Result; //to avoid writing the error type <Box dyn Error> everywhere

// make the following modules public
pub mod api;
pub mod app;
pub mod components {
    pub mod document_preview;
    pub mod input_bar;
    pub mod static_widgets;
    pub mod status_bar;
}
pub mod constants;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;
pub mod views {
    pub mod documents;
    pub mod indices;
    pub mod instances;
    pub mod tasks;
}
pub mod utilities {
    pub mod config_handler;
    pub mod helpers;
    pub mod scrolling_handler;
}

// qualify the modules in this file
use app::App;
use event::{Event, EventHandler};
use serde::{Deserialize, Serialize};
use tui::Tui;
use update::update;

use ratatui::prelude::{CrosstermBackend, Terminal};

pub type Frame<'a> = ratatui::Frame<'a>; //<'a, CrosstermBackend<std::io::Stderr>>; // alias for the frame type

#[derive(Debug, Serialize, Deserialize)]
struct Movies {
    pub id: u64,
    pub title: String,
    pub release_date: i64,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting MeiliFinder");
    println!("The ultimate MeiliSearch client for your terminal!");

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);

    //     println!("{}", serde_json::to_string_pretty(&docs).unwrap());

    let sender = events.sender.clone(); //we can clone it as we can have multiple senders for this channel

    let mut app = App::new().await;
    app.sender = Some(sender);

    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        if app.should_redraw {
            //this makes sure that we don't redraw the screen if there is no change
            tui.draw(&mut app)?;
            app.should_redraw = false;
        }

        match tui.events.next().await? {
            Event::Tick => {} //every 250ms we get a tick event, we ignore it
            Event::Key(key_event) => update(&mut app, key_event).await,
        };
    }

    tui.exit()?;
    return Ok(());
}
