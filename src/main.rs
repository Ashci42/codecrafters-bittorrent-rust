mod args;
mod decoder;
mod torrent_file;

use std::path::PathBuf;

use decoder::Decoder;
use torrent_file::TorrentFile;

fn main() {
    let args = args::Args::new();
    match args.command {
        args::Command::Decode(decode_args) => run_decode(decode_args.value),
        args::Command::Info(info_args) => run_info(info_args.path),
    }
}

fn run_decode(value: String) {
    let mut decoder = Decoder::new(&value);
    let decoded_value = decoder.decode().expect("Failed to decode");
    println!("{}", decoded_value);
}

fn run_info(path: PathBuf) {
    let torrent_file_contents = std::fs::read(path).expect("Cannot read torrent file");
    let torrent_file: TorrentFile =
        serde_bencode::from_bytes(&torrent_file_contents).expect("Cannot parse torrent file");
    torrent_file.print_info();
    torrent_file.print_info_hash();
}
