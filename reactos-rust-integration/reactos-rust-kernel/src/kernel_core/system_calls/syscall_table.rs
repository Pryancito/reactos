//! System Call Table
//! 
//! Tabla que mapea IDs de system calls a sus handlers correspondientes

use crate::kernel_core::system_calls::syscall_handler::{SyscallInfo, SyscallFlags, SyscallCategory};

/// Tabla de System Calls
pub struct SyscallTable {
    pub syscalls: [Option<SyscallInfo>; 512],
    pub syscall_count: u32,
    pub table_state: SyscallTableState,
}

/// Estado de la tabla
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyscallTableState {
    Initialized,
    Active,
    Paused,
    Error,
}

impl SyscallTable {
    /// Crear nueva tabla de system calls
    pub fn new() -> Self {
        Self {
            syscalls: [None; 512],
            syscall_count: 0,
            table_state: SyscallTableState::Initialized,
        }
    }

    /// Registrar un system call
    pub fn register_syscall(&mut self, syscall_info: SyscallInfo) -> bool {
        if syscall_info.id >= 512 {
            return false; // ID fuera de rango
        }

        if self.syscalls[syscall_info.id as usize].is_some() {
            return false; // System call ya registrado
        }

        self.syscalls[syscall_info.id as usize] = Some(syscall_info);
        self.syscall_count += 1;
        true
    }

    /// Desregistrar un system call
    pub fn unregister_syscall(&mut self, syscall_id: u32) -> bool {
        if syscall_id >= 512 {
            return false;
        }

        if self.syscalls[syscall_id as usize].is_some() {
            self.syscalls[syscall_id as usize] = None;
            self.syscall_count -= 1;
            true
        } else {
            false
        }
    }

    /// Obtener información de un system call
    pub fn get_syscall_info(&self, syscall_id: u32) -> Option<SyscallInfo> {
        if syscall_id >= 512 {
            return None;
        }

        self.syscalls[syscall_id as usize]
    }

    /// Buscar system calls por categoría
    pub fn find_syscalls_by_category(&self, category: SyscallCategory) -> u32 {
        let mut count = 0;
        for i in 0..512 {
            if let Some(syscall) = &self.syscalls[i] {
                if syscall.category == category {
                    count += 1;
                }
            }
        }
        count
    }

    /// Buscar system calls por nombre
    pub fn find_syscalls_by_name(&self, name: &str) -> u32 {
        let mut count = 0;
        for i in 0..512 {
            if let Some(syscall) = &self.syscalls[i] {
                if syscall.name == name {
                    count += 1;
                }
            }
        }
        count
    }

    /// Obtener estadísticas de la tabla
    pub fn get_stats(&self) -> SyscallTableStats {
        SyscallTableStats {
            syscall_count: self.syscall_count,
            table_state: self.table_state,
            registered_syscalls: self.syscall_count,
            available_slots: 512 - self.syscall_count,
        }
    }

    /// Cambiar estado de la tabla
    pub fn set_state(&mut self, new_state: SyscallTableState) {
        self.table_state = new_state;
    }

    /// Limpiar tabla
    pub fn clear(&mut self) {
        for i in 0..512 {
            self.syscalls[i] = None;
        }
        self.syscall_count = 0;
    }

    /// Verificar si la tabla está llena
    pub fn is_full(&self) -> bool {
        self.syscall_count >= 512
    }

    /// Obtener siguiente ID disponible
    pub fn get_next_available_id(&self) -> Option<u32> {
        for i in 0..512 {
            if self.syscalls[i].is_none() {
                return Some(i as u32);
            }
        }
        None
    }
}

/// Estadísticas de la tabla de system calls
#[derive(Debug, Clone, Copy)]
pub struct SyscallTableStats {
    pub syscall_count: u32,
    pub table_state: SyscallTableState,
    pub registered_syscalls: u32,
    pub available_slots: u32,
}

/// Instancia global de la tabla de system calls
static mut SYSCALL_TABLE: Option<SyscallTable> = None;

