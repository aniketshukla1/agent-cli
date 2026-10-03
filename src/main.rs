pub mod app;
pub mod config;
pub mod llm;
pub mod mcp;
pub mod session;
pub mod skills;
pub mod theme;
pub mod tools;
pub mod ui;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{Event, EventStream},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::StreamExt;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;
use std::time::Duration;
use tokio::sync::mpsc;

use crate::app::{App, AppEvent};
use crate::config::Config;
use crate::session::Session;
use crate::theme::ThemeKind;

#[derive(Parser, Debug)]
#[command(name = "agent-cli", author, version, about = "A modern, extensible CLI Agent in Rust")]
struct CliArgs {
    /// Initial AI model to use (e.g. claude-3-7-sonnet, gpt-4o, gemini-2.5-pro, llama3.3:latest)
    #[arg(short, long)]
    model: Option<String>,

    /// Color theme (catppuccin_mocha, tokyo_night, nord, gruvbox_dark, cyberpunk, monokai, minimalist_light)
    #[arg(short, long)]
    theme: Option<String>,

    /// Resume the latest conversation session
    #[arg(short, long)]
    resume: bool,

    /// Initial prompt to send immediately upon launch
    #[arg(value_name = "PROMPT")]
    prompt: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    // Load configuration
    let mut config = Config::load();

    if let Some(m) = args.model {
        config.general.default_model = m;
    }

    if let Some(t_str) = args.theme {
        let t_lower = t_str.to_lowercase();
        for kind in ThemeKind::ALL {
            if kind.name().to_lowercase().contains(&t_lower) || format!("{:?}", kind).to_lowercase().contains(&t_lower) {
                config.general.theme = *kind;
                break;
            }
        }
    }

    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Channel for async App events (keys, ticks, LLM streaming chunks)
    let (event_tx, mut event_rx) = mpsc::channel::<AppEvent>(200);

    let mut app = App::new(config, event_tx.clone());

    if args.resume {
        if let Some(prev) = Session::load_latest() {
            app.session = prev;
        }
    }

    // Initialize MCP servers and tools in background
    app.initialize().await;

    // Send initial prompt if provided via CLI argument
    if let Some(initial_prompt) = args.prompt {
        app.session.messages.push(crate::session::ChatMessage::user(initial_prompt));
        app.trigger_llm_stream().await;
    }

    // Spawn tick generator for UI spinners (50ms interval)
    let tick_tx = event_tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(50));
        loop {
            interval.tick().await;
            if tick_tx.send(AppEvent::Tick).await.is_err() {
                break;
            }
        }
    });

    // Event loop
    let mut reader = EventStream::new();

    while app.running {
        app.render(&mut terminal).await?;

        tokio::select! {
            Some(app_event) = event_rx.recv() => {
                match app_event {
                    AppEvent::Tick => {
                        app.on_tick();
                    }
                    AppEvent::LlmToken(_) | AppEvent::LlmToolCall { .. } | AppEvent::LlmDone | AppEvent::LlmError(_) => {
                        app.handle_llm_event(app_event).await;
                    }
                    AppEvent::Key(key) => {
                        app.handle_key_event(key).await?;
                    }
                }
            }
            Some(Ok(event)) = reader.next() => {
                if let Event::Key(key) = event {
                    app.handle_key_event(key).await?;
                }
            }
        }
    }

    // Teardown terminal cleanly
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    println!("\n👋 Agent CLI closed. Thank you for using Agent CLI!\n");

    Ok(())
}
