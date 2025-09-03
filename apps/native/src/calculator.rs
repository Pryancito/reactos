//! Calculadora para ReactOS Rust
//! 
//! Aplicación nativa para cálculos matemáticos
//! con funciones científicas y programador.

use crate::common::*;
use std::collections::HashMap;

/// Modo de la calculadora
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalculatorMode {
    Standard,
    Scientific,
    Programmer,
    Date,
}

/// Base numérica
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumberBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

/// Estado de la calculadora
#[derive(Debug, Clone)]
pub struct CalculatorState {
    pub display: String,
    pub previous_value: f64,
    pub current_operation: Option<Operation>,
    pub memory: f64,
    pub mode: CalculatorMode,
    pub base: NumberBase,
    pub angle_unit: AngleUnit,
    pub history: Vec<CalculationHistory>,
    pub is_error: bool,
    pub config: CalculatorConfig,
}

/// Operación matemática
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
    And,
    Or,
    Xor,
    Not,
    LeftShift,
    RightShift,
}

/// Unidad de ángulo
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngleUnit {
    Degrees,
    Radians,
    Gradians,
}

/// Historial de cálculos
#[derive(Debug, Clone)]
pub struct CalculationHistory {
    pub expression: String,
    pub result: String,
    pub timestamp: std::time::SystemTime,
}

/// Configuración de la calculadora
#[derive(Debug, Clone)]
pub struct CalculatorConfig {
    pub default_mode: CalculatorMode,
    pub default_base: NumberBase,
    pub default_angle_unit: AngleUnit,
    pub precision: usize,
    pub show_history: bool,
    pub auto_save_history: bool,
    pub sound_enabled: bool,
}

impl Default for CalculatorConfig {
    fn default() -> Self {
        Self {
            default_mode: CalculatorMode::Standard,
            default_base: NumberBase::Decimal,
            default_angle_unit: AngleUnit::Degrees,
            precision: 10,
            show_history: true,
            auto_save_history: true,
            sound_enabled: true,
        }
    }
}

