//! Módulo de contexto para WOW64
//! 
//! Implementa el manejo del contexto de ejecución 32-bit

use core::arch::asm;
use kernel::prelude::*;

/// Contexto de ejecución 32-bit
#[repr(C)]
pub struct Context32 {
    /// Registros generales
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub esi: u32,
    pub edi: u32,
    pub ebp: u32,
    pub esp: u32,
    /// Registros de segmento
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
    /// Flags
    pub eflags: u32,
    /// Instrucción actual
    pub eip: u32,
}

/// Selectores de segmento para modo 32-bit
pub const CS_32BIT: u16 = 0x23; // Selector de código 32-bit
pub const DS_32BIT: u16 = 0x2B; // Selector de datos 32-bit
pub const SS_32BIT: u16 = 0x2B; // Selector de stack 32-bit

/// Flags del procesador
pub const EFLAGS_IF: u32 = 0x200; // Interrupt flag
pub const EFLAGS_DF: u32 = 0x400; // Direction flag
pub const EFLAGS_CF: u32 = 0x001; // Carry flag
pub const EFLAGS_PF: u32 = 0x004; // Parity flag
pub const EFLAGS_AF: u32 = 0x010; // Auxiliary carry flag
pub const EFLAGS_ZF: u32 = 0x040; // Zero flag
pub const EFLAGS_SF: u32 = 0x080; // Sign flag
pub const EFLAGS_TF: u32 = 0x100; // Trap flag
pub const EFLAGS_IOPL: u32 = 0x3000; // I/O privilege level

impl Context32 {
    /// Crear nuevo contexto 32-bit
    pub fn new() -> Result<Self> {
        Ok(Self {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
            esi: 0,
            edi: 0,
            ebp: 0,
            esp: 0,
            cs: CS_32BIT,
            ds: DS_32BIT,
            es: DS_32BIT,
            fs: DS_32BIT,
            gs: DS_32BIT,
            ss: SS_32BIT,
            eflags: EFLAGS_IF,
            eip: 0,
        })
    }
    
    /// Configurar contexto inicial
    pub fn setup_initial(&mut self, entry_point: u32, stack_base: u32) -> Result<()> {
        self.eip = entry_point;
        self.esp = stack_base;
        self.ebp = stack_base;
        self.eflags = EFLAGS_IF;
        Ok(())
    }
    
    /// Guardar contexto actual
    pub fn save(&mut self) -> Result<()> {
        unsafe {
            asm!(
                "mov {eax}, eax",
                "mov {ebx}, ebx",
                "mov {ecx}, ecx",
                "mov {edx}, edx",
                "mov {esi}, esi",
                "mov {edi}, edi",
                "mov {ebp}, ebp",
                "mov {esp}, esp",
                "mov {cs}, cs",
                "mov {ds}, ds",
                "mov {es}, es",
                "mov {fs}, fs",
                "mov {gs}, gs",
                "mov {ss}, ss",
                "pushf",
                "pop {eflags}",
                aax = out(reg) self.eax,
                ebx = out(reg) self.ebx,
                ecx = out(reg) self.ecx,
                edx = out(reg) self.edx,
                esi = out(reg) self.esi,
                edi = out(reg) self.edi,
                ebp = out(reg) self.ebp,
                esp = out(reg) self.esp,
                cs = out(reg) self.cs,
                ds = out(reg) self.ds,
                es = out(reg) self.es,
                fs = out(reg) self.fs,
                gs = out(reg) self.gs,
                ss = out(reg) self.ss,
                eflags = out(reg) self.eflags,
            );
        }
        Ok(())
    }
    
