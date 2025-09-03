//! Calculadora Real y Funcional
//! 
//! Aplicación de calculadora completamente funcional para Eclipse OS en Rust

use eframe::egui;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Calculator {
    display: String,
    operation: Option<Operation>,
    operand1: Option<f64>,
    operand2: Option<f64>,
    result: Option<f64>,
    history: VecDeque<String>,
    memory: f64,
    is_new_number: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    SquareRoot,
    Percentage,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            display: "0".to_string(),
            operation: None,
            operand1: None,
            operand2: None,
            result: None,
            history: VecDeque::new(),
            memory: 0.0,
            is_new_number: true,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("🧮 Calculadora de Eclipse OS en Rust");
        ui.separator();

        // Pantalla de la calculadora
        ui.group(|ui| {
            ui.vertical(|ui| {
                // Historial
                if !self.history.is_empty() {
                    ui.label("Historial:");
                    egui::ScrollArea::vertical()
                        .max_height(100.0)
                        .show(ui, |ui| {
                            for entry in &self.history {
                                ui.label(entry);
                            }
                        });
                    ui.separator();
                }

                // Pantalla principal
                ui.add_sized(
                    [ui.available_width(), 40.0],
                    egui::TextEdit::singleline(&mut self.display)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY)
                );
            });
        });

        ui.separator();

        // Botones de la calculadora
        ui.group(|ui| {
            ui.vertical(|ui| {
                // Fila 1: Funciones especiales
                ui.horizontal(|ui| {
                    if ui.button("C").clicked() {
                        self.clear();
                    }
                    if ui.button("CE").clicked() {
                        self.clear_entry();
                    }
                    if ui.button("⌫").clicked() {
                        self.backspace();
                    }
                    if ui.button("±").clicked() {
                        self.toggle_sign();
                    }
                });

                // Fila 2: Memoria y funciones
                ui.horizontal(|ui| {
                    if ui.button("MC").clicked() {
                        self.memory_clear();
                    }
                    if ui.button("MR").clicked() {
                        self.memory_recall();
                    }
                    if ui.button("M+").clicked() {
                        self.memory_add();
                    }
                    if ui.button("M-").clicked() {
                        self.memory_subtract();
                    }
                });

                // Fila 3: Números y operaciones básicas
                ui.horizontal(|ui| {
                    if ui.button("7").clicked() {
                        self.input_number("7");
                    }
                    if ui.button("8").clicked() {
                        self.input_number("8");
                    }
                    if ui.button("9").clicked() {
                        self.input_number("9");
                    }
                    if ui.button("÷").clicked() {
                        self.set_operation(Operation::Divide);
                    }
                });

                // Fila 4
                ui.horizontal(|ui| {
                    if ui.button("4").clicked() {
                        self.input_number("4");
                    }
                    if ui.button("5").clicked() {
                        self.input_number("5");
                    }
                    if ui.button("6").clicked() {
                        self.input_number("6");
                    }
                    if ui.button("×").clicked() {
                        self.set_operation(Operation::Multiply);
                    }
                });

                // Fila 5
                ui.horizontal(|ui| {
                    if ui.button("1").clicked() {
                        self.input_number("1");
                    }
                    if ui.button("2").clicked() {
                        self.input_number("2");
                    }
                    if ui.button("3").clicked() {
                        self.input_number("3");
                    }
                    if ui.button("-").clicked() {
                        self.set_operation(Operation::Subtract);
                    }
                });

                // Fila 6
                ui.horizontal(|ui| {
                    if ui.button("0").clicked() {
                        self.input_number("0");
                    }
                    if ui.button(".").clicked() {
                        self.input_decimal();
                    }
                    if ui.button("=").clicked() {
                        self.calculate();
                    }
                    if ui.button("+").clicked() {
                        self.set_operation(Operation::Add);
                    }
                });

                // Fila 7: Funciones avanzadas
                ui.horizontal(|ui| {
                    if ui.button("√").clicked() {
                        self.square_root();
                    }
                    if ui.button("x²").clicked() {
                        self.square();
                    }
                    if ui.button("%").clicked() {
                        self.percentage();
                    }
                    if ui.button("1/x").clicked() {
                        self.reciprocal();
                    }
                });
            });
        });

        // Información de estado
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(format!("Memoria: {}", self.memory));
            ui.label(format!("Operando 1: {:?}", self.operand1));
            ui.label(format!("Operación: {:?}", self.operation));
            ui.label(format!("Operando 2: {:?}", self.operand2));
        });
    }

    fn input_number(&mut self, digit: &str) {
        if self.is_new_number {
            self.display = digit.to_string();
            self.is_new_number = false;
        } else {
            if self.display == "0" {
                self.display = digit.to_string();
            } else {
                self.display.push_str(digit);
            }
        }
    }

    fn input_decimal(&mut self) {
        if self.is_new_number {
            self.display = "0.".to_string();
            self.is_new_number = false;
        } else if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    fn set_operation(&mut self, op: Operation) {
        if let Ok(value) = self.display.parse::<f64>() {
            if self.operand1.is_none() {
                self.operand1 = Some(value);
            } else if self.operand2.is_none() {
                self.operand2 = Some(value);
                self.calculate();
            }
            self.operation = Some(op);
            self.is_new_number = true;
        }
    }

    fn calculate(&mut self) {
        if let (Some(op1), Some(op), Some(op2)) = (self.operand1, self.operation, self.operand2) {
            let result = match op {
                Operation::Add => op1 + op2,
                Operation::Subtract => op1 - op2,
                Operation::Multiply => op1 * op2,
                Operation::Divide => {
                    if op2 != 0.0 {
                        op1 / op2
                    } else {
                        self.display = "Error: División por cero".to_string();
                        return;
                    }
                },
                Operation::Power => op1.powf(op2),
                Operation::SquareRoot => op1.sqrt(),
                Operation::Percentage => op1 * (op2 / 100.0),
            };

            self.result = Some(result);
            self.display = format_result(result);
            
            // Agregar al historial
            let history_entry = format!("{} {} {} = {}", op1, operation_symbol(op), op2, self.display);
            self.history.push_back(history_entry);
            if self.history.len() > 10 {
                self.history.pop_front();
            }

            // Preparar para siguiente operación
            self.operand1 = Some(result);
            self.operand2 = None;
            self.operation = None;
            self.is_new_number = true;
        } else if let Ok(value) = self.display.parse::<f64>() {
            self.operand1 = Some(value);
            self.is_new_number = true;
        }
    }

    fn square_root(&mut self) {
        if let Ok(value) = self.display.parse::<f64>() {
            if value >= 0.0 {
                let result = value.sqrt();
                self.display = format_result(result);
                self.add_to_history(&format!("√{} = {}", value, self.display));
                self.is_new_number = true;
            } else {
                self.display = "Error: Raíz de número negativo".to_string();
            }
        }
    }

    fn square(&mut self) {
        if let Ok(value) = self.display.parse::<f64>() {
            let result = value * value;
            self.display = format_result(result);
            self.add_to_history(&format!("{}² = {}", value, self.display));
            self.is_new_number = true;
        }
    }

    fn percentage(&mut self) {
        if let Ok(value) = self.display.parse::<f64>() {
            let result = value / 100.0;
            self.display = format_result(result);
            self.add_to_history(&format!("{}% = {}", value, self.display));
            self.is_new_number = true;
        }
    }

    fn reciprocal(&mut self) {
        if let Ok(value) = self.display.parse::<f64>() {
            if value != 0.0 {
                let result = 1.0 / value;
                self.display = format_result(result);
                self.add_to_history(&format!("1/{} = {}", value, self.display));
                self.is_new_number = true;
            } else {
                self.display = "Error: División por cero".to_string();
            }
        }
    }

    fn clear(&mut self) {
        self.display = "0".to_string();
        self.operation = None;
        self.operand1 = None;
        self.operand2 = None;
        self.result = None;
        self.is_new_number = true;
    }

    fn clear_entry(&mut self) {
        self.display = "0".to_string();
        self.is_new_number = true;
    }

    fn backspace(&mut self) {
        if self.display.len() > 1 {
            self.display.pop();
        } else {
            self.display = "0".to_string();
        }
    }

    fn toggle_sign(&mut self) {
        if let Ok(value) = self.display.parse::<f64>() {
            self.display = format_result(-value);
            self.is_new_number = true;
        }
    }

    fn memory_clear(&mut self) {
        self.memory = 0.0;
    }

    fn memory_recall(&mut self) {
        self.display = format_result(self.memory);
        self.is_new_number = true;
    }

    fn memory_add(&mut self) {
        if let Ok(value) = self.display.parse::<f64>() {
            self.memory += value;
        }
    }

    fn memory_subtract(&mut self) {
        if let Ok(value) = self.display.parse::<f64>() {
            self.memory -= value;
        }
    }

    fn add_to_history(&mut self, entry: &str) {
        self.history.push_back(entry.to_string());
        if self.history.len() > 10 {
            self.history.pop_front();
        }
    }
}

fn format_result(value: f64) -> String {
    if value == value.floor() && value.abs() < 1e10 {
        format!("{:.0}", value)
    } else {
        format!("{:.10}", value).trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn operation_symbol(op: Operation) -> &'static str {
    match op {
        Operation::Add => "+",
        Operation::Subtract => "-",
        Operation::Multiply => "×",
        Operation::Divide => "÷",
        Operation::Power => "^",
        Operation::SquareRoot => "√",
        Operation::Percentage => "%",
    }
}
