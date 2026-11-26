use std::io;
use ratatui::{
    backend::Backend,
    layout::{Constraint,Direction,Layout},
    style::{Color,Modifier,Style},
    widgets::{Block,Borders,Gauge,List,ListItem,Paragraph},
    Frame,
    Terminal
};
use crossterm::event::KeyCode;
use crate::{
    audio::AudioPlayer,
    library::MusicLibrary
};
pub struct App{
    pub library:MusicLibrary,
    pub audio:AudioPlayer,
    pub selected:usize,
    pub playing:bool,
    pub current_song:Option<String>,
    pub volume:f32
}
impl App{
    pub fn new(library:MusicLibrary,audio:AudioPlayer)->Self{
        Self{
            library,
            audio,
            selected:0,
            playing:false,
            current_song:None,
            volume:1.0
        }
    }
    pub fn next(&mut self){
        if !self.library.songs.is_empty(){
            let len=self.library.songs.len();
            self.selected=(self.selected+len+1)%len;
        }
    }
    pub fn previous(&mut self){
        if !self.library.songs.is_empty(){
            let len=self.library.songs.len();
            self.selected=(self.selected+len-1)%len;
        }
    }
    pub fn play_selected(&mut self)->Result<(),Box<dyn std::error::Error>>{
        let Some(song)=self.library.songs.get(self.selected)else{
            return Ok(());
        };
        self.audio.play(&song.path)?;
        self.current_song=Some(song.title.clone());
        self.playing=true;
        Ok(())
    }
    pub fn toggle_playback(&mut self)->Result<(),Box<dyn std::error::Error>>{
        if self.playing{
            self.audio.pause();
            self.playing=false;
        }
        else if self.audio.is_empty(){
            self.play_selected()?;
        }
        else{
            self.audio.resume();
            self.playing=true;
        }
        Ok(())
    }
    pub fn set_volume(&mut self,volume:f32){
        self.volume=volume.clamp(0.0,1.0);
        self.audio.set_volume(volume);
    }
}
pub fn ui(frame:&mut Frame<'_>,app:&App){
    let root=Layout::default().direction(Direction::Vertical).constraints([
        Constraint::Length(3),
        Constraint::Min(8),
        Constraint::Length(5),
        Constraint::Length(1)
    ]).split(frame.area());
    let header=Paragraph::new("ZPLAYER / LOCAL MUSIC PLAYER").style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)).block(Block::default().borders(Borders::ALL));
    frame.render_widget(header,root[0]);
    let content=Layout::default().direction(Direction::Horizontal).constraints([
        Constraint::Percentage(50),
        Constraint::Percentage(50)
    ]).split(root[1]);
    let songs:Vec<ListItem>=app.library.songs.iter().enumerate().map(|(index,song)|{
        let selected=index==app.selected;
        let prefix=if selected{
            " > "
        }else{
            "   "
        };
        let style=if selected{
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
        }else{
            Style::default().fg(Color::White)
        };
        ListItem::new(format!("{prefix}{}",song.title)).style(style)
    }).collect();
    let library=List::new(songs).block(
        Block::default()
        .title(format!(" Music Library ({}) ",app.library.songs.len()))
        .borders(Borders::ALL));
    frame.render_widget(library,content[0]);
    let player_layout=Layout::default().direction(Direction::Vertical).constraints([
        Constraint::Length(3),
        Constraint::Length(1)
    ]).split(root[2]);
    let song_name=app.current_song.as_deref().unwrap_or("Nothing playing");
    let status=if app.playing{"PLAYING"}else{"PAUSED"};
    let now_playing=Paragraph::new(format!(" {status} | {song_name} ")).style(Style::default().fg(Color::Green)).block(Block::default().title(" Now playing ").borders(Borders::ALL));
    frame.render_widget(now_playing,player_layout[0]);
    let progress=Gauge::default().block(Block::default()).gauge_style(Style::default().fg(Color::Green));
    frame.render_widget(progress,player_layout[1]);
    let footer=Paragraph::new(":D").style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer,root[3]);
}
pub fn run<B:Backend>(terminal:&mut Terminal<B>,app:&mut App)->io::Result<()>{
    let mut volume:f32=1.0;
    loop{
        terminal.draw(|frame|ui(frame,app)).map_err(|e|io::Error::other(e.to_string()))?;
        if crossterm::event::poll(std::time::Duration::from_millis(16)).map_err(|e|io::Error::other(e.to_string()))?{
            let event=crossterm::event::read().map_err(|e|io::Error::other(e.to_string()))?;
            if let crossterm::event::Event::Key(key)=event{
                match key.code{
                    KeyCode::Esc=>break,
                    KeyCode::Down=>app.next(),
                    KeyCode::Up=>app.previous(),
                    KeyCode::Char('k')=>{
                        volume-=0.1;
                        app.set_volume(volume);
                    }
                    KeyCode::Char('l')=>{
                        volume+=0.1;
                        app.set_volume(volume);
                    }
                    KeyCode::Char('n')=>{
                        app.next();
                        app.play_selected().map_err(|e|io::Error::other(e.to_string()))?;
                    },
                    KeyCode::Char('p')=>{
                        app.previous();
                        app.play_selected().map_err(|e|io::Error::other(e.to_string()))?;
                    },
                    KeyCode::Enter=>{
                        app.play_selected().map_err(|e|io::Error::other(e.to_string()))?;
                    },
                    KeyCode::Char(' ')=>{
                        app.toggle_playback().map_err(|e|io::Error::other(e.to_string()))?;
                    },
                    _=>{}
                }
            }
        }
    }
    Ok(())
}
