//! Protocolo ARP (Address Resolution Protocol)
//! 
//! Implementa el protocolo ARP para resolver direcciones IP a MAC

use super::ethernet::MacAddress;
use super::ip::IpAddress;

/// Tipos de operación ARP
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArpOperation {
    Request = 1,
    Reply = 2,
    Unknown = 0,
}

impl From<u16> for ArpOperation {
    fn from(value: u16) -> Self {
        match value {
            1 => ArpOperation::Request,
            2 => ArpOperation::Reply,
            _ => ArpOperation::Unknown,
        }
    }
}

/// Cabecera ARP
#[derive(Debug, Clone, Copy)]
pub struct ArpHeader {
    pub hardware_type: u16,    // 1 para Ethernet
    pub protocol_type: u16,    // 0x0800 para IPv4
    pub hardware_size: u8,     // 6 para MAC
    pub protocol_size: u8,     // 4 para IPv4
    pub operation: ArpOperation,
    pub sender_mac: MacAddress,
    pub sender_ip: IpAddress,
    pub target_mac: MacAddress,
    pub target_ip: IpAddress,
}

impl ArpHeader {
    /// Crear nueva cabecera ARP
    pub fn new(
        operation: ArpOperation,
        sender_mac: MacAddress,
        sender_ip: IpAddress,
        target_mac: MacAddress,
        target_ip: IpAddress,
    ) -> Self {
        Self {
            hardware_type: 1,      // Ethernet
            protocol_type: 0x0800, // IPv4
            hardware_size: 6,      // MAC address size
            protocol_size: 4,      // IPv4 address size
            operation,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        }
    }
    
    /// Tamaño de la cabecera ARP
    pub const fn size() -> usize {
        28 // 2+2+1+1+2+6+4+6+4
    }
    
    /// Serializar cabecera a bytes
    pub fn to_bytes(&self) -> [u8; 28] {
        let mut bytes = [0u8; 28];
        let mut pos = 0;
        
        // Hardware Type (2 bytes, big-endian)
        let hw_type_bytes = self.hardware_type.to_be_bytes();
        bytes[pos..pos+2].copy_from_slice(&hw_type_bytes);
        pos += 2;
        
        // Protocol Type (2 bytes, big-endian)
        let proto_type_bytes = self.protocol_type.to_be_bytes();
        bytes[pos..pos+2].copy_from_slice(&proto_type_bytes);
        pos += 2;
        
        // Hardware Size
        bytes[pos] = self.hardware_size;
        pos += 1;
        
        // Protocol Size
        bytes[pos] = self.protocol_size;
        pos += 1;
        
        // Operation (2 bytes, big-endian)
        let op_bytes = (self.operation as u16).to_be_bytes();
        bytes[pos..pos+2].copy_from_slice(&op_bytes);
        pos += 2;
        
        // Sender MAC (6 bytes)
        bytes[pos..pos+6].copy_from_slice(&self.sender_mac.bytes);
        pos += 6;
        
        // Sender IP (4 bytes)
        bytes[pos..pos+4].copy_from_slice(&self.sender_ip.bytes);
        pos += 4;
        
        // Target MAC (6 bytes)
        bytes[pos..pos+6].copy_from_slice(&self.target_mac.bytes);
        pos += 6;
        
        // Target IP (4 bytes)
        bytes[pos..pos+4].copy_from_slice(&self.target_ip.bytes);
        
        bytes
    }
    
    /// Deserializar cabecera desde bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 28 {
            return None;
        }
        
        let mut pos = 0;
        
        // Hardware Type
        let hw_type_bytes = [bytes[pos], bytes[pos+1]];
        let hardware_type = u16::from_be_bytes(hw_type_bytes);
        pos += 2;
        
        // Protocol Type
        let proto_type_bytes = [bytes[pos], bytes[pos+1]];
        let protocol_type = u16::from_be_bytes(proto_type_bytes);
        pos += 2;
        
        // Hardware Size
        let hardware_size = bytes[pos];
        pos += 1;
        
        // Protocol Size
        let protocol_size = bytes[pos];
        pos += 1;
        
        // Operation
        let op_bytes = [bytes[pos], bytes[pos+1]];
        let operation = ArpOperation::from(u16::from_be_bytes(op_bytes));
        pos += 2;
        
        // Sender MAC
        let mut sender_mac_bytes = [0u8; 6];
        sender_mac_bytes.copy_from_slice(&bytes[pos..pos+6]);
        let sender_mac = MacAddress::new(sender_mac_bytes);
        pos += 6;
        
        // Sender IP
        let mut sender_ip_bytes = [0u8; 4];
        sender_ip_bytes.copy_from_slice(&bytes[pos..pos+4]);
        let sender_ip = IpAddress::new(sender_ip_bytes);
        pos += 4;
        
        // Target MAC
        let mut target_mac_bytes = [0u8; 6];
        target_mac_bytes.copy_from_slice(&bytes[pos..pos+6]);
        let target_mac = MacAddress::new(target_mac_bytes);
        pos += 6;
        
        // Target IP
        let mut target_ip_bytes = [0u8; 4];
        target_ip_bytes.copy_from_slice(&bytes[pos..pos+4]);
        let target_ip = IpAddress::new(target_ip_bytes);
        
        Some(Self {
            hardware_type,
            protocol_type,
            hardware_size,
            protocol_size,
            operation,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        })
    }
}

/// Paquete ARP completo
#[derive(Debug, Clone)]
pub struct ArpPacket {
    pub header: ArpHeader,
}

