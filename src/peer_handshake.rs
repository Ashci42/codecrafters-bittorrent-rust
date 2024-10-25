use std::{
    io::{Read, Write},
    net::SocketAddrV4,
};

use handshake_message::{HandshakeMessage, SerialisedHandshakeMessage};

use crate::torrent::Torrent;

pub struct PeerHandshake<'t, T>
where
    T: Torrent,
{
    torrent: &'t T,
}

impl<'t, T> PeerHandshake<'t, T>
where
    T: Torrent,
{
    pub fn new(torrent: &'t T) -> Self {
        Self { torrent }
    }

    pub fn establish(&self, peer: &SocketAddrV4) -> std::io::Result<()> {
        let mut tcp_stream = std::net::TcpStream::connect(peer)?;
        let hm = HandshakeMessage::new(self.torrent.info_hash());
        tcp_stream.write_all(&hm.serialise())?;

        let mut response: SerialisedHandshakeMessage =
            [0; handshake_message::HANDSHAKE_MESSAGE_SERIALISE_LENGTH];
        tcp_stream.read_exact(&mut response)?;
        let response = HandshakeMessage::deserialise(response);

        response.print_peer_id();

        Ok(())
    }
}

mod handshake_message {
    use core::str;

    use crate::config::PEER_ID;

    pub type SerialisedHandshakeMessage = [u8; HANDSHAKE_MESSAGE_SERIALISE_LENGTH];

    pub const HANDSHAKE_MESSAGE_SERIALISE_LENGTH: usize = 68;

    const BIT_TORRENT_PROTOCOL: &str = "BitTorrent protocol";

    pub struct HandshakeMessage {
        length: u8,
        protocol: String,
        reserved: [u8; 8],
        info_hash: [u8; 20],
        peer_id: [u8; 20],
    }

    impl HandshakeMessage {
        pub fn new(info_hash: [u8; 20]) -> Self {
            Self {
                length: 19,
                protocol: BIT_TORRENT_PROTOCOL.to_string(),
                reserved: [0, 0, 0, 0, 0, 0, 0, 0],
                info_hash,
                peer_id: PEER_ID
                    .as_bytes()
                    .try_into()
                    .expect("peer id has length 20"),
            }
        }

        pub fn deserialise(bytes: SerialisedHandshakeMessage) -> Self {
            let mut length = [0; 1];
            length.copy_from_slice(&bytes[..1]);
            let length = length[0];

            let mut protocol = [0; 19];
            protocol.clone_from_slice(&bytes[1..20]);
            let protocol = String::from(str::from_utf8(&protocol).expect("Protocol is valid utf8"));

            let mut reserved = [0; 8];
            reserved.clone_from_slice(&bytes[20..28]);

            let mut info_hash = [0; 20];
            info_hash.clone_from_slice(&bytes[28..48]);

            let mut peer_id = [0; 20];
            peer_id.clone_from_slice(&bytes[48..]);

            Self {
                length,
                protocol,
                reserved,
                info_hash,
                peer_id,
            }
        }

        pub fn serialise(&self) -> Vec<u8> {
            let mut ser: Vec<u8> = vec![self.length];
            ser.extend_from_slice(self.protocol.as_bytes());
            ser.extend_from_slice(&self.reserved);
            ser.extend_from_slice(&self.info_hash);
            ser.extend_from_slice(&self.peer_id);

            ser
        }

        pub fn print_peer_id(&self) {
            let peer_id = hex::encode(self.peer_id);
            println!("Peer ID: {}", peer_id);
        }
    }
}
