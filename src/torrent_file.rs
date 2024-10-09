use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

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

    pub fn print_info_hash(&self) {
        let bencoded_info =
            serde_bencode::to_bytes(&self.info).expect("Cannot bencode info dictionary");

        let mut hasher = Sha1::new();
        hasher.update(bencoded_info);
        let result: [u8; 20] = hasher.finalize().into();
        let result = hex::encode(result);
        println!("Info Hash: {}", result);
    }
}

#[derive(Deserialize, Serialize)]
struct Info {
    length: usize,
    name: String,
    #[serde(rename = "piece length")]
    piece_length: usize,
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
}
