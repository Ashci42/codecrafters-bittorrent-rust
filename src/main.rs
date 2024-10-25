mod args;
mod config;
mod decoder;
mod peer_handshake;
mod torrent;
mod torrent_file;
mod tracker;
mod url;

use std::{net::SocketAddrV4, path::Path};

use decoder::Decoder;
use peer_handshake::PeerHandshake;
use torrent_file::TorrentFile;
use tracker::Tracker;

fn main() {
    let args = args::Args::new();
    match args.command {
        args::Command::Decode(decode_args) => run_decode(&decode_args.value),
        args::Command::Info(info_args) => run_info(&info_args.path),
        args::Command::Peers(peers_args) => run_peers(&peers_args.path),
        args::Command::Handshake(handshake_args) => {
            run_handshake(&handshake_args.path, &handshake_args.peer)
        }
    }
}

fn run_decode(value: &str) {
    let mut decoder = Decoder::new(value);
    let decoded_value = decoder.decode().expect("Failed to decode");
    println!("{}", decoded_value);
}

fn run_info(path: &Path) {
    let torrent_file = TorrentFile::try_from_file(path).expect("Failed to create torrent file");
    torrent_file.print_info();
    torrent_file.print_info_hash();
    torrent_file.print_pieces();
}

fn run_peers(path: &Path) {
    let torrent_file = TorrentFile::try_from_file(path).expect("Failed to create torrent file");
    let tracker = Tracker::new(&torrent_file);
    let tracker_response = tracker.make_request().expect("Request failed");
    tracker_response.print_peers();
}

fn run_handshake(path: &Path, peer: &SocketAddrV4) {
    let torrent_file = TorrentFile::try_from_file(path).expect("Failed to create torrent file");
    let peer_handshake = PeerHandshake::new(&torrent_file);
    peer_handshake.establish(peer).unwrap();
}
