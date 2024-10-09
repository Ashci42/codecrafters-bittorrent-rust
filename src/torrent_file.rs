use std::slice::ChunksExact;

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

    pub fn print_pieces(&self) {
        println!("Piece Length: {}", self.info.piece_length);

        println!("Piece Hashes:");
        for piece in self.info.piece_iter() {
            let piece_hash = hex::encode(piece);
            println!("{piece_hash}");
        }
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

impl Info {
    fn piece_iter(&self) -> PieceIterator {
        PieceIterator::new(&self.pieces)
    }
}

struct PieceIterator<'pieces> {
    pieces: ChunksExact<'pieces, u8>,
}

impl<'pieces> PieceIterator<'pieces> {
    fn new(pieces: &'pieces [u8]) -> Self {
        let pieces_chunks = pieces.chunks_exact(20);

        Self {
            pieces: pieces_chunks,
        }
    }
}

impl<'pieces> Iterator for PieceIterator<'pieces> {
    type Item = &'pieces [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.pieces.next()
    }
}
