//! Sistema de Usuarios y Autenticación
//! 
//! Sistema completo de gestión de usuarios para ReactOS Windows en Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
// use std::path::Path; // No utilizado por ahora
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub full_name: String,
    pub email: String,
    pub groups: Vec<String>,
    pub is_admin: bool,
    pub is_active: bool,
    pub created_at: u64,
    pub last_login: Option<u64>,
    pub login_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub members: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub username: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub is_active: bool,
}

pub struct UserManager {
    users: HashMap<String, User>,
    groups: HashMap<String, Group>,
    sessions: HashMap<String, Session>,
    current_user: Option<String>,
    data_dir: String,
}

impl UserManager {
    pub fn new(data_dir: &str) -> Self {
        let mut manager = Self {
            users: HashMap::new(),
            groups: HashMap::new(),
            sessions: HashMap::new(),
            current_user: None,
            data_dir: data_dir.to_string(),
        };
        
        manager.load_data();
        manager.create_default_users();
        manager.create_default_groups();
        
        manager
    }

    fn load_data(&mut self) {
        // Cargar usuarios
        if let Ok(data) = fs::read_to_string(format!("{}/users.json", self.data_dir)) {
            if let Ok(users) = serde_json::from_str::<HashMap<String, User>>(&data) {
                self.users = users;
            }
        }

        // Cargar grupos
        if let Ok(data) = fs::read_to_string(format!("{}/groups.json", self.data_dir)) {
            if let Ok(groups) = serde_json::from_str::<HashMap<String, Group>>(&data) {
                self.groups = groups;
            }
        }

        // Cargar sesiones
        if let Ok(data) = fs::read_to_string(format!("{}/sessions.json", self.data_dir)) {
            if let Ok(sessions) = serde_json::from_str::<HashMap<String, Session>>(&data) {
                self.sessions = sessions;
            }
        }
    }

    fn save_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Crear directorio si no existe
        fs::create_dir_all(&self.data_dir)?;

        // Guardar usuarios
        let users_json = serde_json::to_string_pretty(&self.users)?;
        fs::write(format!("{}/users.json", self.data_dir), users_json)?;

        // Guardar grupos
        let groups_json = serde_json::to_string_pretty(&self.groups)?;
        fs::write(format!("{}/groups.json", self.data_dir), groups_json)?;

        // Guardar sesiones
        let sessions_json = serde_json::to_string_pretty(&self.sessions)?;
        fs::write(format!("{}/sessions.json", self.data_dir), sessions_json)?;

