pub trait Torrent {
    fn tracker_url(&self) -> &str;
    fn info_hash(&self) -> [u8; 20];
    fn left(&self) -> usize;
}
