//! ReactOS Rust Kernel - Network Stack
//!
//! Sistema de red del kernel.

use core::arch::asm;

/// Dirección MAC (6 bytes)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MacAddress {
    pub bytes: [u8; 6],
}

impl MacAddress {
    pub fn new(bytes: [u8; 6]) -> Self {
        Self { bytes }
    }
    
    pub fn broadcast() -> Self {
        Self { bytes: [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF] }
    }
    
    pub fn is_broadcast(&self) -> bool {
        self.bytes == [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
    }
}

/// Dirección IPv4 (4 bytes)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ipv4Address {
    pub bytes: [u8; 4],
}

/// Dirección IPv6 (16 bytes)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ipv6Address {
    pub bytes: [u8; 16],
}

/// Dirección IP (IPv4 o IPv6)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IpAddress {
    V4(Ipv4Address),
    V6(Ipv6Address),
}

impl Ipv4Address {
    pub fn new(bytes: [u8; 4]) -> Self {
        Self { bytes }
    }
    
    pub fn localhost() -> Self {
        Self { bytes: [127, 0, 0, 1] }
    }
    
    pub fn broadcast() -> Self {
        Self { bytes: [255, 255, 255, 255] }
    }
    
    pub fn is_broadcast(&self) -> bool {
        self.bytes == [255, 255, 255, 255]
    }
    
    pub fn is_localhost(&self) -> bool {
        self.bytes == [127, 0, 0, 1]
    }
    
    pub fn is_private(&self) -> bool {
        // 10.0.0.0/8
        self.bytes[0] == 10 ||
        // 172.16.0.0/12
        (self.bytes[0] == 172 && self.bytes[1] >= 16 && self.bytes[1] <= 31) ||
        // 192.168.0.0/16
        (self.bytes[0] == 192 && self.bytes[1] == 168)
    }
}

impl Ipv6Address {
    pub fn new(bytes: [u8; 16]) -> Self {
        Self { bytes }
    }
    
    pub fn localhost() -> Self {
        let mut bytes = [0u8; 16];
        bytes[15] = 1;
        Self { bytes }
    }
    
    pub fn is_localhost(&self) -> bool {
        let mut localhost = [0u8; 16];
        localhost[15] = 1;
        self.bytes == localhost
    }
    
    pub fn is_link_local(&self) -> bool {
        // fe80::/10
        self.bytes[0] == 0xfe && (self.bytes[1] & 0xc0) == 0x80
    }
    
    pub fn is_multicast(&self) -> bool {
        // ff00::/8
        self.bytes[0] == 0xff
    }
    
    pub fn is_unspecified(&self) -> bool {
        self.bytes == [0u8; 16]
    }
}

impl IpAddress {
    pub fn new_v4(bytes: [u8; 4]) -> Self {
        Self::V4(Ipv4Address::new(bytes))
    }
    
    pub fn new_v6(bytes: [u8; 16]) -> Self {
        Self::V6(Ipv6Address::new(bytes))
    }
    
    pub fn localhost_v4() -> Self {
        Self::V4(Ipv4Address::localhost())
    }
    
    pub fn localhost_v6() -> Self {
        Self::V6(Ipv6Address::localhost())
    }
    
    pub fn is_localhost(&self) -> bool {
        match self {
            Self::V4(addr) => addr.is_localhost(),
            Self::V6(addr) => addr.is_localhost(),
        }
    }
    
    pub fn is_broadcast(&self) -> bool {
        match self {
            Self::V4(addr) => addr.is_broadcast(),
            Self::V6(_) => false, // IPv6 no tiene broadcast
        }
    }
}

/// Cabecera Ethernet
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct EthernetHeader {
    pub dest_mac: MacAddress,
    pub src_mac: MacAddress,
    pub ethertype: u16,
}

/// Tipos de protocolo Ethernet
pub const ETHERTYPE_IP: u16 = 0x0800;
pub const ETHERTYPE_ARP: u16 = 0x0806;
pub const ETHERTYPE_IPV6: u16 = 0x86DD;

/// Cabecera IPv4
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Ipv4Header {
    pub version_ihl: u8,
    pub tos: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags_fragment: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub src_ip: Ipv4Address,
    pub dest_ip: Ipv4Address,
}

/// Cabecera IPv6
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Ipv6Header {
    pub version_traffic_class_flow_label: u32,
    pub payload_length: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub src_ip: Ipv6Address,
    pub dest_ip: Ipv6Address,
}

