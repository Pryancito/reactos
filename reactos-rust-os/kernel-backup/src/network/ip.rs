//! Protocolo IP (Internet Protocol)
//! 
//! Implementa el protocolo IP para la capa de red

use super::ethernet::MacAddress;

/// Dirección IP (4 bytes para IPv4)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IpAddress {
    pub bytes: [u8; 4],
}

impl IpAddress {
    /// Crear nueva dirección IP
    pub const fn new(bytes: [u8; 4]) -> Self {
        Self { bytes }
    }
    
    /// Dirección IP de loopback
    pub const fn loopback() -> Self {
        Self { bytes: [127, 0, 0, 1] }
    }
    
    /// Dirección IP nula
    pub const fn null() -> Self {
        Self { bytes: [0, 0, 0, 0] }
    }
    
    /// Dirección IP de broadcast
    pub const fn broadcast() -> Self {
        Self { bytes: [255, 255, 255, 255] }
    }
    
    /// Verificar si es loopback
    pub fn is_loopback(&self) -> bool {
        self.bytes[0] == 127
    }
    
    /// Verificar si es broadcast
    pub fn is_broadcast(&self) -> bool {
        self.bytes == [255, 255, 255, 255]
    }
    
    /// Verificar si es nula
    pub fn is_null(&self) -> bool {
        self.bytes == [0, 0, 0, 0]
    }
    
    /// Convertir a string
    pub fn to_string(&self) -> [u8; 16] {
        let mut result = [0u8; 16];
        let mut pos = 0;
        
        for (i, &byte) in self.bytes.iter().enumerate() {
            if i > 0 {
                result[pos] = b'.';
                pos += 1;
            }
            
            // Convertir byte a string
            if byte >= 100 {
                result[pos] = b'0' + (byte / 100);
                pos += 1;
                result[pos] = b'0' + ((byte % 100) / 10);
                pos += 1;
                result[pos] = b'0' + (byte % 10);
                pos += 1;
            } else if byte >= 10 {
                result[pos] = b'0' + (byte / 10);
                pos += 1;
                result[pos] = b'0' + (byte % 10);
                pos += 1;
            } else {
                result[pos] = b'0' + byte;
                pos += 1;
            }
        }
        
        result
    }
}

/// Tipos de protocolo IP
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IpProtocol {
    ICMP = 1,
    TCP = 6,
    UDP = 17,
    Unknown = 0,
}

impl From<u8> for IpProtocol {
    fn from(value: u8) -> Self {
        match value {
            1 => IpProtocol::ICMP,
            6 => IpProtocol::TCP,
            17 => IpProtocol::UDP,
            _ => IpProtocol::Unknown,
        }
    }
}

/// Cabecera IP
#[derive(Debug, Clone, Copy)]
pub struct IpHeader {
    pub version: u8,        // 4 bits
    pub ihl: u8,           // 4 bits - Internet Header Length
    pub tos: u8,           // Type of Service
    pub total_length: u16,
    pub identification: u16,
    pub flags: u8,         // 3 bits
    pub fragment_offset: u16, // 13 bits
    pub ttl: u8,           // Time To Live
    pub protocol: IpProtocol,
    pub header_checksum: u16,
    pub source: IpAddress,
    pub destination: IpAddress,
}

impl IpHeader {
    /// Crear nueva cabecera IP
    pub fn new(
        source: IpAddress,
        destination: IpAddress,
        protocol: IpProtocol,
        total_length: u16,
    ) -> Self {
        Self {
            version: 4,
            ihl: 5, // 20 bytes / 4 = 5
            tos: 0,
            total_length,
            identification: 0, // Se asignará dinámicamente
            flags: 0,
            fragment_offset: 0,
            ttl: 64,
            protocol,
            header_checksum: 0, // Se calculará
            source,
            destination,
        }
    }
    
    /// Tamaño de la cabecera IP
    pub const fn size() -> usize {
        20
    }
    
    /// Calcular checksum de la cabecera
    pub fn calculate_checksum(&self) -> u16 {
        let mut sum: u32 = 0;
        
        // Sumar todos los campos de 16 bits
        sum += ((self.version as u32) << 12) | ((self.ihl as u32) << 8) | (self.tos as u32);
        sum += self.total_length as u32;
        sum += self.identification as u32;
        sum += ((self.flags as u32) << 13) | (self.fragment_offset as u32);
        sum += (self.ttl as u32) << 8;
        sum += self.protocol as u32;
        // header_checksum se omite (se calcula)
        
        // Direcciones IP (4 bytes cada una = 2 campos de 16 bits)
        sum += ((self.source.bytes[0] as u32) << 8) | (self.source.bytes[1] as u32);
        sum += ((self.source.bytes[2] as u32) << 8) | (self.source.bytes[3] as u32);
        sum += ((self.destination.bytes[0] as u32) << 8) | (self.destination.bytes[1] as u32);
        sum += ((self.destination.bytes[2] as u32) << 8) | (self.destination.bytes[3] as u32);
        
        // Sumar los acarreos
        while (sum >> 16) != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        
        // Complemento a uno
        !(sum as u16)
    }
    
