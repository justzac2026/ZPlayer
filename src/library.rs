use std::{error::Error,path::{Path,PathBuf}};
use walkdir::WalkDir;
use lofty::{
    prelude::*,
    probe::Probe
};
#[derive(Debug,Clone)]
pub struct Song{
    pub path:PathBuf,
    pub title:String
    pub artist:String
}
pub struct MusicLibrary{
    pub songs:Vec<Song>
}
impl MusicLibrary{
    pub fn new()->Self{
        Self{
            songs:Vec::new()
        }
    }
    pub fn scan(&mut self)->Result<(),Box<dyn Error>>{
        let music_dir=dirs::home_dir().ok_or("could not determine home directory")?.join("Music/ZPlayer");
        self.scan_directory(&music_dir)?;
        self.songs.sort_by(|a,b|{a.title.to_lowercase().cmp(&b.title.to_lowercase())});
        Ok(())
    }
    fn scan_directory(&mut self,directory:&Path)->Result<(),Box<dyn Error>>{
        if !directory.exists(){
            std::fs::create_dir_all(directory)?;
        }
        for entry in WalkDir::new(directory).follow_links(false).into_iter().filter_map(Result::ok){
            let path=entry.path();
            if !path.is_file()||!Self::is_supported(path){
                continue;
            }
            let title=path.file_stem().and_then(|name|name.to_str()).unwrap_or("Unknown").to_string();
            self.songs.push(Song{
                path:path.to_path_buf(),
                title,
                artist:get_artist(path)
            });

        }
        Ok(())
    }
    fn get_artist(path:&Path)->{
        let tagged_file=match Probe::open(path).and_then(|probe|probe.read()){
            Ok(file)=>file,
            Err(_)=>return "Unknown Artist".to_string()
        };
        tagged_file.primary_tag(|tag|tag.artist()).map(|artist|artist.to_string()).unwrap_or_else(||"Unknown Artist".to_string())
    }
    fn is_supported(path:&Path)->bool{
        matches!(
            path.extension().and_then(|ext|ext.to_str()).map(|ext|ext.to_lowercase()).as_deref(),
            Some("mp3"|"flac"|"wav"|"ogg")
        )
    }
}