impl ArpPacket {
    /// Crear nuevo paquete ARP
    pub fn new(header: ArpHeader) -> Self {
        Self { header }
    }
    
    /// Tamaño total del paquete
    pub fn total_size(&self) -> usize {
        ArpHeader::size()
    }
    
    /// Serializar paquete completo a bytes
    pub fn to_bytes(&self) -> [u8; 28] {
        self.header.to_bytes()
    }
    
    /// Deserializar paquete desde bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 28 {
            return None;
        }
        
        let header = ArpHeader::from_bytes(bytes)?;
        Some(Self { header })
    }
}

/// Entrada de la tabla ARP
#[derive(Debug, Clone, Copy)]
pub struct ArpEntry {
    pub ip_address: IpAddress,
    pub mac_address: MacAddress,
    pub is_valid: bool,
}

impl ArpEntry {
    /// Crear nueva entrada ARP
    pub fn new(ip_address: IpAddress, mac_address: MacAddress) -> Self {
        Self {
            ip_address,
            mac_address,
            is_valid: true,
        }
    }
    
    /// Invalidar entrada
    pub fn invalidate(&mut self) {
        self.is_valid = false;
    }
}

/// Tabla ARP
pub struct ArpTable {
    pub entries: [Option<ArpEntry>; 256],
    pub requests_sent: u64,
    pub replies_received: u64,
}

impl ArpTable {
    /// Crear nueva tabla ARP
    pub fn new() -> Self {
        Self {
            entries: [None; 256],
            requests_sent: 0,
            replies_received: 0,
        }
    }
    
    /// Buscar dirección MAC por IP
    pub fn lookup(&self, ip_address: IpAddress) -> Option<MacAddress> {
        for entry in &self.entries {
            if let Some(arp_entry) = entry {
                if arp_entry.is_valid && arp_entry.ip_address == ip_address {
                    return Some(arp_entry.mac_address);
                }
            }
        }
        None
    }
    
    /// Agregar entrada a la tabla
    pub fn add_entry(&mut self, ip_address: IpAddress, mac_address: MacAddress) -> bool {
        // Buscar slot vacío
        for i in 0..256 {
            if self.entries[i].is_none() {
                self.entries[i] = Some(ArpEntry::new(ip_address, mac_address));
                return true;
            }
        }
        
        // Si no hay slots vacíos, reemplazar una entrada inválida
        for i in 0..256 {
            if let Some(entry) = &mut self.entries[i] {
                if !entry.is_valid {
                    *entry = ArpEntry::new(ip_address, mac_address);
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Invalidar entrada
    pub fn invalidate_entry(&mut self, ip_address: IpAddress) -> bool {
        for entry in &mut self.entries {
            if let Some(arp_entry) = entry {
                if arp_entry.ip_address == ip_address {
                    arp_entry.invalidate();
                    return true;
                }
            }
        }
        false
    }
    
    /// Obtener estadísticas
    pub fn get_statistics(&self) -> (u64, u64, u32) {
        let mut valid_entries = 0;
        for entry in &self.entries {
            if let Some(arp_entry) = entry {
                if arp_entry.is_valid {
                    valid_entries += 1;
                }
            }
        }
        
        (self.requests_sent, self.replies_received, valid_entries)
    }
}

/// Gestor de protocolo ARP
pub struct ArpManager {
    pub table: ArpTable,
    pub packets_sent: u64,
    pub packets_received: u64,
}

impl ArpManager {
    /// Crear nuevo gestor ARP
    pub fn new() -> Self {
        Self {
            table: ArpTable::new(),
            packets_sent: 0,
            packets_received: 0,
        }
    }
    
    /// Enviar solicitud ARP
    pub fn send_request(&mut self, target_ip: IpAddress, sender_mac: MacAddress, sender_ip: IpAddress) -> bool {
        let header = ArpHeader::new(
            ArpOperation::Request,
            sender_mac,
            sender_ip,
            MacAddress::null(),
            target_ip,
        );
        
        let packet = ArpPacket::new(header);
        self.packets_sent += 1;
        self.table.requests_sent += 1;
        
        // En una implementación real, esto enviaría el paquete por Ethernet
        true
    }
    
    /// Enviar respuesta ARP
    pub fn send_reply(&mut self, target_mac: MacAddress, target_ip: IpAddress, sender_mac: MacAddress, sender_ip: IpAddress) -> bool {
        let header = ArpHeader::new(
            ArpOperation::Reply,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        );
        
        let packet = ArpPacket::new(header);
        self.packets_sent += 1;
        
        // En una implementación real, esto enviaría el paquete por Ethernet
        true
    }
    
    /// Procesar paquete ARP recibido
    pub fn process_packet(&mut self, packet: &ArpPacket) -> bool {
        self.packets_received += 1;
        
        match packet.header.operation {
            ArpOperation::Request => {
                // Procesar solicitud ARP
                true
            }
            ArpOperation::Reply => {
                // Procesar respuesta ARP
                self.table.add_entry(packet.header.sender_ip, packet.header.sender_mac);
                self.table.replies_received += 1;
                true
            }
            ArpOperation::Unknown => false,
        }
    }
    
    /// Obtener estadísticas
    pub fn get_statistics(&self) -> (u64, u64, u64, u64, u32) {
        let (requests, replies, valid_entries) = self.table.get_statistics();
        (self.packets_sent, self.packets_received, requests, replies, valid_entries)
    }
}
