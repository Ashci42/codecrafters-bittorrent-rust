use serde::Deserialize;

#[derive(Deserialize)]
pub struct TorrentFile {
    announce: String,
    info: Info,
}

impl TorrentFile {
    pub fn print_info(&self) {
        println!("Tracker URL: {}", self.announce);
        println!("Length: {}", self.info.length);
    }
}

#[derive(Deserialize)]
struct Info {
    length: usize,
}