    /// Serializar cabecera a bytes
    pub fn to_bytes(&self) -> [u8; 20] {
        let mut bytes = [0u8; 20];
        
        // Version (4 bits) + IHL (4 bits)
        bytes[0] = (self.version << 4) | self.ihl;
        
        // TOS
        bytes[1] = self.tos;
        
        // Total Length (2 bytes, big-endian)
        let total_length_bytes = self.total_length.to_be_bytes();
        bytes[2..4].copy_from_slice(&total_length_bytes);
        
        // Identification (2 bytes, big-endian)
        let id_bytes = self.identification.to_be_bytes();
        bytes[4..6].copy_from_slice(&id_bytes);
        
        // Flags (3 bits) + Fragment Offset (13 bits)
        let flags_frag = ((self.flags as u16) << 13) | self.fragment_offset;
        let flags_frag_bytes = flags_frag.to_be_bytes();
        bytes[6..8].copy_from_slice(&flags_frag_bytes);
        
        // TTL
        bytes[8] = self.ttl;
        
        // Protocol
        bytes[9] = self.protocol as u8;
        
        // Header Checksum (2 bytes, big-endian)
        let checksum_bytes = self.header_checksum.to_be_bytes();
        bytes[10..12].copy_from_slice(&checksum_bytes);
        
        // Source IP (4 bytes)
        bytes[12..16].copy_from_slice(&self.source.bytes);
        
        // Destination IP (4 bytes)
        bytes[16..20].copy_from_slice(&self.destination.bytes);
        
        bytes
    }
    
    /// Deserializar cabecera desde bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 20 {
            return None;
        }
        
        let version_ihl = bytes[0];
        let version = version_ihl >> 4;
        let ihl = version_ihl & 0x0F;
        
        if version != 4 {
            return None; // Solo soportamos IPv4
        }
        
        let tos = bytes[1];
        
        let total_length_bytes = [bytes[2], bytes[3]];
        let total_length = u16::from_be_bytes(total_length_bytes);
        
        let id_bytes = [bytes[4], bytes[5]];
        let identification = u16::from_be_bytes(id_bytes);
        
        let flags_frag_bytes = [bytes[6], bytes[7]];
        let flags_frag = u16::from_be_bytes(flags_frag_bytes);
        let flags = (flags_frag >> 13) as u8;
        let fragment_offset = flags_frag & 0x1FFF;
        
        let ttl = bytes[8];
        let protocol = IpProtocol::from(bytes[9]);
        
        let checksum_bytes = [bytes[10], bytes[11]];
        let header_checksum = u16::from_be_bytes(checksum_bytes);
        
        let mut source_bytes = [0u8; 4];
        source_bytes.copy_from_slice(&bytes[12..16]);
        
        let mut dest_bytes = [0u8; 4];
        dest_bytes.copy_from_slice(&bytes[16..20]);
        
        Some(Self {
            version,
            ihl,
            tos,
            total_length,
            identification,
            flags,
            fragment_offset,
            ttl,
            protocol,
            header_checksum,
            source: IpAddress::new(source_bytes),
            destination: IpAddress::new(dest_bytes),
        })
    }
}

/// Paquete IP completo
#[derive(Debug, Clone)]
pub struct IpPacket {
    pub header: IpHeader,
    pub payload: [u8; 1480], // MTU - cabecera IP
    pub payload_len: usize,
}

impl IpPacket {
    /// Crear nuevo paquete IP
    pub fn new(header: IpHeader, payload: &[u8]) -> Self {
        let mut packet_payload = [0u8; 1480];
        let payload_len = core::cmp::min(payload.len(), 1480);
        packet_payload[..payload_len].copy_from_slice(&payload[..payload_len]);
        
        Self {
            header,
            payload: packet_payload,
            payload_len,
        }
    }
    
    /// Tamaño total del paquete
    pub fn total_size(&self) -> usize {
        IpHeader::size() + self.payload_len
    }
    
    /// Serializar paquete completo a bytes
    pub fn to_bytes(&self) -> [u8; 1500] {
        let mut bytes = [0u8; 1500];
        
        // Cabecera
        let header_bytes = self.header.to_bytes();
        bytes[0..20].copy_from_slice(&header_bytes);
        
        // Payload
        bytes[20..20 + self.payload_len].copy_from_slice(&self.payload[..self.payload_len]);
        
        bytes
    }
    
    /// Deserializar paquete desde bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 20 {
            return None;
        }
        
        let header = IpHeader::from_bytes(bytes)?;
        let payload_len = bytes.len() - 20;
        
        if payload_len > 1480 {
            return None;
        }
        
        let mut packet_payload = [0u8; 1480];
        packet_payload[..payload_len].copy_from_slice(&bytes[20..20 + payload_len]);
        
        Some(Self {
            header,
            payload: packet_payload,
            payload_len,
        })
    }
}

/// Gestor de protocolo IP
pub struct IpManager {
    pub local_ip: IpAddress,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

impl IpManager {
    /// Crear nuevo gestor IP
    pub fn new(local_ip: IpAddress) -> Self {
        Self {
            local_ip,
            packets_sent: 0,
            packets_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
        }
    }
    
    /// Enviar paquete IP
    pub fn send_packet(&mut self, packet: &IpPacket) -> bool {
        // En una implementación real, esto enviaría el paquete por Ethernet
        self.packets_sent += 1;
        self.bytes_sent += packet.total_size() as u64;
        true
    }
    
    /// Recibir paquete IP
    pub fn receive_packet(&mut self, packet: &IpPacket) -> bool {
        // Verificar si el paquete es para nosotros
        if !packet.header.destination.is_broadcast() && 
           packet.header.destination != self.local_ip {
            return false;
        }
        
        self.packets_received += 1;
        self.bytes_received += packet.total_size() as u64;
        true
    }
    
    /// Obtener estadísticas
    pub fn get_statistics(&self) -> (u64, u64, u64, u64) {
        (self.packets_sent, self.packets_received, self.bytes_sent, self.bytes_received)
    }
}
