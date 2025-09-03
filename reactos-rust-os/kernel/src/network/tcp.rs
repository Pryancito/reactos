//! Protocolo TCP (Transmission Control Protocol)
//! 
//! Implementa el protocolo TCP para la capa de transporte

/// Estados de conexión TCP
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
}

/// Cabecera TCP
#[derive(Debug, Clone, Copy)]
pub struct TcpHeader {
    pub source_port: u16,
    pub destination_port: u16,
    pub sequence_number: u32,
    pub acknowledgment_number: u32,
    pub data_offset: u8,    // 4 bits
    pub reserved: u8,       // 3 bits
    pub flags: u8,          // 9 bits
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
}

/// Flags TCP
pub mod tcp_flags {
    pub const FIN: u8 = 0x01;
    pub const SYN: u8 = 0x02;
    pub const RST: u8 = 0x04;
    pub const PSH: u8 = 0x08;
    pub const ACK: u8 = 0x10;
    pub const URG: u8 = 0x20;
}

impl TcpHeader {
    /// Crear nueva cabecera TCP
    pub fn new(
        source_port: u16,
        destination_port: u16,
        sequence_number: u32,
        acknowledgment_number: u32,
        flags: u8,
    ) -> Self {
        Self {
            source_port,
            destination_port,
            sequence_number,
            acknowledgment_number,
            data_offset: 5, // 20 bytes / 4 = 5
            reserved: 0,
            flags,
            window_size: 65535,
            checksum: 0, // Se calculará
            urgent_pointer: 0,
        }
    }
    
    /// Tamaño de la cabecera TCP
    pub const fn size() -> usize {
        20
    }
    
    /// Verificar si tiene flag FIN
    pub fn has_fin(&self) -> bool {
        (self.flags & tcp_flags::FIN) != 0
    }
    
    /// Verificar si tiene flag SYN
    pub fn has_syn(&self) -> bool {
        (self.flags & tcp_flags::SYN) != 0
    }
    
    /// Verificar si tiene flag RST
    pub fn has_rst(&self) -> bool {
        (self.flags & tcp_flags::RST) != 0
    }
    
    /// Verificar si tiene flag PSH
    pub fn has_psh(&self) -> bool {
        (self.flags & tcp_flags::PSH) != 0
    }
    
    /// Verificar si tiene flag ACK
    pub fn has_ack(&self) -> bool {
        (self.flags & tcp_flags::ACK) != 0
    }
    
    /// Verificar si tiene flag URG
    pub fn has_urg(&self) -> bool {
        (self.flags & tcp_flags::URG) != 0
    }
    
    /// Serializar cabecera a bytes
    pub fn to_bytes(&self) -> [u8; 20] {
        let mut bytes = [0u8; 20];
        
        // Source Port (2 bytes, big-endian)
        let src_port_bytes = self.source_port.to_be_bytes();
        bytes[0..2].copy_from_slice(&src_port_bytes);
        
        // Destination Port (2 bytes, big-endian)
        let dst_port_bytes = self.destination_port.to_be_bytes();
        bytes[2..4].copy_from_slice(&dst_port_bytes);
        
        // Sequence Number (4 bytes, big-endian)
        let seq_bytes = self.sequence_number.to_be_bytes();
        bytes[4..8].copy_from_slice(&seq_bytes);
        
        // Acknowledgment Number (4 bytes, big-endian)
        let ack_bytes = self.acknowledgment_number.to_be_bytes();
        bytes[8..12].copy_from_slice(&ack_bytes);
        
        // Data Offset (4 bits) + Reserved (3 bits) + Flags (9 bits)
        let offset_res_flags = ((self.data_offset as u16) << 12) | 
                              ((self.reserved as u16) << 9) | 
                              (self.flags as u16);
        let offset_res_flags_bytes = offset_res_flags.to_be_bytes();
        bytes[12..14].copy_from_slice(&offset_res_flags_bytes);
        
        // Window Size (2 bytes, big-endian)
        let window_bytes = self.window_size.to_be_bytes();
        bytes[14..16].copy_from_slice(&window_bytes);
        
        // Checksum (2 bytes, big-endian)
        let checksum_bytes = self.checksum.to_be_bytes();
        bytes[16..18].copy_from_slice(&checksum_bytes);
        
        // Urgent Pointer (2 bytes, big-endian)
        let urgent_bytes = self.urgent_pointer.to_be_bytes();
        bytes[18..20].copy_from_slice(&urgent_bytes);
        
        bytes
    }
    