/// Protocolos IP
pub const IP_PROTOCOL_ICMP: u8 = 1;
pub const IP_PROTOCOL_TCP: u8 = 6;
pub const IP_PROTOCOL_UDP: u8 = 17;
pub const IP_PROTOCOL_ICMPV6: u8 = 58;

/// Protocolos IPv6
pub const IPV6_PROTOCOL_ICMPV6: u8 = 58;
pub const IPV6_PROTOCOL_TCP: u8 = 6;
pub const IPV6_PROTOCOL_UDP: u8 = 17;

/// Cabecera TCP
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct TcpHeader {
    pub src_port: u16,
    pub dest_port: u16,
    pub sequence: u32,
    pub ack_sequence: u32,
    pub flags: u16,
    pub window: u16,
    pub checksum: u16,
    pub urgent: u16,
}

/// Flags TCP
pub const TCP_FLAG_FIN: u16 = 0x001;
pub const TCP_FLAG_SYN: u16 = 0x002;
pub const TCP_FLAG_RST: u16 = 0x004;
pub const TCP_FLAG_PSH: u16 = 0x008;
pub const TCP_FLAG_ACK: u16 = 0x010;
pub const TCP_FLAG_URG: u16 = 0x020;

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

/// Conexión TCP
#[derive(Debug, Clone, Copy)]
pub struct TcpConnection {
    pub local_ip: IpAddress,
    pub local_port: u16,
    pub remote_ip: IpAddress,
    pub remote_port: u16,
    pub state: TcpState,
    pub sequence: u32,
    pub ack_sequence: u32,
    pub window: u16,
    pub buffer: [u8; 1024],
    pub buffer_size: usize,
    pub ip_version: u8, // 4 o 6
}

/// Cabecera UDP
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dest_port: u16,
    pub length: u16,
    pub checksum: u16,
}

/// Socket UDP
#[derive(Debug, Clone, Copy)]
pub struct UdpSocket {
    pub local_ip: IpAddress,
    pub local_port: u16,
    pub buffer: [u8; 1024],
    pub buffer_size: usize,
    pub ip_version: u8, // 4 o 6
}

/// Cabecera ARP
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct ArpHeader {
    pub hardware_type: u16,
    pub protocol_type: u16,
    pub hardware_size: u8,
    pub protocol_size: u8,
    pub opcode: u16,
    pub sender_mac: MacAddress,
    pub sender_ip: IpAddress,
    pub target_mac: MacAddress,
    pub target_ip: IpAddress,
}

/// Opcodes ARP
pub const ARP_REQUEST: u16 = 1;
pub const ARP_REPLY: u16 = 2;

/// Entrada de tabla ARP
#[derive(Debug, Clone, Copy)]
pub struct ArpEntry {
    pub ip: IpAddress,
    pub mac: MacAddress,
    pub age: u32,
}

/// Stack de red
pub struct NetworkStack {
    pub local_ipv4: Ipv4Address,
    pub local_ipv6: Ipv6Address,
    pub local_mac: MacAddress,
    pub gateway_ipv4: Ipv4Address,
    pub gateway_ipv6: Ipv6Address,
    pub subnet_mask_ipv4: Ipv4Address,
    pub tcp_connections: [Option<TcpConnection>; 64],
    pub udp_sockets: [Option<UdpSocket>; 64],
    pub arp_table: [Option<ArpEntry>; 256],
    pub packet_buffer: [u8; 2048],
    pub stats: NetworkStats,
}

/// Estadísticas de red
#[derive(Debug, Clone, Copy)]
pub struct NetworkStats {
    pub packets_sent: u32,
    pub packets_received: u32,
    pub bytes_sent: u32,
    pub bytes_received: u32,
    pub tcp_connections: u32,
    pub udp_sockets: u32,
    pub arp_entries: u32,
    pub errors: u32,
}

