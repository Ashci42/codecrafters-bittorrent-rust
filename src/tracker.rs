use std::net::{Ipv4Addr, SocketAddrV4};

use serde::{de::Visitor, Deserialize, Serialize};

use crate::{config::PEER_ID, torrent::Torrent, url};

pub struct Tracker<'t, T>
where
    T: Torrent,
{
    torrent: &'t T,
}

impl<'t, T> Tracker<'t, T>
where
    T: Torrent,
{
    pub fn new(torrent: &'t T) -> Self {
        Self { torrent }
    }

    pub fn make_request(&self) -> Result<TrackerResponse, TrackerError> {
        let client = reqwest::blocking::Client::new();
        let info_hash = url::encode(&self.torrent.info_hash());
        let left = self.torrent.left();
        let tracker_query = TrackerQuery::new(info_hash, left);
        let url = format!(
            "{}?info_hash={}",
            self.torrent.tracker_url(),
            tracker_query.info_hash
        );
        let tracker_response = client.get(url).query(&tracker_query).send()?;
        let tracker_response = tracker_response.bytes()?;
        let tracker_response: TrackerResponse = serde_bencode::from_bytes(&tracker_response)?;

        Ok(tracker_response)
    }
}

#[derive(Debug)]
pub enum TrackerError {
    Request(reqwest::Error),
    Decode(serde_bencode::Error),
}

impl std::fmt::Display for TrackerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackerError::Request(error) => write!(f, "Failed to make tracker request: {}", error),
            TrackerError::Decode(error) => write!(f, "Failed to decode response: {}", error),
        }
    }
}

impl std::error::Error for TrackerError {}

impl From<reqwest::Error> for TrackerError {
    fn from(value: reqwest::Error) -> Self {
        Self::Request(value)
    }
}

impl From<serde_bencode::Error> for TrackerError {
    fn from(value: serde_bencode::Error) -> Self {
        Self::Decode(value)
    }
}

#[derive(Deserialize)]
pub struct TrackerResponse {
    peers: Peers,
}

impl TrackerResponse {
    pub fn print_peers(&self) {
        for peer in self.peers.iter() {
            println!("{peer}");
        }
    }
}

#[derive(Serialize)]
struct TrackerQuery {
    #[serde(skip_serializing)]
    info_hash: String,
    peer_id: &'static str,
    port: u16,
    uploaded: u32,
    downloaded: u32,
    left: usize,
    compact: u8,
}

impl TrackerQuery {
    fn new(info_hash: String, left: usize) -> Self {
        Self {
            info_hash,
            peer_id: PEER_ID,
            port: 6881,
            uploaded: 0,
            downloaded: 0,
            left,
            compact: 1,
        }
    }
}

struct Peers(Vec<SocketAddrV4>);

impl Peers {
    fn new(peers: Vec<SocketAddrV4>) -> Self {
        Self(peers)
    }

    fn iter(&self) -> std::slice::Iter<'_, SocketAddrV4> {
        self.0.iter()
    }
}

impl<'de> Deserialize<'de> for Peers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(PeersVisitor)
    }
}

struct PeersVisitor;

impl<'de> Visitor<'de> for PeersVisitor {
    type Value = Peers;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a byte array whose length is a multiple of 6")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() % 6 != 0 {
            Err(E::custom(format!("Length of byte array is {}", v.len())))
        } else {
            let peers: Vec<SocketAddrV4> = v
                .chunks_exact(6)
                .map(|chunk| {
                    SocketAddrV4::new(
                        Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]),
                        u16::from_be_bytes([chunk[4], chunk[5]]),
                    )
                })
                .collect();

            Ok(Peers::new(peers))
        }
    }
}
