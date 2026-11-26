mod app;
mod audio;
mod library;
use std::io;
use crossterm::{
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal
};
fn main()->Result<(),Box<dyn std::error::Error>>{
    println!("WELCOME TO ZPLAYER :D")
    let mut library=library::MusicLibrary::new();
    library.scan()?;
    let audio=audio::AudioPlayer::new()?;
    enable_raw_mode()?;
    let mut stdout=io::stdout();
    execute!(stdout,EnterAlternateScreen)?;
    let backend=CrosstermBackend::new(stdout);
    let mut terminal=Terminal::new(backend)?;
    let mut app= app::App::new(library,audio);
    let result=app::run(&mut terminal, &mut app);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(),LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result?;
    Ok(())
}