    /// Deserializar cabecera desde bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 20 {
            return None;
        }
        
        let src_port_bytes = [bytes[0], bytes[1]];
        let source_port = u16::from_be_bytes(src_port_bytes);
        
        let dst_port_bytes = [bytes[2], bytes[3]];
        let destination_port = u16::from_be_bytes(dst_port_bytes);
        
        let seq_bytes = [bytes[4], bytes[5], bytes[6], bytes[7]];
        let sequence_number = u32::from_be_bytes(seq_bytes);
        
        let ack_bytes = [bytes[8], bytes[9], bytes[10], bytes[11]];
        let acknowledgment_number = u32::from_be_bytes(ack_bytes);
        
        let offset_res_flags_bytes = [bytes[12], bytes[13]];
        let offset_res_flags = u16::from_be_bytes(offset_res_flags_bytes);
        let data_offset = ((offset_res_flags >> 12) & 0x0F) as u8;
        let reserved = ((offset_res_flags >> 9) & 0x07) as u8;
        let flags = (offset_res_flags & 0x1FF) as u8;
        
        let window_bytes = [bytes[14], bytes[15]];
        let window_size = u16::from_be_bytes(window_bytes);
        
        let checksum_bytes = [bytes[16], bytes[17]];
        let checksum = u16::from_be_bytes(checksum_bytes);
        
        let urgent_bytes = [bytes[18], bytes[19]];
        let urgent_pointer = u16::from_be_bytes(urgent_bytes);
        
        Some(Self {
            source_port,
            destination_port,
            sequence_number,
            acknowledgment_number,
            data_offset,
            reserved,
            flags,
            window_size,
            checksum,
            urgent_pointer,
        })
    }
}

/// Segmento TCP completo
#[derive(Debug, Clone)]
pub struct TcpSegment {
    pub header: TcpHeader,
    pub payload: [u8; 1460], // MSS típico
    pub payload_len: usize,
}

impl TcpSegment {
    /// Crear nuevo segmento TCP
    pub fn new(header: TcpHeader, payload: &[u8]) -> Self {
        let mut segment_payload = [0u8; 1460];
        let payload_len = core::cmp::min(payload.len(), 1460);
        segment_payload[..payload_len].copy_from_slice(&payload[..payload_len]);
        
        Self {
            header,
            payload: segment_payload,
            payload_len,
        }
    }
    
    /// Tamaño total del segmento
    pub fn total_size(&self) -> usize {
        TcpHeader::size() + self.payload_len
    }
    
    /// Serializar segmento completo a bytes
    pub fn to_bytes(&self) -> [u8; 1480] {
        let mut bytes = [0u8; 1480];
        
        // Cabecera
        let header_bytes = self.header.to_bytes();
        bytes[0..20].copy_from_slice(&header_bytes);
        
        // Payload
        bytes[20..20 + self.payload_len].copy_from_slice(&self.payload[..self.payload_len]);
        
        bytes
    }
    
    /// Deserializar segmento desde bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 20 {
            return None;
        }
        
        let header = TcpHeader::from_bytes(bytes)?;
        let payload_len = bytes.len() - 20;
        
        if payload_len > 1460 {
            return None;
        }
        
        let mut segment_payload = [0u8; 1460];
        segment_payload[..payload_len].copy_from_slice(&bytes[20..20 + payload_len]);
        
        Some(Self {
            header,
            payload: segment_payload,
            payload_len,
        })
    }
}

/// Conexión TCP
#[derive(Debug, Clone, Copy)]
pub struct TcpConnection {
    pub local_port: u16,
    pub remote_port: u16,
    pub state: TcpState,
    pub sequence_number: u32,
    pub acknowledgment_number: u32,
    pub window_size: u16,
    pub send_buffer: [u8; 8192],
    pub recv_buffer: [u8; 8192],
    pub send_buffer_len: usize,
    pub recv_buffer_len: usize,
}

