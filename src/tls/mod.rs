pub mod cipher_suite;
pub mod compression_methods;

#[derive(Debug)]
pub enum RecordType {
    ChangeCipherSpec,
    Alert,
    Handshake,
    ApplicationData,
    Heartbeat,
}
#[derive(Debug)]
pub enum TLSVersion {
    TLS1,
    TLS1_1,
    TLS1_2,
}

impl From<[u8; 2]> for TLSVersion {
    #[allow(clippy::similar_names)]
    fn from([msb, lsb]: [u8; 2]) -> Self {
        let version = u16::from_ne_bytes([lsb, msb]);

        match version {
            0x301 => TLSVersion::TLS1,
            0x302 => TLSVersion::TLS1_1,
            0x303 => TLSVersion::TLS1_2,
            _ => panic!("Invalid TLS version"),
        }
    }
}

#[derive(Debug)]
pub struct RecordHeader {
    record_type: RecordType,
    tls_version: TLSVersion,
    record_length: u16,
}

impl From<&[u8]> for RecordHeader {
    fn from(value: &[u8]) -> Self {
        let record_type = value[0];

        let record_length = u16::from_ne_bytes([value[4], value[3]]);

        let tls_version = TLSVersion::from([value[1], value[2]]);

        Self {
            record_type: match record_type {
                20 => RecordType::ChangeCipherSpec,
                21 => RecordType::Alert,
                22 => RecordType::Handshake,
                23 => RecordType::ApplicationData,
                24 => RecordType::Heartbeat,
                _ => panic!("invalid record_type"),
            },
            tls_version,
            record_length,
        }
    }
}

#[derive(Debug)]
pub enum HandshakeType {
    HelloRequest,
    ClientHello,
    ServerHello,
    NewSessionTicket,
    Certificate,
    ServerKeyExchange,
    CertificateRequest,
    ServerDone,
    CertificateVerify,
    ClientKeyExchange,
    Finished,
}

#[derive(Debug)]
pub struct HandshakeHeader {
    pub handshake_type: HandshakeType,
    pub data_length: u32,
}

impl From<&[u8]> for HandshakeHeader {
    fn from(value: &[u8]) -> Self {
        let handshake_type = match value[0] {
            0 => HandshakeType::HelloRequest,
            1 => HandshakeType::ClientHello,
            2 => HandshakeType::ServerHello,
            4 => HandshakeType::NewSessionTicket,
            11 => HandshakeType::Certificate,
            12 => HandshakeType::ServerKeyExchange,
            13 => HandshakeType::CertificateRequest,
            14 => HandshakeType::ServerDone,
            15 => HandshakeType::CertificateVerify,
            16 => HandshakeType::ClientKeyExchange,
            20 => HandshakeType::Finished,
            _ => panic!("invalid handshake type"),
        };

        Self {
            handshake_type,
            data_length: u32::from_le_bytes([value[3], value[2], value[1], 0]),
        }
    }
}

enum ClientVersion {}
