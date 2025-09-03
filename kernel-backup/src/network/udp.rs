//! Protocolo UDP (User Datagram Protocol)
//! 
//! Implementa el protocolo UDP para la capa de transporte

/// Cabecera UDP
#[derive(Debug, Clone, Copy)]
pub struct UdpHeader {
    pub source_port: u16,
    pub destination_port: u16,
    pub length: u16,
    pub checksum: u16,
}

impl UdpHeader {
    /// Crear nueva cabecera UDP
    pub fn new(source_port: u16, destination_port: u16, length: u16) -> Self {
        Self {
            source_port,
            destination_port,
            length,
            checksum: 0, // Se calculará
        }
    }
    
    /// Tamaño de la cabecera UDP
    pub const fn size() -> usize {
        8
    }
    
    /// Serializar cabecera a bytes
    pub fn to_bytes(&self) -> [u8; 8] {
        let mut bytes = [0u8; 8];
        
        // Source Port (2 bytes, big-endian)
        let src_port_bytes = self.source_port.to_be_bytes();
        bytes[0..2].copy_from_slice(&src_port_bytes);
        
        // Destination Port (2 bytes, big-endian)
        let dst_port_bytes = self.destination_port.to_be_bytes();
        bytes[2..4].copy_from_slice(&dst_port_bytes);
        
        // Length (2 bytes, big-endian)
        let length_bytes = self.length.to_be_bytes();
        bytes[4..6].copy_from_slice(&length_bytes);
        
        // Checksum (2 bytes, big-endian)
        let checksum_bytes = self.checksum.to_be_bytes();
        bytes[6..8].copy_from_slice(&checksum_bytes);
        
        bytes
    }
    
    /// Deserializar cabecera desde bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }
        
        let src_port_bytes = [bytes[0], bytes[1]];
        let source_port = u16::from_be_bytes(src_port_bytes);
        
        let dst_port_bytes = [bytes[2], bytes[3]];
        let destination_port = u16::from_be_bytes(dst_port_bytes);
        
        let length_bytes = [bytes[4], bytes[5]];
        let length = u16::from_be_bytes(length_bytes);
        
        let checksum_bytes = [bytes[6], bytes[7]];
        let checksum = u16::from_be_bytes(checksum_bytes);
        
        Some(Self {
            source_port,
            destination_port,
            length,
            checksum,
        })
    }
}

/// Datagrama UDP completo
#[derive(Debug, Clone)]
pub struct UdpDatagram {
    pub header: UdpHeader,
    pub payload: [u8; 1472], // MTU - cabeceras IP y UDP
    pub payload_len: usize,
}

impl UdpDatagram {
    /// Crear nuevo datagrama UDP
    pub fn new(header: UdpHeader, payload: &[u8]) -> Self {
        let mut datagram_payload = [0u8; 1472];
        let payload_len = core::cmp::min(payload.len(), 1472);
        datagram_payload[..payload_len].copy_from_slice(&payload[..payload_len]);
        
        Self {
            header,
            payload: datagram_payload,
            payload_len,
        }
    }
    
    /// Tamaño total del datagrama
    pub fn total_size(&self) -> usize {
        UdpHeader::size() + self.payload_len
    }
    
    /// Serializar datagrama completo a bytes
    pub fn to_bytes(&self) -> [u8; 1480] {
        let mut bytes = [0u8; 1480];
        
        // Cabecera
        let header_bytes = self.header.to_bytes();
        bytes[0..8].copy_from_slice(&header_bytes);
        
        // Payload
        bytes[8..8 + self.payload_len].copy_from_slice(&self.payload[..self.payload_len]);
        
        bytes
    }
    
    /// Deserializar datagrama desde bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }
        
        let header = UdpHeader::from_bytes(bytes)?;
        let payload_len = bytes.len() - 8;
        
        if payload_len > 1472 {
            return None;
        }
        
        let mut datagram_payload = [0u8; 1472];
        datagram_payload[..payload_len].copy_from_slice(&bytes[8..8 + payload_len]);
        
        Some(Self {
            header,
            payload: datagram_payload,
            payload_len,
        })
    }
}

/// Gestor de protocolo UDP
pub struct UdpManager {
    pub datagrams_sent: u64,
    pub datagrams_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

impl UdpManager {
    /// Crear nuevo gestor UDP
    pub fn new() -> Self {
        Self {
            datagrams_sent: 0,
            datagrams_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
        }
    }
    
    /// Enviar datagrama UDP
    pub fn send_datagram(&mut self, datagram: &UdpDatagram) -> bool {
        self.datagrams_sent += 1;
        self.bytes_sent += datagram.total_size() as u64;
        true
    }
    
    /// Recibir datagrama UDP
    pub fn receive_datagram(&mut self, datagram: &UdpDatagram) -> bool {
        self.datagrams_received += 1;
        self.bytes_received += datagram.total_size() as u64;
        true
    }
    
    /// Obtener estadísticas
    pub fn get_statistics(&self) -> (u64, u64, u64, u64) {
        (self.datagrams_sent, self.datagrams_received, self.bytes_sent, self.bytes_received)
    }
}
