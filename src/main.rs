mod args;
mod decoder;
mod torrent_file;
mod tracker;
mod url;

use std::path::Path;

use decoder::Decoder;
use torrent_file::TorrentFile;
use tracker::Tracker;

fn main() {
    let args = args::Args::new();
    match args.command {
        args::Command::Decode(decode_args) => run_decode(&decode_args.value),
        args::Command::Info(info_args) => run_info(&info_args.path),
        args::Command::Peers(peers_args) => run_peers(&peers_args.path),
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
