//! Protocolo ICMP (Internet Control Message Protocol)
//! 
//! Implementa el protocolo ICMP para mensajes de control

/// Tipos de mensaje ICMP
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IcmpType {
    EchoReply = 0,
    EchoRequest = 8,
    DestinationUnreachable = 3,
    TimeExceeded = 11,
    Unknown = 255,
}

impl From<u8> for IcmpType {
    fn from(value: u8) -> Self {
        match value {
            0 => IcmpType::EchoReply,
            8 => IcmpType::EchoRequest,
            3 => IcmpType::DestinationUnreachable,
            11 => IcmpType::TimeExceeded,
            _ => IcmpType::Unknown,
        }
    }
}

/// Códigos ICMP
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IcmpCode {
    None = 0,
    NetworkUnreachable = 1,
    HostUnreachable = 3,
    ProtocolUnreachable = 2,
    PortUnreachable = 16,
    FragmentationNeeded = 17,
    SourceRouteFailed = 18,
    NetworkUnknown = 19,
    HostUnknown = 20,
    HostIsolated = 21,
    NetworkProhibited = 22,
    HostProhibited = 23,
    NetworkUnreachableForTos = 24,
    HostUnreachableForTos = 25,
    CommunicationProhibited = 26,
    HostPrecedenceViolation = 27,
    PrecedenceCutoff = 28,
    TtlExpired = 29,
    FragmentReassemblyTimeExceeded = 30,
    Unknown = 255,
}

impl From<u8> for IcmpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => IcmpCode::None,
            1 => IcmpCode::HostUnreachable,
            2 => IcmpCode::ProtocolUnreachable,
            3 => IcmpCode::PortUnreachable,
            4 => IcmpCode::FragmentationNeeded,
            5 => IcmpCode::SourceRouteFailed,
            6 => IcmpCode::NetworkUnknown,
            7 => IcmpCode::HostUnknown,
            8 => IcmpCode::HostIsolated,
            9 => IcmpCode::NetworkProhibited,
            10 => IcmpCode::HostProhibited,
            11 => IcmpCode::NetworkUnreachableForTos,
            12 => IcmpCode::HostUnreachableForTos,
            13 => IcmpCode::CommunicationProhibited,
            14 => IcmpCode::HostPrecedenceViolation,
            15 => IcmpCode::PrecedenceCutoff,
            _ => IcmpCode::Unknown,
        }
    }
}

/// Cabecera ICMP
#[derive(Debug, Clone, Copy)]
pub struct IcmpHeader {
    pub message_type: IcmpType,
    pub code: IcmpCode,
    pub checksum: u16,
    pub identifier: u16,
    pub sequence_number: u16,
}

impl IcmpHeader {
    /// Crear nueva cabecera ICMP
    pub fn new(message_type: IcmpType, code: IcmpCode, identifier: u16, sequence_number: u16) -> Self {
        Self {
            message_type,
            code,
            checksum: 0, // Se calculará
            identifier,
            sequence_number,
        }
    }
    
    /// Tamaño de la cabecera ICMP
    pub const fn size() -> usize {
        8
    }
    
    /// Calcular checksum ICMP
    pub fn calculate_checksum(&self, payload: &[u8]) -> u16 {
        let mut sum: u32 = 0;
        
        // Sumar campos de la cabecera
        sum += (self.message_type as u32) << 8;
        sum += self.code as u32;
        sum += self.identifier as u32;
        sum += self.sequence_number as u32;
        
        // Sumar payload en palabras de 16 bits
        let mut i = 0;
        while i < payload.len() {
            if i + 1 < payload.len() {
                sum += ((payload[i] as u32) << 8) | (payload[i + 1] as u32);
            } else {
                sum += (payload[i] as u32) << 8;
            }
            i += 2;
        }
        
        // Sumar los acarreos
        while (sum >> 16) != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        
        // Complemento a uno
        !(sum as u16)
    }
    
    /// Serializar cabecera a bytes
    pub fn to_bytes(&self) -> [u8; 8] {
        let mut bytes = [0u8; 8];
        
        // Message Type
        bytes[0] = self.message_type as u8;
        
        // Code
        bytes[1] = self.code as u8;
        
        // Checksum (2 bytes, big-endian)
        let checksum_bytes = self.checksum.to_be_bytes();
        bytes[2..4].copy_from_slice(&checksum_bytes);
        
        // Identifier (2 bytes, big-endian)
        let id_bytes = self.identifier.to_be_bytes();
        bytes[4..6].copy_from_slice(&id_bytes);
        
        // Sequence Number (2 bytes, big-endian)
        let seq_bytes = self.sequence_number.to_be_bytes();
        bytes[6..8].copy_from_slice(&seq_bytes);
        
        bytes
    }
    
    /// Deserializar cabecera desde bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }
        
        let message_type = IcmpType::from(bytes[0]);
        let code = IcmpCode::from(bytes[1]);
        
        let checksum_bytes = [bytes[2], bytes[3]];
        let checksum = u16::from_be_bytes(checksum_bytes);
        
        let id_bytes = [bytes[4], bytes[5]];
        let identifier = u16::from_be_bytes(id_bytes);
        
        let seq_bytes = [bytes[6], bytes[7]];
        let sequence_number = u16::from_be_bytes(seq_bytes);
        
        Some(Self {
            message_type,
            code,
            checksum,
            identifier,
            sequence_number,
        })
    }
}

/// Paquete ICMP completo
#[derive(Debug, Clone)]
pub struct IcmpPacket {
    pub header: IcmpHeader,
    pub payload: [u8; 1472], // MTU - cabeceras IP e ICMP
    pub payload_len: usize,
}