    /// Restaurar contexto
    pub fn restore(&self) -> Result<()> {
        unsafe {
            asm!(
                "mov eax, {eax}",
                "mov ebx, {ebx}",
                "mov ecx, {ecx}",
                "mov edx, {edx}",
                "mov esi, {esi}",
                "mov edi, {edi}",
                "mov ebp, {ebp}",
                "mov esp, {esp}",
                "mov cs, {cs}",
                "mov ds, {ds}",
                "mov es, {es}",
                "mov fs, {fs}",
                "mov gs, {gs}",
                "mov ss, {ss}",
                "push {eflags}",
                "popf",
                aax = in(reg) self.eax,
                ebx = in(reg) self.ebx,
                ecx = in(reg) self.ecx,
                edx = in(reg) self.edx,
                esi = in(reg) self.esi,
                edi = in(reg) self.edi,
                ebp = in(reg) self.ebp,
                esp = in(reg) self.esp,
                cs = in(reg) self.cs,
                ds = in(reg) self.ds,
                es = in(reg) self.es,
                fs = in(reg) self.fs,
                gs = in(reg) self.gs,
                ss = in(reg) self.ss,
                eflags = in(reg) self.eflags,
            );
        }
        Ok(())
    }
    
    /// Cambiar a modo 32-bit
    pub fn switch_to_32bit(&self) -> Result<()> {
        unsafe {
            asm!(
                "push {cs}",
                "push {eip}",
                "retf",
                cs = in(reg) self.cs,
                eip = in(reg) self.eip,
            );
        }
        Ok(())
    }
    
    /// Cambiar a modo 64-bit
    pub fn switch_to_64bit(&self) -> Result<()> {
        unsafe {
            asm!(
                "push {cs}",
                "push {eip}",
                "retf",
                cs = in(reg) 0x08, // Selector de código 64-bit
                eip = in(reg) 0x100000, // Dirección de retorno
            );
        }
        Ok(())
    }
    
    /// Ejecutar instrucción 32-bit
    pub fn execute_32bit_instruction(&mut self, instruction: &[u8]) -> Result<()> {
        // Guardar contexto actual
        self.save()?;
        
        // Cambiar a modo 32-bit
        self.switch_to_32bit()?;
        
        // Ejecutar instrucción
        unsafe {
            let func: extern "C" fn() -> u32 = core::mem::transmute(instruction.as_ptr());
            let result = func();
            self.eax = result;
        }
        
        // Restaurar contexto
        self.restore()?;
        
        Ok(())
    }
    
    /// Manejar excepción 32-bit
    pub fn handle_32bit_exception(&mut self, exception_code: u32) -> Result<()> {
        match exception_code {
            0x0D => self.handle_general_protection_fault()?,
            0x0E => self.handle_page_fault()?,
            0x06 => self.handle_invalid_opcode()?,
            _ => self.handle_unknown_exception(exception_code)?,
        }
        Ok(())
    }
    
    /// Manejar fallo de protección general
    fn handle_general_protection_fault(&mut self) -> Result<()> {
        log::error!("General Protection Fault en modo 32-bit");
        log::error!("EIP: 0x{:08X}, ESP: 0x{:08X}", self.eip, self.esp);
        Ok(())
    }
    
    /// Manejar fallo de página
    fn handle_page_fault(&mut self) -> Result<()> {
        log::error!("Page Fault en modo 32-bit");
        log::error!("EIP: 0x{:08X}, ESP: 0x{:08X}", self.eip, self.esp);
        Ok(())
    }
    
    /// Manejar código de operación inválido
    fn handle_invalid_opcode(&mut self) -> Result<()> {
        log::error!("Invalid Opcode en modo 32-bit");
        log::error!("EIP: 0x{:08X}, ESP: 0x{:08X}", self.eip, self.esp);
        Ok(())
    }
    
    /// Manejar excepción desconocida
    fn handle_unknown_exception(&mut self, exception_code: u32) -> Result<()> {
        log::error!("Excepción desconocida {} en modo 32-bit", exception_code);
        log::error!("EIP: 0x{:08X}, ESP: 0x{:08X}", self.eip, self.esp);
        Ok(())
    }
}