impl TcpConnection {
    /// Crear nueva conexión TCP
    pub fn new(local_port: u16, remote_port: u16) -> Self {
        Self {
            local_port,
            remote_port,
            state: TcpState::Closed,
            sequence_number: 0,
            acknowledgment_number: 0,
            window_size: 65535,
            send_buffer: [0u8; 8192],
            recv_buffer: [0u8; 8192],
            send_buffer_len: 0,
            recv_buffer_len: 0,
        }
    }
    
    /// Iniciar conexión (enviar SYN)
    pub fn connect(&mut self) {
        self.state = TcpState::SynSent;
        self.sequence_number = 1; // Número de secuencia inicial
    }
    
    /// Aceptar conexión (enviar SYN+ACK)
    pub fn accept(&mut self) {
        self.state = TcpState::SynReceived;
    }
    
    /// Establecer conexión
    pub fn establish(&mut self) {
        self.state = TcpState::Established;
    }
    
    /// Cerrar conexión
    pub fn close(&mut self) {
        self.state = TcpState::FinWait1;
    }
    
    /// Enviar datos
    pub fn send_data(&mut self, data: &[u8]) -> bool {
        if self.state != TcpState::Established {
            return false;
        }
        
        let available_space = 8192 - self.send_buffer_len;
        let data_to_copy = core::cmp::min(data.len(), available_space);
        
        if data_to_copy == 0 {
            return false;
        }
        
        self.send_buffer[self.send_buffer_len..self.send_buffer_len + data_to_copy]
            .copy_from_slice(&data[..data_to_copy]);
        self.send_buffer_len += data_to_copy;
        
        true
    }
    
    /// Recibir datos
    pub fn receive_data(&mut self, data: &mut [u8]) -> usize {
        if self.state != TcpState::Established {
            return 0;
        }
        
        let data_to_copy = core::cmp::min(data.len(), self.recv_buffer_len);
        
        if data_to_copy > 0 {
            data[..data_to_copy].copy_from_slice(&self.recv_buffer[..data_to_copy]);
            
            // Mover datos restantes al inicio del buffer
            if data_to_copy < self.recv_buffer_len {
                self.recv_buffer.copy_within(data_to_copy..self.recv_buffer_len, 0);
            }
            self.recv_buffer_len -= data_to_copy;
        }
        
        data_to_copy
    }
}

/// Gestor de protocolo TCP
pub struct TcpManager {
    pub connections: [Option<TcpConnection>; 64],
    pub segments_sent: u64,
    pub segments_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

impl TcpManager {
    /// Crear nuevo gestor TCP
    pub fn new() -> Self {
        Self {
            connections: [None; 64],
            segments_sent: 0,
            segments_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
        }
    }
    
    /// Crear nueva conexión
    pub fn create_connection(&mut self, local_port: u16, remote_port: u16) -> Option<usize> {
        for i in 0..64 {
            if self.connections[i].is_none() {
                self.connections[i] = Some(TcpConnection::new(local_port, remote_port));
                return Some(i);
            }
        }
        None
    }
    
    /// Cerrar conexión
    pub fn close_connection(&mut self, connection_id: usize) -> bool {
        if connection_id >= 64 {
            return false;
        }
        
        if let Some(conn) = &mut self.connections[connection_id] {
            conn.close();
            return true;
        }
        
        false
    }
    
    /// Enviar segmento TCP
    pub fn send_segment(&mut self, segment: &TcpSegment) -> bool {
        self.segments_sent += 1;
        self.bytes_sent += segment.total_size() as u64;
        true
    }
    
    /// Recibir segmento TCP
    pub fn receive_segment(&mut self, segment: &TcpSegment) -> bool {
        self.segments_received += 1;
        self.bytes_received += segment.total_size() as u64;
        true
    }
    
    /// Obtener estadísticas
    pub fn get_statistics(&self) -> (u64, u64, u64, u64) {
        (self.segments_sent, self.segments_received, self.bytes_sent, self.bytes_received)
    }
    
    /// Obtener número de conexiones activas
    pub fn get_active_connections(&self) -> u32 {
        let mut count = 0;
        for conn in &self.connections {
            if let Some(connection) = conn {
                if connection.state != TcpState::Closed {
                    count += 1;
                }
            }
        }
        count
    }
}
