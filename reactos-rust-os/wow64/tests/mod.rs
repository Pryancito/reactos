//! Módulo de pruebas para WOW64
//!
//! Contiene todas las pruebas de integración y unitarias

pub mod integration_tests;
pub mod test_runner;

/// Re-exportar funciones principales
pub use integration_tests::run_integration_tests;
pub use test_runner::main as run_tests;