impl IcmpPacket {
    /// Crear nuevo paquete ICMP
    pub fn new(header: IcmpHeader, payload: &[u8]) -> Self {
        let mut packet_payload = [0u8; 1472];
        let payload_len = core::cmp::min(payload.len(), 1472);
        packet_payload[..payload_len].copy_from_slice(&payload[..payload_len]);
        
        Self {
            header,
            payload: packet_payload,
            payload_len,
        }
    }
    
    /// Tamaño total del paquete
    pub fn total_size(&self) -> usize {
        IcmpHeader::size() + self.payload_len
    }
    
    /// Serializar paquete completo a bytes
    pub fn to_bytes(&self) -> [u8; 1480] {
        let mut bytes = [0u8; 1480];
        
        // Cabecera
        let header_bytes = self.header.to_bytes();
        bytes[0..8].copy_from_slice(&header_bytes);
        
        // Payload
        bytes[8..8 + self.payload_len].copy_from_slice(&self.payload[..self.payload_len]);
        
        bytes
    }
    
    /// Deserializar paquete desde bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }
        
        let header = IcmpHeader::from_bytes(bytes)?;
        let payload_len = bytes.len() - 8;
        
        if payload_len > 1472 {
            return None;
        }
        
        let mut packet_payload = [0u8; 1472];
        packet_payload[..payload_len].copy_from_slice(&bytes[8..8 + payload_len]);
        
        Some(Self {
            header,
            payload: packet_payload,
            payload_len,
        })
    }
}

/// Gestor de protocolo ICMP
pub struct IcmpManager {
    pub packets_sent: u64,
    pub packets_received: u64,
    pub echo_requests: u64,
    pub echo_replies: u64,
    pub destination_unreachable: u64,
    pub time_exceeded: u64,
}

impl IcmpManager {
    /// Crear nuevo gestor ICMP
    pub fn new() -> Self {
        Self {
            packets_sent: 0,
            packets_received: 0,
            echo_requests: 0,
            echo_replies: 0,
            destination_unreachable: 0,
            time_exceeded: 0,
        }
    }
    
    /// Enviar solicitud de eco (ping)
    pub fn send_echo_request(&mut self, identifier: u16, sequence_number: u16, payload: &[u8]) -> bool {
        let header = IcmpHeader::new(
            IcmpType::EchoRequest,
            IcmpCode::None,
            identifier,
            sequence_number,
        );
        
        let mut packet = IcmpPacket::new(header, payload);
        
        // Calcular checksum
        packet.header.checksum = packet.header.calculate_checksum(&packet.payload[..packet.payload_len]);
        
        self.packets_sent += 1;
        self.echo_requests += 1;
        
        // En una implementación real, esto enviaría el paquete por IP
        true
    }
    
    /// Enviar respuesta de eco (pong)
    pub fn send_echo_reply(&mut self, identifier: u16, sequence_number: u16, payload: &[u8]) -> bool {
        let header = IcmpHeader::new(
            IcmpType::EchoReply,
            IcmpCode::None,
            identifier,
            sequence_number,
        );
        
        let mut packet = IcmpPacket::new(header, payload);
        
        // Calcular checksum
        packet.header.checksum = packet.header.calculate_checksum(&packet.payload[..packet.payload_len]);
        
        self.packets_sent += 1;
        self.echo_replies += 1;
        
        // En una implementación real, esto enviaría el paquete por IP
        true
    }
    
    /// Enviar mensaje de destino inalcanzable
    pub fn send_destination_unreachable(&mut self, code: IcmpCode) -> bool {
        let header = IcmpHeader::new(
            IcmpType::DestinationUnreachable,
            code,
            0,
            0,
        );
        
        let packet = IcmpPacket::new(header, &[]);
        
        self.packets_sent += 1;
        self.destination_unreachable += 1;
        
        // En una implementación real, esto enviaría el paquete por IP
        true
    }
    
    /// Enviar mensaje de tiempo excedido
    pub fn send_time_exceeded(&mut self, code: IcmpCode) -> bool {
        let header = IcmpHeader::new(
            IcmpType::TimeExceeded,
            code,
            0,
            0,
        );
        
        let packet = IcmpPacket::new(header, &[]);
        
        self.packets_sent += 1;
        self.time_exceeded += 1;
        
        // En una implementación real, esto enviaría el paquete por IP
        true
    }
    
    /// Procesar paquete ICMP recibido
    pub fn process_packet(&mut self, packet: &IcmpPacket) -> bool {
        self.packets_received += 1;
        
        match packet.header.message_type {
            IcmpType::EchoRequest => {
                // Responder con echo reply
                self.send_echo_reply(
                    packet.header.identifier,
                    packet.header.sequence_number,
                    &packet.payload[..packet.payload_len],
                );
                true
            }
            IcmpType::EchoReply => {
                // Procesar respuesta de eco
                true
            }
            IcmpType::DestinationUnreachable => {
                // Procesar mensaje de destino inalcanzable
                true
            }
            IcmpType::TimeExceeded => {
                // Procesar mensaje de tiempo excedido
                true
            }
            IcmpType::Unknown => false,
        }
    }
    
    /// Obtener estadísticas
    pub fn get_statistics(&self) -> (u64, u64, u64, u64, u64, u64) {
        (
            self.packets_sent,
            self.packets_received,
            self.echo_requests,
            self.echo_replies,
            self.destination_unreachable,
            self.time_exceeded,
        )
    }
}