/// Calculadora
pub struct Calculator {
    pub state: CalculatorState,
    pub is_running: bool,
    pub window_handle: Option<u32>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            state: CalculatorState {
                display: "0".to_string(),
                previous_value: 0.0,
                current_operation: None,
                memory: 0.0,
                mode: CalculatorMode::Standard,
                base: NumberBase::Decimal,
                angle_unit: AngleUnit::Degrees,
                history: Vec::new(),
                is_error: false,
                config: CalculatorConfig::default(),
            },
            is_running: false,
            window_handle: None,
        }
    }
    
    /// Inicializar calculadora
    pub fn init(&mut self) -> Result<(), String> {
        if self.is_running {
            return Ok(());
        }
        
        self.is_running = true;
        Ok(())
    }
    
    /// Limpiar pantalla
    pub fn clear(&mut self) {
        self.state.display = "0".to_string();
        self.state.previous_value = 0.0;
        self.state.current_operation = None;
        self.state.is_error = false;
    }
    
    /// Limpiar entrada
    pub fn clear_entry(&mut self) {
        self.state.display = "0".to_string();
    }
    
    /// Eliminar último carácter
    pub fn backspace(&mut self) {
        if self.state.display.len() > 1 {
            self.state.display.pop();
        } else {
            self.state.display = "0".to_string();
        }
    }
    
    /// Insertar dígito
    pub fn input_digit(&mut self, digit: u8) {
        if self.state.is_error {
            self.clear();
        }
        
        if self.state.display == "0" {
            self.state.display = digit.to_string();
        } else {
            self.state.display.push_str(&digit.to_string());
        }
    }
    
    /// Insertar punto decimal
    pub fn input_decimal(&mut self) {
        if self.state.is_error {
            self.clear();
        }
        
        if !self.state.display.contains('.') {
            self.state.display.push('.');
        }
    }
    
    /// Cambiar signo
    pub fn toggle_sign(&mut self) {
        if self.state.display != "0" {
            if self.state.display.starts_with('-') {
                self.state.display = self.state.display[1..].to_string();
            } else {
                self.state.display = format!("-{}", self.state.display);
            }
        }
    }
    
    /// Establecer operación
    pub fn set_operation(&mut self, operation: Operation) {
        if let Ok(value) = self.get_display_value() {
            self.state.previous_value = value;
            self.state.current_operation = Some(operation);
            self.state.display = "0".to_string();
        }
    }
    
    /// Calcular resultado
    pub fn calculate(&mut self) -> Result<(), String> {
        if let Some(operation) = self.state.current_operation {
            let current_value = self.get_display_value()?;
            let result = self.perform_operation(self.state.previous_value, current_value, operation)?;
            
            // Agregar al historial
            let expression = format!("{} {} {} = {}", 
                self.state.previous_value, 
                self.operation_to_string(operation),
                current_value,
                result
            );
            
            self.state.history.push(CalculationHistory {
                expression,
                result: result.to_string(),
                timestamp: std::time::SystemTime::now(),
            });
            
            // Limitar historial a 100 entradas
            if self.state.history.len() > 100 {
                self.state.history.remove(0);
            }
            
            self.state.display = self.format_number(result);
            self.state.current_operation = None;
            self.state.previous_value = 0.0;
        }
        Ok(())
    }
    
    /// Realizar operación matemática
    fn perform_operation(&self, left: f64, right: f64, operation: Operation) -> Result<f64, String> {
        match operation {
            Operation::Add => Ok(left + right),
            Operation::Subtract => Ok(left - right),
            Operation::Multiply => Ok(left * right),
            Operation::Divide => {
                if right == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left / right)
                }
            }
            Operation::Power => Ok(left.powf(right)),
            Operation::Modulo => {
                if right == 0.0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(left % right)
                }
            }
            Operation::And => Ok((left as i64 & right as i64) as f64),
            Operation::Or => Ok((left as i64 | right as i64) as f64),
            Operation::Xor => Ok((left as i64 ^ right as i64) as f64),
            Operation::Not => Ok(!(left as i64) as f64),
            Operation::LeftShift => Ok((left as i64 << right as i64) as f64),
            Operation::RightShift => Ok((left as i64 >> right as i64) as f64),
        }
    }
    
    /// Obtener valor de la pantalla
    fn get_display_value(&self) -> Result<f64, String> {
        match self.state.base {
            NumberBase::Decimal => {
                self.state.display.parse::<f64>()
                    .map_err(|_| "Invalid number".to_string())
            }
            NumberBase::Binary => {
                i64::from_str_radix(&self.state.display, 2)
                    .map(|v| v as f64)
                    .map_err(|_| "Invalid binary number".to_string())
            }
            NumberBase::Octal => {
                i64::from_str_radix(&self.state.display, 8)
                    .map(|v| v as f64)
                    .map_err(|_| "Invalid octal number".to_string())
            }
            NumberBase::Hexadecimal => {
                i64::from_str_radix(&self.state.display, 16)
                    .map(|v| v as f64)
                    .map_err(|_| "Invalid hexadecimal number".to_string())
            }
        }
    }
    
    /// Formatear número para mostrar
    fn format_number(&self, number: f64) -> String {
        match self.state.base {
            NumberBase::Decimal => {
                if number.fract() == 0.0 {
                    format!("{:.0}", number)
                } else {
                    format!("{:.10}", number).trim_end_matches('0').trim_end_matches('.').to_string()
                }
            }
            NumberBase::Binary => {
                format!("{:b}", number as i64)
            }
            NumberBase::Octal => {
                format!("{:o}", number as i64)
            }
            NumberBase::Hexadecimal => {
                format!("{:X}", number as i64)
            }
        }
    }
    
    /// Convertir operación a string
    fn operation_to_string(&self, operation: Operation) -> &'static str {
        match operation {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "×",
            Operation::Divide => "÷",
            Operation::Power => "^",
            Operation::Modulo => "%",
            Operation::And => "AND",
            Operation::Or => "OR",
            Operation::Xor => "XOR",
            Operation::Not => "NOT",
            Operation::LeftShift => "<<",
            Operation::RightShift => ">>",
        }
    }
    
    /// Funciones científicas
    pub fn sin(&mut self) -> Result<(), String> {
        let value = self.get_display_value()?;
        let angle = self.convert_angle(value);
        let result = MathUtils::sin(angle);
        self.state.display = self.format_number(result);
        Ok(())
    }
    
    pub fn cos(&mut self) -> Result<(), String> {
        let value = self.get_display_value()?;
        let angle = self.convert_angle(value);
        let result = MathUtils::cos(angle);
        self.state.display = self.format_number(result);
        Ok(())
    }
    
    pub fn tan(&mut self) -> Result<(), String> {
        let value = self.get_display_value()?;
        let angle = self.convert_angle(value);
        let result = MathUtils::tan(angle);
        self.state.display = self.format_number(result);
        Ok(())
    }
    
    pub fn ln(&mut self) -> Result<(), String> {
        let value = self.get_display_value()?;
        let result = MathUtils::ln(value)?;
        self.state.display = self.format_number(result);
        Ok(())
    }
    
    pub fn log10(&mut self) -> Result<(), String> {
        let value = self.get_display_value()?;
        let result = MathUtils::log10(value)?;
        self.state.display = self.format_number(result);
        Ok(())
    }
    
    pub fn sqrt(&mut self) -> Result<(), String> {
        let value = self.get_display_value()?;
        let result = MathUtils::sqrt(value)?;
        self.state.display = self.format_number(result);
        Ok(())
    }
    
    pub fn square(&mut self) -> Result<(), String> {
        let value = self.get_display_value()?;
        let result = value * value;
        self.state.display = self.format_number(result);
        Ok(())
    }
    
    pub fn factorial(&mut self) -> Result<(), String> {
        let value = self.get_display_value()?;
        if value < 0.0 || value.fract() != 0.0 {
            return Err("Factorial only defined for non-negative integers".to_string());
        }
        
        let n = value as u64;
        let result = (1..=n).product::<u64>() as f64;
        self.state.display = self.format_number(result);
        Ok(())
    }
    
    /// Convertir ángulo según la unidad configurada
    fn convert_angle(&self, angle: f64) -> f64 {
        match self.state.angle_unit {
            AngleUnit::Degrees => angle.to_radians(),
            AngleUnit::Radians => angle,
            AngleUnit::Gradians => angle * std::f64::consts::PI / 200.0,
        }
    }
    
    /// Funciones de memoria
    pub fn memory_clear(&mut self) {
        self.state.memory = 0.0;
    }
    
    pub fn memory_recall(&mut self) {
        self.state.display = self.format_number(self.state.memory);
    }
    
    pub fn memory_add(&mut self) -> Result<(), String> {
        let value = self.get_display_value()?;
        self.state.memory += value;
        Ok(())
    }
    
    pub fn memory_subtract(&mut self) -> Result<(), String> {
        let value = self.get_display_value()?;
        self.state.memory -= value;
        Ok(())
    }
    
    pub fn memory_store(&mut self) -> Result<(), String> {
        let value = self.get_display_value()?;
        self.state.memory = value;
        Ok(())
    }
    
    /// Cambiar modo de la calculadora
    pub fn set_mode(&mut self, mode: CalculatorMode) {
        self.state.mode = mode;
        self.clear();
    }
    
    /// Cambiar base numérica
    pub fn set_base(&mut self, base: NumberBase) {
        if self.state.mode == CalculatorMode::Programmer {
            self.state.base = base;
            self.clear();
        }
    }
    
    /// Cambiar unidad de ángulo
    pub fn set_angle_unit(&mut self, unit: AngleUnit) {
        self.state.angle_unit = unit;
    }
    
    /// Obtener historial de cálculos
    pub fn get_history(&self) -> &Vec<CalculationHistory> {
        &self.state.history
    }
    
    /// Limpiar historial
    pub fn clear_history(&mut self) {
        self.state.history.clear();
    }
    
    /// Procesar eventos
    pub fn process_events(&mut self, events: Vec<AppEvent>) {
        for event in events {
            match event {
                AppEvent::KeyPress { key, .. } => {
                    match key.as_str() {
                        "0" => self.input_digit(0),
                        "1" => self.input_digit(1),
                        "2" => self.input_digit(2),
                        "3" => self.input_digit(3),
                        "4" => self.input_digit(4),
                        "5" => self.input_digit(5),
                        "6" => self.input_digit(6),
                        "7" => self.input_digit(7),
                        "8" => self.input_digit(8),
                        "9" => self.input_digit(9),
                        "." => self.input_decimal(),
                        "+" => self.set_operation(Operation::Add),
                        "-" => self.set_operation(Operation::Subtract),
                        "*" => self.set_operation(Operation::Multiply),
                        "/" => self.set_operation(Operation::Divide),
                        "=" | "Enter" => {
                            if let Err(e) = self.calculate() {
                                self.state.display = format!("Error: {}", e);
                                self.state.is_error = true;
                            }
                        }
                        "Escape" | "C" => self.clear(),
                        "Backspace" => self.backspace(),
                        "F2" => self.set_mode(CalculatorMode::Standard),
                        "F3" => self.set_mode(CalculatorMode::Scientific),
                        "F4" => self.set_mode(CalculatorMode::Programmer),
                        "F5" => self.set_mode(CalculatorMode::Date),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    
    /// Renderizar interfaz
    pub fn render(&self) {
        // En una implementación real, se usaría una librería de UI como egui
        println!("=== Calculadora ===");
        println!("Modo: {:?}", self.state.mode);
        println!("Base: {:?}", self.state.base);
        println!("Ángulo: {:?}", self.state.angle_unit);
        println!();
        println!("Pantalla: {}", self.state.display);
        println!();
        
        if self.state.mode == CalculatorMode::Scientific {
            println!("Funciones científicas disponibles:");
            println!("sin, cos, tan, ln, log, sqrt, x², x!");
        }
        
        if self.state.mode == CalculatorMode::Programmer {
            println!("Bases numéricas: Binario, Octal, Decimal, Hexadecimal");
            println!("Operaciones: AND, OR, XOR, NOT, <<, >>");
        }
        
        if !self.state.history.is_empty() {
            println!();
            println!("Historial:");
            for entry in self.state.history.iter().rev().take(5) {
                println!("  {}", entry.expression);
            }
        }
        
        println!();
        println!("Memoria: {}", self.state.memory);
    }
    
    /// Shutdown de la calculadora
    pub fn shutdown(&mut self) {
        self.is_running = false;
        self.window_handle = None;
    }
}

/// Función principal de la calculadora
pub fn main() {
    let mut calculator = Calculator::new();
    
    if let Err(e) = calculator.init() {
        eprintln!("Error inicializando calculadora: {}", e);
        return;
    }
    
    println!("Calculadora iniciada");
    
    // Simular algunos cálculos
    let events = vec![
        AppEvent::KeyPress { key: "2".to_string(), modifiers: vec![] },
        AppEvent::KeyPress { key: "+".to_string(), modifiers: vec![] },
        AppEvent::KeyPress { key: "3".to_string(), modifiers: vec![] },
        AppEvent::KeyPress { key: "=".to_string(), modifiers: vec![] },
    ];
    
    calculator.process_events(events);
    calculator.render();
    
    // Cambiar a modo científico
    calculator.set_mode(CalculatorMode::Scientific);
    calculator.input_digit(9);
    calculator.input_digit(0);
    calculator.sin();
    calculator.render();
    
    calculator.shutdown();
    println!("Calculadora cerrada");
}
