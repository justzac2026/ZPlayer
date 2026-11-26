use std::{fs::File,io::BufReader,path::Path};
use rodio::{Decoder,OutputStream,OutputStreamBuilder,Sink};
pub struct AudioPlayer{
    _stream:OutputStream,
    sink:Sink
}
impl AudioPlayer{
    pub fn new()->Result<Self,Box<dyn std::error::Error>>{
        let stream=OutputStreamBuilder::open_default_stream()?;
        let sink=Sink::connect_new(stream.mixer());
        Ok(Self{
            _stream:stream,
            sink
        })
    }
    pub fn play(&mut self,path:impl AsRef<Path>)->Result<(),Box<dyn std::error::Error>>{
        let file=File::open(path)?;
        let source=Decoder::try_from(BufReader::new(file))?;
        self.sink.stop();
        self.sink=Sink::connect_new(self._stream.mixer());
        self.sink.append(source);
        Ok(())
    }
    pub fn pause(&self){
        self.sink.pause();
    }
    pub fn resume(&self){
        self.sink.play();
    }
    pub fn toggle(&self){
        if self.sink.is_paused(){
            self.resume();
        }
        else{
            self.pause();
        }
    }
    pub fn set_volume(&self,volume:f32){
        self.sink.set_volume(volume.clamp(0.0,1.0));
    }
    pub fn volume(&self)->f32{
        self.sink.volume()
    }
    pub fn is_paused(&self)->bool{
        self.sink.is_paused()
    }
    pub fn is_empty(&self)->bool{
        self.sink.empty()
    }
    pub fn stop(&self){
        self.sink.stop();
    }
}