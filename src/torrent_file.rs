use std::path::Path;

use serde::{de::Visitor, Deserialize, Serialize};
use sha1::{Digest, Sha1};

use crate::torrent::Torrent;

#[derive(Deserialize)]
pub struct TorrentFile {
    announce: String,
    info: Info,
}

impl TorrentFile {
    pub fn try_from_file(path: &Path) -> Result<Self, FileError> {
        let torrent_file_contents = std::fs::read(path)?;
        let torrent_file: Self = serde_bencode::from_bytes(&torrent_file_contents)?;

        Ok(torrent_file)
    }

    pub fn print_info(&self) {
        println!("Tracker URL: {}", self.tracker_url());
        println!("Length: {}", self.info.length);
    }

    pub fn print_info_hash(&self) {
        let info_hash = self.info_hash();
        let result = hex::encode(info_hash);
        println!("Info Hash: {}", result);
    }

    pub fn print_pieces(&self) {
        println!("Piece Length: {}", self.info.piece_length);

        println!("Piece Hashes:");
        for piece in self.info.pieces.iter() {
            let piece_hash = hex::encode(piece);
            println!("{piece_hash}");
        }
    }
}

impl Torrent for TorrentFile {
    fn tracker_url(&self) -> &str {
        &self.announce
    }

    fn info_hash(&self) -> [u8; 20] {
        let bencoded_info = serde_bencode::to_bytes(&self.info);
        assert!(bencoded_info.is_ok());
        let bencoded_info = bencoded_info.unwrap();

        let mut hasher = Sha1::new();
        hasher.update(bencoded_info);

        hasher.finalize().into()
    }

    fn left(&self) -> usize {
        self.info.length
    }
}

#[derive(Debug)]
pub enum FileError {
    Read,
    Decode(serde_bencode::Error),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read => write!(f, "Failed to read file"),
            Self::Decode(e) => write!(f, "Failed to decode file contents: {}", e),
        }
    }
}

impl std::error::Error for FileError {}

impl From<std::io::Error> for FileError {
    fn from(_value: std::io::Error) -> Self {
        Self::Read
    }
}

impl From<serde_bencode::Error> for FileError {
    fn from(value: serde_bencode::Error) -> Self {
        Self::Decode(value)
    }
}

#[derive(Deserialize, Serialize)]
struct Info {
    length: usize,
    name: String,
    #[serde(rename = "piece length")]
    piece_length: usize,
    pieces: Pieces,
}

struct Pieces(Vec<[u8; 20]>);

impl Pieces {
    fn new(pieces: Vec<[u8; 20]>) -> Self {
        Self(pieces)
    }

    fn iter(&self) -> std::slice::Iter<'_, [u8; 20]> {
        self.0.iter()
    }
}

impl<'de> Deserialize<'de> for Pieces {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(PiecesVisitor)
    }
}

impl Serialize for Pieces {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let pieces = self.0.concat();

        serializer.serialize_bytes(&pieces)
    }
}

struct PiecesVisitor;

impl<'de> Visitor<'de> for PiecesVisitor {
    type Value = Pieces;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a byte array whose length is a multiple of 20")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() % 20 != 0 {
            Err(E::custom(format!("Length of byte array is {}", v.len())))
        } else {
            let pieces: Vec<[u8; 20]> = v
                .chunks_exact(20)
                .map(|chunk| chunk.try_into().expect("Has lenght 20"))
                .collect();

            Ok(Pieces::new(pieces))
        }
    }
}