/// Inicializar la tabla de system calls
pub fn init() {
    unsafe {
        SYSCALL_TABLE = Some(SyscallTable::new());
        
        // Registrar system calls básicos
        let mut table = SYSCALL_TABLE.as_mut().unwrap();
        
        // System call 0: exit
        table.register_syscall(SyscallInfo {
            id: 0,
            name: "exit",
            handler: syscall_exit,
            flags: SyscallFlags {
                requires_privilege: false,
                can_block: false,
                can_fail: false,
                is_deprecated: false,
            },
            category: SyscallCategory::Process,
        });

        // System call 1: read
        table.register_syscall(SyscallInfo {
            id: 1,
            name: "read",
            handler: syscall_read,
            flags: SyscallFlags {
                requires_privilege: false,
                can_block: true,
                can_fail: true,
                is_deprecated: false,
            },
            category: SyscallCategory::FileSystem,
        });

        // System call 2: write
        table.register_syscall(SyscallInfo {
            id: 2,
            name: "write",
            handler: syscall_write,
            flags: SyscallFlags {
                requires_privilege: false,
                can_block: true,
                can_fail: true,
                is_deprecated: false,
            },
            category: SyscallCategory::FileSystem,
        });

        // System call 3: open
        table.register_syscall(SyscallInfo {
            id: 3,
            name: "open",
            handler: syscall_open,
            flags: SyscallFlags {
                requires_privilege: false,
                can_block: true,
                can_fail: true,
                is_deprecated: false,
            },
            category: SyscallCategory::FileSystem,
        });

        // System call 4: close
        table.register_syscall(SyscallInfo {
            id: 4,
            name: "close",
            handler: syscall_close,
            flags: SyscallFlags {
                requires_privilege: false,
                can_block: false,
                can_fail: true,
                is_deprecated: false,
            },
            category: SyscallCategory::FileSystem,
        });

        // System call 5: fork
        table.register_syscall(SyscallInfo {
            id: 5,
            name: "fork",
            handler: syscall_fork,
            flags: SyscallFlags {
                requires_privilege: false,
                can_block: true,
                can_fail: true,
                is_deprecated: false,
            },
            category: SyscallCategory::Process,
        });

        // System call 6: exec
        table.register_syscall(SyscallInfo {
            id: 6,
            name: "exec",
            handler: syscall_exec,
            flags: SyscallFlags {
                requires_privilege: false,
                can_block: true,
                can_fail: true,
                is_deprecated: false,
            },
            category: SyscallCategory::Process,
        });

        // System call 7: wait
        table.register_syscall(SyscallInfo {
            id: 7,
            name: "wait",
            handler: syscall_wait,
            flags: SyscallFlags {
                requires_privilege: false,
                can_block: true,
                can_fail: true,
                is_deprecated: false,
            },
            category: SyscallCategory::Process,
        });

        // System call 8: brk
        table.register_syscall(SyscallInfo {
            id: 8,
            name: "brk",
            handler: syscall_brk,
            flags: SyscallFlags {
                requires_privilege: false,
                can_block: false,
                can_fail: true,
                is_deprecated: false,
            },
            category: SyscallCategory::Memory,
        });

        // System call 9: mmap
        table.register_syscall(SyscallInfo {
            id: 9,
            name: "mmap",
            handler: syscall_mmap,
            flags: SyscallFlags {
                requires_privilege: false,
                can_block: false,
                can_fail: true,
                is_deprecated: false,
            },
            category: SyscallCategory::Memory,
        });

        // System call 10: munmap
        table.register_syscall(SyscallInfo {
            id: 10,
            name: "munmap",
            handler: syscall_munmap,
            flags: SyscallFlags {
                requires_privilege: false,
                can_block: false,
                can_fail: true,
                is_deprecated: false,
            },
            category: SyscallCategory::Memory,
        });

        table.set_state(SyscallTableState::Active);
    }
}

/// Obtener instancia de la tabla de system calls
pub fn get_table() -> &'static mut SyscallTable {
    unsafe {
        SYSCALL_TABLE.as_mut().unwrap()
    }
}

/// Obtener información de un system call
pub fn get_syscall_info(syscall_id: u32) -> Option<SyscallInfo> {
    get_table().get_syscall_info(syscall_id)
}

// Handlers de system calls básicos

fn syscall_exit(_arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64, _arg6: u64) -> u64 {
    // Implementación simplificada
    0
}

fn syscall_read(_arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64, _arg6: u64) -> u64 {
    // Implementación simplificada
    0
}

fn syscall_write(_arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64, _arg6: u64) -> u64 {
    // Implementación simplificada
    0
}

fn syscall_open(_arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64, _arg6: u64) -> u64 {
    // Implementación simplificada
    0
}

fn syscall_close(_arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64, _arg6: u64) -> u64 {
    // Implementación simplificada
    0
}

fn syscall_fork(_arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64, _arg6: u64) -> u64 {
    // Implementación simplificada
    0
}

fn syscall_exec(_arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64, _arg6: u64) -> u64 {
    // Implementación simplificada
    0
}

fn syscall_wait(_arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64, _arg6: u64) -> u64 {
    // Implementación simplificada
    0
}

fn syscall_brk(_arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64, _arg6: u64) -> u64 {
    // Implementación simplificada
    0
}

fn syscall_mmap(_arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64, _arg6: u64) -> u64 {
    // Implementación simplificada
    0
}

fn syscall_munmap(_arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64, _arg6: u64) -> u64 {
    // Implementación simplificada
    0
}