impl NetworkStack {
    /// Crear un nuevo stack de red
    pub fn new() -> Self {
        Self {
            local_ipv4: Ipv4Address::new([192, 168, 1, 100]),
            local_ipv6: Ipv6Address::new([
                0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            ]),
            local_mac: MacAddress::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]),
            gateway_ipv4: Ipv4Address::new([192, 168, 1, 1]),
            gateway_ipv6: Ipv6Address::new([
                0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            ]),
            subnet_mask_ipv4: Ipv4Address::new([255, 255, 255, 0]),
            tcp_connections: [None; 64],
            udp_sockets: [None; 64],
            arp_table: [None; 256],
            packet_buffer: [0; 2048],
            stats: NetworkStats {
                packets_sent: 0,
                packets_received: 0,
                bytes_sent: 0,
                bytes_received: 0,
                tcp_connections: 0,
                udp_sockets: 0,
                arp_entries: 0,
                errors: 0,
            },
        }
    }
    
    /// Inicializar stack de red
    pub fn init(&mut self) {
        // Inicializar tablas
        for i in 0..64 {
            self.tcp_connections[i] = None;
            self.udp_sockets[i] = None;
        }
        
        for i in 0..256 {
            self.arp_table[i] = None;
        }
        
        // Agregar entrada ARP para gateway IPv4
        self.add_arp_entry(IpAddress::V4(self.gateway_ipv4), MacAddress::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x02]));
    }
    
    /// Procesar paquete recibido
    pub fn process_packet(&mut self, packet: &[u8]) -> Result<(), &'static str> {
        if packet.len() < 14 {
            return Err("Packet too short");
        }
        
        // Parsear cabecera Ethernet
        let eth_header = EthernetHeader {
            dest_mac: MacAddress::new([packet[0], packet[1], packet[2], packet[3], packet[4], packet[5]]),
            src_mac: MacAddress::new([packet[6], packet[7], packet[8], packet[9], packet[10], packet[11]]),
            ethertype: u16::from_be_bytes([packet[12], packet[13]]),
        };
        
        // Verificar si el paquete es para nosotros
        if !eth_header.dest_mac.is_broadcast() && eth_header.dest_mac != self.local_mac {
            return Ok(()); // Ignorar paquete no destinado a nosotros
        }
        
        self.stats.packets_received += 1;
        self.stats.bytes_received += packet.len() as u32;
        
        // Procesar según tipo de protocolo
        match eth_header.ethertype {
            ETHERTYPE_IP => self.process_ipv4_packet(&packet[14..])?,
            ETHERTYPE_IPV6 => self.process_ipv6_packet(&packet[14..])?,
            ETHERTYPE_ARP => self.process_arp_packet(&packet[14..])?,
            _ => {
                // Protocolo no soportado
                self.stats.errors += 1;
            }
        }
        
        Ok(())
    }
    
    /// Procesar paquete IPv4
    fn process_ipv4_packet(&mut self, packet: &[u8]) -> Result<(), &'static str> {
        if packet.len() < 20 {
            return Err("IP packet too short");
        }
        
        // Parsear cabecera IPv4
        let ip_header = Ipv4Header {
            version_ihl: packet[0],
            tos: packet[1],
            total_length: u16::from_be_bytes([packet[2], packet[3]]),
            identification: u16::from_be_bytes([packet[4], packet[5]]),
            flags_fragment: u16::from_be_bytes([packet[6], packet[7]]),
            ttl: packet[8],
            protocol: packet[9],
            checksum: u16::from_be_bytes([packet[10], packet[11]]),
            src_ip: Ipv4Address::new([packet[12], packet[13], packet[14], packet[15]]),
            dest_ip: Ipv4Address::new([packet[16], packet[17], packet[18], packet[19]]),
        };
        
        // Verificar si el paquete es para nosotros
        if ip_header.dest_ip != self.local_ipv4 && !ip_header.dest_ip.is_broadcast() {
            return Ok(()); // Ignorar paquete no destinado a nosotros
        }
        
        // Procesar según protocolo
        match ip_header.protocol {
            IP_PROTOCOL_ICMP => self.process_icmp_packet(&packet[20..])?,
            IP_PROTOCOL_TCP => self.process_tcp_packet(&packet[20..])?,
            IP_PROTOCOL_UDP => self.process_udp_packet(&packet[20..])?,
            _ => {
                // Protocolo no soportado
                self.stats.errors += 1;
            }
        }
        
        Ok(())
    }
    
    /// Procesar paquete IPv6
    fn process_ipv6_packet(&mut self, packet: &[u8]) -> Result<(), &'static str> {
        if packet.len() < 40 {
            return Err("IPv6 packet too short");
        }
        
        // Parsear cabecera IPv6
        let ipv6_header = Ipv6Header {
            version_traffic_class_flow_label: u32::from_be_bytes([packet[0], packet[1], packet[2], packet[3]]),
            payload_length: u16::from_be_bytes([packet[4], packet[5]]),
            next_header: packet[6],
            hop_limit: packet[7],
            src_ip: Ipv6Address::new([
                packet[8], packet[9], packet[10], packet[11], packet[12], packet[13], packet[14], packet[15],
                packet[16], packet[17], packet[18], packet[19], packet[20], packet[21], packet[22], packet[23]
            ]),
            dest_ip: Ipv6Address::new([
                packet[24], packet[25], packet[26], packet[27], packet[28], packet[29], packet[30], packet[31],
                packet[32], packet[33], packet[34], packet[35], packet[36], packet[37], packet[38], packet[39]
            ]),
        };
        
        // Verificar si el paquete es para nosotros
        if ipv6_header.dest_ip != self.local_ipv6 && !ipv6_header.dest_ip.is_multicast() {
            return Ok(()); // Ignorar paquete no destinado a nosotros
        }
        
        // Procesar según protocolo
        match ipv6_header.next_header {
            IPV6_PROTOCOL_ICMPV6 => self.process_icmpv6_packet(&packet[40..])?,
            IPV6_PROTOCOL_TCP => self.process_tcp_packet(&packet[40..])?,
            IPV6_PROTOCOL_UDP => self.process_udp_packet(&packet[40..])?,
            _ => {
                // Protocolo no soportado
                self.stats.errors += 1;
            }
        }
        
        Ok(())
    }
    
    /// Procesar paquete ARP
    fn process_arp_packet(&mut self, packet: &[u8]) -> Result<(), &'static str> {
        if packet.len() < 28 {
            return Err("ARP packet too short");
        }
        
        // Parsear cabecera ARP
        let arp_header = ArpHeader {
            hardware_type: u16::from_be_bytes([packet[0], packet[1]]),
            protocol_type: u16::from_be_bytes([packet[2], packet[3]]),
            hardware_size: packet[4],
            protocol_size: packet[5],
            opcode: u16::from_be_bytes([packet[6], packet[7]]),
            sender_mac: MacAddress::new([packet[8], packet[9], packet[10], packet[11], packet[12], packet[13]]),
            sender_ip: IpAddress::new_v4([packet[14], packet[15], packet[16], packet[17]]),
            target_mac: MacAddress::new([packet[18], packet[19], packet[20], packet[21], packet[22], packet[23]]),
            target_ip: IpAddress::new_v4([packet[24], packet[25], packet[26], packet[27]]),
        };
        
        // Agregar entrada ARP
        self.add_arp_entry(arp_header.sender_ip, arp_header.sender_mac);
        
        // Si es una solicitud ARP para nosotros, responder
        if arp_header.opcode == ARP_REQUEST && arp_header.target_ip == IpAddress::V4(self.local_ipv4) {
            self.send_arp_reply(arp_header.sender_ip, arp_header.sender_mac)?;
        }
        
        Ok(())
    }
    
    /// Procesar paquete ICMP
    fn process_icmp_packet(&mut self, packet: &[u8]) -> Result<(), &'static str> {
        if packet.len() < 8 {
            return Err("ICMP packet too short");
        }
        
        let icmp_type = packet[0];
        let icmp_code = packet[1];
        
        // Responder a ping (ICMP Echo Request)
        if icmp_type == 8 && icmp_code == 0 {
            self.send_icmp_reply(packet)?;
        }
        
        Ok(())
    }
    
    /// Procesar paquete ICMPv6
    fn process_icmpv6_packet(&mut self, packet: &[u8]) -> Result<(), &'static str> {
        if packet.len() < 8 {
            return Err("ICMPv6 packet too short");
        }
        
        let icmpv6_type = packet[0];
        let icmpv6_code = packet[1];
        
        // Responder a ping IPv6 (ICMPv6 Echo Request)
        if icmpv6_type == 128 && icmpv6_code == 0 {
            self.send_icmpv6_reply(packet)?;
        }
        
        Ok(())
    }
    
    /// Procesar paquete TCP
    fn process_tcp_packet(&mut self, packet: &[u8]) -> Result<(), &'static str> {
        if packet.len() < 20 {
            return Err("TCP packet too short");
        }
        
        // Parsear cabecera TCP
        let tcp_header = TcpHeader {
            src_port: u16::from_be_bytes([packet[0], packet[1]]),
            dest_port: u16::from_be_bytes([packet[2], packet[3]]),
            sequence: u32::from_be_bytes([packet[4], packet[5], packet[6], packet[7]]),
            ack_sequence: u32::from_be_bytes([packet[8], packet[9], packet[10], packet[11]]),
            flags: u16::from_be_bytes([packet[12], packet[13]]),
            window: u16::from_be_bytes([packet[14], packet[15]]),
            checksum: u16::from_be_bytes([packet[16], packet[17]]),
            urgent: u16::from_be_bytes([packet[18], packet[19]]),
        };
        
        // Buscar conexión TCP
        if let Some(connection) = self.find_tcp_connection(tcp_header.src_port, tcp_header.dest_port) {
            self.handle_tcp_packet(connection, &tcp_header, &packet[20..])?;
        }
        
        Ok(())
    }
    
    /// Procesar paquete UDP
    fn process_udp_packet(&mut self, packet: &[u8]) -> Result<(), &'static str> {
        if packet.len() < 8 {
            return Err("UDP packet too short");
        }
        
        // Parsear cabecera UDP
        let udp_header = UdpHeader {
            src_port: u16::from_be_bytes([packet[0], packet[1]]),
            dest_port: u16::from_be_bytes([packet[2], packet[3]]),
            length: u16::from_be_bytes([packet[4], packet[5]]),
            checksum: u16::from_be_bytes([packet[6], packet[7]]),
        };
        
        // Buscar socket UDP
        if let Some(socket) = self.find_udp_socket(udp_header.dest_port) {
            self.handle_udp_packet(socket, &udp_header, &packet[8..])?;
        }
        
        Ok(())
    }
    
    /// Enviar paquete
    pub fn send_packet(&mut self, packet: &[u8]) -> Result<(), &'static str> {
        // TODO: Implementar envío real de paquetes
        // Por ahora, simular envío exitoso
        self.stats.packets_sent += 1;
        self.stats.bytes_sent += packet.len() as u32;
        Ok(())
    }
    
    /// Enviar respuesta ARP
    fn send_arp_reply(&mut self, target_ip: IpAddress, target_mac: MacAddress) -> Result<(), &'static str> {
        let mut packet = [0u8; 42];
        
        // Cabecera Ethernet
        packet[0..6].copy_from_slice(&target_mac.bytes);
        packet[6..12].copy_from_slice(&self.local_mac.bytes);
        packet[12..14].copy_from_slice(&ETHERTYPE_ARP.to_be_bytes());
        
        // Cabecera ARP
        packet[14..16].copy_from_slice(&1u16.to_be_bytes()); // Hardware type (Ethernet)
        packet[16..18].copy_from_slice(&ETHERTYPE_IP.to_be_bytes()); // Protocol type (IP)
        packet[18] = 6; // Hardware size
        packet[19] = 4; // Protocol size
        packet[20..22].copy_from_slice(&ARP_REPLY.to_be_bytes()); // Opcode (Reply)
        packet[22..28].copy_from_slice(&self.local_mac.bytes); // Sender MAC
        packet[28..32].copy_from_slice(&self.local_ipv4.bytes); // Sender IP
        packet[32..38].copy_from_slice(&target_mac.bytes); // Target MAC
        match target_ip {
            IpAddress::V4(ipv4) => packet[38..42].copy_from_slice(&ipv4.bytes), // Target IP
            IpAddress::V6(_) => return Err("IPv6 not supported in ARP"),
        }
        
        self.send_packet(&packet)?;
        Ok(())
    }
    
    /// Enviar respuesta ICMP
    fn send_icmp_reply(&mut self, _original_packet: &[u8]) -> Result<(), &'static str> {
        // TODO: Implementar respuesta ICMP completa
        // Por ahora, simular respuesta exitosa
        Ok(())
    }
    
    /// Enviar respuesta ICMPv6
    fn send_icmpv6_reply(&mut self, _original_packet: &[u8]) -> Result<(), &'static str> {
        // TODO: Implementar respuesta ICMPv6 completa
        // Por ahora, simular respuesta exitosa
        Ok(())
    }
    
    /// Agregar entrada ARP
    fn add_arp_entry(&mut self, ip: IpAddress, mac: MacAddress) {
        for i in 0..256 {
            if let Some(ref mut entry) = self.arp_table[i] {
                if entry.ip == ip {
                    entry.mac = mac;
                    entry.age = 0;
                    return;
                }
            } else {
                self.arp_table[i] = Some(ArpEntry { ip, mac, age: 0 });
                self.stats.arp_entries += 1;
                return;
            }
        }
    }
    
    /// Buscar conexión TCP
    fn find_tcp_connection(&mut self, src_port: u16, dest_port: u16) -> Option<usize> {
        for i in 0..64 {
            if let Some(connection) = self.tcp_connections[i] {
                if connection.local_port == dest_port && connection.remote_port == src_port {
                    return Some(i);
                }
            }
        }
        None
    }
    
    /// Buscar socket UDP
    fn find_udp_socket(&mut self, port: u16) -> Option<usize> {
        for i in 0..64 {
            if let Some(socket) = self.udp_sockets[i] {
                if socket.local_port == port {
                    return Some(i);
                }
            }
        }
        None
    }
    
    /// Manejar paquete TCP
    fn handle_tcp_packet(&mut self, connection_index: usize, header: &TcpHeader, data: &[u8]) -> Result<(), &'static str> {
        // TODO: Implementar manejo completo de TCP
        // Por ahora, simular manejo exitoso
        Ok(())
    }
    
    /// Manejar paquete UDP
    fn handle_udp_packet(&mut self, socket_index: usize, header: &UdpHeader, data: &[u8]) -> Result<(), &'static str> {
        // TODO: Implementar manejo completo de UDP
        // Por ahora, simular manejo exitoso
        Ok(())
    }
    
    /// Crear conexión TCP
    pub fn create_tcp_connection(&mut self, remote_ip: IpAddress, remote_port: u16, local_port: u16) -> Result<usize, &'static str> {
        for i in 0..64 {
            if self.tcp_connections[i].is_none() {
                let (local_ip, ip_version) = match remote_ip {
                    IpAddress::V4(_) => (IpAddress::V4(self.local_ipv4), 4),
                    IpAddress::V6(_) => (IpAddress::V6(self.local_ipv6), 6),
                };
                
                let connection = TcpConnection {
                    local_ip,
                    local_port,
                    remote_ip,
                    remote_port,
                    state: TcpState::Closed,
                    sequence: 0,
                    ack_sequence: 0,
                    window: 1024,
                    buffer: [0; 1024],
                    buffer_size: 0,
                    ip_version,
                };
                
                self.tcp_connections[i] = Some(connection);
                self.stats.tcp_connections += 1;
                return Ok(i);
            }
        }
        Err("No free TCP connections")
    }
    
    /// Crear socket UDP
    pub fn create_udp_socket(&mut self, local_port: u16, ip_version: u8) -> Result<usize, &'static str> {
        for i in 0..64 {
            if self.udp_sockets[i].is_none() {
                let local_ip = match ip_version {
                    4 => IpAddress::V4(self.local_ipv4),
                    6 => IpAddress::V6(self.local_ipv6),
                    _ => return Err("Invalid IP version"),
                };
                
                let socket = UdpSocket {
                    local_ip,
                    local_port,
                    buffer: [0; 1024],
                    buffer_size: 0,
                    ip_version,
                };
                
                self.udp_sockets[i] = Some(socket);
                self.stats.udp_sockets += 1;
                return Ok(i);
            }
        }
        Err("No free UDP sockets")
    }
    
    /// Obtener estadísticas de red
    pub fn get_stats(&self) -> NetworkStats {
        self.stats
    }
}

/// Instancia global del stack de red
static mut NETWORK_STACK: Option<NetworkStack> = None;

/// Inicializar stack de red
pub fn init_network() {
    unsafe {
        NETWORK_STACK = Some(NetworkStack::new());
        if let Some(ref mut stack) = NETWORK_STACK {
            stack.init();
        }
    }
}

/// Procesar paquete de red
pub fn process_network_packet(packet: &[u8]) -> Result<(), &'static str> {
    unsafe {
        if let Some(ref mut stack) = NETWORK_STACK {
            stack.process_packet(packet)
        } else {
            Err("Network stack not initialized")
        }
    }
}

/// Enviar paquete de red
pub fn send_network_packet(packet: &[u8]) -> Result<(), &'static str> {
    unsafe {
        if let Some(ref mut stack) = NETWORK_STACK {
            stack.send_packet(packet)
        } else {
            Err("Network stack not initialized")
        }
    }
}

/// Obtener estadísticas de red
pub fn get_network_stats() -> Option<NetworkStats> {
    unsafe {
        if let Some(ref stack) = NETWORK_STACK {
            Some(stack.get_stats())
        } else {
            None
        }
    }
}