        Ok(())
    }

    fn create_default_users(&mut self) {
        if self.users.is_empty() {
            // Crear usuario administrador
            let admin_user = User {
                username: "admin".to_string(),
                password_hash: self.hash_password("admin123"),
                full_name: "Administrator".to_string(),
                email: "admin@reactos-rust.local".to_string(),
                groups: vec!["administrators".to_string()],
                is_admin: true,
                is_active: true,
                created_at: self.get_current_timestamp(),
                last_login: None,
                login_count: 0,
            };

            // Crear usuario invitado
            let guest_user = User {
                username: "guest".to_string(),
                password_hash: self.hash_password("guest"),
                full_name: "Guest User".to_string(),
                email: "guest@reactos-rust.local".to_string(),
                groups: vec!["users".to_string()],
                is_admin: false,
                is_active: true,
                created_at: self.get_current_timestamp(),
                last_login: None,
                login_count: 0,
            };

            self.users.insert("admin".to_string(), admin_user);
            self.users.insert("guest".to_string(), guest_user);
        }
    }

    fn create_default_groups(&mut self) {
        if self.groups.is_empty() {
            // Grupo de administradores
            let admin_group = Group {
                name: "administrators".to_string(),
                description: "Administradores del sistema".to_string(),
                permissions: vec![
                    "user_management".to_string(),
                    "system_config".to_string(),
                    "file_access".to_string(),
                    "network_access".to_string(),
                ],
                members: vec!["admin".to_string()],
            };

            // Grupo de usuarios
            let user_group = Group {
                name: "users".to_string(),
                description: "Usuarios regulares".to_string(),
                permissions: vec![
                    "file_access".to_string(),
                    "network_access".to_string(),
                ],
                members: vec!["guest".to_string()],
            };

            self.groups.insert("administrators".to_string(), admin_group);
            self.groups.insert("users".to_string(), user_group);
        }
    }

    fn hash_password(&self, password: &str) -> String {
        // Hash simple para demostración (en producción usar bcrypt o similar)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        password.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn get_current_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<String, String> {
        // Verificar si el usuario existe y está activo
        let user_exists = if let Some(user) = self.users.get(username) {
            user.is_active
        } else {
            return Err("Usuario no encontrado".to_string());
        };

        if !user_exists {
            return Err("Usuario desactivado".to_string());
        }

        // Obtener datos necesarios antes de modificar
        let password_hash = self.hash_password(password);
        let current_timestamp = self.get_current_timestamp();

        // Verificar contraseña y actualizar usuario
        if let Some(user) = self.users.get_mut(username) {
            if user.password_hash == password_hash {
                // Actualizar información de login
                user.last_login = Some(current_timestamp);
                user.login_count += 1;

                // Crear sesión
                let session_id = format!("session_{}_{}", username, current_timestamp);
                let session = Session {
                    session_id: session_id.clone(),
                    username: username.to_string(),
                    created_at: current_timestamp,
                    expires_at: current_timestamp + 3600, // 1 hora
                    is_active: true,
                };

                self.sessions.insert(session_id.clone(), session);
                self.current_user = Some(username.to_string());

                // Guardar datos
                if let Err(e) = self.save_data() {
                    eprintln!("Error al guardar datos: {}", e);
                }

                Ok(session_id)
            } else {
                Err("Contraseña incorrecta".to_string())
            }
        } else {
            Err("Usuario no encontrado".to_string())
        }
    }

    pub fn logout(&mut self, session_id: &str) -> Result<(), String> {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.is_active = false;
            self.current_user = None;
            Ok(())
        } else {
            Err("Sesión no encontrada".to_string())
        }
    }

    pub fn get_current_user(&self) -> Option<&User> {
        if let Some(username) = &self.current_user {
            self.users.get(username)
        } else {
            None
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.current_user.is_some()
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        if let Some(user) = self.get_current_user() {
            for group_name in &user.groups {
                if let Some(group) = self.groups.get(group_name) {
                    if group.permissions.contains(&permission.to_string()) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn create_user(&mut self, username: &str, password: &str, full_name: &str, email: &str) -> Result<(), String> {
        if !self.has_permission("user_management") {
            return Err("No tiene permisos para crear usuarios".to_string());
        }

        if self.users.contains_key(username) {
            return Err("El usuario ya existe".to_string());
        }

        let user = User {
            username: username.to_string(),
            password_hash: self.hash_password(password),
            full_name: full_name.to_string(),
            email: email.to_string(),
            groups: vec!["users".to_string()],
            is_admin: false,
            is_active: true,
            created_at: self.get_current_timestamp(),
            last_login: None,
            login_count: 0,
        };

        self.users.insert(username.to_string(), user);
        
        if let Err(e) = self.save_data() {
            eprintln!("Error al guardar datos: {}", e);
        }

        Ok(())
    }

    pub fn list_users(&self) -> Vec<&User> {
        if !self.has_permission("user_management") {
            return Vec::new();
        }

        self.users.values().collect()
    }

    pub fn get_user_info(&self, username: &str) -> Option<&User> {
        if !self.has_permission("user_management") && self.current_user.as_ref() != Some(&username.to_string()) {
            return None;
        }

        self.users.get(username)
    }

    pub fn change_password(&mut self, username: &str, old_password: &str, new_password: &str) -> Result<(), String> {
        let can_change = self.has_permission("user_management") || 
                        self.current_user.as_ref() == Some(&username.to_string());

        if !can_change {
            return Err("No tiene permisos para cambiar la contraseña".to_string());
        }

        // Obtener hashes antes de modificar
        let old_hash = self.hash_password(old_password);
        let new_hash = self.hash_password(new_password);

        if let Some(user) = self.users.get_mut(username) {
            if user.password_hash == old_hash {
                user.password_hash = new_hash;
                
                if let Err(e) = self.save_data() {
                    eprintln!("Error al guardar datos: {}", e);
                }
                
                Ok(())
            } else {
                Err("Contraseña actual incorrecta".to_string())
            }
        } else {
            Err("Usuario no encontrado".to_string())
        }
    }

    pub fn list_groups(&self) -> Vec<&Group> {
        if !self.has_permission("user_management") {
            return Vec::new();
        }

        self.groups.values().collect()
    }

    pub fn get_system_info(&self) -> String {
        let total_users = self.users.len();
        let active_users = self.users.values().filter(|u| u.is_active).count();
        let total_sessions = self.sessions.values().filter(|s| s.is_active).count();
        let current_user = self.current_user.as_deref().unwrap_or("Ninguno");

        format!(
            "Información del Sistema de Usuarios:\n\
             Usuarios totales: {}\n\
             Usuarios activos: {}\n\
             Sesiones activas: {}\n\
             Usuario actual: {}\n\
             Autenticado: {}",
            total_users,
            active_users,
            total_sessions,
            current_user,
            if self.is_authenticated() { "Sí" } else { "No" }
        )
    }
}
